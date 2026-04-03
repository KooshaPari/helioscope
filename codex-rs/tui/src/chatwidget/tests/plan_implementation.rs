use super::*;

#[tokio::test]
async fn plan_implementation_popup_snapshot() {
    let (mut chat, _rx, _op_rx) = make_chatwidget_manual(Some("gpt-5")).await;
    chat.open_plan_implementation_prompt();

    let popup = render_bottom_popup(&chat, 80);
    assert_snapshot!("plan_implementation_popup", popup);
}

#[tokio::test]
async fn plan_implementation_popup_no_selected_snapshot() {
    let (mut chat, _rx, _op_rx) = make_chatwidget_manual(Some("gpt-5")).await;
    chat.open_plan_implementation_prompt();
    chat.handle_key_event(KeyEvent::from(KeyCode::Down));

    let popup = render_bottom_popup(&chat, 80);
    assert_snapshot!("plan_implementation_popup_no_selected", popup);
}

#[tokio::test]
async fn plan_implementation_popup_yes_emits_submit_message_event() {
    let (mut chat, mut rx, _op_rx) = make_chatwidget_manual(Some("gpt-5")).await;
    chat.open_plan_implementation_prompt();

    chat.handle_key_event(KeyEvent::from(KeyCode::Enter));

    let event = rx.try_recv().expect("expected AppEvent");
    let AppEvent::SubmitUserMessageWithMode {
        text,
        collaboration_mode,
    } = event
    else {
        panic!("expected SubmitUserMessageWithMode, got {event:?}");
    };
    assert_eq!(text, PLAN_IMPLEMENTATION_CODING_MESSAGE);
    assert_eq!(collaboration_mode.mode, Some(ModeKind::Default));
}

#[tokio::test]
async fn plan_implementation_popup_skips_replayed_turn_complete() {
    let (mut chat, _rx, _op_rx) = make_chatwidget_manual(Some("gpt-5")).await;
    chat.set_feature_enabled(Feature::CollaborationModes, true);
    let plan_mask =
        collaboration_modes::mask_for_kind(chat.models_manager.as_ref(), ModeKind::Plan)
            .expect("expected plan collaboration mask");
    chat.set_collaboration_mask(plan_mask);

    chat.replay_initial_messages(vec![EventMsg::TurnComplete(TurnCompleteEvent {
        turn_id: "turn-1".to_string(),
        last_agent_message: Some("Plan details".to_string()),
    })]);

    let popup = render_bottom_popup(&chat, 80);
    assert!(
        !popup.contains(PLAN_IMPLEMENTATION_TITLE),
        "expected no plan popup for replayed turn, got {popup:?}"
    );
}

#[tokio::test]
async fn plan_implementation_popup_shows_once_when_replay_precedes_live_turn_complete() {
    let (mut chat, _rx, _op_rx) = make_chatwidget_manual(Some("gpt-5")).await;
    chat.set_feature_enabled(Feature::CollaborationModes, true);
    let plan_mask =
        collaboration_modes::mask_for_kind(chat.models_manager.as_ref(), ModeKind::Plan)
            .expect("expected plan collaboration mask");
    chat.set_collaboration_mask(plan_mask);

    chat.on_task_started();
    chat.on_plan_delta("- Step 1\n- Step 2\n".to_string());
    chat.on_plan_item_completed("- Step 1\n- Step 2\n".to_string());

    chat.replay_initial_messages(vec![EventMsg::TurnComplete(TurnCompleteEvent {
        turn_id: "turn-1".to_string(),
        last_agent_message: Some("Plan details".to_string()),
    })]);
    let replay_popup = render_bottom_popup(&chat, 80);
    assert!(
        !replay_popup.contains(PLAN_IMPLEMENTATION_TITLE),
        "expected no prompt for replayed turn completion, got {replay_popup:?}"
    );

    chat.handle_codex_event(Event {
        id: "live-turn-complete-1".to_string(),
        msg: EventMsg::TurnComplete(TurnCompleteEvent {
            turn_id: "turn-1".to_string(),
            last_agent_message: Some("Plan details".to_string()),
        }),
    });

    let popup = render_bottom_popup(&chat, 80);
    assert!(
        popup.contains(PLAN_IMPLEMENTATION_TITLE),
        "expected prompt for first live turn completion after replay, got {popup:?}"
    );

    chat.handle_key_event(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE));
    let dismissed_popup = render_bottom_popup(&chat, 80);
    assert!(
        !dismissed_popup.contains(PLAN_IMPLEMENTATION_TITLE),
        "expected prompt to dismiss on Esc, got {dismissed_popup:?}"
    );

    chat.handle_codex_event(Event {
        id: "live-turn-complete-2".to_string(),
        msg: EventMsg::TurnComplete(TurnCompleteEvent {
            turn_id: "turn-1".to_string(),
            last_agent_message: Some("Plan details".to_string()),
        }),
    });
    let duplicate_popup = render_bottom_popup(&chat, 80);
    assert!(
        !duplicate_popup.contains(PLAN_IMPLEMENTATION_TITLE),
        "expected no prompt for duplicate live completion, got {duplicate_popup:?}"
    );
}

