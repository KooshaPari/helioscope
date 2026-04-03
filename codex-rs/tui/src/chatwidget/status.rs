use super::*;

impl ChatWidget {
    fn restore_reasoning_status_header(&mut self) {
        if let Some(header) = extract_first_bold(&self.reasoning_buffer) {
            self.set_status_header(header);
        } else if self.bottom_pane.is_task_running() {
            self.set_status_header(String::from("Working"));
        }
    }

    /// Restore the status indicator only after commentary completion is pending,
    /// the turn is still running, and all stream queues have drained.
    ///
    /// This gate prevents flicker while normal output is still actively
    /// streaming, but still restores a visible "working" affordance when a
    /// commentary block ends before the turn itself has completed.
    fn maybe_restore_status_indicator_after_stream_idle(&mut self) {
        if !self.pending_status_indicator_restore
            || !self.bottom_pane.is_task_running()
            || !self.stream_controllers_idle()
        {
            return;
        }

        self.bottom_pane.ensure_status_indicator();
        self.set_status_header(self.current_status_header.clone());
        self.pending_status_indicator_restore = false;
    }

    /// Update the status indicator header and details.
    ///
    /// Passing `None` clears any existing details.
    fn set_status(
        &mut self,
        header: String,
        details: Option<String>,
        details_capitalization: StatusDetailsCapitalization,
        details_max_lines: usize,
    ) {
        self.current_status_header = header.clone();
        self.bottom_pane
            .update_status(header, details, details_capitalization, details_max_lines);
    }

    /// Convenience wrapper around [`Self::set_status`];
    /// updates the status indicator header and clears any existing details.
    fn set_status_header(&mut self, header: String) {
        self.set_status(
            header,
            None,
            StatusDetailsCapitalization::CapitalizeFirst,
            STATUS_DETAILS_DEFAULT_MAX_LINES,
        );
    }

    /// Sets the currently rendered footer status-line value.
    pub(crate) fn set_status_line(&mut self, status_line: Option<Line<'static>>) {
        self.bottom_pane.set_status_line(status_line);
    }

    /// Recomputes footer status-line content from config and current runtime state.
    ///
    /// This method is the status-line orchestrator: it parses configured item identifiers,
    /// warns once per session about invalid items, updates whether status-line mode is enabled,
    /// schedules async git-branch lookup when needed, and renders only values that are currently
    /// available.
    ///
    /// The omission behavior is intentional. If selected items are unavailable (for example before
    /// a session id exists or before branch lookup completes), those items are skipped without
    /// placeholders so the line remains compact and stable.
    pub(crate) fn refresh_status_line(&mut self) {
        let (items, invalid_items) = self.status_line_items_with_invalids();
        if self.thread_id.is_some()
            && !invalid_items.is_empty()
            && self
                .status_line_invalid_items_warned
                .compare_exchange(false, true, Ordering::Relaxed, Ordering::Relaxed)
                .is_ok()
        {
            let label = if invalid_items.len() == 1 {
                "item"
            } else {
                "items"
            };
            let message = format!(
                "Ignored invalid status line {label}: {}.",
                proper_join(invalid_items.as_slice())
            );
            self.on_warning(message);
        }
        if !items.contains(&StatusLineItem::GitBranch) {
            self.status_line_branch = None;
            self.status_line_branch_pending = false;
            self.status_line_branch_lookup_complete = false;
        }
        let enabled = !items.is_empty();
        self.bottom_pane.set_status_line_enabled(enabled);
        if !enabled {
            self.set_status_line(None);
            return;
        }

        let cwd = self.status_line_cwd().to_path_buf();
        self.sync_status_line_branch_state(&cwd);

        if items.contains(&StatusLineItem::GitBranch) && !self.status_line_branch_lookup_complete {
            self.request_status_line_branch(cwd);
        }

        let mut parts = Vec::new();
        for item in items {
            if let Some(value) = self.status_line_value_for_item(&item) {
                parts.push(value);
            }
        }

        let line = if parts.is_empty() {
            None
        } else {
            Some(Line::from(parts.join(" · ")))
        };
        self.set_status_line(line);
    }

    /// Records that status-line setup was canceled.
    ///
    /// Cancellation is intentionally side-effect free for config state; the existing configuration
    /// remains active and no persistence is attempted.
    pub(crate) fn cancel_status_line_setup(&self) {
        tracing::info!("Status line setup canceled by user");
    }

    /// Applies status-line item selection from the setup view to in-memory config.
    ///
    /// An empty selection persists as an explicit empty list.
    pub(crate) fn setup_status_line(&mut self, items: Vec<StatusLineItem>) {
        tracing::info!("status line setup confirmed with items: {items:#?}");
        let ids = items.iter().map(ToString::to_string).collect::<Vec<_>>();
        self.config.tui_status_line = Some(ids);
        self.refresh_status_line();
    }

    /// Stores async git-branch lookup results for the current status-line cwd.
    ///
    /// Results are dropped when they target an out-of-date cwd to avoid rendering stale branch
    /// names after directory changes.
    pub(crate) fn set_status_line_branch(&mut self, cwd: PathBuf, branch: Option<String>) {
        if self.status_line_branch_cwd.as_ref() != Some(&cwd) {
            self.status_line_branch_pending = false;
            return;
        }
        self.status_line_branch = branch;
        self.status_line_branch_pending = false;
        self.status_line_branch_lookup_complete = true;
    }

    /// Forces a new git-branch lookup when `GitBranch` is part of the configured status line.
    fn request_status_line_branch_refresh(&mut self) {
        let (items, _) = self.status_line_items_with_invalids();
        if items.is_empty() || !items.contains(&StatusLineItem::GitBranch) {
            return;
        }
        let cwd = self.status_line_cwd().to_path_buf();
        self.sync_status_line_branch_state(&cwd);
        self.request_status_line_branch(cwd);
    }

    fn restore_retry_status_header_if_present(&mut self) {
        if let Some(header) = self.retry_status_header.take() {
            self.set_status_header(header);
        }
    }
}
