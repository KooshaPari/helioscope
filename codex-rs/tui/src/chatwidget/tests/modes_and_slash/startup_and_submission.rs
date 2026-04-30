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
