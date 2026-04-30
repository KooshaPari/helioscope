use super::super::*;

pub(crate) async fn test_config() -> Config {
    // Use base defaults to avoid depending on host state.
    let codex_home = std::env::temp_dir();
    ConfigBuilder::default()
        .codex_home(codex_home.clone())
        .build()
        .await
        .expect("config")
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
