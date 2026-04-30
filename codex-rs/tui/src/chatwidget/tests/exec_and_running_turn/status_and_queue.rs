#[tokio::test]
async fn streaming_final_answer_keeps_task_running_state() {
    let (mut chat, mut rx, mut op_rx) = make_chatwidget_manual(None).await;
    chat.thread_id = Some(ThreadId::new());

    chat.on_task_started();
    chat.on_agent_message_delta("Final answer line\n".to_string());
    chat.on_commit_tick();
    drain_insert_history(&mut rx);

    assert!(chat.bottom_pane.is_task_running());
    assert!(!chat.bottom_pane.status_indicator_visible());

    chat.bottom_pane
        .set_composer_text("queued submission".to_string(), Vec::new(), Vec::new());
    chat.handle_key_event(KeyEvent::new(KeyCode::Tab, KeyModifiers::NONE));

    assert_eq!(chat.queued_user_messages.len(), 1);
    assert_eq!(
        chat.queued_user_messages.front().unwrap().text,
        "queued submission"
    );
    assert_matches!(op_rx.try_recv(), Err(TryRecvError::Empty));

    chat.handle_key_event(KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL));
    match op_rx.try_recv() {
        Ok(Op::Interrupt) => {}
        other => panic!("expected Op::Interrupt, got {other:?}"),
    }
    assert!(!chat.bottom_pane.quit_shortcut_hint_visible());
}

#[tokio::test]
async fn idle_commit_ticks_do_not_restore_status_without_commentary_completion() {
    let (mut chat, mut rx, _op_rx) = make_chatwidget_manual(None).await;

    chat.on_task_started();
    assert_eq!(chat.bottom_pane.status_indicator_visible(), true);

    chat.on_agent_message_delta("Final answer line\n".to_string());
    chat.on_commit_tick();
    drain_insert_history(&mut rx);

    assert_eq!(chat.bottom_pane.status_indicator_visible(), false);
    assert_eq!(chat.bottom_pane.is_task_running(), true);

    // A second idle tick should not toggle the row back on and cause jitter.
    chat.on_commit_tick();
    assert_eq!(chat.bottom_pane.status_indicator_visible(), false);
}

#[tokio::test]
async fn commentary_completion_restores_status_indicator_before_exec_begin() {
    let (mut chat, mut rx, _op_rx) = make_chatwidget_manual(None).await;

    chat.on_task_started();
    assert_eq!(chat.bottom_pane.status_indicator_visible(), true);

    chat.on_agent_message_delta("Preamble line\n".to_string());
    chat.on_commit_tick();
    drain_insert_history(&mut rx);

    assert_eq!(chat.bottom_pane.status_indicator_visible(), false);

    complete_assistant_message(
        &mut chat,
        "msg-commentary",
        "Preamble line\n",
        Some(MessagePhase::Commentary),
    );

    assert_eq!(chat.bottom_pane.status_indicator_visible(), true);
    assert_eq!(chat.bottom_pane.is_task_running(), true);

    begin_exec(&mut chat, "call-1", "echo hi");
    assert_eq!(chat.bottom_pane.status_indicator_visible(), true);
}

#[tokio::test]
async fn plan_completion_restores_status_indicator_after_streaming_plan_output() {
    let (mut chat, mut rx, _op_rx) = make_chatwidget_manual(None).await;
    chat.set_feature_enabled(Feature::CollaborationModes, true);
    let plan_mask =
        collaboration_modes::mask_for_kind(chat.models_manager.as_ref(), ModeKind::Plan)
            .expect("expected plan collaboration mask");
    chat.set_collaboration_mask(plan_mask);

    chat.on_task_started();
    assert_eq!(chat.bottom_pane.status_indicator_visible(), true);

    chat.on_plan_delta("- Step 1\n".to_string());
    chat.on_commit_tick();
    drain_insert_history(&mut rx);

    assert_eq!(chat.bottom_pane.status_indicator_visible(), false);
    assert_eq!(chat.bottom_pane.is_task_running(), true);

    chat.on_plan_item_completed("- Step 1\n".to_string());

    assert_eq!(chat.bottom_pane.status_indicator_visible(), true);
    assert_eq!(chat.bottom_pane.is_task_running(), true);
}

