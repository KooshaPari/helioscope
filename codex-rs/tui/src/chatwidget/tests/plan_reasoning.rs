use super::*;

#[tokio::test]
async fn submit_user_message_with_mode_sets_coding_collaboration_mode() {
    let (mut chat, _rx, mut op_rx) = make_chatwidget_manual(Some("gpt-5")).await;
    chat.thread_id = Some(ThreadId::new());
    chat.set_feature_enabled(Feature::CollaborationModes, true);

    let default_mode = collaboration_modes::default_mode_mask(chat.models_manager.as_ref())
        .expect("expected default collaboration mode");
    chat.submit_user_message_with_mode("Implement the plan.".to_string(), default_mode);

    match next_submit_op(&mut op_rx) {
        Op::UserTurn {
            collaboration_mode:
                Some(CollaborationMode {
                    mode: ModeKind::Default,
                    ..
                }),
            personality: None,
            ..
        } => {}
        other => {
            panic!("expected Op::UserTurn with default collab mode, got {other:?}")
        }
    }
}

#[tokio::test]
async fn reasoning_selection_in_plan_mode_opens_scope_prompt_event() {
    let (mut chat, mut rx, _op_rx) = make_chatwidget_manual(Some("gpt-5.1-codex-max")).await;
    chat.thread_id = Some(ThreadId::new());
    chat.set_feature_enabled(Feature::CollaborationModes, true);
    let plan_mask = collaboration_modes::plan_mask(chat.models_manager.as_ref())
        .expect("expected plan collaboration mode");
    chat.set_collaboration_mask(plan_mask);
    let _ = drain_insert_history(&mut rx);
    set_chatgpt_auth(&mut chat);
    chat.set_reasoning_effort(Some(ReasoningEffortConfig::High));

    let preset = get_available_model(&chat, "gpt-5.1-codex-max");
    chat.open_reasoning_popup(preset);
    chat.handle_key_event(KeyEvent::from(KeyCode::Down));
    chat.handle_key_event(KeyEvent::from(KeyCode::Enter));

    let event = rx.try_recv().expect("expected AppEvent");
    assert_matches!(
        event,
        AppEvent::OpenPlanReasoningScopePrompt {
            model,
            effort: Some(_)
        } if model == "gpt-5.1-codex-max"
    );
}

#[tokio::test]
async fn reasoning_selection_in_plan_mode_without_effort_change_does_not_open_scope_prompt_event() {
    let (mut chat, mut rx, _op_rx) = make_chatwidget_manual(Some("gpt-5.1-codex-max")).await;
    chat.thread_id = Some(ThreadId::new());
    chat.set_feature_enabled(Feature::CollaborationModes, true);
    let plan_mask = collaboration_modes::plan_mask(chat.models_manager.as_ref())
        .expect("expected plan collaboration mode");
    chat.set_collaboration_mask(plan_mask);
    let _ = drain_insert_history(&mut rx);
    set_chatgpt_auth(&mut chat);

    let current_preset = get_available_model(&chat, "gpt-5.1-codex-max");
    chat.set_reasoning_effort(Some(current_preset.default_reasoning_effort));

    let preset = get_available_model(&chat, "gpt-5.1-codex-max");
    chat.open_reasoning_popup(preset);
    chat.handle_key_event(KeyEvent::from(KeyCode::Enter));

    let events = std::iter::from_fn(|| rx.try_recv().ok()).collect::<Vec<_>>();
    assert!(
        events.iter().any(|event| matches!(
            event,
            AppEvent::UpdateModel(model) if model == "gpt-5.1-codex-max"
        )),
        "expected model update event; events: {events:?}"
    );
    assert!(
        events
            .iter()
            .any(|event| matches!(event, AppEvent::UpdateReasoningEffort(Some(_)))),
        "expected reasoning update event; events: {events:?}"
    );
}

#[tokio::test]
async fn reasoning_selection_in_plan_mode_matching_plan_effort_but_different_global_opens_scope_prompt()
 {
    let (mut chat, mut rx, _op_rx) = make_chatwidget_manual(Some("gpt-5.1-codex-max")).await;
    chat.thread_id = Some(ThreadId::new());
    chat.set_feature_enabled(Feature::CollaborationModes, true);
    let plan_mask = collaboration_modes::plan_mask(chat.models_manager.as_ref())
        .expect("expected plan collaboration mode");
    chat.set_collaboration_mask(plan_mask);
    let _ = drain_insert_history(&mut rx);
    set_chatgpt_auth(&mut chat);

    chat.set_reasoning_effort(Some(ReasoningEffortConfig::High));

    let preset = get_available_model(&chat, "gpt-5.1-codex-max");
    chat.open_reasoning_popup(preset);
    chat.handle_key_event(KeyEvent::from(KeyCode::Enter));

    let event = rx.try_recv().expect("expected AppEvent");
    assert_matches!(
        event,
        AppEvent::OpenPlanReasoningScopePrompt {
            model,
            effort: Some(ReasoningEffortConfig::Medium)
        } if model == "gpt-5.1-codex-max"
    );
}

