use super::super::*;
use codex_protocol::items::AgentMessageItem;
use codex_protocol::models::MessagePhase;

impl ChatWidget {
    pub(in crate::chatwidget) fn on_agent_message(&mut self, message: String) {
        if self.stream_controller.is_none() && !message.is_empty() {
            self.handle_streaming_delta(message);
        }
        self.flush_answer_stream_with_separator();
        self.handle_stream_finished();
        self.request_redraw();
    }

    /// Handle completion of an `AgentMessage` turn item.
    ///
    /// Commentary completion sets a deferred restore flag so the status row
    /// returns once stream queues are idle. Final-answer completion (or absent
    /// phase for legacy models) clears the flag to preserve historical behavior.
    pub(in crate::chatwidget) fn on_agent_message_item_completed(
        &mut self,
        item: AgentMessageItem,
    ) {
        self.pending_status_indicator_restore = match item.phase {
            Some(MessagePhase::FinalAnswer) | None => false,
            Some(MessagePhase::Commentary) => true,
        };
        self.maybe_restore_status_indicator_after_stream_idle();
    }

    /// Periodic tick for stream commits. In smooth mode this preserves one-line pacing, while
    /// catch-up mode drains larger batches to reduce queue lag.
    pub(crate) fn on_commit_tick(&mut self) {
        self.run_commit_tick();
    }

    /// Runs a regular periodic commit tick.
    fn run_commit_tick(&mut self) {
        self.run_commit_tick_with_scope(CommitTickScope::AnyMode);
    }

    /// Runs an opportunistic commit tick only if catch-up mode is active.
    fn run_catch_up_commit_tick(&mut self) {
        self.run_commit_tick_with_scope(CommitTickScope::CatchUpOnly);
    }

    /// Runs a commit tick for the current stream queue snapshot.
    ///
    /// `scope` controls whether this call may commit in smooth mode or only when catch-up
    /// is currently active. While lines are actively streaming we hide the status row to avoid
    /// duplicate "in progress" affordances. Restoration is gated separately so we only re-show
    /// the row after commentary completion once stream queues are idle.
    fn run_commit_tick_with_scope(&mut self, scope: CommitTickScope) {
        let now = Instant::now();
        let outcome = run_commit_tick(
            &mut self.adaptive_chunking,
            self.stream_controller.as_mut(),
            self.plan_stream_controller.as_mut(),
            scope,
            now,
        );
        for cell in outcome.cells {
            self.bottom_pane.hide_status_indicator();
            self.add_boxed_history(cell);
        }

        if outcome.has_controller && outcome.all_idle {
            self.maybe_restore_status_indicator_after_stream_idle();
            self.app_event_tx.send(AppEvent::StopCommitAnimation);
        }

        if self.agent_turn_running {
            self.refresh_runtime_metrics();
        }
    }

    pub(in crate::chatwidget) fn handle_stream_finished(&mut self) {
        if self.task_complete_pending {
            self.bottom_pane.hide_status_indicator();
            self.task_complete_pending = false;
        }
        self.flush_interrupt_queue();
    }

    #[inline]
    pub(in crate::chatwidget) fn handle_streaming_delta(&mut self, delta: String) {
        self.flush_unified_exec_wait_streak();
        self.flush_active_cell();

        if self.stream_controller.is_none() {
            if self.needs_final_message_separator && self.had_work_activity {
                let elapsed_seconds = self
                    .bottom_pane
                    .status_widget()
                    .map(crate::status_indicator_widget::StatusIndicatorWidget::elapsed_seconds)
                    .map(|current| self.worked_elapsed_from(current));
                self.add_to_history(history_cell::FinalMessageSeparator::new(
                    elapsed_seconds,
                    None,
                ));
                self.needs_final_message_separator = false;
                self.had_work_activity = false;
            } else if self.needs_final_message_separator {
                self.needs_final_message_separator = false;
            }
            self.stream_controller = Some(StreamController::new(
                self.last_rendered_width.get().map(|w| w.saturating_sub(2)),
            ));
        }
        if let Some(controller) = self.stream_controller.as_mut()
            && controller.push(&delta)
        {
            self.app_event_tx.send(AppEvent::StartCommitAnimation);
            self.run_catch_up_commit_tick();
        }
        self.request_redraw();
    }

