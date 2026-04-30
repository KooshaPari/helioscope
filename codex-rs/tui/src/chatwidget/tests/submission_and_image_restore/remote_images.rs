#[tokio::test]
async fn enter_with_only_remote_images_submits_user_turn() {
    let (mut chat, mut rx, mut op_rx) = make_chatwidget_manual(None).await;

    let conversation_id = ThreadId::new();
    let rollout_file = NamedTempFile::new().unwrap();
    let configured = codex_protocol::protocol::SessionConfiguredEvent {
        session_id: conversation_id,
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
        rollout_path: Some(rollout_file.path().to_path_buf()),
    };
    chat.handle_codex_event(Event {
        id: "initial".into(),
        msg: EventMsg::SessionConfigured(configured),
    });
    drain_insert_history(&mut rx);

    let remote_url = "https://example.com/remote-only.png".to_string();
    chat.set_remote_image_urls(vec![remote_url.clone()]);
    assert_eq!(chat.bottom_pane.composer_text(), "");

    chat.handle_key_event(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));

    let (items, summary) = match next_submit_op(&mut op_rx) {
        Op::UserTurn { items, summary, .. } => (items, summary),
        other => panic!("expected Op::UserTurn, got {other:?}"),
    };
    assert_eq!(
        items,
        vec![UserInput::Image {
            image_url: remote_url.clone(),
        }]
    );
    assert_eq!(summary, None);
    assert!(chat.remote_image_urls().is_empty());

    let mut user_cell = None;
    while let Ok(ev) = rx.try_recv() {
        if let AppEvent::InsertHistoryCell(cell) = ev
            && let Some(cell) = cell.as_any().downcast_ref::<UserHistoryCell>()
        {
            user_cell = Some((cell.message.clone(), cell.remote_image_urls.clone()));
            break;
        }
    }

    let (stored_message, stored_remote_image_urls) =
        user_cell.expect("expected submitted user history cell");
    assert_eq!(stored_message, String::new());
    assert_eq!(stored_remote_image_urls, vec![remote_url]);
}

#[tokio::test]
async fn shift_enter_with_only_remote_images_does_not_submit_user_turn() {
    let (mut chat, mut rx, mut op_rx) = make_chatwidget_manual(None).await;

    let conversation_id = ThreadId::new();
    let rollout_file = NamedTempFile::new().unwrap();
    let configured = codex_protocol::protocol::SessionConfiguredEvent {
        session_id: conversation_id,
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
        rollout_path: Some(rollout_file.path().to_path_buf()),
    };
    chat.handle_codex_event(Event {
        id: "initial".into(),
        msg: EventMsg::SessionConfigured(configured),
    });
    drain_insert_history(&mut rx);

    let remote_url = "https://example.com/remote-only.png".to_string();
    chat.set_remote_image_urls(vec![remote_url.clone()]);
    assert_eq!(chat.bottom_pane.composer_text(), "");

    chat.handle_key_event(KeyEvent::new(KeyCode::Enter, KeyModifiers::SHIFT));

    assert_no_submit_op(&mut op_rx);
    assert_eq!(chat.remote_image_urls(), vec![remote_url]);
}

#[tokio::test]
async fn enter_with_only_remote_images_does_not_submit_when_modal_is_active() {
    let (mut chat, mut rx, mut op_rx) = make_chatwidget_manual(None).await;

    let conversation_id = ThreadId::new();
    let rollout_file = NamedTempFile::new().unwrap();
    let configured = codex_protocol::protocol::SessionConfiguredEvent {
        session_id: conversation_id,
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
        rollout_path: Some(rollout_file.path().to_path_buf()),
    };
    chat.handle_codex_event(Event {
        id: "initial".into(),
        msg: EventMsg::SessionConfigured(configured),
    });
    drain_insert_history(&mut rx);

    let remote_url = "https://example.com/remote-only.png".to_string();
    chat.set_remote_image_urls(vec![remote_url.clone()]);

    chat.open_review_popup();
    chat.handle_key_event(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));

    assert_eq!(chat.remote_image_urls(), vec![remote_url]);
    assert_no_submit_op(&mut op_rx);
}

#[tokio::test]
async fn enter_with_only_remote_images_does_not_submit_when_input_disabled() {
    let (mut chat, mut rx, mut op_rx) = make_chatwidget_manual(None).await;

    let conversation_id = ThreadId::new();
    let rollout_file = NamedTempFile::new().unwrap();
    let configured = codex_protocol::protocol::SessionConfiguredEvent {
        session_id: conversation_id,
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
        rollout_path: Some(rollout_file.path().to_path_buf()),
    };
    chat.handle_codex_event(Event {
        id: "initial".into(),
        msg: EventMsg::SessionConfigured(configured),
    });
    drain_insert_history(&mut rx);

    let remote_url = "https://example.com/remote-only.png".to_string();
    chat.set_remote_image_urls(vec![remote_url.clone()]);
    chat.bottom_pane
        .set_composer_input_enabled(false, Some("Input disabled for test.".to_string()));

    chat.handle_key_event(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));

    assert_eq!(chat.remote_image_urls(), vec![remote_url]);
    assert_no_submit_op(&mut op_rx);
}
