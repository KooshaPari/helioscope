use super::*;
use codex_protocol::protocol::UserMessageEvent;
use pretty_assertions::assert_eq;

#[tokio::test]
async fn resumed_initial_messages_render_history() {
    let (mut chat, mut rx, _ops) = make_chatwidget_manual(None).await;

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
        initial_messages: Some(vec![
            EventMsg::UserMessage(UserMessageEvent {
                message: "hello from user".to_string(),
                images: None,
                text_elements: Vec::new(),
                local_images: Vec::new(),
            }),
            EventMsg::AgentMessage(AgentMessageEvent {
                message: "assistant reply".to_string(),
                phase: None,
            }),
        ]),
        network_proxy: None,
        rollout_path: Some(rollout_file.path().to_path_buf()),
    };

    chat.handle_codex_event(Event {
        id: "initial".into(),
        msg: EventMsg::SessionConfigured(configured),
    });

    let cells = drain_insert_history(&mut rx);
    let mut merged_lines = Vec::new();
    for lines in cells {
        let text = lines
            .iter()
            .flat_map(|line| line.spans.iter())
            .map(|span| span.content.clone())
            .collect::<String>();
        merged_lines.push(text);
    }

    let text_blob = merged_lines.join("\n");
    assert!(
        text_blob.contains("hello from user"),
        "expected replayed user message",
    );
    assert!(
        text_blob.contains("assistant reply"),
        "expected replayed agent message",
    );
}

#[tokio::test]
async fn replayed_user_message_preserves_text_elements_and_local_images() {
    let (mut chat, mut rx, _ops) = make_chatwidget_manual(None).await;

    let placeholder = "[Image #1]";
    let message = format!("{placeholder} replayed");
    let text_elements = vec![TextElement::new(
        (0..placeholder.len()).into(),
        Some(placeholder.to_string()),
    )];
    let local_images = vec![PathBuf::from("/tmp/replay.png")];

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
        initial_messages: Some(vec![EventMsg::UserMessage(UserMessageEvent {
            message: message.clone(),
            images: None,
            text_elements: text_elements.clone(),
            local_images: local_images.clone(),
        })]),
        network_proxy: None,
        rollout_path: Some(rollout_file.path().to_path_buf()),
    };

    chat.handle_codex_event(Event {
        id: "initial".into(),
        msg: EventMsg::SessionConfigured(configured),
    });

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
        user_cell.expect("expected a replayed user history cell");
    assert_eq!(stored_message, message);
    assert_eq!(stored_elements, text_elements);
    assert_eq!(stored_images, local_images);
    assert!(stored_remote_image_urls.is_empty());
}

#[tokio::test]
async fn replayed_user_message_preserves_remote_image_urls() {
    let (mut chat, mut rx, _ops) = make_chatwidget_manual(None).await;

    let message = "replayed with remote image".to_string();
    let remote_image_urls = vec!["https://example.com/image.png".to_string()];

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
        initial_messages: Some(vec![EventMsg::UserMessage(UserMessageEvent {
            message: message.clone(),
            images: Some(remote_image_urls.clone()),
            text_elements: Vec::new(),
            local_images: Vec::new(),
        })]),
        network_proxy: None,
        rollout_path: Some(rollout_file.path().to_path_buf()),
    };

    chat.handle_codex_event(Event {
        id: "initial".into(),
        msg: EventMsg::SessionConfigured(configured),
    });

    let mut user_cell = None;
    while let Ok(ev) = rx.try_recv() {
        if let AppEvent::InsertHistoryCell(cell) = ev
            && let Some(cell) = cell.as_any().downcast_ref::<UserHistoryCell>()
        {
            user_cell = Some((
                cell.message.clone(),
                cell.local_image_paths.clone(),
                cell.remote_image_urls.clone(),
            ));
            break;
        }
    }

    let (stored_message, stored_local_images, stored_remote_image_urls) =
        user_cell.expect("expected a replayed user history cell");
    assert_eq!(stored_message, message);
    assert!(stored_local_images.is_empty());
    assert_eq!(stored_remote_image_urls, remote_image_urls);
}

