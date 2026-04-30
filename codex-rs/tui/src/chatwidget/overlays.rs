use super::*;

impl ChatWidget {
    pub(crate) fn open_feedback_note(
        &mut self,
        category: crate::app_event::FeedbackCategory,
        include_logs: bool,
    ) {
        let snapshot = self.feedback.snapshot(self.thread_id);
        let rollout = if include_logs {
            self.current_rollout_path.clone()
        } else {
            None
        };
        let view = crate::bottom_pane::FeedbackNoteView::new(
            category,
            snapshot,
            rollout,
            self.app_event_tx.clone(),
            include_logs,
            self.feedback_audience,
        );
        self.bottom_pane.show_view(Box::new(view));
        self.request_redraw();
    }

    pub(crate) fn open_app_link_view(&mut self, params: crate::bottom_pane::AppLinkViewParams) {
        let view = crate::bottom_pane::AppLinkView::new(params, self.app_event_tx.clone());
        self.bottom_pane.show_view(Box::new(view));
        self.request_redraw();
    }

    pub(crate) fn open_feedback_consent(&mut self, category: crate::app_event::FeedbackCategory) {
        let params = crate::bottom_pane::feedback_upload_consent_params(
            self.app_event_tx.clone(),
            category,
            self.current_rollout_path.clone(),
        );
        self.bottom_pane.show_selection_view(params);
        self.request_redraw();
    }
}
