#[tokio::test]
async fn submission_preserves_text_elements_and_local_images() {
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

    let placeholder = "[Image #1]";
    let text = format!("{placeholder} submit");
    let text_elements = vec![TextElement::new(
        (0..placeholder.len()).into(),
        Some(placeholder.to_string()),
    )];
    let local_images = vec![PathBuf::from("/tmp/submitted.png")];

    chat.bottom_pane
        .set_composer_text(text.clone(), text_elements.clone(), local_images.clone());
    chat.handle_key_event(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));

    let items = match next_submit_op(&mut op_rx) {
        Op::UserTurn { items, .. } => items,
        other => panic!("expected Op::UserTurn, got {other:?}"),
    };
    assert_eq!(items.len(), 2);
    assert_eq!(
        items[0],
        UserInput::LocalImage {
            path: local_images[0].clone()
        }
    );
    assert_eq!(
        items[1],
        UserInput::Text {
            text: text.clone(),
            text_elements: text_elements.clone(),
        }
    );

    let mut user_cell = None;
    while let Ok(ev) = rx.try_recv() {
        if let AppEvent::InsertHistoryCell(cell) = ev
            && let Some(cell) = cell.as_any().downcast_ref::<UserHistoryCell>()
        {
            user_cell = Some((
                cell.message.clone(),
                cell.text_elements.clone(),
                cell.local_image_paths.clone(),
                cell.remote_image_urls.clone(),
            ));
            break;
        }
    }

    let (stored_message, stored_elements, stored_images, stored_remote_image_urls) =
        user_cell.expect("expected submitted user history cell");
    assert_eq!(stored_message, text);
    assert_eq!(stored_elements, text_elements);
    assert_eq!(stored_images, local_images);
    assert!(stored_remote_image_urls.is_empty());
}

#[tokio::test]
async fn submission_with_remote_and_local_images_keeps_local_placeholder_numbering() {
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

    let remote_url = "https://example.com/remote.png".to_string();
    chat.set_remote_image_urls(vec![remote_url.clone()]);

    let placeholder = "[Image #2]";
    let text = format!("{placeholder} submit mixed");
    let text_elements = vec![TextElement::new(
        (0..placeholder.len()).into(),
        Some(placeholder.to_string()),
    )];
    let local_images = vec![PathBuf::from("/tmp/submitted-mixed.png")];

    chat.bottom_pane
        .set_composer_text(text.clone(), text_elements.clone(), local_images.clone());
    assert_eq!(chat.bottom_pane.composer_text(), "[Image #2] submit mixed");
    chat.handle_key_event(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));

    let items = match next_submit_op(&mut op_rx) {
        Op::UserTurn { items, .. } => items,
        other => panic!("expected Op::UserTurn, got {other:?}"),
    };
    assert_eq!(items.len(), 3);
    assert_eq!(
        items[0],
        UserInput::Image {
            image_url: remote_url.clone(),
        }
    );
    assert_eq!(
        items[1],
        UserInput::LocalImage {
            path: local_images[0].clone(),
        }
    );
    assert_eq!(
        items[2],
        UserInput::Text {
            text: text.clone(),
            text_elements: text_elements.clone(),
        }
    );
    assert_eq!(text_elements[0].placeholder(&text), Some("[Image #2]"));

    let mut user_cell = None;
    while let Ok(ev) = rx.try_recv() {
        if let AppEvent::InsertHistoryCell(cell) = ev
            && let Some(cell) = cell.as_any().downcast_ref::<UserHistoryCell>()
        {
            user_cell = Some((
                cell.message.clone(),
                cell.text_elements.clone(),
                cell.local_image_paths.clone(),
                cell.remote_image_urls.clone(),
            ));
            break;
        }
    }

    let (stored_message, stored_elements, stored_images, stored_remote_image_urls) =
        user_cell.expect("expected submitted user history cell");
    assert_eq!(stored_message, text);
    assert_eq!(stored_elements, text_elements);
    assert_eq!(stored_images, local_images);
    assert_eq!(stored_remote_image_urls, vec![remote_url]);
}
