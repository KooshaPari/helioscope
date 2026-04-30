#[tokio::test]
async fn submission_prefers_selected_duplicate_skill_path() {
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

    let repo_skill_path = PathBuf::from("/tmp/repo/figma/SKILL.md");
    let user_skill_path = PathBuf::from("/tmp/user/figma/SKILL.md");
    chat.set_skills(Some(vec![
        SkillMetadata {
            name: "figma".to_string(),
            description: "Repo skill".to_string(),
            short_description: None,
            interface: None,
            dependencies: None,
            policy: None,
            permission_profile: None,
            permissions: None,
            path_to_skills_md: repo_skill_path,
            scope: SkillScope::Repo,
        },
        SkillMetadata {
            name: "figma".to_string(),
            description: "User skill".to_string(),
            short_description: None,
            interface: None,
            dependencies: None,
            policy: None,
            permission_profile: None,
            permissions: None,
            path_to_skills_md: user_skill_path.clone(),
            scope: SkillScope::User,
        },
    ]));

    chat.bottom_pane.set_composer_text_with_mention_bindings(
        "please use $figma now".to_string(),
        Vec::new(),
        Vec::new(),
        vec![MentionBinding {
            mention: "figma".to_string(),
            path: user_skill_path.to_string_lossy().into_owned(),
        }],
    );
    chat.handle_key_event(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));

    let items = match next_submit_op(&mut op_rx) {
        Op::UserTurn { items, .. } => items,
        other => panic!("expected Op::UserTurn, got {other:?}"),
    };
    let selected_skill_paths = items
        .iter()
        .filter_map(|item| match item {
            UserInput::Skill { path, .. } => Some(path.clone()),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(selected_skill_paths, vec![user_skill_path]);
}

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
