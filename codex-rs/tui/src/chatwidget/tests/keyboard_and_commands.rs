use super::*;
use pretty_assertions::assert_eq;

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
    for message in &chat.queued_user_messages {
        assert_eq!(message.text, "repeat me");
    }
}

#[tokio::test]
async fn ctrl_c_shutdown_works_with_caps_lock() {
    let (mut chat, mut rx, _op_rx) = make_chatwidget_manual(None).await;

    chat.handle_key_event(KeyEvent::new(KeyCode::Char('C'), KeyModifiers::CONTROL));

    assert_matches!(rx.try_recv(), Ok(AppEvent::Exit(ExitMode::ShutdownFirst)));
}

#[tokio::test]
async fn ctrl_d_quits_without_prompt() {
    let (mut chat, mut rx, _op_rx) = make_chatwidget_manual(None).await;

    chat.handle_key_event(KeyEvent::new(KeyCode::Char('d'), KeyModifiers::CONTROL));
    assert_matches!(rx.try_recv(), Ok(AppEvent::Exit(ExitMode::ShutdownFirst)));
}

#[tokio::test]
async fn ctrl_d_with_modal_open_does_not_quit() {
    let (mut chat, mut rx, _op_rx) = make_chatwidget_manual(None).await;

    chat.open_approvals_popup();
    chat.handle_key_event(KeyEvent::new(KeyCode::Char('d'), KeyModifiers::CONTROL));

    assert_matches!(rx.try_recv(), Err(TryRecvError::Empty));
}

#[tokio::test]
async fn ctrl_c_cleared_prompt_is_recoverable_via_history() {
    let (mut chat, _rx, mut op_rx) = make_chatwidget_manual(None).await;

    chat.bottom_pane.insert_str("draft message ");
    chat.bottom_pane
        .attach_image(PathBuf::from("/tmp/preview.png"));
    let placeholder = "[Image #1]";
    assert!(
        chat.bottom_pane.composer_text().ends_with(placeholder),
        "expected placeholder {placeholder:?} in composer text"
    );

    chat.handle_key_event(KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL));
    assert!(chat.bottom_pane.composer_text().is_empty());
    assert_matches!(op_rx.try_recv(), Err(TryRecvError::Empty));
    assert!(!chat.bottom_pane.quit_shortcut_hint_visible());

    chat.handle_key_event(KeyEvent::new(KeyCode::Up, KeyModifiers::NONE));
    let restored_text = chat.bottom_pane.composer_text();
    assert!(
        restored_text.ends_with(placeholder),
        "expected placeholder {placeholder:?} after history recall"
    );
    assert!(restored_text.starts_with("draft message "));
    assert!(!chat.bottom_pane.quit_shortcut_hint_visible());

    let images = chat.bottom_pane.take_recent_submission_images();
    assert_eq!(vec![PathBuf::from("/tmp/preview.png")], images);
}
