use super::*;

#[tokio::test]
async fn empty_enter_during_task_does_not_queue() {
    let (mut chat, _rx, _op_rx) = make_chatwidget_manual(None).await;

    chat.bottom_pane.set_task_running(true);
    chat.handle_key_event(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));

    assert!(chat.queued_user_messages.is_empty());
}

#[tokio::test]
async fn alt_up_edits_most_recent_queued_message() {
    let (mut chat, _rx, _op_rx) = make_chatwidget_manual(None).await;
    chat.queued_message_edit_binding = crate::key_hint::alt(KeyCode::Up);
    chat.bottom_pane
        .set_queued_message_edit_binding(crate::key_hint::alt(KeyCode::Up));
    chat.bottom_pane.set_task_running(true);

    chat.queued_user_messages
        .push_back(UserMessage::from("first queued".to_string()));
    chat.queued_user_messages
        .push_back(UserMessage::from("second queued".to_string()));
    chat.refresh_queued_user_messages();

    chat.handle_key_event(KeyEvent::new(KeyCode::Up, KeyModifiers::ALT));

    assert_eq!(
        chat.bottom_pane.composer_text(),
        "second queued".to_string()
    );
    assert_eq!(chat.queued_user_messages.len(), 1);
    assert_eq!(
        chat.queued_user_messages.front().unwrap().text,
        "first queued"
    );
}

async fn assert_shift_left_edits_most_recent_queued_message_for_terminal(
    terminal_name: TerminalName,
) {
    let (mut chat, _rx, _op_rx) = make_chatwidget_manual(None).await;
    chat.queued_message_edit_binding = queued_message_edit_binding_for_terminal(terminal_name);
    chat.bottom_pane
        .set_queued_message_edit_binding(chat.queued_message_edit_binding);
    chat.bottom_pane.set_task_running(true);

    chat.queued_user_messages
        .push_back(UserMessage::from("first queued".to_string()));
    chat.queued_user_messages
        .push_back(UserMessage::from("second queued".to_string()));
    chat.refresh_queued_user_messages();

    chat.handle_key_event(KeyEvent::new(KeyCode::Left, KeyModifiers::SHIFT));

    assert_eq!(
        chat.bottom_pane.composer_text(),
        "second queued".to_string()
    );
    assert_eq!(chat.queued_user_messages.len(), 1);
    assert_eq!(
        chat.queued_user_messages.front().unwrap().text,
        "first queued"
    );
}

#[tokio::test]
async fn shift_left_edits_most_recent_queued_message_in_apple_terminal() {
    assert_shift_left_edits_most_recent_queued_message_for_terminal(TerminalName::AppleTerminal)
        .await;
}

#[tokio::test]
async fn shift_left_edits_most_recent_queued_message_in_warp_terminal() {
    assert_shift_left_edits_most_recent_queued_message_for_terminal(TerminalName::WarpTerminal)
        .await;
}

#[tokio::test]
async fn shift_left_edits_most_recent_queued_message_in_vscode_terminal() {
    assert_shift_left_edits_most_recent_queued_message_for_terminal(TerminalName::VsCode).await;
}

#[test]
fn queued_message_edit_binding_mapping_covers_special_terminals() {
    assert_eq!(
        queued_message_edit_binding_for_terminal(TerminalName::AppleTerminal),
        crate::key_hint::shift(KeyCode::Left)
    );
    assert_eq!(
        queued_message_edit_binding_for_terminal(TerminalName::WarpTerminal),
        crate::key_hint::shift(KeyCode::Left)
    );
    assert_eq!(
        queued_message_edit_binding_for_terminal(TerminalName::VsCode),
        crate::key_hint::shift(KeyCode::Left)
    );
    assert_eq!(
        queued_message_edit_binding_for_terminal(TerminalName::Iterm2),
        crate::key_hint::alt(KeyCode::Up)
    );
}

#[tokio::test]
async fn enqueueing_history_prompt_multiple_times_is_stable() {
    let (mut chat, _rx, _op_rx) = make_chatwidget_manual(None).await;
    chat.thread_id = Some(ThreadId::new());

    chat.bottom_pane
        .set_composer_text("repeat me".to_string(), Vec::new(), Vec::new());
    chat.handle_key_event(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
    chat.bottom_pane.set_task_running(true);

    for _ in 0..3 {
        chat.handle_key_event(KeyEvent::new(KeyCode::Up, KeyModifiers::NONE));
        assert_eq!(chat.bottom_pane.composer_text(), "repeat me");
        chat.handle_key_event(KeyEvent::new(KeyCode::Tab, KeyModifiers::NONE));
    }

    assert_eq!(chat.queued_user_messages.len(), 3);
    for message in chat.queued_user_messages.iter() {
        assert_eq!(message.text, "repeat me");
    }
}

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
    assert_snapshot!("preamble_keeps_working_status", terminal.backend());
}

#[tokio::test]
async fn unified_exec_begin_restores_status_indicator_after_preamble() {
    let (mut chat, _rx, _op_rx) = make_chatwidget_manual(None).await;

    chat.on_task_started();
    assert_eq!(chat.bottom_pane.status_indicator_visible(), true);

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
    terminal.set_viewport_area(Rect::new(0, 0, width, height));
    terminal
        .draw(|f| chat.render(f.area(), f.buffer_mut()))
        .expect("draw chatwidget");
    assert_snapshot!(
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
