use super::*;
use pretty_assertions::assert_eq;

/// Selecting the custom prompt option from the review popup sends
/// OpenReviewCustomPrompt to the app event channel.
#[tokio::test]
async fn review_popup_custom_prompt_action_sends_event() {
    let (mut chat, mut rx, _op_rx) = make_chatwidget_manual(None).await;

    chat.open_review_popup();
    chat.handle_key_event(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));
    chat.handle_key_event(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));
    chat.handle_key_event(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));
    chat.handle_key_event(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));

    let mut found = false;
    while let Ok(ev) = rx.try_recv() {
        if let AppEvent::OpenReviewCustomPrompt = ev {
            found = true;
            break;
        }
    }
    assert!(found, "expected OpenReviewCustomPrompt event to be sent");
}

#[tokio::test]
async fn slash_init_skips_when_project_doc_exists() {
    let (mut chat, mut rx, mut op_rx) = make_chatwidget_manual(None).await;
    let tempdir = tempdir().unwrap();
    let existing_path = tempdir.path().join(DEFAULT_PROJECT_DOC_FILENAME);
    std::fs::write(&existing_path, "existing instructions").unwrap();
    chat.config.cwd = tempdir.path().to_path_buf();

    chat.dispatch_command(SlashCommand::Init);

    match op_rx.try_recv() {
        Err(TryRecvError::Empty) => {}
        other => panic!("expected no Codex op to be sent, got {other:?}"),
    }

    let cells = drain_insert_history(&mut rx);
    assert_eq!(cells.len(), 1, "expected one info message");
    let rendered = lines_to_single_string(&cells[0]);
    assert!(
        rendered.contains(DEFAULT_PROJECT_DOC_FILENAME),
        "info message should mention the existing file: {rendered:?}"
    );
    assert!(
        rendered.contains("Skipping /init"),
        "info message should explain why /init was skipped: {rendered:?}"
    );
    assert_eq!(
        std::fs::read_to_string(existing_path).unwrap(),
        "existing instructions"
    );
}

#[tokio::test]
async fn collab_mode_shift_tab_cycles_only_when_idle() {
    let (mut chat, _rx, _op_rx) = make_chatwidget_manual(None).await;

    let initial = chat.current_collaboration_mode().clone();
    chat.handle_key_event(KeyEvent::from(KeyCode::BackTab));
    assert_eq!(chat.active_collaboration_mode_kind(), ModeKind::Plan);
    assert_eq!(chat.current_collaboration_mode(), &initial);

    chat.handle_key_event(KeyEvent::from(KeyCode::BackTab));
    assert_eq!(chat.active_collaboration_mode_kind(), ModeKind::Default);
    assert_eq!(chat.current_collaboration_mode(), &initial);

    chat.on_task_started();
    let before = chat.active_collaboration_mode_kind();
    chat.handle_key_event(KeyEvent::from(KeyCode::BackTab));
    assert_eq!(chat.active_collaboration_mode_kind(), before);
}

#[tokio::test]
async fn mode_switch_surfaces_model_change_notification_when_effective_model_changes() {
    let (mut chat, mut rx, _op_rx) = make_chatwidget_manual(Some("gpt-5")).await;
    chat.set_feature_enabled(Feature::CollaborationModes, true);
    let default_model = chat.current_model().to_string();

    let mut plan_mask =
        collaboration_modes::mask_for_kind(chat.models_manager.as_ref(), ModeKind::Plan)
            .expect("expected plan collaboration mode");
    plan_mask.model = Some("gpt-5.1-codex-mini".to_string());
    chat.set_collaboration_mask(plan_mask);

    let plan_messages = drain_insert_history(&mut rx)
        .iter()
        .map(|lines| lines_to_single_string(lines))
        .collect::<Vec<_>>()
        .join("\n");
    assert!(
        plan_messages.contains("Model changed to gpt-5.1-codex-mini medium for Plan mode."),
        "expected Plan-mode model switch notice, got: {plan_messages:?}"
    );

    let default_mask = collaboration_modes::default_mask(chat.models_manager.as_ref())
        .expect("expected default collaboration mode");
    chat.set_collaboration_mask(default_mask);

    let default_messages = drain_insert_history(&mut rx)
        .iter()
        .map(|lines| lines_to_single_string(lines))
        .collect::<Vec<_>>()
        .join("\n");
    let expected_default_message =
        format!("Model changed to {default_model} default for Default mode.");
    assert!(
        default_messages.contains(&expected_default_message),
        "expected Default-mode model switch notice, got: {default_messages:?}"
    );
}

