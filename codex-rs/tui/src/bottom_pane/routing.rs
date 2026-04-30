use crossterm::event::KeyCode;
use crossterm::event::KeyEvent;
use crossterm::event::KeyEventKind;

use crate::key_hint;

use super::BottomPane;
use super::CancellationEvent;
use super::ChatComposer;
use super::InputResult;

impl BottomPane {
    /// Forward a key event to the active view or the composer.
    pub fn handle_key_event(&mut self, key_event: KeyEvent) -> InputResult {
        // Do not globally intercept space; only composer handles hold-to-talk.
        // While recording, route all keys to the composer so it can stop on release or next key.
        #[cfg(not(target_os = "linux"))]
        if self.composer.is_recording() {
            let (_ir, needs_redraw) = self.composer.handle_key_event(key_event);
            if needs_redraw {
                self.request_redraw();
            }
            return InputResult::None;
        }

        // If a modal/view is active, handle it here; otherwise forward to composer.
        if !self.view_stack.is_empty() {
            if key_event.kind == KeyEventKind::Release {
                return InputResult::None;
            }

            // We need three pieces of information after routing the key:
            // whether Esc completed the view, whether the view finished for any
            // reason, and whether a paste-burst timer should be scheduled.
            let (ctrl_c_completed, view_complete, view_in_paste_burst) = {
                let last_index = self.view_stack.len() - 1;
                let view = &mut self.view_stack[last_index];
                let prefer_esc =
                    key_event.code == KeyCode::Esc && view.prefer_esc_to_handle_key_event();
                let ctrl_c_completed = key_event.code == KeyCode::Esc
                    && !prefer_esc
                    && matches!(view.on_ctrl_c(), CancellationEvent::Handled)
                    && view.is_complete();
                if ctrl_c_completed {
                    (true, true, false)
                } else {
                    view.handle_key_event(key_event);
                    (false, view.is_complete(), view.is_in_paste_burst())
                }
            };

            if ctrl_c_completed {
                self.view_stack.pop();
                self.on_active_view_complete();
                if let Some(next_view) = self.view_stack.last()
                    && next_view.is_in_paste_burst()
                {
                    self.request_redraw_in(ChatComposer::recommended_paste_flush_delay());
                }
            } else if view_complete {
                self.view_stack.clear();
                self.on_active_view_complete();
            } else if view_in_paste_burst {
                self.request_redraw_in(ChatComposer::recommended_paste_flush_delay());
            }
            self.request_redraw();
            InputResult::None
        } else {
            // If a task is running and a status line is visible, allow Esc to
            // send an interrupt even while the composer has focus.
            // When a popup is active, prefer dismissing it over interrupting the task.
            if key_event.code == KeyCode::Esc
                && self.is_task_running
                && !self.composer.popup_active()
                && let Some(status) = &self.status
            {
                // Send Op::Interrupt
                status.interrupt();
                self.request_redraw();
                return InputResult::None;
            }
            let (input_result, needs_redraw) = self.composer.handle_key_event(key_event);
            if needs_redraw {
                self.request_redraw();
            }
            if self.composer.is_in_paste_burst() {
                self.request_redraw_in(ChatComposer::recommended_paste_flush_delay());
            }
            input_result
        }
    }

    /// Handles a Ctrl+C press within the bottom pane.
    ///
    /// An active modal view is given the first chance to consume the key (typically to dismiss
    /// itself). If no view is active, Ctrl+C clears draft composer input.
    ///
    /// This method may show the quit shortcut hint as a user-visible acknowledgement that Ctrl+C
    /// was received, but it does not decide whether the process should exit; `ChatWidget` owns the
    /// quit/interrupt state machine and uses the result to decide what happens next.
    pub(crate) fn on_ctrl_c(&mut self) -> CancellationEvent {
        if let Some(view) = self.view_stack.last_mut() {
            let event = view.on_ctrl_c();
            if matches!(event, CancellationEvent::Handled) {
                if view.is_complete() {
                    self.view_stack.pop();
                    self.on_active_view_complete();
                }
                self.show_quit_shortcut_hint(key_hint::ctrl(KeyCode::Char('c')));
                self.request_redraw();
            }
            event
        } else if self.composer_is_empty() {
            CancellationEvent::NotHandled
        } else {
            self.view_stack.pop();
            self.clear_composer_for_ctrl_c();
            self.show_quit_shortcut_hint(key_hint::ctrl(KeyCode::Char('c')));
            self.request_redraw();
            CancellationEvent::Handled
        }
    }

    pub fn handle_paste(&mut self, pasted: String) {
        if let Some(view) = self.view_stack.last_mut() {
            let needs_redraw = view.handle_paste(pasted);
            if view.is_complete() {
                self.on_active_view_complete();
            }
            if needs_redraw {
                self.request_redraw();
            }
        } else {
            let needs_redraw = self.composer.handle_paste(pasted);
            self.composer.sync_popups();
            if needs_redraw {
                self.request_redraw();
            }
        }
    }

    pub(crate) fn flush_paste_burst_if_due(&mut self) -> bool {
        // Give the active view the first chance to flush paste-burst state so
        // overlays that reuse the composer behave consistently.
        if let Some(view) = self.view_stack.last_mut()
            && view.flush_paste_burst_if_due()
        {
            return true;
        }
        self.composer.flush_paste_burst_if_due()
    }

    pub(crate) fn is_in_paste_burst(&self) -> bool {
        // A view can hold paste-burst state independently of the primary
        // composer, so check it first.
        self.view_stack
            .last()
            .is_some_and(|view| view.is_in_paste_burst())
            || self.composer.is_in_paste_burst()
    }

    fn on_active_view_complete(&mut self) {
        self.resume_status_timer_after_modal();
        self.set_composer_input_enabled(true, None);
    }
}
