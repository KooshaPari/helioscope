use super::super::*;
use codex_protocol::approvals::ElicitationRequestEvent;
use codex_protocol::request_user_input::RequestUserInputEvent;

impl ChatWidget {
    pub(in crate::chatwidget) fn on_elicitation_request(&mut self, ev: ElicitationRequestEvent) {
        let ev2 = ev.clone();
        self.defer_or_handle(
            |q| q.push_elicitation(ev),
            |s| s.handle_elicitation_request_now(ev2),
        );
    }

    pub(in crate::chatwidget) fn on_request_user_input(&mut self, ev: RequestUserInputEvent) {
        let ev2 = ev.clone();
        self.defer_or_handle(
            |q| q.push_user_input(ev),
            |s| s.handle_request_user_input_now(ev2),
        );
    }

    pub(crate) fn handle_elicitation_request_now(&mut self, ev: ElicitationRequestEvent) {
        self.flush_answer_stream_with_separator();

        self.notify(Notification::ElicitationRequested {
            server_name: ev.server_name.clone(),
        });

        let request = ApprovalRequest::McpElicitation {
            thread_id: self.thread_id.unwrap_or_default(),
            thread_label: None,
            server_name: ev.server_name,
            request_id: ev.id,
            message: ev.message,
        };
        self.bottom_pane
            .push_approval_request(request, &self.config.features);
        self.request_redraw();
    }

    pub(crate) fn handle_request_user_input_now(&mut self, ev: RequestUserInputEvent) {
        self.flush_answer_stream_with_separator();
        self.bottom_pane.push_user_input_request(ev);
        self.request_redraw();
    }
}