#[tokio::test]
async fn mode_switch_surfaces_reasoning_change_notification_when_model_stays_same() {
    let (mut chat, mut rx, _op_rx) = make_chatwidget_manual(Some("gpt-5.3-codex")).await;
    chat.set_feature_enabled(Feature::CollaborationModes, true);
    chat.set_reasoning_effort(Some(ReasoningEffortConfig::High));

    let plan_mask = collaboration_modes::plan_mask(chat.models_manager.as_ref())
        .expect("expected plan collaboration mode");
    chat.set_collaboration_mask(plan_mask);

    let plan_messages = drain_insert_history(&mut rx)
        .iter()
        .map(|lines| lines_to_single_string(lines))
        .collect::<Vec<_>>()
        .join("\n");
    assert!(
        plan_messages.contains("Model changed to gpt-5.3-codex medium for Plan mode."),
        "expected reasoning-change notice in Plan mode, got: {plan_messages:?}"
    );
}

#[tokio::test]
async fn collab_slash_command_opens_picker_and_updates_mode() {
    let (mut chat, mut rx, mut op_rx) = make_chatwidget_manual(None).await;
    chat.thread_id = Some(ThreadId::new());
    chat.set_feature_enabled(Feature::CollaborationModes, true);

    chat.dispatch_command(SlashCommand::Collab);
    let popup = render_bottom_popup(&chat, 80);
    assert!(
        popup.contains("Select Collaboration Mode"),
        "expected collaboration picker: {popup}"
    );

    chat.handle_key_event(KeyEvent::from(KeyCode::Enter));
    let selected_mask = match rx.try_recv() {
        Ok(AppEvent::UpdateCollaborationMode(mask)) => mask,
        other => panic!("expected UpdateCollaborationMode event, got {other:?}"),
    };
    chat.set_collaboration_mask(selected_mask);

    chat.bottom_pane
        .set_composer_text("hello".to_string(), Vec::new(), Vec::new());
    chat.handle_key_event(KeyEvent::from(KeyCode::Enter));
    match next_submit_op(&mut op_rx) {
        Op::UserTurn {
            collaboration_mode:
                Some(CollaborationMode {
                    mode: ModeKind::Default,
                    ..
                }),
            personality: Some(Personality::Pragmatic),
            ..
        } => {}
        other => {
            panic!("expected Op::UserTurn with code collab mode, got {other:?}")
        }
    }

    chat.bottom_pane
        .set_composer_text("follow up".to_string(), Vec::new(), Vec::new());
    chat.handle_key_event(KeyEvent::from(KeyCode::Enter));
    match next_submit_op(&mut op_rx) {
        Op::UserTurn {
            collaboration_mode:
                Some(CollaborationMode {
                    mode: ModeKind::Default,
                    ..
                }),
            personality: Some(Personality::Pragmatic),
            ..
        } => {}
        other => {
            panic!("expected Op::UserTurn with code collab mode, got {other:?}")
        }
    }
}

#[tokio::test]
async fn plan_slash_command_switches_to_plan_mode() {
    let (mut chat, mut rx, _op_rx) = make_chatwidget_manual(None).await;
    chat.set_feature_enabled(Feature::CollaborationModes, true);
    let initial = chat.current_collaboration_mode().clone();

    chat.dispatch_command(SlashCommand::Plan);

    while let Ok(event) = rx.try_recv() {
        assert!(
            matches!(event, AppEvent::InsertHistoryCell(_)),
            "plan should not emit a non-history app event: {event:?}"
        );
    }
    assert_eq!(chat.active_collaboration_mode_kind(), ModeKind::Plan);
    assert_eq!(chat.current_collaboration_mode(), &initial);
}

#[tokio::test]
async fn plan_slash_command_with_args_submits_prompt_in_plan_mode() {
    let (mut chat, _rx, mut op_rx) = make_chatwidget_manual(None).await;
    chat.set_feature_enabled(Feature::CollaborationModes, true);

    let configured = codex_protocol::protocol::SessionConfiguredEvent {
        session_id: ThreadId::new(),
        forked_from_id: None,
        thread_name: None,
        model: "test-model".to_string(),
        model_provider_id: "test-provider".to_string(),
        approval_policy: AskForApproval::Never,
        sandbox_policy: SandboxPolicy::new_read_only_policy(),
        cwd: PathBuf::from("/home/user/project"),
        reasoning_effort: Some(ReasoningEffortConfig::default()),
        history_log_id: 0,
        history_entry_count: 0,
        initial_messages: None,
        network_proxy: None,
        rollout_path: None,
    };
    chat.handle_codex_event(Event {
        id: "configured".into(),
        msg: EventMsg::SessionConfigured(configured),
    });

    chat.bottom_pane
        .set_composer_text("/plan build the plan".to_string(), Vec::new(), Vec::new());
    chat.handle_key_event(KeyEvent::from(KeyCode::Enter));

    let items = match next_submit_op(&mut op_rx) {
        Op::UserTurn { items, .. } => items,
        other => panic!("expected Op::UserTurn, got {other:?}"),
    };
    assert_eq!(items.len(), 1);
    assert_eq!(
        items[0],
        UserInput::Text {
            text: "build the plan".to_string(),
            text_elements: Vec::new(),
        }
    );
    assert_eq!(chat.active_collaboration_mode_kind(), ModeKind::Plan);
}

