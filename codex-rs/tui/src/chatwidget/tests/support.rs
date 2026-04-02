use super::*;

pub(crate) async fn test_config() -> Config {
    // Use base defaults to avoid depending on host state.
    let codex_home = std::env::temp_dir();
    ConfigBuilder::default()
        .codex_home(codex_home.clone())
        .build()
        .await
        .expect("config")
}

pub(crate) fn invalid_value(
    candidate: impl Into<String>,
    allowed: impl Into<String>,
) -> ConstraintError {
    ConstraintError::InvalidValue {
        field_name: "<unknown>",
        candidate: candidate.into(),
        allowed: allowed.into(),
        requirement_source: RequirementSource::Unknown,
    }
}

pub(crate) fn snapshot(percent: f64) -> RateLimitSnapshot {
    RateLimitSnapshot {
        limit_id: None,
        limit_name: None,
        primary: Some(RateLimitWindow {
            used_percent: percent,
            window_minutes: Some(60),
            resets_at: None,
        }),
        secondary: None,
        credits: None,
        plan_type: None,
    }
}

pub(crate) fn test_otel_manager(config: &Config, model: &str) -> OtelManager {
    let model_info = codex_core::test_support::construct_model_info_offline(model, config);
    OtelManager::new(
        ThreadId::new(),
        model,
        model_info.slug.as_str(),
        None,
        None,
        None,
        "test_originator".to_string(),
        false,
        "test".to_string(),
        SessionSource::Cli,
    )
}

// --- Helpers for tests that need direct construction and event draining ---
pub(crate) async fn make_chatwidget_manual(
    model_override: Option<&str>,
) -> (
    ChatWidget,
    tokio::sync::mpsc::UnboundedReceiver<AppEvent>,
    tokio::sync::mpsc::UnboundedReceiver<Op>,
) {
    let (tx_raw, rx) = unbounded_channel::<AppEvent>();
    let app_event_tx = AppEventSender::new(tx_raw);
    let (op_tx, op_rx) = unbounded_channel::<Op>();
    let mut cfg = test_config().await;
    let resolved_model = model_override
        .map(str::to_owned)
        .unwrap_or_else(|| codex_core::test_support::get_model_offline(cfg.model.as_deref()));
    if let Some(model) = model_override {
        cfg.model = Some(model.to_string());
    }
    let prevent_idle_sleep = cfg.features.enabled(Feature::PreventIdleSleep);
    let otel_manager = test_otel_manager(&cfg, resolved_model.as_str());
    let mut bottom = BottomPane::new(BottomPaneParams {
        app_event_tx: app_event_tx.clone(),
        frame_requester: FrameRequester::test_dummy(),
        has_input_focus: true,
        enhanced_keys_supported: false,
        placeholder_text: "Ask Codex to do anything".to_string(),
        disable_paste_burst: false,
        animations_enabled: cfg.animations,
        skills: None,
    });
    bottom.set_collaboration_modes_enabled(true);
    let auth_manager =
        codex_core::test_support::auth_manager_from_auth(CodexAuth::from_api_key("test"));
    let codex_home = cfg.codex_home.clone();
    let models_manager = Arc::new(ModelsManager::new(
        codex_home,
        auth_manager.clone(),
        None,
        CollaborationModesConfig::default(),
    ));
    let reasoning_effort = None;
    let base_mode = CollaborationMode {
        mode: ModeKind::Default,
        settings: Settings {
            model: resolved_model.clone(),
            reasoning_effort,
            developer_instructions: None,
        },
    };
    let current_collaboration_mode = base_mode;
    let active_collaboration_mask = collaboration_modes::default_mask(models_manager.as_ref());
    let mut widget = ChatWidget {
        app_event_tx,
        codex_op_tx: op_tx,
        bottom_pane: bottom,
        active_cell: None,
        active_cell_revision: 0,
        config: cfg,
        current_collaboration_mode,
        active_collaboration_mask,
        auth_manager,
        models_manager,
        otel_manager,
        session_header: SessionHeader::new(resolved_model.clone()),
        initial_user_message: None,
        token_info: None,
        rate_limit_snapshots_by_limit_id: BTreeMap::new(),
        plan_type: None,
        rate_limit_warnings: RateLimitWarningState::default(),
        rate_limit_switch_prompt: RateLimitSwitchPromptState::default(),
        rate_limit_poller: None,
        adaptive_chunking: crate::streaming::chunking::AdaptiveChunkingPolicy::default(),
        stream_controller: None,
        plan_stream_controller: None,
        last_copyable_output: None,
        running_commands: HashMap::new(),
        suppressed_exec_calls: HashSet::new(),
        skills_all: Vec::new(),
        skills_initial_state: None,
        last_unified_wait: None,
        unified_exec_wait_streak: None,
        turn_sleep_inhibitor: SleepInhibitor::new(prevent_idle_sleep),
        task_complete_pending: false,
        unified_exec_processes: Vec::new(),
        agent_turn_running: false,
        mcp_startup_status: None,
        connectors_cache: ConnectorsCacheState::default(),
        connectors_prefetch_in_flight: false,
        connectors_force_refetch_pending: false,
        interrupts: InterruptManager::new(),
        reasoning_buffer: String::new(),
        full_reasoning_buffer: String::new(),
        current_status_header: String::from("Working"),
        retry_status_header: None,
        pending_status_indicator_restore: false,
        thread_id: None,
        thread_name: None,
        forked_from: None,
        frame_requester: FrameRequester::test_dummy(),
        show_welcome_banner: true,
        startup_tooltip_override: None,
        queued_user_messages: VecDeque::new(),
        queued_message_edit_binding: crate::key_hint::alt(KeyCode::Up),
        suppress_session_configured_redraw: false,
        pending_notification: None,
        quit_shortcut_expires_at: None,
        quit_shortcut_key: None,
        is_review_mode: false,
        pre_review_token_info: None,
        needs_final_message_separator: false,
        had_work_activity: false,
        saw_plan_update_this_turn: false,
        saw_plan_item_this_turn: false,
        plan_delta_buffer: String::new(),
        plan_item_active: false,
        last_separator_elapsed_secs: None,
        turn_runtime_metrics: RuntimeMetricsSummary::default(),
        last_rendered_width: std::cell::Cell::new(None),
        feedback: codex_feedback::CodexFeedback::new(),
        feedback_audience: FeedbackAudience::External,
        current_rollout_path: None,
        current_cwd: None,
        session_network_proxy: None,
        status_line_invalid_items_warned: Arc::new(AtomicBool::new(false)),
        status_line_branch: None,
        status_line_branch_cwd: None,
        status_line_branch_pending: false,
        status_line_branch_lookup_complete: false,
        external_editor_state: ExternalEditorState::Closed,
        realtime_conversation: RealtimeConversationUiState::default(),
        last_rendered_user_message_event: None,
    };
    widget.set_model(&resolved_model);
    (widget, rx, op_rx)
}

