use super::*;

mod draft;
mod slash_submission;
use self::draft::PreparedSubmission;
use self::draft::prepare_submission_draft;
use self::slash_submission::SlashCommandLookupOptions;
use self::slash_submission::is_known_slash_command_name;
use self::slash_submission::slash_command_args_elements;
use self::slash_submission::unrecognized_slash_command_message;

impl ChatComposer {
    /// Prepare text for submission/queuing. Returns None if submission should be suppressed.
    /// On success, clears pending paste payloads because placeholders have been expanded.
    ///
    /// When `record_history` is true, the final submission is stored for ↑/↓ recall.
    pub(crate) fn prepare_submission_text(
        &mut self,
        record_history: bool,
    ) -> Option<(String, Vec<TextElement>)> {
        let restore_state = self.capture_submission_restore_state();
        let input_starts_with_space = restore_state.input.starts_with(' ');
        let PreparedSubmission {
            mut text,
            mut text_elements,
        } = prepare_submission_draft(
            &restore_state.input,
            restore_state.text_elements.clone(),
            &self.pending_pastes,
        );
        self.recent_submission_mention_bindings.clear();
        self.textarea.set_text_clearing_elements("");

        if self.slash_commands_enabled()
            && let Some((name, _rest, _rest_offset)) = parse_slash_name(&text)
        {
            let treat_as_plain_text = input_starts_with_space || name.contains('/');
            let lookup_options = SlashCommandLookupOptions {
                collaboration_modes_enabled: self.collaboration_modes_enabled,
                connectors_enabled: self.connectors_enabled,
                personality_command_enabled: self.personality_command_enabled,
                realtime_conversation_enabled: self.realtime_conversation_enabled,
                audio_device_selection_enabled: self.audio_device_selection_enabled,
                windows_degraded_sandbox_active: self.windows_degraded_sandbox_active,
            };
            if !treat_as_plain_text
                && !is_known_slash_command_name(name, &self.custom_prompts, lookup_options)
            {
                self.app_event_tx.send(AppEvent::InsertHistoryCell(Box::new(
                    history_cell::new_info_event(unrecognized_slash_command_message(name), None),
                )));
                self.restore_submission_restore_state(&restore_state);
                return None;
            }
        }

        if self.slash_commands_enabled() {
            let expanded_prompt =
                match expand_custom_prompt(&text, &text_elements, &self.custom_prompts) {
                    Ok(expanded) => expanded,
                    Err(err) => {
                        self.app_event_tx.send(AppEvent::InsertHistoryCell(Box::new(
                            history_cell::new_error_event(err.user_message()),
                        )));
                        self.restore_submission_restore_state(&restore_state);
                        return None;
                    }
                };
            if let Some(expanded) = expanded_prompt {
                text = expanded.text;
                text_elements = expanded.text_elements;
            }
        }
        let actual_chars = text.chars().count();
        if actual_chars > MAX_USER_INPUT_TEXT_CHARS {
            let message = user_input_too_large_message(actual_chars);
            self.app_event_tx.send(AppEvent::InsertHistoryCell(Box::new(
                history_cell::new_error_event(message),
            )));
            self.restore_submission_restore_state(&restore_state);
            return None;
        }
        chat_composer_images::prune_attached_images_for_submission(
            &mut self.attached_images,
            &text,
            &text_elements,
        );
        if text.is_empty() && self.attached_images.is_empty() && self.remote_image_urls.is_empty() {
            return None;
        }
        self.recent_submission_mention_bindings = restore_state.mention_bindings.clone();
        if record_history
            && (!text.is_empty()
                || !self.attached_images.is_empty()
                || !self.remote_image_urls.is_empty())
        {
            let local_image_paths = self
                .attached_images
                .iter()
                .map(|img| img.path.clone())
                .collect();
            self.history.record_local_submission(HistoryEntry {
                text: text.clone(),
                text_elements: text_elements.clone(),
                local_image_paths,
                remote_image_urls: self.remote_image_urls.clone(),
                mention_bindings: restore_state.mention_bindings,
                pending_pastes: Vec::new(),
            });
        }
        self.pending_pastes.clear();
        Some((text, text_elements))
    }

    /// Common logic for handling message submission/queuing.
    /// Returns the appropriate InputResult based on `should_queue`.
    pub(super) fn handle_submission(&mut self, should_queue: bool) -> (InputResult, bool) {
        self.handle_submission_with_time(should_queue, Instant::now())
    }