    pub(in crate::chatwidget) fn on_agent_message_delta(&mut self, delta: String) {
        self.handle_streaming_delta(delta);
    }

    pub(in crate::chatwidget) fn on_plan_delta(&mut self, delta: String) {
        if self.active_mode_kind() != ModeKind::Plan {
            return;
        }
        if !self.plan_item_active {
            self.plan_item_active = true;
            self.plan_delta_buffer.clear();
        }
        self.plan_delta_buffer.push_str(&delta);
        self.flush_unified_exec_wait_streak();
        self.flush_active_cell();

        if self.plan_stream_controller.is_none() {
            self.plan_stream_controller = Some(PlanStreamController::new(
                self.last_rendered_width.get().map(|w| w.saturating_sub(4)),
            ));
        }
        if let Some(controller) = self.plan_stream_controller.as_mut()
            && controller.push(&delta)
        {
            self.app_event_tx.send(AppEvent::StartCommitAnimation);
            self.run_catch_up_commit_tick();
        }
        self.request_redraw();
    }

    pub(in crate::chatwidget) fn on_plan_item_completed(&mut self, text: String) {
        let streamed_plan = self.plan_delta_buffer.trim().to_string();
        let plan_text = if text.trim().is_empty() {
            streamed_plan
        } else {
            text
        };
        if !plan_text.trim().is_empty() {
            self.last_copyable_output = Some(plan_text.clone());
        }
        let should_restore_after_stream = self.plan_stream_controller.is_some();
        self.plan_delta_buffer.clear();
        self.plan_item_active = false;
        self.saw_plan_item_this_turn = true;
        let finalized_streamed_cell =
            if let Some(mut controller) = self.plan_stream_controller.take() {
                controller.finalize()
            } else {
                None
            };
        if let Some(cell) = finalized_streamed_cell {
            self.add_boxed_history(cell);
        } else if !plan_text.is_empty() {
            self.add_to_history(history_cell::new_proposed_plan(plan_text));
        }
        if should_restore_after_stream {
            self.pending_status_indicator_restore = true;
            self.maybe_restore_status_indicator_after_stream_idle();
        }
    }

    pub(in crate::chatwidget) fn on_agent_reasoning_delta(&mut self, delta: String) {
        self.reasoning_buffer.push_str(&delta);

        if self.unified_exec_wait_streak.is_some() {
            self.request_redraw();
            return;
        }

        if let Some(header) = extract_first_bold(&self.reasoning_buffer) {
            self.set_status_header(header);
        }
        self.request_redraw();
    }

    pub(in crate::chatwidget) fn on_agent_reasoning_final(&mut self) {
        self.full_reasoning_buffer.push_str(&self.reasoning_buffer);
        if !self.full_reasoning_buffer.is_empty() {
            let cell =
                history_cell::new_reasoning_summary_block(self.full_reasoning_buffer.clone());
            self.add_boxed_history(cell);
        }
        self.reasoning_buffer.clear();
        self.full_reasoning_buffer.clear();
        self.request_redraw();
    }

    pub(in crate::chatwidget) fn on_reasoning_section_break(&mut self) {
        self.full_reasoning_buffer.push_str(&self.reasoning_buffer);
        self.full_reasoning_buffer.push_str("\n\n");
        self.reasoning_buffer.clear();
    }

    pub(in crate::chatwidget) fn on_plan_update(&mut self, update: UpdatePlanArgs) {
        self.saw_plan_update_this_turn = true;
        self.add_to_history(history_cell::new_plan_update(update));
    }
}
