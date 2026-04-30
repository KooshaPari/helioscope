use super::*;

impl ChatWidget {
    pub(crate) fn on_task_started(&mut self) {
        self.agent_turn_running = true;
        self.turn_sleep_inhibitor.set_turn_running(true);
        self.saw_plan_update_this_turn = false;
        self.saw_plan_item_this_turn = false;
        self.plan_delta_buffer.clear();
        self.plan_item_active = false;
        self.adaptive_chunking.reset();
        self.plan_stream_controller = None;
        self.turn_runtime_metrics = RuntimeMetricsSummary::default();
        self.otel_manager.reset_runtime_metrics();
        self.bottom_pane.clear_quit_shortcut_hint();
        self.quit_shortcut_expires_at = None;
        self.quit_shortcut_key = None;
        self.update_task_running_state();
        self.retry_status_header = None;
        self.pending_status_indicator_restore = false;
        self.bottom_pane.set_interrupt_hint_visible(true);
        self.set_status_header(String::from("Working"));
        self.full_reasoning_buffer.clear();
        self.reasoning_buffer.clear();
        self.request_redraw();
    }

    pub(super) fn on_task_complete(
        &mut self,
        last_agent_message: Option<String>,
        from_replay: bool,
    ) {
        if let Some(message) = last_agent_message.as_ref()
            && !message.trim().is_empty()
        {
            self.last_copyable_output = Some(message.clone());
        }
        self.flush_answer_stream_with_separator();
        if let Some(mut controller) = self.plan_stream_controller.take()
            && let Some(cell) = controller.finalize()
        {
            self.add_boxed_history(cell);
        }
        self.flush_unified_exec_wait_streak();
        if !from_replay {
            self.collect_runtime_metrics_delta();
            let runtime_metrics =
                (!self.turn_runtime_metrics.is_empty()).then_some(self.turn_runtime_metrics);
            let show_work_separator = self.needs_final_message_separator && self.had_work_activity;
            if show_work_separator || runtime_metrics.is_some() {
                let elapsed_seconds = if show_work_separator {
                    self.bottom_pane
                        .status_widget()
                        .map(crate::status_indicator_widget::StatusIndicatorWidget::elapsed_seconds)
                        .map(|current| self.worked_elapsed_from(current))
                } else {
                    None
                };
                self.add_to_history(history_cell::FinalMessageSeparator::new(
                    elapsed_seconds,
                    runtime_metrics,
                ));
            }
            self.turn_runtime_metrics = RuntimeMetricsSummary::default();
            self.needs_final_message_separator = false;
            self.had_work_activity = false;
            self.request_status_line_branch_refresh();
        }
        self.pending_status_indicator_restore = false;
        self.agent_turn_running = false;
        self.turn_sleep_inhibitor.set_turn_running(false);
        self.update_task_running_state();
        self.running_commands.clear();
        self.suppressed_exec_calls.clear();
        self.last_unified_wait = None;
        self.unified_exec_wait_streak = None;
        self.request_redraw();

        if !from_replay && self.queued_user_messages.is_empty() {
            self.maybe_prompt_plan_implementation();
        }
        if !from_replay {
            self.saw_plan_item_this_turn = false;
        }
        self.maybe_send_next_queued_input();
        self.notify(Notification::AgentTurnComplete {
            response: last_agent_message.unwrap_or_default(),
        });

        self.maybe_show_pending_rate_limit_prompt();
    }

    /// Handle a turn aborted due to user interrupt (Esc).
    /// When there are queued user messages, restore them into the composer
    /// separated by newlines rather than auto-submitting the next one.
    pub(in crate::chatwidget) fn on_interrupted_turn(&mut self, reason: TurnAbortReason) {
        self.finalize_turn();
        if reason == TurnAbortReason::Interrupted {
            self.clear_unified_exec_processes();
        }

        if reason != TurnAbortReason::ReviewEnded {
            self.add_to_history(history_cell::new_error_event(
                "Conversation interrupted - tell the model what to do differently. Something went wrong? Hit `/feedback` to report the issue.".to_owned(),
            ));
        }

        if let Some(combined) = self.drain_queued_messages_for_restore() {
            self.restore_user_message_to_composer(combined);
            self.refresh_queued_user_messages();
        }

        self.request_redraw();
    }

    /// Merge queued drafts (plus the current composer state) into a single message for restore.
    ///
    /// Each queued draft numbers attachments from `[Image #1]`. When we concatenate drafts, we
    /// must renumber placeholders in a stable order so the merged attachment list stays aligned
    /// with the labels embedded in text. This helper drains the queue, remaps placeholders, and
    /// fixes text element byte ranges as content is appended. Returns `None` when there is nothing
    /// to restore.
    fn drain_queued_messages_for_restore(&mut self) -> Option<UserMessage> {
        if self.queued_user_messages.is_empty() {
            return None;
        }

        let existing_message = UserMessage {
            text: self.bottom_pane.composer_text(),
            text_elements: self.bottom_pane.composer_text_elements(),
            local_images: self.bottom_pane.composer_local_images(),
            remote_image_urls: self.bottom_pane.remote_image_urls(),
            mention_bindings: self.bottom_pane.composer_mention_bindings(),
        };

        let mut to_merge: Vec<UserMessage> = self.queued_user_messages.drain(..).collect();
        if !existing_message.text.is_empty()
            || !existing_message.local_images.is_empty()
            || !existing_message.remote_image_urls.is_empty()
        {
            to_merge.push(existing_message);
        }

        let mut combined = UserMessage {
            text: String::new(),
            text_elements: Vec::new(),
            local_images: Vec::new(),
            remote_image_urls: Vec::new(),
            mention_bindings: Vec::new(),
        };
        let mut combined_offset = 0usize;
        let total_remote_images = to_merge
            .iter()
            .map(|message| message.remote_image_urls.len())
            .sum::<usize>();
        let mut next_image_label = total_remote_images + 1;

        for (idx, message) in to_merge.into_iter().enumerate() {
            if idx > 0 {
                combined.text.push('\n');
                combined_offset += 1;
            }
            let message = remap_placeholders_for_message(message, &mut next_image_label);
            let base = combined_offset;
            combined.text.push_str(&message.text);
            combined_offset += message.text.len();
            combined
                .text_elements
                .extend(message.text_elements.into_iter().map(|mut elem| {
                    elem.byte_range.start += base;
                    elem.byte_range.end += base;
                    elem
                }));
            combined.local_images.extend(message.local_images);
            combined.remote_image_urls.extend(message.remote_image_urls);
            combined.mention_bindings.extend(message.mention_bindings);
        }

        Some(combined)
    }

