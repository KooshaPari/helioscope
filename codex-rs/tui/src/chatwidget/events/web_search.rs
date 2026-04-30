use super::super::*;

impl ChatWidget {
    pub(in crate::chatwidget) fn on_web_search_begin(&mut self, ev: WebSearchBeginEvent) {
        self.flush_answer_stream_with_separator();
        self.flush_active_cell();
        self.active_cell = Some(Box::new(history_cell::new_active_web_search_call(
            ev.call_id,
            String::new(),
            self.config.animations,
        )));
        self.bump_active_cell_revision();
        self.request_redraw();
    }

    pub(in crate::chatwidget) fn on_web_search_end(&mut self, ev: WebSearchEndEvent) {
        self.flush_answer_stream_with_separator();
        let WebSearchEndEvent {
            call_id,
            query,
            action,
        } = ev;
        let mut handled = false;
        if let Some(cell) = self
            .active_cell
            .as_mut()
            .and_then(|cell| cell.as_any_mut().downcast_mut::<WebSearchCell>())
            && cell.call_id() == call_id
        {
            cell.update(action.clone(), query.clone());
            cell.complete();
            self.bump_active_cell_revision();
            self.flush_active_cell();
            handled = true;
        }

        if !handled {
            self.add_to_history(history_cell::new_web_search_call(call_id, query, action));
        }
        self.had_work_activity = true;
    }
}
