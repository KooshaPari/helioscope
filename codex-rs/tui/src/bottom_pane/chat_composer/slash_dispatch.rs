use super::*;

fn slash_command_args_elements(
    rest: &str,
    rest_offset: usize,
    text_elements: &[TextElement],
) -> Vec<TextElement> {
    if rest.is_empty() || text_elements.is_empty() {
        return Vec::new();
    }
    text_elements
        .iter()
        .filter_map(|elem| {
            if elem.byte_range.end <= rest_offset {
                return None;
            }
            let start = elem.byte_range.start.saturating_sub(rest_offset);
            let mut end = elem.byte_range.end.saturating_sub(rest_offset);
            if start >= rest.len() {
                return None;
            }
            end = end.min(rest.len());
            (start < end).then_some(elem.map_range(|_| ByteRange { start, end }))
        })
        .collect()
}

impl ChatComposer {
    /// Check if the first line is a bare slash command (no args) and dispatch it.
    /// Returns Some(InputResult) if a command was dispatched, None otherwise.
    pub(super) fn try_dispatch_bare_slash_command(&mut self) -> Option<InputResult> {
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
    pub(super) fn try_dispatch_slash_command_with_args(&mut self) -> Option<InputResult> {
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
        args_elements = Self::trim_text_elements(rest, trimmed_rest, args_elements);
        Some(InputResult::CommandWithArgs(
            cmd,
            trimmed_rest.to_string(),
            args_elements,
        ))
    }

    pub(super) fn reject_slash_command_if_unavailable(&self, cmd: SlashCommand) -> bool {
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

    pub(super) fn slash_command_args_elements_for_submission(
        prepared_rest: &str,
        prepared_rest_offset: usize,
        prepared_elements: &[TextElement],
    ) -> Vec<TextElement> {
        slash_command_args_elements(prepared_rest, prepared_rest_offset, prepared_elements)
    }
}
