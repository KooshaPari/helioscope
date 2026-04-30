use crate::app_event::AppEvent;
use crate::app_event_sender::AppEventSender;
use crate::bottom_pane::BottomPaneView;
use crate::bottom_pane::CancellationEvent;
use crate::bottom_pane::list_selection_view::ListSelectionView;
use crate::history_cell;
use crate::render::renderable::Renderable;
use codex_core::features::Features;
use codex_protocol::mcp::RequestId;
use codex_protocol::protocol::ElicitationAction;
use codex_protocol::protocol::Op;
use codex_protocol::protocol::ReviewDecision;
use crossterm::event::KeyCode;
use crossterm::event::KeyEvent;
use crossterm::event::KeyEventKind;
use crossterm::event::KeyModifiers;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;

mod header;
mod options;
mod request;

use header::build_header;
use options::ApprovalDecision;
use options::ApprovalOption;
use options::build_options;
pub(crate) use request::ApprovalRequest;

/// Modal overlay asking the user to approve or deny one or more requests.
pub(crate) struct ApprovalOverlay {
    current_request: Option<ApprovalRequest>,
    queue: Vec<ApprovalRequest>,
    app_event_tx: AppEventSender,
    list: ListSelectionView,
    options: Vec<ApprovalOption>,
    current_complete: bool,
    done: bool,
    features: Features,
}

impl ApprovalOverlay {
    pub fn new(request: ApprovalRequest, app_event_tx: AppEventSender, features: Features) -> Self {
        let mut view = Self {
            current_request: None,
            queue: Vec::new(),
            app_event_tx: app_event_tx.clone(),
            list: ListSelectionView::new(Default::default(), app_event_tx),
            options: Vec::new(),
            current_complete: false,
            done: false,
            features,
        };
        view.set_current(request);
        view
    }

    pub fn enqueue_request(&mut self, req: ApprovalRequest) {
        self.queue.push(req);
    }

    fn set_current(&mut self, request: ApprovalRequest) {
        self.current_complete = false;
        let header = build_header(&request);
        let (options, params) = build_options(&request, header, &self.features);
        self.current_request = Some(request);
        self.options = options;
        self.list = ListSelectionView::new(params, self.app_event_tx.clone());
    }

    fn apply_selection(&mut self, actual_idx: usize) {
        if self.current_complete {
            return;
        }
        let Some(option) = self.options.get(actual_idx) else {
            return;
        };
        if let Some(request) = self.current_request.as_ref() {
            match (request, &option.decision) {
                (ApprovalRequest::Exec { id, command, .. }, ApprovalDecision::Review(decision)) => {
                    self.handle_exec_decision(id, command, decision.clone());
                }
                (ApprovalRequest::ApplyPatch { id, .. }, ApprovalDecision::Review(decision)) => {
                    self.handle_patch_decision(id, decision.clone());
                }
                (
                    ApprovalRequest::McpElicitation {
                        server_name,
                        request_id,
                        ..
                    },
                    ApprovalDecision::McpElicitation(decision),
                ) => {
                    self.handle_elicitation_decision(server_name, request_id, *decision);
                }
                _ => {}
            }
        }

        self.current_complete = true;
        self.advance_queue();
    }

    fn handle_exec_decision(&self, id: &str, command: &[String], decision: ReviewDecision) {
        let Some(request) = self.current_request.as_ref() else {
            return;
        };
        if request.thread_label().is_none() {
            let cell = history_cell::new_approval_decision_cell(command.to_vec(), decision.clone());
            self.app_event_tx.send(AppEvent::InsertHistoryCell(cell));
        }
        let thread_id = request.thread_id();
        self.app_event_tx.send(AppEvent::SubmitThreadOp {
            thread_id,
            op: Op::ExecApproval {
                id: id.to_string(),
                turn_id: None,
                decision,
            },
        });
    }