    // If idle and there are queued inputs, submit exactly one to start the next turn.
    pub(in crate::chatwidget) fn maybe_send_next_queued_input(&mut self) {
        if self.bottom_pane.is_task_running() {
            return;
        }
        if let Some(user_message) = self.queued_user_messages.pop_front() {
            self.submit_user_message(user_message);
        }
        self.refresh_queued_user_messages();
    }

    /// Rebuild and update the queued user messages from the current queue.
    pub(in crate::chatwidget) fn refresh_queued_user_messages(&mut self) {
        let messages: Vec<String> = self
            .queued_user_messages
            .iter()
            .map(|m| m.text.clone())
            .collect();
        self.bottom_pane.set_queued_user_messages(messages);
    }

    fn maybe_prompt_plan_implementation(&mut self) {
        if !self.collaboration_modes_enabled() {
            return;
        }
        if !self.queued_user_messages.is_empty() {
            return;
        }
        if self.active_mode_kind() != ModeKind::Plan {
            return;
        }
        if !self.saw_plan_item_this_turn {
            return;
        }
        if !self.bottom_pane.no_modal_or_popup_active() {
            return;
        }

        if matches!(
            self.rate_limit_switch_prompt,
            RateLimitSwitchPromptState::Pending
        ) {
            return;
        }

        self.open_plan_implementation_prompt();
    }

    pub(crate) fn open_plan_implementation_prompt(&mut self) {
        let default_mask = collaboration_modes::default_mode_mask(self.models_manager.as_ref());
        let (implement_actions, implement_disabled_reason) = match default_mask {
            Some(mask) => {
                let user_text = PLAN_IMPLEMENTATION_CODING_MESSAGE.to_string();
                let actions: Vec<SelectionAction> = vec![Box::new(move |tx| {
                    tx.send(AppEvent::SubmitUserMessageWithMode {
                        text: user_text.clone(),
                        collaboration_mode: mask.clone(),
                    });
                })];
                (actions, None)
            }
            None => (Vec::new(), Some("Default mode unavailable".to_string())),
        };
        let items = vec![
            SelectionItem {
                name: PLAN_IMPLEMENTATION_YES.to_string(),
                description: Some("Switch to Default and start coding.".to_string()),
                selected_description: None,
                is_current: false,
                actions: implement_actions,
                disabled_reason: implement_disabled_reason,
                dismiss_on_select: true,
                ..Default::default()
            },
            SelectionItem {
                name: PLAN_IMPLEMENTATION_NO.to_string(),
                description: Some("Continue planning with the model.".to_string()),
                selected_description: None,
                is_current: false,
                actions: Vec::new(),
                dismiss_on_select: true,
                ..Default::default()
            },
        ];

        self.bottom_pane.show_selection_view(SelectionViewParams {
            title: Some(PLAN_IMPLEMENTATION_TITLE.to_string()),
            subtitle: None,
            footer_hint: Some(standard_popup_hint_line()),
            items,
            ..Default::default()
        });
    }

    /// Finalize any active exec as failed and stop/clear agent-turn UI state.
    ///
    /// This does not clear MCP startup tracking, because MCP startup can overlap with turn cleanup
    /// and should continue to drive the bottom-pane running indicator while it is in progress.
    pub(super) fn finalize_turn(&mut self) {
        self.finalize_active_cell_as_failed();
        self.agent_turn_running = false;
        self.turn_sleep_inhibitor.set_turn_running(false);
        self.update_task_running_state();
        self.running_commands.clear();
        self.suppressed_exec_calls.clear();
        self.last_unified_wait = None;
        self.unified_exec_wait_streak = None;
        self.adaptive_chunking.reset();
        self.stream_controller = None;
        self.plan_stream_controller = None;
        self.pending_status_indicator_restore = false;
        self.request_status_line_branch_refresh();
        self.maybe_show_pending_rate_limit_prompt();
    }

    pub(super) fn on_server_overloaded_error(&mut self, message: String) {
        self.finalize_turn();

        let message = if message.trim().is_empty() {
            "Codex is currently experiencing high load.".to_string()
        } else {
            message
        };

        self.add_to_history(history_cell::new_warning_event(message));
        self.request_redraw();
        self.maybe_send_next_queued_input();
    }

    pub(super) fn on_error(&mut self, message: String) {
        self.finalize_turn();
        self.add_to_history(history_cell::new_error_event(message));
        self.request_redraw();
        self.maybe_send_next_queued_input();
    }
}
