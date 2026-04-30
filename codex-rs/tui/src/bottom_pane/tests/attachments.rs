#[test]
fn remote_images_render_above_composer_text() {
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

    pane.set_remote_image_urls(vec![
        "https://example.com/one.png".to_string(),
        "data:image/png;base64,aGVsbG8=".to_string(),
    ]);

    assert_eq!(pane.composer_text(), "");
    let width = 48;
    let height = pane.desired_height(width);
    let area = Rect::new(0, 0, width, height);
    let snapshot = render_snapshot(&pane, area);
    assert!(snapshot.contains("[Image #1]"));
    assert!(snapshot.contains("[Image #2]"));
}

#[test]
fn drain_pending_submission_state_clears_remote_image_urls() {
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

    pane.set_remote_image_urls(vec!["https://example.com/one.png".to_string()]);
    assert_eq!(pane.remote_image_urls().len(), 1);

    pane.drain_pending_submission_state();

    assert!(pane.remote_image_urls().is_empty());
}
