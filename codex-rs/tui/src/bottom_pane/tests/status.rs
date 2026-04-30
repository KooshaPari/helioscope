macro_rules! assert_bottom_pane_snapshot {
    ($name:expr, $value:expr) => {
        insta::with_settings!({ snapshot_path => "../snapshots" }, {
            assert_snapshot!($name, $value);
        });
    };
}

#[test]
fn ctrl_c_on_modal_consumes_without_showing_quit_hint() {
    let (tx_raw, _rx) = unbounded_channel::<AppEvent>();
    let tx = AppEventSender::new(tx_raw);
    let features = Features::with_defaults();
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
    pane.push_approval_request(exec_request(), &features);
    assert_eq!(CancellationEvent::Handled, pane.on_ctrl_c());
    assert!(!pane.quit_shortcut_hint_visible());
    assert_eq!(CancellationEvent::NotHandled, pane.on_ctrl_c());
}

// live ring removed; related tests deleted.

#[test]
fn overlay_not_shown_above_approval_modal() {
    let (tx_raw, _rx) = unbounded_channel::<AppEvent>();
    let tx = AppEventSender::new(tx_raw);
    let features = Features::with_defaults();
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

    pane.push_approval_request(exec_request(), &features);

    let area = Rect::new(0, 0, 60, 6);
    let mut buf = Buffer::empty(area);
    pane.render(area, &mut buf);

    let mut r0 = String::new();
    for x in 0..area.width {
        r0.push(buf[(x, 0)].symbol().chars().next().unwrap_or(' '));
    }
    assert!(
        !r0.contains("Working"),
        "overlay should not render above modal"
    );
}

#[test]
fn composer_shown_after_denied_while_task_running() {
    let (tx_raw, _rx) = unbounded_channel::<AppEvent>();
    let tx = AppEventSender::new(tx_raw);
    let features = Features::with_defaults();
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
    pane.push_approval_request(exec_request(), &features);
    pane.handle_key_event(KeyEvent::new(KeyCode::Char('n'), KeyModifiers::NONE));

    assert!(
        pane.view_stack.is_empty(),
        "no active modal view after denial"
    );

    std::thread::sleep(Duration::from_millis(120));
    let area = Rect::new(0, 0, 40, 6);
    let mut buf = Buffer::empty(area);
    pane.render(area, &mut buf);
    let mut row0 = String::new();
    for x in 0..area.width {
        row0.push(buf[(x, 0)].symbol().chars().next().unwrap_or(' '));
    }
    assert!(
        row0.contains("Working"),
        "expected Working header after denial on row 0: {row0:?}"
    );

    let mut found_composer = false;
    for y in 1..area.height {
        let mut row = String::new();
        for x in 0..area.width {
            row.push(buf[(x, y)].symbol().chars().next().unwrap_or(' '));
        }
        if row.contains("Ask Codex") {
            found_composer = true;
            break;
        }
    }
    assert!(
        found_composer,
        "expected composer visible under status line"
    );
}

#[test]
fn status_indicator_visible_during_command_execution() {
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

    pane.set_task_running(true);

    let area = Rect::new(0, 0, 40, 6);
    let mut buf = Buffer::empty(area);
    pane.render(area, &mut buf);

    let bufs = snapshot_buffer(&buf);
    assert!(bufs.contains("• Working"), "expected Working header");
}

#[test]
fn status_and_composer_fill_height_without_bottom_padding() {
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

    pane.set_task_running(true);

    let height = pane.desired_height(30);
    assert!(
        height >= 3,
        "expected at least 3 rows to render spacer, status, and composer; got {height}"
    );
    let area = Rect::new(0, 0, 30, height);
    assert_bottom_pane_snapshot!(
        "status_and_composer_fill_height_without_bottom_padding",
        render_snapshot(&pane, area)
    );
}

#[test]
fn status_only_snapshot() {
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

    pane.set_task_running(true);

    let width = 48;
    let height = pane.desired_height(width);
    let area = Rect::new(0, 0, width, height);
    assert_bottom_pane_snapshot!("status_only_snapshot", render_snapshot(&pane, area));
}

#[test]
fn unified_exec_summary_does_not_increase_height_when_status_visible() {
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

    pane.set_task_running(true);
    let width = 120;
    let before = pane.desired_height(width);

    pane.set_unified_exec_processes(vec!["sleep 5".to_string()]);
    let after = pane.desired_height(width);

    assert_eq!(after, before);

    let area = Rect::new(0, 0, width, after);
    let rendered = render_snapshot(&pane, area);
    assert!(rendered.contains("background terminal running · /ps to view"));
}

#[test]
fn status_with_details_and_queued_messages_snapshot() {
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

    pane.set_task_running(true);
    pane.update_status(
        "Working".to_string(),
        Some("First detail line\nSecond detail line".to_string()),
        StatusDetailsCapitalization::CapitalizeFirst,
        STATUS_DETAILS_DEFAULT_MAX_LINES,
    );
    pane.set_queued_user_messages(vec!["Queued follow-up question".to_string()]);

    let width = 48;
    let height = pane.desired_height(width);
    let area = Rect::new(0, 0, width, height);
    assert_bottom_pane_snapshot!(
        "status_with_details_and_queued_messages_snapshot",
        render_snapshot(&pane, area)
    );
}

#[test]
fn queued_messages_visible_when_status_hidden_snapshot() {
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

    pane.set_task_running(true);
    pane.set_queued_user_messages(vec!["Queued follow-up question".to_string()]);
    pane.hide_status_indicator();

    let width = 48;
    let height = pane.desired_height(width);
    let area = Rect::new(0, 0, width, height);
    assert_bottom_pane_snapshot!(
        "queued_messages_visible_when_status_hidden_snapshot",
        render_snapshot(&pane, area)
    );
}

#[test]
fn status_and_queued_messages_snapshot() {
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

    pane.set_task_running(true);
    pane.set_queued_user_messages(vec!["Queued follow-up question".to_string()]);

    let width = 48;
    let height = pane.desired_height(width);
    let area = Rect::new(0, 0, width, height);
    assert_bottom_pane_snapshot!(
        "status_and_queued_messages_snapshot",
        render_snapshot(&pane, area)
    );
}
