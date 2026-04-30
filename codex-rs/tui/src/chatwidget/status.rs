use super::*;
use crate::app_event::StatusLineEvent;
use crate::status::RateLimitSnapshotDisplay;
use crate::status::rate_limit_snapshot_display_for_limit;
use chrono::Local;
use codex_protocol::protocol::CreditsSnapshot;
use codex_protocol::protocol::RateLimitSnapshot;
use codex_protocol::protocol::TokenUsage;
use codex_protocol::protocol::TokenUsageInfo;

impl ChatWidget {
    pub(crate) fn add_status_output(&mut self) {
        let default_usage = TokenUsage::default();
        let token_info = self.token_info.as_ref();
        let total_usage = token_info
            .map(|ti| &ti.total_token_usage)
            .unwrap_or(&default_usage);
        let collaboration_mode = self.collaboration_mode_label();
        let reasoning_effort_override = Some(self.effective_reasoning_effort());
        let rate_limit_snapshots: Vec<RateLimitSnapshotDisplay> = self
            .rate_limit_snapshots_by_limit_id
            .values()
            .cloned()
            .collect();
        self.add_to_history(crate::status::new_status_output_with_rate_limits(
            &self.config,
            self.auth_manager.as_ref(),
            token_info,
            total_usage,
            &self.thread_id,
            self.thread_name.clone(),
            self.forked_from,
            rate_limit_snapshots.as_slice(),
            self.plan_type,
            Local::now(),
            self.model_display_name(),
            collaboration_mode,
            reasoning_effort_override,
        ));
    }

    pub(crate) fn set_token_info(&mut self, info: Option<TokenUsageInfo>) {
        match info {
            Some(info) => self.apply_token_info(info),
            None => {
                self.bottom_pane.set_context_window(None, None);
                self.token_info = None;
            }
        }
    }

    fn apply_token_info(&mut self, info: TokenUsageInfo) {
        let percent = self.context_remaining_percent(&info);
        let used_tokens = self.context_used_tokens(&info, percent.is_some());
        self.bottom_pane.set_context_window(percent, used_tokens);
        self.token_info = Some(info);
    }

    fn context_remaining_percent(&self, info: &TokenUsageInfo) -> Option<i64> {
        info.model_context_window.map(|window| {
            info.last_token_usage
                .percent_of_context_window_remaining(window)
        })
    }

    fn context_used_tokens(&self, info: &TokenUsageInfo, percent_known: bool) -> Option<i64> {
        if percent_known {
            return None;
        }

        Some(info.total_token_usage.tokens_in_context_window())
    }

    pub(in crate::chatwidget) fn restore_pre_review_token_info(&mut self) {
        if let Some(saved) = self.pre_review_token_info.take() {
            match saved {
                Some(info) => self.apply_token_info(info),
                None => {
                    self.bottom_pane.set_context_window(None, None);
                    self.token_info = None;
                }
            }
        }
    }