#[tokio::test]
async fn plan_mode_reasoning_override_is_marked_current_in_reasoning_popup() {
    let (mut chat, _rx, _op_rx) = make_chatwidget_manual(Some("gpt-5.1-codex-max")).await;
    chat.set_feature_enabled(Feature::CollaborationModes, true);
    set_chatgpt_auth(&mut chat);
    chat.set_reasoning_effort(Some(ReasoningEffortConfig::High));
    chat.set_plan_mode_reasoning_effort(Some(ReasoningEffortConfig::Low));

    let plan_mask = collaboration_modes::plan_mask(chat.models_manager.as_ref())
        .expect("expected plan collaboration mode");
    chat.set_collaboration_mask(plan_mask);

    let preset = get_available_model(&chat, "gpt-5.1-codex-max");
    chat.open_reasoning_popup(preset);

    let popup = render_bottom_popup(&chat, 100);
    assert!(popup.contains("Low (current)"));
    assert!(
        !popup.contains("High (current)"),
        "expected Plan override to drive current reasoning label, got: {popup}"
    );
}

#[tokio::test]
async fn reasoning_selection_in_plan_mode_model_switch_does_not_open_scope_prompt_event() {
    let (mut chat, mut rx, _op_rx) = make_chatwidget_manual(Some("gpt-5.1-codex-max")).await;
    chat.thread_id = Some(ThreadId::new());
    chat.set_feature_enabled(Feature::CollaborationModes, true);
    let plan_mask = collaboration_modes::plan_mask(chat.models_manager.as_ref())
        .expect("expected plan collaboration mode");
    chat.set_collaboration_mask(plan_mask);
    let _ = drain_insert_history(&mut rx);
    set_chatgpt_auth(&mut chat);

    let preset = get_available_model(&chat, "gpt-5");
    chat.open_reasoning_popup(preset);
    chat.handle_key_event(KeyEvent::from(KeyCode::Enter));

    let events = std::iter::from_fn(|| rx.try_recv().ok()).collect::<Vec<_>>();
    assert!(
        events.iter().any(|event| matches!(
            event,
            AppEvent::UpdateModel(model) if model == "gpt-5"
        )),
        "expected model update event; events: {events:?}"
    );
    assert!(
        events
            .iter()
            .any(|event| matches!(event, AppEvent::UpdateReasoningEffort(Some(_)))),
        "expected reasoning update event; events: {events:?}"
    );
}

#[tokio::test]
async fn plan_reasoning_scope_popup_all_modes_persists_global_and_plan_override() {
    let (mut chat, mut rx, _op_rx) = make_chatwidget_manual(Some("gpt-5.1-codex-max")).await;
    chat.open_plan_reasoning_scope_prompt(
        "gpt-5.1-codex-max".to_string(),
        Some(ReasoningEffortConfig::High),
    );

    chat.handle_key_event(KeyEvent::from(KeyCode::Down));
    chat.handle_key_event(KeyEvent::from(KeyCode::Enter));

    let events = std::iter::from_fn(|| rx.try_recv().ok()).collect::<Vec<_>>();
    assert!(
        events.iter().any(|event| matches!(
            event,
            AppEvent::UpdatePlanModeReasoningEffort(Some(ReasoningEffortConfig::High))
        )),
        "expected plan override to be updated; events: {events:?}"
    );
    assert!(
        events.iter().any(|event| matches!(
            event,
            AppEvent::PersistPlanModeReasoningEffort(Some(ReasoningEffortConfig::High))
        )),
        "expected updated plan override to be persisted; events: {events:?}"
    );
    assert!(
        events.iter().any(|event| matches!(
            event,
            AppEvent::PersistModelSelection { model, effort: Some(ReasoningEffortConfig::High) }
                if model == "gpt-5.1-codex-max"
        )),
        "expected global model reasoning selection persistence; events: {events:?}"
    );
}

#[tokio::test]
async fn plan_reasoning_scope_popup_mentions_selected_reasoning() {
    let (mut chat, _rx, _op_rx) = make_chatwidget_manual(Some("gpt-5.1-codex-max")).await;
    chat.set_plan_mode_reasoning_effort(Some(ReasoningEffortConfig::Low));
    chat.open_plan_reasoning_scope_prompt(
        "gpt-5.1-codex-max".to_string(),
        Some(ReasoningEffortConfig::Medium),
    );

    let popup = render_bottom_popup(&chat, 100);
    assert!(popup.contains("Choose where to apply medium reasoning."));
    assert!(popup.contains("Always use medium reasoning in Plan mode."));
    assert!(popup.contains("Apply to Plan mode override"));
    assert!(popup.contains("Apply to global default and Plan mode override"));
    assert!(popup.contains("user-chosen Plan override (low)"));
}

#[tokio::test]
async fn plan_reasoning_scope_popup_mentions_built_in_plan_default_when_no_override() {
    let (mut chat, _rx, _op_rx) = make_chatwidget_manual(Some("gpt-5.1-codex-max")).await;
    chat.open_plan_reasoning_scope_prompt(
        "gpt-5.1-codex-max".to_string(),
        Some(ReasoningEffortConfig::Medium),
    );

    let popup = render_bottom_popup(&chat, 100);
    assert!(popup.contains("built-in Plan default (medium)"));
}