#[tokio::test]
async fn preamble_keeps_working_status_snapshot() {
    let (mut chat, mut rx, _op_rx) = make_chatwidget_manual(None).await;
    chat.thread_id = Some(ThreadId::new());

    // Regression sequence: a preamble line is committed to history before any exec/tool event.
    // After commentary completes, the status row should be restored before subsequent work.
    chat.on_task_started();
    chat.on_agent_message_delta("Preamble line\n".to_string());
    chat.on_commit_tick();
    drain_insert_history(&mut rx);
    complete_assistant_message(
        &mut chat,
        "msg-commentary-snapshot",
        "Preamble line\n",
        Some(MessagePhase::Commentary),
    );

    let height = chat.desired_height(80);
    let mut terminal = ratatui::Terminal::new(ratatui::backend::TestBackend::new(80, height))
        .expect("create terminal");
    terminal
        .draw(|f| chat.render(f.area(), f.buffer_mut()))
        .expect("draw preamble + status widget");
    assert_exec_running_turn_snapshot!("preamble_keeps_working_status", terminal.backend());
}

#[tokio::test]
async fn unified_exec_begin_restores_status_indicator_after_preamble() {
    let (mut chat, _rx, _op_rx) = make_chatwidget_manual(None).await;

    chat.on_task_started();
    assert_eq!(chat.bottom_pane.status_indicator_visible(), true);

    // Simulate a hidden status row during an active turn.
    chat.bottom_pane.hide_status_indicator();
    assert_eq!(chat.bottom_pane.status_indicator_visible(), false);
    assert_eq!(chat.bottom_pane.is_task_running(), true);

    begin_unified_exec_startup(&mut chat, "call-1", "proc-1", "sleep 2");

    assert_eq!(chat.bottom_pane.status_indicator_visible(), true);
}

#[tokio::test]
async fn unified_exec_begin_restores_working_status_snapshot() {
    let (mut chat, mut rx, _op_rx) = make_chatwidget_manual(None).await;

    chat.on_task_started();
    chat.on_agent_message_delta("Preamble line\n".to_string());
    chat.on_commit_tick();
    drain_insert_history(&mut rx);

    begin_unified_exec_startup(&mut chat, "call-1", "proc-1", "sleep 2");

    let width: u16 = 80;
    let height = chat.desired_height(width);
    let mut terminal = ratatui::Terminal::new(ratatui::backend::TestBackend::new(width, height))
        .expect("create terminal");
    terminal
        .draw(|f| chat.render(f.area(), f.buffer_mut()))
        .expect("draw chatwidget");
    assert_exec_running_turn_snapshot!(
        "unified_exec_begin_restores_working_status",
        terminal.backend()
    );
}

#[tokio::test]
async fn enter_queues_while_plan_stream_is_active() {
    let (mut chat, _rx, mut op_rx) = make_chatwidget_manual(None).await;
    chat.thread_id = Some(ThreadId::new());
    chat.set_feature_enabled(Feature::CollaborationModes, true);
    let plan_mask =
        collaboration_modes::mask_for_kind(chat.models_manager.as_ref(), ModeKind::Plan)
            .expect("expected plan collaboration mask");
    chat.set_collaboration_mask(plan_mask);
    chat.on_task_started();
    chat.on_plan_delta("- Step 1".to_string());

    chat.bottom_pane
        .set_composer_text("queued submission".to_string(), Vec::new(), Vec::new());
    chat.handle_key_event(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));

    assert_eq!(chat.active_collaboration_mode_kind(), ModeKind::Plan);
    assert_eq!(chat.queued_user_messages.len(), 1);
    assert_eq!(
        chat.queued_user_messages.front().unwrap().text,
        "queued submission"
    );
    assert_matches!(op_rx.try_recv(), Err(TryRecvError::Empty));
}