    fn handle_patch_decision(&self, id: &str, decision: ReviewDecision) {
        let Some(thread_id) = self
            .current_request
            .as_ref()
            .map(ApprovalRequest::thread_id)
        else {
            return;
        };
        self.app_event_tx.send(AppEvent::SubmitThreadOp {
            thread_id,
            op: Op::PatchApproval {
                id: id.to_string(),
                decision,
            },
        });
    }

    fn handle_elicitation_decision(
        &self,
        server_name: &str,
        request_id: &RequestId,
        decision: ElicitationAction,
    ) {
        let Some(thread_id) = self
            .current_request
            .as_ref()
            .map(ApprovalRequest::thread_id)
        else {
            return;
        };
        self.app_event_tx.send(AppEvent::SubmitThreadOp {
            thread_id,
            op: Op::ResolveElicitation {
                server_name: server_name.to_string(),
                request_id: request_id.clone(),
                decision,
            },
        });
    }

    fn advance_queue(&mut self) {
        if let Some(next) = self.queue.pop() {
            self.set_current(next);
        } else {
            self.done = true;
        }
    }

    fn try_handle_shortcut(&mut self, key_event: &KeyEvent) -> bool {
        match key_event {
            KeyEvent {
                kind: KeyEventKind::Press,
                code: KeyCode::Char('a'),
                modifiers,
                ..
            } if modifiers.contains(KeyModifiers::CONTROL) => {
                if let Some(request) = self.current_request.as_ref() {
                    self.app_event_tx
                        .send(AppEvent::FullScreenApprovalRequest(request.clone()));
                    true
                } else {
                    false
                }
            }
            KeyEvent {
                kind: KeyEventKind::Press,
                code: KeyCode::Char('o'),
                ..
            } => {
                if let Some(request) = self.current_request.as_ref() {
                    if request.thread_label().is_some() {
                        self.app_event_tx
                            .send(AppEvent::SelectAgentThread(request.thread_id()));
                        true
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
            e => {
                if let Some(idx) = self
                    .options
                    .iter()
                    .position(|opt| opt.shortcuts().any(|s| s.is_press(*e)))
                {
                    self.apply_selection(idx);
                    true
                } else {
                    false
                }
            }
        }
    }
}

impl BottomPaneView for ApprovalOverlay {
    fn handle_key_event(&mut self, key_event: KeyEvent) {
        if self.try_handle_shortcut(&key_event) {
            return;
        }
        self.list.handle_key_event(key_event);
        if let Some(idx) = self.list.take_last_selected_index() {
            self.apply_selection(idx);
        }
    }

    fn on_ctrl_c(&mut self) -> CancellationEvent {
        if self.done {
            return CancellationEvent::Handled;
        }
        if !self.current_complete
            && let Some(request) = self.current_request.as_ref()
        {
            match request {
                ApprovalRequest::Exec { id, command, .. } => {
                    self.handle_exec_decision(id, command, ReviewDecision::Abort);
                }
                ApprovalRequest::ApplyPatch { id, .. } => {
                    self.handle_patch_decision(id, ReviewDecision::Abort);
                }
                ApprovalRequest::McpElicitation {
                    server_name,
                    request_id,
                    ..
                } => {
                    self.handle_elicitation_decision(
                        server_name,
                        request_id,
                        ElicitationAction::Cancel,
                    );
                }
            }
        }
        self.queue.clear();
        self.done = true;
        CancellationEvent::Handled
    }

    fn is_complete(&self) -> bool {
        self.done
    }

    fn try_consume_approval_request(
        &mut self,
        request: ApprovalRequest,
    ) -> Option<ApprovalRequest> {
        self.enqueue_request(request);
        None
    }
}

impl Renderable for ApprovalOverlay {
    fn desired_height(&self, width: u16) -> u16 {
        self.list.desired_height(width)
    }

    fn render(&self, area: Rect, buf: &mut Buffer) {
        self.list.render(area, buf);
    }

    fn cursor_pos(&self, area: Rect) -> Option<(u16, u16)> {
        self.list.cursor_pos(area)
    }
}

#[cfg(test)]
mod tests;