#[tokio::test]
async fn collaboration_modes_defaults_to_code_on_startup() {
    let codex_home = tempdir().expect("tempdir");
    let cfg = ConfigBuilder::default()
        .codex_home(codex_home.path().to_path_buf())
        .cli_overrides(vec![(
            "features.collaboration_modes".to_string(),
            TomlValue::Boolean(true),
        )])
        .build()
        .await
        .expect("config");
    let resolved_model = codex_core::test_support::get_model_offline(cfg.model.as_deref());
    let otel_manager = test_otel_manager(&cfg, resolved_model.as_str());
    let thread_manager = Arc::new(
        codex_core::test_support::thread_manager_with_models_provider(
            CodexAuth::from_api_key("test"),
            cfg.model_provider.clone(),
        ),
    );
    let auth_manager =
        codex_core::test_support::auth_manager_from_auth(CodexAuth::from_api_key("test"));
    let init = ChatWidgetInit {
        config: cfg,
        frame_requester: FrameRequester::test_dummy(),
        app_event_tx: AppEventSender::new(unbounded_channel::<AppEvent>().0),
        initial_user_message: None,
        enhanced_keys_supported: false,
        auth_manager,
        models_manager: thread_manager.get_models_manager(),
        feedback: codex_feedback::CodexFeedback::new(),
        is_first_run: true,
        feedback_audience: FeedbackAudience::External,
        model: Some(resolved_model.clone()),
        startup_tooltip_override: None,
        status_line_invalid_items_warned: Arc::new(AtomicBool::new(false)),
        otel_manager,
    };

    let chat = ChatWidget::new(init, thread_manager);
    assert_eq!(chat.active_collaboration_mode_kind(), ModeKind::Default);
    assert_eq!(chat.current_model(), resolved_model);
}

#[tokio::test]
async fn experimental_mode_plan_is_ignored_on_startup() {
    let codex_home = tempdir().expect("tempdir");
    let cfg = ConfigBuilder::default()
        .codex_home(codex_home.path().to_path_buf())
        .cli_overrides(vec![
            (
                "features.collaboration_modes".to_string(),
                TomlValue::Boolean(true),
            ),
            (
                "tui.experimental_mode".to_string(),
                TomlValue::String("plan".to_string()),
            ),
        ])
        .build()
        .await
        .expect("config");
    let resolved_model = codex_core::test_support::get_model_offline(cfg.model.as_deref());
    let otel_manager = test_otel_manager(&cfg, resolved_model.as_str());
    let thread_manager = Arc::new(
        codex_core::test_support::thread_manager_with_models_provider(
            CodexAuth::from_api_key("test"),
            cfg.model_provider.clone(),
        ),
    );
    let auth_manager =
        codex_core::test_support::auth_manager_from_auth(CodexAuth::from_api_key("test"));
    let init = ChatWidgetInit {
        config: cfg,
        frame_requester: FrameRequester::test_dummy(),
        app_event_tx: AppEventSender::new(unbounded_channel::<AppEvent>().0),
        initial_user_message: None,
        enhanced_keys_supported: false,
        auth_manager,
        models_manager: thread_manager.get_models_manager(),
        feedback: codex_feedback::CodexFeedback::new(),
        is_first_run: true,
        feedback_audience: FeedbackAudience::External,
        model: Some(resolved_model.clone()),
        startup_tooltip_override: None,
        status_line_invalid_items_warned: Arc::new(AtomicBool::new(false)),
        otel_manager,
    };

    let chat = ChatWidget::new(init, thread_manager);
    assert_eq!(chat.active_collaboration_mode_kind(), ModeKind::Default);
    assert_eq!(chat.current_model(), resolved_model);
}

