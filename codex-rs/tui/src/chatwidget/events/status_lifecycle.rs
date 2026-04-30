use super::super::*;

impl ChatWidget {
    pub(in crate::chatwidget) fn on_warning(&mut self, message: impl Into<String>) {
        self.add_to_history(history_cell::new_warning_event(message.into()));
        self.request_redraw();
    }

    pub(in crate::chatwidget) fn on_mcp_startup_update(&mut self, ev: McpStartupUpdateEvent) {
        let mut status = self.mcp_startup_status.take().unwrap_or_default();
        if let McpStartupStatus::Failed { error } = &ev.status {
            self.on_warning(error);
        }
        status.insert(ev.server, ev.status);
        self.mcp_startup_status = Some(status);
        self.update_task_running_state();
        if let Some(current) = &self.mcp_startup_status {
            let total = current.len();
            let mut starting: Vec<_> = current
                .iter()
                .filter_map(|(name, state)| {
                    if matches!(state, McpStartupStatus::Starting) {
                        Some(name)
                    } else {
                        None
                    }
                })
                .collect();
            starting.sort();
            if let Some(first) = starting.first() {
                let completed = total.saturating_sub(starting.len());
                let max_to_show = 3;
                let mut to_show: Vec<String> = starting
                    .iter()
                    .take(max_to_show)
                    .map(ToString::to_string)
                    .collect();
                if starting.len() > max_to_show {
                    to_show.push("…".to_string());
                }
                let header = if total > 1 {
                    format!(
                        "Starting MCP servers ({completed}/{total}): {}",
                        to_show.join(", ")
                    )
                } else {
                    format!("Booting MCP server: {first}")
                };
                self.set_status_header(header);
            }
        }
        self.request_redraw();
    }

    pub(in crate::chatwidget) fn on_mcp_startup_complete(&mut self, ev: McpStartupCompleteEvent) {
        let mut parts = Vec::new();
        if !ev.failed.is_empty() {
            let failed_servers: Vec<_> = ev.failed.iter().map(|f| f.server.clone()).collect();
            parts.push(format!("failed: {}", failed_servers.join(", ")));
        }
        if !ev.cancelled.is_empty() {
            self.on_warning(format!(
                "MCP startup interrupted. The following servers were not initialized: {}",
                ev.cancelled.join(", ")
            ));
        }
        if !parts.is_empty() {
            self.on_warning(format!("MCP startup incomplete ({})", parts.join("; ")));
        }

        self.mcp_startup_status = None;
        self.update_task_running_state();
        self.maybe_send_next_queued_input();
        self.request_redraw();
    }

    pub(in crate::chatwidget) fn on_turn_diff(&mut self, unified_diff: String) {
        debug!("TurnDiffEvent: {unified_diff}");
        self.refresh_status_line();
    }

    pub(in crate::chatwidget) fn on_deprecation_notice(&mut self, event: DeprecationNoticeEvent) {
        let DeprecationNoticeEvent { summary, details } = event;
        self.add_to_history(history_cell::new_deprecation_notice(summary, details));
        self.request_redraw();
    }

    pub(in crate::chatwidget) fn on_background_event(&mut self, message: String) {
        debug!("BackgroundEvent: {message}");
        self.bottom_pane.ensure_status_indicator();
        self.bottom_pane.set_interrupt_hint_visible(true);
        self.set_status_header(message);
    }

    pub(in crate::chatwidget) fn on_undo_started(&mut self, event: UndoStartedEvent) {
        self.bottom_pane.ensure_status_indicator();
        self.bottom_pane.set_interrupt_hint_visible(false);
        let message = event
            .message
            .unwrap_or_else(|| "Undo in progress...".to_string());
        self.set_status_header(message);
    }

    pub(in crate::chatwidget) fn on_undo_completed(&mut self, event: UndoCompletedEvent) {
        let UndoCompletedEvent { success, message } = event;
        self.bottom_pane.hide_status_indicator();
        let message = message.unwrap_or_else(|| {
            if success {
                "Undo completed successfully.".to_string()
            } else {
                "Undo failed.".to_string()
            }
        });
        if success {
            self.add_info_message(message, None);
        } else {
            self.add_error_message(message);
        }
    }

    pub(in crate::chatwidget) fn on_stream_error(
        &mut self,
        message: String,
        additional_details: Option<String>,
    ) {
        if self.retry_status_header.is_none() {
            self.retry_status_header = Some(self.current_status_header.clone());
        }
        self.bottom_pane.ensure_status_indicator();
        self.set_status(
            message,
            additional_details,
            StatusDetailsCapitalization::CapitalizeFirst,
            STATUS_DETAILS_DEFAULT_MAX_LINES,
        );
    }
}