    pub(crate) fn handle_submission_with_time(
        &mut self,
        should_queue: bool,
        now: Instant,
    ) -> (InputResult, bool) {
        if let Some(result) = self.try_dispatch_bare_slash_command() {
            return (result, true);
        }

        let in_slash_context = self.slash_commands_enabled()
            && (matches!(self.active_popup, ActivePopup::Command(_))
                || self
                    .textarea
                    .text()
                    .lines()
                    .next()
                    .unwrap_or("")
                    .starts_with('/'));
        if !self.disable_paste_burst
            && self.paste_burst.is_active()
            && !in_slash_context
            && self.paste_burst.append_newline_if_active(now)
        {
            return (InputResult::None, true);
        }

        if !in_slash_context
            && !self.disable_paste_burst
            && self
                .paste_burst
                .newline_should_insert_instead_of_submit(now)
        {
            self.textarea.insert_str("\n");
            self.paste_burst.extend_window(now);
            return (InputResult::None, true);
        }

        let restore_state = self.capture_submission_restore_state();
        if let Some(result) = self.try_dispatch_slash_command_with_args() {
            return (result, true);
        }

        if let Some((text, text_elements)) = self.prepare_submission_text(true) {
            if should_queue {
                (
                    InputResult::Queued {
                        text,
                        text_elements,
                    },
                    true,
                )
            } else {
                (
                    InputResult::Submitted {
                        text,
                        text_elements,
                    },
                    true,
                )
            }
        } else {
            self.restore_submission_restore_state(&restore_state);
            (InputResult::None, true)
        }
    }

    /// Check if the first line is a bare slash command (no args) and dispatch it.
    /// Returns Some(InputResult) if a command was dispatched, None otherwise.
    fn try_dispatch_bare_slash_command(&mut self) -> Option<InputResult> {
        if !self.slash_commands_enabled() {
            return None;
        }
        let first_line = self.textarea.text().lines().next().unwrap_or("");
        if let Some((name, rest, _rest_offset)) = parse_slash_name(first_line)
            && rest.is_empty()
            && let Some(cmd) = slash_commands::find_builtin_command(
                name,
                self.collaboration_modes_enabled,
                self.connectors_enabled,
                self.personality_command_enabled,
                self.realtime_conversation_enabled,
                self.audio_device_selection_enabled,
                self.windows_degraded_sandbox_active,
            )
        {
            if self.reject_slash_command_if_unavailable(cmd) {
                return Some(InputResult::None);
            }
            self.textarea.set_text_clearing_elements("");
            Some(InputResult::Command(cmd))
        } else {
            None
        }
    }

    /// Check if the input is a slash command with args (e.g., /review args) and dispatch it.
    /// Returns Some(InputResult) if a command was dispatched, None otherwise.
    fn try_dispatch_slash_command_with_args(&mut self) -> Option<InputResult> {
        if !self.slash_commands_enabled() {
            return None;
        }
        let text = self.textarea.text().to_string();
        if text.starts_with(' ') {
            return None;
        }

        let (name, rest, rest_offset) = parse_slash_name(&text)?;
        if rest.is_empty() || name.contains('/') {
            return None;
        }

        let cmd = slash_commands::find_builtin_command(
            name,
            self.collaboration_modes_enabled,
            self.connectors_enabled,
            self.personality_command_enabled,
            self.realtime_conversation_enabled,
            self.audio_device_selection_enabled,
            self.windows_degraded_sandbox_active,
        )?;

        if !cmd.supports_inline_args() {
            return None;
        }
        if self.reject_slash_command_if_unavailable(cmd) {
            return Some(InputResult::None);
        }

        let mut args_elements =
            slash_command_args_elements(rest, rest_offset, &self.textarea.text_elements());
        let trimmed_rest = rest.trim();
        args_elements = text_manipulation::trim_text_elements(rest, trimmed_rest, args_elements);
        Some(InputResult::CommandWithArgs(
            cmd,
            trimmed_rest.to_string(),
            args_elements,
        ))
    }

    /// Expand pending placeholders and extract normalized inline-command args.
    ///
    /// Inline-arg commands are initially dispatched using the raw draft so command rejection does
    /// not consume user input. Once a command is accepted, this helper performs the usual
    /// submission preparation (paste expansion, element trimming) and rebases element ranges from
    /// full-text offsets to command-arg offsets.
    pub(crate) fn prepare_inline_args_submission(
        &mut self,
        record_history: bool,
    ) -> Option<(String, Vec<TextElement>)> {
        let (prepared_text, prepared_elements) = self.prepare_submission_text(record_history)?;
        let (_, prepared_rest, prepared_rest_offset) = parse_slash_name(&prepared_text)?;
        let mut args_elements =
            slash_command_args_elements(prepared_rest, prepared_rest_offset, &prepared_elements);
        let trimmed_rest = prepared_rest.trim();
        args_elements =
            text_manipulation::trim_text_elements(prepared_rest, trimmed_rest, args_elements);
        Some((trimmed_rest.to_string(), args_elements))
    }

    fn reject_slash_command_if_unavailable(&self, cmd: SlashCommand) -> bool {
        if !self.is_task_running || cmd.available_during_task() {
            return false;
        }
        let message = format!(
            "'/{}' is disabled while a task is in progress.",
            cmd.command()
        );
        self.app_event_tx.send(AppEvent::InsertHistoryCell(Box::new(
            history_cell::new_error_event(message),
        )));
        true
    }
}
