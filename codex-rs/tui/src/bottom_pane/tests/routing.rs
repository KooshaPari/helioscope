#[test]
fn esc_with_skill_popup_does_not_interrupt_task() {
    let (tx_raw, mut rx) = unbounded_channel::<AppEvent>();
    let tx = AppEventSender::new(tx_raw);
    let mut pane = BottomPane::new(BottomPaneParams {
        app_event_tx: tx,
        frame_requester: FrameRequester::test_dummy(),
        has_input_focus: true,
        enhanced_keys_supported: false,
        placeholder_text: "Ask Codex to do anything".to_string(),
        disable_paste_burst: false,
        animations_enabled: true,
        skills: Some(vec![SkillMetadata {
            name: "test-skill".to_string(),
            description: "test skill".to_string(),
            short_description: None,
            interface: None,
            dependencies: None,
            policy: None,
            permission_profile: None,
            permissions: None,
            path_to_skills_md: PathBuf::from("test-skill"),
            scope: SkillScope::User,
        }]),
    });

    pane.set_task_running(true);

    pane.insert_str("$");
    assert!(
        pane.composer.popup_active(),
        "expected skill popup after typing `$`"
    );

    pane.handle_key_event(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE));

    while let Ok(ev) = rx.try_recv() {
        assert!(
            !matches!(ev, AppEvent::CodexOp(Op::Interrupt)),
            "expected Esc to not send Op::Interrupt when dismissing skill popup"
        );
    }
    assert!(
        !pane.composer.popup_active(),
        "expected Esc to dismiss skill popup"
    );
}

#[test]
fn esc_with_slash_command_popup_does_not_interrupt_task() {
    let (tx_raw, mut rx) = unbounded_channel::<AppEvent>();
    let tx = AppEventSender::new(tx_raw);
    let mut pane = BottomPane::new(BottomPaneParams {
        app_event_tx: tx,
        frame_requester: FrameRequester::test_dummy(),
        has_input_focus: true,
        enhanced_keys_supported: false,
        placeholder_text: "Ask Codex to do anything".to_string(),
        disable_paste_burst: false,
        animations_enabled: true,
        skills: Some(Vec::new()),
    });

    pane.set_task_running(true);

    pane.insert_str("/");
    assert!(
        pane.composer.popup_active(),
        "expected command popup after typing `/`"
    );

    pane.handle_key_event(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE));

    while let Ok(ev) = rx.try_recv() {
        assert!(
            !matches!(ev, AppEvent::CodexOp(Op::Interrupt)),
            "expected Esc to not send Op::Interrupt while command popup is active"
        );
    }
    assert_eq!(pane.composer_text(), "/");
}

#[test]
fn esc_interrupts_running_task_when_no_popup() {
    let (tx_raw, mut rx) = unbounded_channel::<AppEvent>();
    let tx = AppEventSender::new(tx_raw);
    let mut pane = BottomPane::new(BottomPaneParams {
        app_event_tx: tx,
        frame_requester: FrameRequester::test_dummy(),
        has_input_focus: true,
        enhanced_keys_supported: false,
        placeholder_text: "Ask Codex to do anything".to_string(),
        disable_paste_burst: false,
        animations_enabled: true,
        skills: Some(Vec::new()),
    });

    pane.set_task_running(true);

    pane.handle_key_event(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE));

    assert!(
        matches!(rx.try_recv(), Ok(AppEvent::CodexOp(Op::Interrupt))),
        "expected Esc to send Op::Interrupt while a task is running"
    );
}

#[test]
fn esc_routes_to_handle_key_event_when_requested() {
    #[derive(Default)]
    struct EscRoutingView {
        on_ctrl_c_calls: Rc<Cell<usize>>,
        handle_calls: Rc<Cell<usize>>,
    }

    impl Renderable for EscRoutingView {
        fn render(&self, _area: Rect, _buf: &mut Buffer) {}

        fn desired_height(&self, _width: u16) -> u16 {
            0
        }
    }

    impl BottomPaneView for EscRoutingView {
        fn handle_key_event(&mut self, _key_event: KeyEvent) {
            self.handle_calls
                .set(self.handle_calls.get().saturating_add(1));
        }

        fn on_ctrl_c(&mut self) -> CancellationEvent {
            self.on_ctrl_c_calls
                .set(self.on_ctrl_c_calls.get().saturating_add(1));
            CancellationEvent::Handled
        }

        fn prefer_esc_to_handle_key_event(&self) -> bool {
            true
        }
    }

    let (tx_raw, _rx) = unbounded_channel::<AppEvent>();
    let tx = AppEventSender::new(tx_raw);
    let mut pane = BottomPane::new(BottomPaneParams {
        app_event_tx: tx,
        frame_requester: FrameRequester::test_dummy(),
        has_input_focus: true,
        enhanced_keys_supported: false,
        placeholder_text: "Ask Codex to do anything".to_string(),
        disable_paste_burst: false,
        animations_enabled: true,
        skills: Some(Vec::new()),
    });

    let on_ctrl_c_calls = Rc::new(Cell::new(0));
    let handle_calls = Rc::new(Cell::new(0));
    pane.push_view(Box::new(EscRoutingView {
        on_ctrl_c_calls: Rc::clone(&on_ctrl_c_calls),
        handle_calls: Rc::clone(&handle_calls),
    }));

    pane.handle_key_event(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE));

    assert_eq!(on_ctrl_c_calls.get(), 0);
    assert_eq!(handle_calls.get(), 1);
}

#[test]
fn release_events_are_ignored_for_active_view() {
    #[derive(Default)]
    struct CountingView {
        handle_calls: Rc<Cell<usize>>,
    }

    impl Renderable for CountingView {
        fn render(&self, _area: Rect, _buf: &mut Buffer) {}

        fn desired_height(&self, _width: u16) -> u16 {
            0
        }
    }

    impl BottomPaneView for CountingView {
        fn handle_key_event(&mut self, _key_event: KeyEvent) {
            self.handle_calls
                .set(self.handle_calls.get().saturating_add(1));
        }
    }

    let (tx_raw, _rx) = unbounded_channel::<AppEvent>();
    let tx = AppEventSender::new(tx_raw);
    let mut pane = BottomPane::new(BottomPaneParams {
        app_event_tx: tx,
        frame_requester: FrameRequester::test_dummy(),
        has_input_focus: true,
        enhanced_keys_supported: false,
        placeholder_text: "Ask Codex to do anything".to_string(),
        disable_paste_burst: false,
        animations_enabled: true,
        skills: Some(Vec::new()),
    });

    let handle_calls = Rc::new(Cell::new(0));
    pane.push_view(Box::new(CountingView {
        handle_calls: Rc::clone(&handle_calls),
    }));

    pane.handle_key_event(KeyEvent::new_with_kind(
        KeyCode::Down,
        KeyModifiers::NONE,
        KeyEventKind::Press,
    ));
    pane.handle_key_event(KeyEvent::new_with_kind(
        KeyCode::Down,
        KeyModifiers::NONE,
        KeyEventKind::Release,
    ));

    assert_eq!(handle_calls.get(), 1);
}