    pub(crate) fn on_rate_limit_snapshot(&mut self, snapshot: Option<RateLimitSnapshot>) {
        if let Some(mut snapshot) = snapshot {
            let limit_id = snapshot
                .limit_id
                .clone()
                .unwrap_or_else(|| "codex".to_string());
            let limit_label = snapshot
                .limit_name
                .clone()
                .unwrap_or_else(|| limit_id.clone());
            if snapshot.credits.is_none() {
                snapshot.credits = self
                    .rate_limit_snapshots_by_limit_id
                    .get(&limit_id)
                    .and_then(|display| display.credits.as_ref())
                    .map(|credits| CreditsSnapshot {
                        has_credits: credits.has_credits,
                        unlimited: credits.unlimited,
                        balance: credits.balance.clone(),
                    });
            }

            self.plan_type = snapshot.plan_type.or(self.plan_type);

            let is_codex_limit = limit_id.eq_ignore_ascii_case("codex");
            let warnings = if is_codex_limit {
                self.rate_limit_warnings.take_warnings(
                    snapshot
                        .secondary
                        .as_ref()
                        .map(|window| window.used_percent),
                    snapshot
                        .secondary
                        .as_ref()
                        .and_then(|window| window.window_minutes),
                    snapshot.primary.as_ref().map(|window| window.used_percent),
                    snapshot
                        .primary
                        .as_ref()
                        .and_then(|window| window.window_minutes),
                )
            } else {
                vec![]
            };

            let high_usage = is_codex_limit
                && (snapshot
                    .secondary
                    .as_ref()
                    .map(|w| w.used_percent >= RATE_LIMIT_SWITCH_PROMPT_THRESHOLD)
                    .unwrap_or(false)
                    || snapshot
                        .primary
                        .as_ref()
                        .map(|w| w.used_percent >= RATE_LIMIT_SWITCH_PROMPT_THRESHOLD)
                        .unwrap_or(false));

            if high_usage
                && !self.rate_limit_switch_prompt_hidden()
                && self.current_model() != NUDGE_MODEL_SLUG
                && !matches!(
                    self.rate_limit_switch_prompt,
                    RateLimitSwitchPromptState::Shown
                )
            {
                self.rate_limit_switch_prompt = RateLimitSwitchPromptState::Pending;
            }

            let display =
                rate_limit_snapshot_display_for_limit(&snapshot, limit_label, Local::now());
            self.rate_limit_snapshots_by_limit_id
                .insert(limit_id, display);

            if !warnings.is_empty() {
                for warning in warnings {
                    self.add_to_history(history_cell::new_warning_event(warning));
                }
                self.request_redraw();
            }
        } else {
            self.rate_limit_snapshots_by_limit_id.clear();
        }
        self.refresh_status_line();
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
    pub(crate) fn request_status_line_branch_refresh(&mut self) {
        let (items, _) = self.status_line_items_with_invalids();
        if items.is_empty() || !items.contains(&StatusLineItem::GitBranch) {
            return;
        }
        let cwd = self.status_line_cwd().to_path_buf();
        self.sync_status_line_branch_state(&cwd);
        self.request_status_line_branch(cwd);
    }

    /// Parses configured status-line ids into known items and collects unknown ids.
    ///
    /// Unknown ids are deduplicated in insertion order for warning messages.
    fn status_line_items_with_invalids(&self) -> (Vec<StatusLineItem>, Vec<String>) {
        let mut invalid = Vec::new();
        let mut invalid_seen = HashSet::new();
        let mut items = Vec::new();
        for id in self.configured_status_line_items() {
            match id.parse::<StatusLineItem>() {
                Ok(item) => items.push(item),
                Err(_) => {
                    if invalid_seen.insert(id.clone()) {
                        invalid.push(format!(r#""{id}""#));
                    }
                }
            }
        }
        (items, invalid)
    }

    pub(crate) fn configured_status_line_items(&self) -> Vec<String> {
        self.config.tui_status_line.clone().unwrap_or_else(|| {
            DEFAULT_STATUS_LINE_ITEMS
                .iter()
                .map(ToString::to_string)
                .collect()
        })
    }

    fn status_line_cwd(&self) -> &Path {
        self.current_cwd.as_ref().unwrap_or(&self.config.cwd)
    }

    fn status_line_project_root(&self) -> Option<PathBuf> {
        let cwd = self.status_line_cwd();
        if let Some(repo_root) = get_git_repo_root(cwd) {
            return Some(repo_root);
        }

        self.config
            .config_layer_stack
            .get_layers(ConfigLayerStackOrdering::LowestPrecedenceFirst, true)
            .iter()
            .find_map(|layer| match &layer.name {
                ConfigLayerSource::Project { dot_codex_folder } => {
                    dot_codex_folder.as_path().parent().map(Path::to_path_buf)
                }
                _ => None,
            })
    }

    fn status_line_project_root_name(&self) -> Option<String> {
        self.status_line_project_root().map(|root| {
            root.file_name()
                .map(|name| name.to_string_lossy().to_string())
                .unwrap_or_else(|| format_directory_display(&root, None))
        })
    }

    /// Resets git-branch cache state when the status-line cwd changes.
    ///
    /// The branch cache is keyed by cwd because branch lookup is performed relative to that path.
    /// Keeping stale branch values across cwd changes would surface incorrect repository context.
    fn sync_status_line_branch_state(&mut self, cwd: &Path) {
        if self
            .status_line_branch_cwd
            .as_ref()
            .is_some_and(|path| path == cwd)
        {
            return;
        }
        self.status_line_branch_cwd = Some(cwd.to_path_buf());
        self.status_line_branch = None;
        self.status_line_branch_pending = false;
        self.status_line_branch_lookup_complete = false;
    }

    /// Starts an async git-branch lookup unless one is already running.
    ///
    /// The resulting status-line event carries the lookup cwd so callers can reject stale
    /// completions after directory changes.
    fn request_status_line_branch(&mut self, cwd: PathBuf) {
        if self.status_line_branch_pending {
            return;
        }
        self.status_line_branch_pending = true;
        let tx = self.app_event_tx.clone();
        tokio::spawn(async move {
            let branch = current_branch_name(&cwd).await;
            tx.send(AppEvent::StatusLine(StatusLineEvent::BranchUpdated {
                cwd,
                branch,
            }));
        });
    }

    /// Resolves a display string for one configured status-line item.
    ///
    /// Returning `None` means "omit this item for now", not "configuration error". Callers rely on
    /// this to keep partially available status lines readable while waiting for session, token, or
    /// git metadata.
    fn status_line_value_for_item(&self, item: &StatusLineItem) -> Option<String> {
        match item {
            StatusLineItem::ModelName => Some(self.model_display_name().to_string()),
            StatusLineItem::ModelWithReasoning => {
                let label =
                    Self::status_line_reasoning_effort_label(self.effective_reasoning_effort());
                Some(format!("{} {label}", self.model_display_name()))
            }
            StatusLineItem::CurrentDir => {
                Some(format_directory_display(self.status_line_cwd(), None))
            }
            StatusLineItem::ProjectRoot => self.status_line_project_root_name(),
            StatusLineItem::GitBranch => self.status_line_branch.clone(),
            StatusLineItem::UsedTokens => {
                let usage = self.status_line_total_usage();
                let total = usage.tokens_in_context_window();
                if total <= 0 {
                    None
                } else {
                    Some(format!("{} used", format_tokens_compact(total)))
                }
            }
            StatusLineItem::ContextRemaining => self
                .status_line_context_remaining_percent()
                .map(|remaining| format!("{remaining}% left")),
            StatusLineItem::ContextUsed => self
                .status_line_context_used_percent()
                .map(|used| format!("{used}% used")),
            StatusLineItem::FiveHourLimit => {
                let window = self
                    .rate_limit_snapshots_by_limit_id
                    .get("codex")
                    .and_then(|s| s.primary.as_ref());
                let label = window
                    .and_then(|window| window.window_minutes)
                    .map(get_limits_duration)
                    .unwrap_or_else(|| "5h".to_string());
                self.status_line_limit_display(window, &label)
            }
            StatusLineItem::WeeklyLimit => {
                let window = self
                    .rate_limit_snapshots_by_limit_id
                    .get("codex")
                    .and_then(|s| s.secondary.as_ref());
                let label = window
                    .and_then(|window| window.window_minutes)
                    .map(get_limits_duration)
                    .unwrap_or_else(|| "weekly".to_string());
                self.status_line_limit_display(window, &label)
            }
            StatusLineItem::CodexVersion => Some(CODEX_CLI_VERSION.to_string()),
            StatusLineItem::ContextWindowSize => self
                .status_line_context_window_size()
                .map(|cws| format!("{} window", format_tokens_compact(cws))),
            StatusLineItem::TotalInputTokens => Some(format!(
                "{} in",
                format_tokens_compact(self.status_line_total_usage().input_tokens)
            )),
            StatusLineItem::TotalOutputTokens => Some(format!(
                "{} out",
                format_tokens_compact(self.status_line_total_usage().output_tokens)
            )),
            StatusLineItem::SessionId => self.thread_id.map(|id| id.to_string()),
        }
    }

    fn status_line_context_window_size(&self) -> Option<i64> {
        self.token_info
            .as_ref()
            .and_then(|info| info.model_context_window)
            .or(self.config.model_context_window)
    }

    fn status_line_context_remaining_percent(&self) -> Option<i64> {
        let Some(context_window) = self.status_line_context_window_size() else {
            return Some(100);
        };
        let default_usage = TokenUsage::default();
        let usage = self
            .token_info
            .as_ref()
            .map(|info| &info.last_token_usage)
            .unwrap_or(&default_usage);
        Some(
            usage
                .percent_of_context_window_remaining(context_window)
                .clamp(0, 100),
        )
    }

    fn status_line_context_used_percent(&self) -> Option<i64> {
        let remaining = self.status_line_context_remaining_percent().unwrap_or(100);
        Some((100 - remaining).clamp(0, 100))
    }

    fn status_line_total_usage(&self) -> TokenUsage {
        self.token_info
            .as_ref()
            .map(|info| info.total_token_usage.clone())
            .unwrap_or_default()
    }

    fn status_line_limit_display(
        &self,
        window: Option<&RateLimitWindowDisplay>,
        label: &str,
    ) -> Option<String> {
        let window = window?;
        let remaining = (100.0f64 - window.used_percent).clamp(0.0f64, 100.0f64);
        Some(format!("{label} {remaining:.0}%"))
    }

    fn status_line_reasoning_effort_label(effort: Option<ReasoningEffortConfig>) -> &'static str {
        match effort {
            Some(ReasoningEffortConfig::Minimal) => "minimal",
            Some(ReasoningEffortConfig::Low) => "low",
            Some(ReasoningEffortConfig::Medium) => "medium",
            Some(ReasoningEffortConfig::High) => "high",
            Some(ReasoningEffortConfig::XHigh) => "xhigh",
            None | Some(ReasoningEffortConfig::None) => "default",
        }
    }
}