// ChatWidget may emit other `Op`s (e.g. history/logging updates) on the same channel; this helper
// filters until we see a submission op.
pub(crate) fn next_submit_op(op_rx: &mut tokio::sync::mpsc::UnboundedReceiver<Op>) -> Op {
    loop {
        match op_rx.try_recv() {
            Ok(op @ Op::UserTurn { .. }) => return op,
            Ok(_) => continue,
            Err(TryRecvError::Empty) => panic!("expected a submit op but queue was empty"),
            Err(TryRecvError::Disconnected) => panic!("expected submit op but channel closed"),
        }
    }
}

pub(crate) fn assert_no_submit_op(op_rx: &mut tokio::sync::mpsc::UnboundedReceiver<Op>) {
    while let Ok(op) = op_rx.try_recv() {
        assert!(
            !matches!(op, Op::UserTurn { .. }),
            "unexpected submit op: {op:?}"
        );
    }
}

pub(crate) fn set_chatgpt_auth(chat: &mut ChatWidget) {
    chat.auth_manager = codex_core::test_support::auth_manager_from_auth(
        CodexAuth::create_dummy_chatgpt_auth_for_testing(),
    );
    chat.models_manager = Arc::new(ModelsManager::new(
        chat.config.codex_home.clone(),
        chat.auth_manager.clone(),
        None,
        CollaborationModesConfig::default(),
    ));
}

pub(crate) async fn make_chatwidget_manual_with_sender() -> (
    ChatWidget,
    AppEventSender,
    tokio::sync::mpsc::UnboundedReceiver<AppEvent>,
    tokio::sync::mpsc::UnboundedReceiver<Op>,
) {
    let (widget, rx, op_rx) = make_chatwidget_manual(None).await;
    let app_event_tx = widget.app_event_tx.clone();
    (widget, app_event_tx, rx, op_rx)
}

pub(crate) fn drain_insert_history(
    rx: &mut tokio::sync::mpsc::UnboundedReceiver<AppEvent>,
) -> Vec<Vec<ratatui::text::Line<'static>>> {
    let mut out = Vec::new();
    while let Ok(ev) = rx.try_recv() {
        if let AppEvent::InsertHistoryCell(cell) = ev {
            let mut lines = cell.display_lines(80);
            if !cell.is_stream_continuation() && !out.is_empty() && !lines.is_empty() {
                lines.insert(0, "".into());
            }
            out.push(lines)
        }
    }
    out
}