#[tokio::test]
async fn set_model_updates_active_collaboration_mask() {
    let (mut chat, _rx, _op_rx) = make_chatwidget_manual(Some("gpt-5.1")).await;
    chat.set_feature_enabled(Feature::CollaborationModes, true);
    let plan_mask =
        collaboration_modes::mask_for_kind(chat.models_manager.as_ref(), ModeKind::Plan)
            .expect("expected plan collaboration mask");
    chat.set_collaboration_mask(plan_mask);

    chat.set_model("gpt-5.1-codex-mini");

    assert_eq!(chat.current_model(), "gpt-5.1-codex-mini");
    assert_eq!(chat.active_collaboration_mode_kind(), ModeKind::Plan);
}

#[tokio::test]
async fn set_reasoning_effort_updates_active_collaboration_mask() {
    let (mut chat, _rx, _op_rx) = make_chatwidget_manual(Some("gpt-5.1")).await;
    chat.set_feature_enabled(Feature::CollaborationModes, true);
    let plan_mask =
        collaboration_modes::mask_for_kind(chat.models_manager.as_ref(), ModeKind::Plan)
            .expect("expected plan collaboration mask");
    chat.set_collaboration_mask(plan_mask);

    chat.set_reasoning_effort(None);

    assert_eq!(
        chat.current_reasoning_effort(),
        Some(ReasoningEffortConfig::Medium)
    );
    assert_eq!(chat.active_collaboration_mode_kind(), ModeKind::Plan);
}

#[tokio::test]
async fn set_reasoning_effort_does_not_override_active_plan_override() {
    let (mut chat, _rx, _op_rx) = make_chatwidget_manual(Some("gpt-5.1")).await;
    chat.set_feature_enabled(Feature::CollaborationModes, true);
    chat.set_plan_mode_reasoning_effort(Some(ReasoningEffortConfig::High));
    let plan_mask =
        collaboration_modes::mask_for_kind(chat.models_manager.as_ref(), ModeKind::Plan)
            .expect("expected plan collaboration mask");
    chat.set_collaboration_mask(plan_mask);

    chat.set_reasoning_effort(Some(ReasoningEffortConfig::Low));

    assert_eq!(
        chat.current_reasoning_effort(),
        Some(ReasoningEffortConfig::High)
    );
    assert_eq!(chat.active_collaboration_mode_kind(), ModeKind::Plan);
}

#[tokio::test]
async fn collab_mode_is_sent_after_enabling() {
    let (mut chat, _rx, mut op_rx) = make_chatwidget_manual(None).await;
    chat.thread_id = Some(ThreadId::new());
    chat.set_feature_enabled(Feature::CollaborationModes, true);

    chat.bottom_pane
        .set_composer_text("hello".to_string(), Vec::new(), Vec::new());
    chat.handle_key_event(KeyEvent::from(KeyCode::Enter));
    match next_submit_op(&mut op_rx) {
        Op::UserTurn {
            collaboration_mode:
                Some(CollaborationMode {
                    mode: ModeKind::Default,
                    ..
                }),
            personality: Some(Personality::Pragmatic),
            ..
        } => {}
        other => {
            panic!("expected Op::UserTurn, got {other:?}")
        }
    }
}

#[tokio::test]
async fn collab_mode_applies_default_preset() {
    let (mut chat, _rx, mut op_rx) = make_chatwidget_manual(None).await;
    chat.thread_id = Some(ThreadId::new());

    chat.bottom_pane
        .set_composer_text("hello".to_string(), Vec::new(), Vec::new());
    chat.handle_key_event(KeyEvent::from(KeyCode::Enter));
    match next_submit_op(&mut op_rx) {
        Op::UserTurn {
            collaboration_mode:
                Some(CollaborationMode {
                    mode: ModeKind::Default,
                    ..
                }),
            personality: Some(Personality::Pragmatic),
            ..
        } => {}
        other => {
            panic!("expected Op::UserTurn with default collaboration_mode, got {other:?}")
        }
    }

    assert_eq!(chat.active_collaboration_mode_kind(), ModeKind::Default);
    assert_eq!(chat.current_collaboration_mode().mode, ModeKind::Default);
}

#[tokio::test]
async fn user_turn_includes_personality_from_config() {
    let (mut chat, _rx, mut op_rx) = make_chatwidget_manual(Some("gpt-5.2-codex")).await;
    chat.set_feature_enabled(Feature::Personality, true);
    chat.thread_id = Some(ThreadId::new());
    chat.set_model("gpt-5.2-codex");
    chat.set_personality(Personality::Friendly);

    chat.bottom_pane
        .set_composer_text("hello".to_string(), Vec::new(), Vec::new());
    chat.handle_key_event(KeyEvent::from(KeyCode::Enter));
    match next_submit_op(&mut op_rx) {
        Op::UserTurn {
            personality: Some(Personality::Friendly),
            ..
        } => {}
        other => panic!("expected Op::UserTurn with friendly personality, got {other:?}"),
    }
}