#[tokio::test]
async fn replayed_thread_rollback_emits_ordered_app_event() {
    let (mut chat, mut rx, _op_rx) = make_chatwidget_manual(Some("gpt-5")).await;

    chat.replay_initial_messages(vec![EventMsg::ThreadRolledBack(ThreadRolledBackEvent {
        num_turns: 2,
    })]);

    let mut saw = false;
    while let Ok(event) = rx.try_recv() {
        if let AppEvent::ApplyThreadRollback { num_turns } = event {
            saw = true;
            assert_eq!(num_turns, 2);
            break;
        }
    }

    assert!(saw, "expected replay rollback app event");
}

#[tokio::test]
async fn plan_implementation_popup_skips_when_messages_queued() {
    let (mut chat, _rx, _op_rx) = make_chatwidget_manual(Some("gpt-5")).await;
    chat.set_feature_enabled(Feature::CollaborationModes, true);
    let plan_mask =
        collaboration_modes::mask_for_kind(chat.models_manager.as_ref(), ModeKind::Plan)
            .expect("expected plan collaboration mask");
    chat.set_collaboration_mask(plan_mask);
    chat.bottom_pane.set_task_running(true);
    chat.queue_user_message("Queued message".into());

    chat.on_task_complete(Some("Plan details".to_string()), false);

    let popup = render_bottom_popup(&chat, 80);
    assert!(
        !popup.contains(PLAN_IMPLEMENTATION_TITLE),
        "expected no plan popup with queued messages, got {popup:?}"
    );
}

#[tokio::test]
async fn plan_implementation_popup_skips_without_proposed_plan() {
    let (mut chat, _rx, _op_rx) = make_chatwidget_manual(Some("gpt-5")).await;
    chat.set_feature_enabled(Feature::CollaborationModes, true);
    let plan_mask =
        collaboration_modes::mask_for_kind(chat.models_manager.as_ref(), ModeKind::Plan)
            .expect("expected plan collaboration mask");
    chat.set_collaboration_mask(plan_mask);

    chat.on_task_started();
    chat.on_plan_update(UpdatePlanArgs {
        explanation: None,
        plan: vec![PlanItemArg {
            step: "First".to_string(),
            status: StepStatus::Pending,
        }],
    });
    chat.on_task_complete(None, false);

    let popup = render_bottom_popup(&chat, 80);
    assert!(
        !popup.contains(PLAN_IMPLEMENTATION_TITLE),
        "expected no plan popup without proposed plan output, got {popup:?}"
    );
}

#[tokio::test]
async fn plan_implementation_popup_shows_after_proposed_plan_output() {
    let (mut chat, _rx, _op_rx) = make_chatwidget_manual(Some("gpt-5")).await;
    chat.set_feature_enabled(Feature::CollaborationModes, true);
    let plan_mask =
        collaboration_modes::mask_for_kind(chat.models_manager.as_ref(), ModeKind::Plan)
            .expect("expected plan collaboration mask");
    chat.set_collaboration_mask(plan_mask);

    chat.on_task_started();
    chat.on_plan_delta("- Step 1\n- Step 2\n".to_string());
    chat.on_plan_item_completed("- Step 1\n- Step 2\n".to_string());
    chat.on_task_complete(None, false);

    let popup = render_bottom_popup(&chat, 80);
    assert!(
        popup.contains(PLAN_IMPLEMENTATION_TITLE),
        "expected plan popup after proposed plan output, got {popup:?}"
    );
}

#[tokio::test]
async fn plan_implementation_popup_skips_when_rate_limit_prompt_pending() {
    let (mut chat, _rx, _op_rx) = make_chatwidget_manual(Some("gpt-5")).await;
    chat.auth_manager = codex_core::test_support::auth_manager_from_auth(
        CodexAuth::create_dummy_chatgpt_auth_for_testing(),
    );
    chat.set_feature_enabled(Feature::CollaborationModes, true);
    let plan_mask =
        collaboration_modes::mask_for_kind(chat.models_manager.as_ref(), ModeKind::Plan)
            .expect("expected plan collaboration mask");
    chat.set_collaboration_mask(plan_mask);

    chat.on_task_started();
    chat.on_plan_update(UpdatePlanArgs {
        explanation: None,
        plan: vec![PlanItemArg {
            step: "First".to_string(),
            status: StepStatus::Pending,
        }],
    });
    chat.on_rate_limit_snapshot(Some(snapshot(92.0)));
    chat.on_task_complete(None, false);

    let popup = render_bottom_popup(&chat, 80);
    assert!(
        popup.contains("Approaching rate limits"),
        "expected rate limit popup, got {popup:?}"
    );
    assert!(
        !popup.contains(PLAN_IMPLEMENTATION_TITLE),
        "expected plan popup to be skipped, got {popup:?}"
    );
}
