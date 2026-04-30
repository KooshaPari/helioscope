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
async fn submit_user_message_with_mode_sets_coding_collaboration_mode() {
    let (mut chat, _rx, mut op_rx) = make_chatwidget_manual(Some("gpt-5")).await;
    chat.thread_id = Some(ThreadId::new());
    chat.set_feature_enabled(Feature::CollaborationModes, true);

    let default_mode = collaboration_modes::default_mode_mask(chat.models_manager.as_ref())
        .expect("expected default collaboration mode");
    chat.submit_user_message_with_mode("Implement the plan.".to_string(), default_mode);

    match next_submit_op(&mut op_rx) {
        Op::UserTurn {
            collaboration_mode:
                Some(CollaborationMode {
                    mode: ModeKind::Default,
                    ..
                }),
            personality: None,
            ..
        } => {}
        other => {
            panic!("expected Op::UserTurn with default collab mode, got {other:?}")
        }
    }
}
