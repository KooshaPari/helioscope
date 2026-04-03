use super::*;

#[tokio::test]
async fn blocked_image_restore_preserves_mention_bindings() {
    let (mut chat, mut rx, _op_rx) = make_chatwidget_manual(None).await;

    let placeholder = "[Image #1]";
    let text = format!("{placeholder} check $file");
    let text_elements = vec![TextElement::new(
        (0..placeholder.len()).into(),
        Some(placeholder.to_string()),
    )];
    let local_images = vec![LocalImageAttachment {
        placeholder: placeholder.to_string(),
        path: PathBuf::from("/tmp/blocked.png"),
    }];
    let mention_bindings = vec![MentionBinding {
        mention: "file".to_string(),
        path: "/tmp/skills/file/SKILL.md".to_string(),
    }];

    chat.restore_blocked_image_submission(
        text.clone(),
        text_elements,
        local_images.clone(),
        mention_bindings.clone(),
        Vec::new(),
    );

    let mention_start = text.find("$file").expect("mention token exists");
    let expected_elements = vec![
        TextElement::new((0..placeholder.len()).into(), Some(placeholder.to_string())),
        TextElement::new(
            (mention_start..mention_start + "$file".len()).into(),
            Some("$file".to_string()),
        ),
    ];
    assert_eq!(chat.bottom_pane.composer_text(), text);
    assert_eq!(chat.bottom_pane.composer_text_elements(), expected_elements);
    assert_eq!(
        chat.bottom_pane.composer_local_image_paths(),
        vec![local_images[0].path.clone()],
    );
    assert_eq!(chat.bottom_pane.take_mention_bindings(), mention_bindings);

    let cells = drain_insert_history(&mut rx);
    let warning = cells
        .last()
        .map(|lines| lines_to_single_string(lines))
        .expect("expected warning cell");
    assert!(
        warning.contains("does not support image inputs"),
        "expected image warning, got: {warning:?}"
    );
}

#[tokio::test]
async fn blocked_image_restore_with_remote_images_keeps_local_placeholder_mapping() {
    let (mut chat, _rx, _op_rx) = make_chatwidget_manual(None).await;

    let first_placeholder = "[Image #2]";
    let second_placeholder = "[Image #3]";
    let text = format!("{first_placeholder} first\n{second_placeholder} second");
    let second_start = text.find(second_placeholder).expect("second placeholder");
    let text_elements = vec![
        TextElement::new(
            (0..first_placeholder.len()).into(),
            Some(first_placeholder.to_string()),
        ),
        TextElement::new(
            (second_start..second_start + second_placeholder.len()).into(),
            Some(second_placeholder.to_string()),
        ),
    ];
    let local_images = vec![
        LocalImageAttachment {
            placeholder: first_placeholder.to_string(),
            path: PathBuf::from("/tmp/blocked-first.png"),
        },
        LocalImageAttachment {
            placeholder: second_placeholder.to_string(),
            path: PathBuf::from("/tmp/blocked-second.png"),
        },
    ];
    let remote_image_urls = vec!["https://example.com/blocked-remote.png".to_string()];

    chat.restore_blocked_image_submission(
        text.clone(),
        text_elements.clone(),
        local_images.clone(),
        Vec::new(),
        remote_image_urls.clone(),
    );

    assert_eq!(chat.bottom_pane.composer_text(), text);
    assert_eq!(chat.bottom_pane.composer_text_elements(), text_elements);
    assert_eq!(chat.bottom_pane.composer_local_images(), local_images);
    assert_eq!(chat.remote_image_urls(), remote_image_urls);
}

#[tokio::test]
async fn queued_restore_with_remote_images_keeps_local_placeholder_mapping() {
    let (mut chat, _rx, _op_rx) = make_chatwidget_manual(None).await;

    let first_placeholder = "[Image #2]";
    let second_placeholder = "[Image #3]";
    let text = format!("{first_placeholder} first\n{second_placeholder} second");
    let second_start = text.find(second_placeholder).expect("second placeholder");
    let text_elements = vec![
        TextElement::new(
            (0..first_placeholder.len()).into(),
            Some(first_placeholder.to_string()),
        ),
        TextElement::new(
            (second_start..second_start + second_placeholder.len()).into(),
            Some(second_placeholder.to_string()),
        ),
    ];
    let local_images = vec![
        LocalImageAttachment {
            placeholder: first_placeholder.to_string(),
            path: PathBuf::from("/tmp/queued-first.png"),
        },
        LocalImageAttachment {
            placeholder: second_placeholder.to_string(),
            path: PathBuf::from("/tmp/queued-second.png"),
        },
    ];
    let remote_image_urls = vec!["https://example.com/queued-remote.png".to_string()];

    chat.restore_user_message_to_composer(UserMessage {
        text: text.clone(),
        local_images: local_images.clone(),
        remote_image_urls: remote_image_urls.clone(),
        text_elements: text_elements.clone(),
        mention_bindings: Vec::new(),
    });

    assert_eq!(chat.bottom_pane.composer_text(), text);
    assert_eq!(chat.bottom_pane.composer_text_elements(), text_elements);
    assert_eq!(chat.bottom_pane.composer_local_images(), local_images);
    assert_eq!(chat.remote_image_urls(), remote_image_urls);
}