pub(crate) fn lines_to_single_string(lines: &[ratatui::text::Line<'static>]) -> String {
    let mut s = String::new();
    for line in lines {
        for span in &line.spans {
            s.push_str(&span.content);
        }
        s.push('\n');
    }
    s
}

pub(crate) fn make_token_info(total_tokens: i64, context_window: i64) -> TokenUsageInfo {
    fn usage(total_tokens: i64) -> TokenUsage {
        TokenUsage {
            total_tokens,
            ..TokenUsage::default()
        }
    }

    TokenUsageInfo {
        total_token_usage: usage(total_tokens),
        last_token_usage: usage(total_tokens),
        model_context_window: Some(context_window),
    }
}

// --- Small helpers to tersely drive exec begin/end and snapshot active cell ---
pub(crate) fn begin_exec_with_source(
    chat: &mut ChatWidget,
    call_id: &str,
    raw_cmd: &str,
    source: ExecCommandSource,
) -> ExecCommandBeginEvent {
    let command = vec!["bash".to_string(), "-lc".to_string(), raw_cmd.to_string()];
    let parsed_cmd: Vec<ParsedCommand> =
        codex_shell_command::parse_command::parse_command(&command);
    let cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let interaction_input = None;
    let event = ExecCommandBeginEvent {
        call_id: call_id.to_string(),
        process_id: None,
        turn_id: "turn-1".to_string(),
        command,
        cwd,
        parsed_cmd,
        source,
        interaction_input,
    };
    chat.handle_codex_event(Event {
        id: call_id.to_string(),
        msg: EventMsg::ExecCommandBegin(event.clone()),
    });
    event
}

pub(crate) fn begin_unified_exec_startup(
    chat: &mut ChatWidget,
    call_id: &str,
    process_id: &str,
    raw_cmd: &str,
) -> ExecCommandBeginEvent {
    let command = vec!["bash".to_string(), "-lc".to_string(), raw_cmd.to_string()];
    let cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let event = ExecCommandBeginEvent {
        call_id: call_id.to_string(),
        process_id: Some(process_id.to_string()),
        turn_id: "turn-1".to_string(),
        command,
        cwd,
        parsed_cmd: Vec::new(),
        source: ExecCommandSource::UnifiedExecStartup,
        interaction_input: None,
    };
    chat.handle_codex_event(Event {
        id: call_id.to_string(),
        msg: EventMsg::ExecCommandBegin(event.clone()),
    });
    event
}

pub(crate) fn terminal_interaction(
    chat: &mut ChatWidget,
    call_id: &str,
    process_id: &str,
    stdin: &str,
) {
    chat.handle_codex_event(Event {
        id: call_id.to_string(),
        msg: EventMsg::TerminalInteraction(TerminalInteractionEvent {
            call_id: call_id.to_string(),
            process_id: process_id.to_string(),
            stdin: stdin.to_string(),
        }),
    });
}

pub(crate) fn complete_assistant_message(
    chat: &mut ChatWidget,
    item_id: &str,
    text: &str,
    phase: Option<MessagePhase>,
) {
    chat.handle_codex_event(Event {
        id: format!("raw-{item_id}"),
        msg: EventMsg::ItemCompleted(ItemCompletedEvent {
            thread_id: ThreadId::new(),
            turn_id: "turn-1".to_string(),
            item: TurnItem::AgentMessage(AgentMessageItem {
                id: item_id.to_string(),
                content: vec![AgentMessageContent::Text {
                    text: text.to_string(),
                }],
                phase,
            }),
        }),
    });
}

pub(crate) fn begin_exec(
    chat: &mut ChatWidget,
    call_id: &str,
    raw_cmd: &str,
) -> ExecCommandBeginEvent {
    begin_exec_with_source(chat, call_id, raw_cmd, ExecCommandSource::Agent)
}

pub(crate) fn end_exec(
    chat: &mut ChatWidget,
    begin_event: ExecCommandBeginEvent,
    stdout: &str,
    stderr: &str,
    exit_code: i32,
) {
    let aggregated = if stderr.is_empty() {
        stdout.to_string()
    } else {
        format!("{stdout}{stderr}")
    };
    let ExecCommandBeginEvent {
        call_id,
        turn_id,
        command,
        cwd,
        parsed_cmd,
        source,
        interaction_input,
        process_id,
    } = begin_event;
    chat.handle_codex_event(Event {
        id: call_id.clone(),
        msg: EventMsg::ExecCommandEnd(ExecCommandEndEvent {
            call_id,
            process_id,
            turn_id,
            command,
            cwd,
            parsed_cmd,
            source,
            interaction_input,
            stdout: stdout.to_string(),
            stderr: stderr.to_string(),
            aggregated_output: aggregated.clone(),
            exit_code,
            duration: std::time::Duration::from_millis(5),
            formatted_output: aggregated,
            status: if exit_code == 0 {
                CoreExecCommandStatus::Completed
            } else {
                CoreExecCommandStatus::Failed
            },
        }),
    });
}

pub(crate) fn active_blob(chat: &ChatWidget) -> String {
    let lines = chat
        .active_cell
        .as_ref()
        .expect("active cell present")
        .display_lines(80);
    lines_to_single_string(&lines)
}

pub(crate) fn get_available_model(chat: &ChatWidget, model: &str) -> ModelPreset {
    let models = chat
        .models_manager
        .try_list_models()
        .expect("models lock available");
    models
        .iter()
        .find(|&preset| preset.model == model)
        .cloned()
        .unwrap_or_else(|| panic!("{model} preset not found"))
}