#[tokio::test]
async fn steer_enter_queues_while_final_answer_stream_is_active() {
    let (mut chat, _rx, mut op_rx) = make_chatwidget_manual(None).await;
    chat.thread_id = Some(ThreadId::new());
    chat.on_task_started();
    // Keep the assistant stream open (no commit tick/finalize) to model the repro window:
    // user presses Enter while the final answer is still streaming.
    chat.on_agent_message_delta("Final answer line\n".to_string());

    chat.bottom_pane.set_composer_text(
        "queued while streaming".to_string(),
        Vec::new(),
        Vec::new(),
    );
    chat.handle_key_event(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));

    assert_eq!(chat.queued_user_messages.len(), 1);
    assert_eq!(
        chat.queued_user_messages.front().unwrap().text,
        "queued while streaming"
    );
    assert_no_submit_op(&mut op_rx);

    // Once final output ends, the queued input must be submitted automatically.
    chat.on_task_complete(None, false);

    assert!(chat.queued_user_messages.is_empty());
    match next_submit_op(&mut op_rx) {
        Op::UserTurn { .. } => {}
        other => panic!("expected Op::UserTurn after stream completion, got {other:?}"),
    }
}

#[tokio::test]
async fn steer_enter_during_final_stream_preserves_follow_up_prompts_in_order() {
    let (mut chat, _rx, mut op_rx) = make_chatwidget_manual(None).await;
    chat.thread_id = Some(ThreadId::new());
    chat.on_task_started();
    // Simulate "dead mode" repro timing by keeping a final-answer stream active while the
    // user submits multiple follow-up prompts.
    chat.on_agent_message_delta("Final answer line\n".to_string());

    chat.bottom_pane
        .set_composer_text("first follow-up".to_string(), Vec::new(), Vec::new());
    chat.handle_key_event(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
    chat.bottom_pane
        .set_composer_text("second follow-up".to_string(), Vec::new(), Vec::new());
    chat.handle_key_event(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));

    assert_eq!(chat.queued_user_messages.len(), 2);
    assert_eq!(
        chat.queued_user_messages.front().unwrap().text,
        "first follow-up"
    );
    assert_eq!(
        chat.queued_user_messages.back().unwrap().text,
        "second follow-up"
    );
    assert_no_submit_op(&mut op_rx);

    // Completion must recover by submitting the oldest queued prompt first.
    chat.on_task_complete(None, false);

    let first_items = match next_submit_op(&mut op_rx) {
        Op::UserTurn { items, .. } => items,
        other => panic!("expected Op::UserTurn, got {other:?}"),
    };
    assert_eq!(
        first_items,
        vec![UserInput::Text {
            text: "first follow-up".to_string(),
            text_elements: Vec::new(),
        }]
    );
    assert_eq!(chat.queued_user_messages.len(), 1);
    assert_eq!(
        chat.queued_user_messages.front().unwrap().text,
        "second follow-up"
    );

    // A subsequent turn lifecycle should continue draining remaining queued prompts, proving
    // the widget did not enter a permanently stuck state.
    chat.on_task_started();
    chat.on_task_complete(None, false);

    let second_items = match next_submit_op(&mut op_rx) {
        Op::UserTurn { items, .. } => items,
        other => panic!("expected Op::UserTurn, got {other:?}"),
    };
    assert_eq!(
        second_items,
        vec![UserInput::Text {
            text: "second follow-up".to_string(),
            text_elements: Vec::new(),
        }]
    );
    assert!(chat.queued_user_messages.is_empty());
}

#[tokio::test]
async fn enter_submits_when_plan_stream_is_not_active() {
    let (mut chat, _rx, mut op_rx) = make_chatwidget_manual(None).await;
    chat.thread_id = Some(ThreadId::new());
    chat.set_feature_enabled(Feature::CollaborationModes, true);
    let plan_mask =
        collaboration_modes::mask_for_kind(chat.models_manager.as_ref(), ModeKind::Plan)
            .expect("expected plan collaboration mask");
    chat.set_collaboration_mask(plan_mask);
    chat.on_task_started();

    chat.bottom_pane
        .set_composer_text("submitted immediately".to_string(), Vec::new(), Vec::new());
    chat.handle_key_event(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));

    assert!(chat.queued_user_messages.is_empty());
    match next_submit_op(&mut op_rx) {
        Op::UserTurn {
            personality: Some(Personality::Pragmatic),
            ..
        } => {}
        other => panic!("expected Op::UserTurn, got {other:?}"),
    }
}