#[tokio::test]
async fn interrupted_turn_restores_queued_messages_with_images_and_elements() {
    let (mut chat, _rx, _op_rx) = make_chatwidget_manual(None).await;

    let first_placeholder = "[Image #1]";
    let first_text = format!("{first_placeholder} first");
    let first_elements = vec![TextElement::new(
        (0..first_placeholder.len()).into(),
        Some(first_placeholder.to_string()),
    )];
    let first_images = [PathBuf::from("/tmp/first.png")];

    let second_placeholder = "[Image #1]";
    let second_text = format!("{second_placeholder} second");
    let second_elements = vec![TextElement::new(
        (0..second_placeholder.len()).into(),
        Some(second_placeholder.to_string()),
    )];
    let second_images = [PathBuf::from("/tmp/second.png")];

    let existing_placeholder = "[Image #1]";
    let existing_text = format!("{existing_placeholder} existing");
    let existing_elements = vec![TextElement::new(
        (0..existing_placeholder.len()).into(),
        Some(existing_placeholder.to_string()),
    )];
    let existing_images = vec![PathBuf::from("/tmp/existing.png")];

    chat.queued_user_messages.push_back(UserMessage {
        text: first_text,
        local_images: vec![LocalImageAttachment {
            placeholder: first_placeholder.to_string(),
            path: first_images[0].clone(),
        }],
        remote_image_urls: Vec::new(),
        text_elements: first_elements,
        mention_bindings: Vec::new(),
    });
    chat.queued_user_messages.push_back(UserMessage {
        text: second_text,
        local_images: vec![LocalImageAttachment {
            placeholder: second_placeholder.to_string(),
            path: second_images[0].clone(),
        }],
        remote_image_urls: Vec::new(),
        text_elements: second_elements,
        mention_bindings: Vec::new(),
    });
    chat.refresh_queued_user_messages();

    chat.bottom_pane
        .set_composer_text(existing_text, existing_elements, existing_images.clone());

    // When interrupted, queued messages are merged into the composer; image placeholders
    // must be renumbered to match the combined local image list.
    chat.handle_codex_event(Event {
        id: "interrupt".into(),
        msg: EventMsg::TurnAborted(codex_protocol::protocol::TurnAbortedEvent {
            turn_id: Some("turn-1".to_string()),
            reason: TurnAbortReason::Interrupted,
        }),
    });

    let first = "[Image #1] first".to_string();
    let second = "[Image #2] second".to_string();
    let third = "[Image #3] existing".to_string();
    let expected_text = format!("{first}\n{second}\n{third}");
    assert_eq!(chat.bottom_pane.composer_text(), expected_text);

    let first_start = 0;
    let second_start = first.len() + 1;
    let third_start = second_start + second.len() + 1;
    let expected_elements = vec![
        TextElement::new(
            (first_start..first_start + "[Image #1]".len()).into(),
            Some("[Image #1]".to_string()),
        ),
        TextElement::new(
            (second_start..second_start + "[Image #2]".len()).into(),
            Some("[Image #2]".to_string()),
        ),
        TextElement::new(
            (third_start..third_start + "[Image #3]".len()).into(),
            Some("[Image #3]".to_string()),
        ),
    ];
    assert_eq!(chat.bottom_pane.composer_text_elements(), expected_elements);
    assert_eq!(
        chat.bottom_pane.composer_local_image_paths(),
        vec![
            first_images[0].clone(),
            second_images[0].clone(),
            existing_images[0].clone(),
        ]
    );
}

#[tokio::test]
async fn interrupted_turn_restore_keeps_active_mode_for_resubmission() {
    let (mut chat, _rx, mut op_rx) = make_chatwidget_manual(Some("gpt-5")).await;
    chat.thread_id = Some(ThreadId::new());
    chat.set_feature_enabled(Feature::CollaborationModes, true);

    let plan_mask = collaboration_modes::plan_mask(chat.models_manager.as_ref())
        .expect("expected plan collaboration mode");
    let expected_mode = plan_mask
        .mode
        .expect("expected mode kind on plan collaboration mode");

    chat.set_collaboration_mask(plan_mask);
    chat.on_task_started();
    chat.queued_user_messages.push_back(UserMessage {
        text: "Implement the plan.".to_string(),
        local_images: Vec::new(),
        remote_image_urls: Vec::new(),
        text_elements: Vec::new(),
        mention_bindings: Vec::new(),
    });
    chat.refresh_queued_user_messages();

    chat.handle_codex_event(Event {
        id: "interrupt".into(),
        msg: EventMsg::TurnAborted(codex_protocol::protocol::TurnAbortedEvent {
            turn_id: Some("turn-1".to_string()),
            reason: TurnAbortReason::Interrupted,
        }),
    });

    assert_eq!(chat.bottom_pane.composer_text(), "Implement the plan.");
    assert!(chat.queued_user_messages.is_empty());
    assert_eq!(chat.active_collaboration_mode_kind(), expected_mode);

    chat.handle_key_event(KeyEvent::from(KeyCode::Enter));

    match next_submit_op(&mut op_rx) {
        Op::UserTurn {
            collaboration_mode: Some(CollaborationMode { mode, .. }),
            personality: None,
            ..
        } => assert_eq!(mode, expected_mode),
        other => {
            panic!("expected Op::UserTurn with active mode, got {other:?}")
        }
    }
    assert_eq!(chat.active_collaboration_mode_kind(), expected_mode);
}