#[tokio::test]
async fn plan_reasoning_scope_popup_plan_only_does_not_update_all_modes_reasoning() {
    let (mut chat, mut rx, _op_rx) = make_chatwidget_manual(Some("gpt-5.1-codex-max")).await;
    chat.open_plan_reasoning_scope_prompt(
        "gpt-5.1-codex-max".to_string(),
        Some(ReasoningEffortConfig::High),
    );

    chat.handle_key_event(KeyEvent::from(KeyCode::Enter));

    let events = std::iter::from_fn(|| rx.try_recv().ok()).collect::<Vec<_>>();
    assert!(
        events.iter().any(|event| matches!(
            event,
            AppEvent::UpdatePlanModeReasoningEffort(Some(ReasoningEffortConfig::High))
        )),
        "expected plan-only reasoning update; events: {events:?}"
    );
    assert!(
        events
            .iter()
            .all(|event| !matches!(event, AppEvent::UpdateReasoningEffort(_))),
        "did not expect all-modes reasoning update; events: {events:?}"
    );
}

#[tokio::test]
async fn submit_user_message_with_mode_errors_when_mode_changes_during_running_turn() {
    let (mut chat, mut rx, mut op_rx) = make_chatwidget_manual(Some("gpt-5")).await;
    chat.thread_id = Some(ThreadId::new());
    chat.set_feature_enabled(Feature::CollaborationModes, true);
    let plan_mask =
        collaboration_modes::mask_for_kind(chat.models_manager.as_ref(), ModeKind::Plan)
            .expect("expected plan collaboration mask");
    chat.set_collaboration_mask(plan_mask);
    chat.on_task_started();

    let default_mode = collaboration_modes::default_mask(chat.models_manager.as_ref())
        .expect("expected default collaboration mode");
    chat.submit_user_message_with_mode("Implement the plan.".to_string(), default_mode);

    assert_eq!(chat.active_collaboration_mode_kind(), ModeKind::Plan);
    assert!(chat.queued_user_messages.is_empty());
    assert_matches!(op_rx.try_recv(), Err(TryRecvError::Empty));
    let rendered = drain_insert_history(&mut rx)
        .iter()
        .map(|lines| lines_to_single_string(lines))
        .collect::<Vec<_>>()
        .join("\n");
    assert!(
        rendered.contains("Cannot switch collaboration mode while a turn is running."),
        "expected running-turn error message, got: {rendered:?}"
    );
}

#[tokio::test]
async fn submit_user_message_with_mode_allows_same_mode_during_running_turn() {
    let (mut chat, _rx, mut op_rx) = make_chatwidget_manual(Some("gpt-5")).await;
    chat.thread_id = Some(ThreadId::new());
    chat.set_feature_enabled(Feature::CollaborationModes, true);
    let plan_mask =
        collaboration_modes::mask_for_kind(chat.models_manager.as_ref(), ModeKind::Plan)
            .expect("expected plan collaboration mask");
    chat.set_collaboration_mask(plan_mask.clone());
    chat.on_task_started();

    chat.submit_user_message_with_mode("Continue planning.".to_string(), plan_mask);

    assert_eq!(chat.active_collaboration_mode_kind(), ModeKind::Plan);
    assert!(chat.queued_user_messages.is_empty());
    match next_submit_op(&mut op_rx) {
        Op::UserTurn {
            collaboration_mode:
                Some(CollaborationMode {
                    mode: ModeKind::Plan,
                    ..
                }),
            personality: None,
            ..
        } => {}
        other => {
            panic!("expected Op::UserTurn with plan collab mode, got {other:?}")
        }
    }
}

#[tokio::test]
async fn submit_user_message_with_mode_submits_when_plan_stream_is_not_active() {
    let (mut chat, _rx, mut op_rx) = make_chatwidget_manual(Some("gpt-5")).await;
    chat.thread_id = Some(ThreadId::new());
    chat.set_feature_enabled(Feature::CollaborationModes, true);
    let plan_mask =
        collaboration_modes::mask_for_kind(chat.models_manager.as_ref(), ModeKind::Plan)
            .expect("expected plan collaboration mask");
    chat.set_collaboration_mask(plan_mask);

    let default_mode = collaboration_modes::default_mask(chat.models_manager.as_ref())
        .expect("expected default collaboration mode");
    let expected_mode = default_mode
        .mode
        .expect("expected default collaboration mode kind");
    chat.submit_user_message_with_mode("Implement the plan.".to_string(), default_mode);

    assert_eq!(chat.active_collaboration_mode_kind(), expected_mode);
    assert!(chat.queued_user_messages.is_empty());
    match next_submit_op(&mut op_rx) {
        Op::UserTurn {
            collaboration_mode: Some(CollaborationMode { mode, .. }),
            personality: None,
            ..
        } => assert_eq!(mode, expected_mode),
        other => {
            panic!("expected Op::UserTurn with default collab mode, got {other:?}")
        }
    }
}
