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