#[tokio::test]
async fn remap_placeholders_uses_attachment_labels() {
    let placeholder_one = "[Image #1]";
    let placeholder_two = "[Image #2]";
    let text = format!("{placeholder_two} before {placeholder_one}");
    let elements = vec![
        TextElement::new(
            (0..placeholder_two.len()).into(),
            Some(placeholder_two.to_string()),
        ),
        TextElement::new(
            ("[Image #2] before ".len().."[Image #2] before [Image #1]".len()).into(),
            Some(placeholder_one.to_string()),
        ),
    ];

    let attachments = vec![
        LocalImageAttachment {
            placeholder: placeholder_one.to_string(),
            path: PathBuf::from("/tmp/one.png"),
        },
        LocalImageAttachment {
            placeholder: placeholder_two.to_string(),
            path: PathBuf::from("/tmp/two.png"),
        },
    ];
    let message = UserMessage {
        text,
        text_elements: elements,
        local_images: attachments,
        remote_image_urls: vec!["https://example.com/a.png".to_string()],
        mention_bindings: Vec::new(),
    };
    let mut next_label = 3usize;
    let remapped = remap_placeholders_for_message(message, &mut next_label);

    assert_eq!(remapped.text, "[Image #4] before [Image #3]");
    assert_eq!(
        remapped.text_elements,
        vec![
            TextElement::new(
                (0.."[Image #4]".len()).into(),
                Some("[Image #4]".to_string()),
            ),
            TextElement::new(
                ("[Image #4] before ".len().."[Image #4] before [Image #3]".len()).into(),
                Some("[Image #3]".to_string()),
            ),
        ]
    );
    assert_eq!(
        remapped.local_images,
        vec![
            LocalImageAttachment {
                placeholder: "[Image #3]".to_string(),
                path: PathBuf::from("/tmp/one.png"),
            },
            LocalImageAttachment {
                placeholder: "[Image #4]".to_string(),
                path: PathBuf::from("/tmp/two.png"),
            },
        ]
    );
    assert_eq!(
        remapped.remote_image_urls,
        vec!["https://example.com/a.png".to_string()]
    );
}

#[tokio::test]
async fn remap_placeholders_uses_byte_ranges_when_placeholder_missing() {
    let placeholder_one = "[Image #1]";
    let placeholder_two = "[Image #2]";
    let text = format!("{placeholder_two} before {placeholder_one}");
    let elements = vec![
        TextElement::new((0..placeholder_two.len()).into(), None),
        TextElement::new(
            ("[Image #2] before ".len().."[Image #2] before [Image #1]".len()).into(),
            None,
        ),
    ];

    let attachments = vec![
        LocalImageAttachment {
            placeholder: placeholder_one.to_string(),
            path: PathBuf::from("/tmp/one.png"),
        },
        LocalImageAttachment {
            placeholder: placeholder_two.to_string(),
            path: PathBuf::from("/tmp/two.png"),
        },
    ];
    let message = UserMessage {
        text,
        text_elements: elements,
        local_images: attachments,
        remote_image_urls: Vec::new(),
        mention_bindings: Vec::new(),
    };
    let mut next_label = 3usize;
    let remapped = remap_placeholders_for_message(message, &mut next_label);

    assert_eq!(remapped.text, "[Image #4] before [Image #3]");
    assert_eq!(
        remapped.text_elements,
        vec![
            TextElement::new(
                (0.."[Image #4]".len()).into(),
                Some("[Image #4]".to_string()),
            ),
            TextElement::new(
                ("[Image #4] before ".len().."[Image #4] before [Image #3]".len()).into(),
                Some("[Image #3]".to_string()),
            ),
        ]
    );
    assert_eq!(
        remapped.local_images,
        vec![
            LocalImageAttachment {
                placeholder: "[Image #3]".to_string(),
                path: PathBuf::from("/tmp/one.png"),
            },
            LocalImageAttachment {
                placeholder: "[Image #4]".to_string(),
                path: PathBuf::from("/tmp/two.png"),
            },
        ]
    );
}