#[tokio::test]
async fn replayed_user_message_with_only_remote_images_renders_history_cell() {
    let (mut chat, mut rx, _ops) = make_chatwidget_manual(None).await;

    let remote_image_urls = vec!["https://example.com/remote-only.png".to_string()];

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
        initial_messages: Some(vec![EventMsg::UserMessage(UserMessageEvent {
            message: String::new(),
            images: Some(remote_image_urls.clone()),
            text_elements: Vec::new(),
            local_images: Vec::new(),
        })]),
        network_proxy: None,
        rollout_path: Some(rollout_file.path().to_path_buf()),
    };

    chat.handle_codex_event(Event {
        id: "initial".into(),
        msg: EventMsg::SessionConfigured(configured),
    });

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
        user_cell.expect("expected a replayed remote-image-only user history cell");
    assert!(stored_message.is_empty());
    assert_eq!(stored_remote_image_urls, remote_image_urls);
}

#[tokio::test]
async fn replayed_user_message_with_only_local_images_does_not_render_history_cell() {
    let (mut chat, mut rx, _ops) = make_chatwidget_manual(None).await;

    let local_images = vec![PathBuf::from("/tmp/replay-local-only.png")];

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
        initial_messages: Some(vec![EventMsg::UserMessage(UserMessageEvent {
            message: String::new(),
            images: None,
            text_elements: Vec::new(),
            local_images,
        })]),
        network_proxy: None,
        rollout_path: Some(rollout_file.path().to_path_buf()),
    };

    chat.handle_codex_event(Event {
        id: "initial".into(),
        msg: EventMsg::SessionConfigured(configured),
    });

    let mut found_user_history_cell = false;
    while let Ok(ev) = rx.try_recv() {
        if let AppEvent::InsertHistoryCell(cell) = ev
            && cell.as_any().downcast_ref::<UserHistoryCell>().is_some()
        {
            found_user_history_cell = true;
            break;
        }
    }

    assert!(!found_user_history_cell);
}

#[tokio::test]
async fn forked_thread_history_line_includes_name_and_id_snapshot() {
    let (chat, mut rx, _op_rx) = make_chatwidget_manual(None).await;
    let mut chat = chat;
    let temp = tempdir().expect("tempdir");
    chat.config.codex_home = temp.path().to_path_buf();

    let forked_from_id =
        ThreadId::from_string("e9f18a88-8081-4e51-9d4e-8af5cde2d8dd").expect("forked id");
    let session_index_entry = format!(
        "{{\"id\":\"{forked_from_id}\",\"thread_name\":\"named-thread\",\"updated_at\":\"2024-01-02T00:00:00Z\"}}\n"
    );
    std::fs::write(temp.path().join("session_index.jsonl"), session_index_entry)
        .expect("write session index");

    chat.emit_forked_thread_event(forked_from_id);

    let history_cell = tokio::time::timeout(std::time::Duration::from_secs(2), async {
        loop {
            match rx.recv().await {
                Some(AppEvent::InsertHistoryCell(cell)) => break cell,
                Some(_) => continue,
                None => panic!("app event channel closed before forked thread history was emitted"),
            }
        }
    })
    .await
    .expect("timed out waiting for forked thread history");
    let combined = lines_to_single_string(&history_cell.display_lines(80));

    assert!(
        combined.contains("Thread forked from"),
        "expected forked thread message in history"
    );
    assert_snapshot!("forked_thread_history_line", combined);
}

#[tokio::test]
async fn forked_thread_history_line_without_name_shows_id_once_snapshot() {
    let (chat, mut rx, _op_rx) = make_chatwidget_manual(None).await;
    let mut chat = chat;
    let temp = tempdir().expect("tempdir");
    chat.config.codex_home = temp.path().to_path_buf();

    let forked_from_id =
        ThreadId::from_string("019c2d47-4935-7423-a190-05691f566092").expect("forked id");
    chat.emit_forked_thread_event(forked_from_id);

    let history_cell = tokio::time::timeout(std::time::Duration::from_secs(2), async {
        loop {
            match rx.recv().await {
                Some(AppEvent::InsertHistoryCell(cell)) => break cell,
                Some(_) => continue,
                None => panic!("app event channel closed before forked thread history was emitted"),
            }
        }
    })
    .await
    .expect("timed out waiting for forked thread history");
    let combined = lines_to_single_string(&history_cell.display_lines(80));

    assert_snapshot!("forked_thread_history_line_without_name", combined);
}
