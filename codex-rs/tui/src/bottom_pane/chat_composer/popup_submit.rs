use super::*;

impl ChatComposer {
    pub(super) fn handle_command_popup_tab(
        &mut self,
        popup: &mut CommandPopup,
    ) -> (InputResult, bool) {
        let first_line = self.textarea.text().lines().next().unwrap_or("");
        popup.on_composer_text_change(first_line.to_string());
        if let Some(sel) = popup.selected_item() {
            let mut cursor_target: Option<usize> = None;
            match sel {
                CommandItem::Builtin(cmd) => {
                    if cmd == SlashCommand::Skills {
                        self.textarea.set_text_clearing_elements("");
                        return (InputResult::Command(cmd), true);
                    }

                    let starts_with_cmd = first_line
                        .trim_start()
                        .starts_with(&format!("/{}", cmd.command()));
                    if !starts_with_cmd {
                        self.textarea
                            .set_text_clearing_elements(&format!("/{} ", cmd.command()));
                    }
                    if !self.textarea.text().is_empty() {
                        cursor_target = Some(self.textarea.text().len());
                    }
                }
                CommandItem::UserPrompt(idx) => {
                    if let Some(prompt) = popup.prompt(idx) {
                        match prompt_selection_action(
                            prompt,
                            first_line,
                            PromptSelectionMode::Completion,
                            &self.textarea.text_elements(),
                        ) {
                            PromptSelectionAction::Insert { text, cursor } => {
                                let target = cursor.unwrap_or(text.len());
                                self.textarea.set_text_clearing_elements(&text);
                                cursor_target = Some(target);
                            }
                            PromptSelectionAction::Submit { .. } => {}
                        }
                    }
                }
            }
            if let Some(pos) = cursor_target {
                self.textarea.set_cursor(pos);
            }
        }
        (InputResult::None, true)
    }

    pub(super) fn handle_command_popup_enter(
        &mut self,
        popup: &CommandPopup,
        key_event: KeyEvent,
    ) -> (InputResult, bool) {
        if let Some(result) = self.try_submit_numeric_prompt_from_popup() {
            return (result, true);
        }

        if let Some(result) = self.submit_selected_command_popup_item(popup) {
            return (result, true);
        }

        self.handle_key_event_without_popup(key_event)
    }

    fn try_submit_numeric_prompt_from_popup(&mut self) -> Option<InputResult> {
        let mut text = self.textarea.text().to_string();
        let mut text_elements = self.textarea.text_elements();
        if !self.pending_pastes.is_empty() {
            let (expanded, expanded_elements) =
                Self::expand_pending_pastes(&text, text_elements, &self.pending_pastes);
            text = expanded;
            text_elements = expanded_elements;
        }
        let first_line = text.lines().next().unwrap_or("");
        if let Some((name, _rest, _rest_offset)) = parse_slash_name(first_line)
            && let Some(prompt_name) = name.strip_prefix(&format!("{PROMPTS_CMD_PREFIX}:"))
            && let Some(prompt) = self.custom_prompts.iter().find(|p| p.name == prompt_name)
            && let Some(expanded) =
                expand_if_numeric_with_positional_args(prompt, first_line, &text_elements)
        {
            self.prune_attached_images_for_submission(&expanded.text, &expanded.text_elements);
            self.pending_pastes.clear();
            self.textarea.set_text_clearing_elements("");
            Some(InputResult::Submitted {
                text: expanded.text,
                text_elements: expanded.text_elements,
            })
        } else {
            None
        }
    }

    fn submit_selected_command_popup_item(&mut self, popup: &CommandPopup) -> Option<InputResult> {
        let first_line = self
            .textarea
            .text()
            .lines()
            .next()
            .unwrap_or("")
            .to_string();
        let selection = popup.selected_item()?;
        match selection {
            CommandItem::Builtin(cmd) => {
                self.textarea.set_text_clearing_elements("");
                Some(InputResult::Command(cmd))
            }
            CommandItem::UserPrompt(idx) => {
                let prompt = popup.prompt(idx)?;
                match prompt_selection_action(
                    prompt,
                    &first_line,
                    PromptSelectionMode::Submit,
                    &self.textarea.text_elements(),
                ) {
                    PromptSelectionAction::Submit {
                        text,
                        text_elements,
                    } => {
                        self.prune_attached_images_for_submission(&text, &text_elements);
                        self.textarea.set_text_clearing_elements("");
                        Some(InputResult::Submitted {
                            text,
                            text_elements,
                        })
                    }
                    PromptSelectionAction::Insert { text, cursor } => {
                        let target = cursor.unwrap_or(text.len());
                        self.textarea.set_text_clearing_elements(&text);
                        self.textarea.set_cursor(target);
                        Some(InputResult::None)
                    }
                }
            }
        }
    }
}
