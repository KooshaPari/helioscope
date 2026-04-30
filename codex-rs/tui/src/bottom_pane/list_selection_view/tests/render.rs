#[test]
fn theme_picker_subtitle_uses_fallback_text_in_94x35_terminal() {
    let (tx_raw, _rx) = unbounded_channel::<AppEvent>();
    let tx = AppEventSender::new(tx_raw);
    let home = dirs::home_dir().expect("home directory should be available");
    let codex_home = home.join(".codex");
    let params = crate::theme_picker::build_theme_picker_params(None, Some(&codex_home), Some(94));
    let view = ListSelectionView::new(params, tx);

    let rendered = render_lines_in_area(&view, 94, 35);
    assert!(rendered.contains("Move up/down to live preview themes"));
}

#[test]
fn theme_picker_enables_side_content_background_preservation() {
    let params = crate::theme_picker::build_theme_picker_params(None, None, Some(120));
    assert!(
        params.preserve_side_content_bg,
        "theme picker should preserve side-content backgrounds to keep diff preview styling",
    );
}

#[test]
fn preserve_side_content_bg_keeps_rendered_background_colors() {
    let (tx_raw, _rx) = unbounded_channel::<AppEvent>();
    let tx = AppEventSender::new(tx_raw);
    let view = ListSelectionView::new(
        SelectionViewParams {
            title: Some("Debug".to_string()),
            items: vec![SelectionItem {
                name: "Item 1".to_string(),
                dismiss_on_select: true,
                ..Default::default()
            }],
            side_content: Box::new(StyledMarkerRenderable {
                marker: "+",
                style: Style::default().bg(Color::Blue),
                height: 1,
            }),
            side_content_width: SideContentWidth::Half,
            side_content_min_width: 10,
            preserve_side_content_bg: true,
            ..Default::default()
        },
        tx,
    );
    let area = Rect::new(0, 0, 120, 35);
    let mut buf = Buffer::empty(area);

    view.render(area, &mut buf);

    let plus_bg = (0..area.height)
        .flat_map(|y| (0..area.width).map(move |x| (x, y)))
        .find_map(|(x, y)| {
            let cell = &buf[(x, y)];
            (cell.symbol() == "+").then(|| cell.style().bg)
        })
        .expect("expected side content to render at least one '+' marker");
    assert_eq!(
        plus_bg,
        Some(Color::Blue),
        "expected side-content marker to preserve custom background styling",
    );
}

#[test]
fn renders_search_query_line_when_enabled() {
    let (tx_raw, _rx) = unbounded_channel::<AppEvent>();
    let tx = AppEventSender::new(tx_raw);
    let items = vec![SelectionItem {
        name: "Read Only".to_string(),
        description: Some("Codex can read files".to_string()),
        is_current: false,
        dismiss_on_select: true,
        ..Default::default()
    }];
    let mut view = ListSelectionView::new(
        SelectionViewParams {
            title: Some("Select Approval Mode".to_string()),
            footer_hint: Some(standard_popup_hint_line()),
            items,
            is_searchable: true,
            search_placeholder: Some("Type to search branches".to_string()),
            ..Default::default()
        },
        tx,
    );
    view.set_search_query("filters".to_string());

    let lines = render_lines(&view);
    assert!(
        lines.contains("filters"),
        "expected search query line to include rendered query, got {lines:?}"
    );
}

#[test]
fn enter_with_no_matches_triggers_cancel_callback() {
    let (tx_raw, mut rx) = unbounded_channel::<AppEvent>();
    let tx = AppEventSender::new(tx_raw);
    let mut view = ListSelectionView::new(
        SelectionViewParams {
            items: vec![SelectionItem {
                name: "Read Only".to_string(),
                dismiss_on_select: true,
                ..Default::default()
            }],
            is_searchable: true,
            on_cancel: Some(Box::new(|tx: &_| {
                tx.send(AppEvent::OpenApprovalsPopup);
            })),
            ..Default::default()
        },
        tx,
    );
    view.set_search_query("no-matches".to_string());

    view.handle_key_event(KeyEvent::from(KeyCode::Enter));

    assert!(view.is_complete());
    match rx.try_recv() {
        Ok(AppEvent::OpenApprovalsPopup) => {}
        Ok(other) => panic!("expected OpenApprovalsPopup cancel event, got {other:?}"),
        Err(err) => panic!("expected cancel callback event, got {err}"),
    }
}

#[test]
fn move_down_without_selection_change_does_not_fire_callback() {
    let (tx_raw, mut rx) = unbounded_channel::<AppEvent>();
    let tx = AppEventSender::new(tx_raw);
    let mut view = ListSelectionView::new(
        SelectionViewParams {
            items: vec![SelectionItem {
                name: "Only choice".to_string(),
                dismiss_on_select: true,
                ..Default::default()
            }],
            on_selection_changed: Some(Box::new(|_idx, tx: &_| {
                tx.send(AppEvent::OpenApprovalsPopup);
            })),
            ..Default::default()
        },
        tx,
    );

    while rx.try_recv().is_ok() {}

    view.handle_key_event(KeyEvent::from(KeyCode::Down));

    assert!(
        rx.try_recv().is_err(),
        "moving down in a single-item list should not fire on_selection_changed",
    );
}

#[test]
fn wraps_long_option_without_overflowing_columns() {
    let (tx_raw, _rx) = unbounded_channel::<AppEvent>();
    let tx = AppEventSender::new(tx_raw);
    let items = vec![
        SelectionItem {
            name: "Yes, proceed".to_string(),
            dismiss_on_select: true,
            ..Default::default()
        },
        SelectionItem {
            name: "Yes, and don't ask again for commands that start with `python -mpre_commit run --files eslint-plugin/no-mixed-const-enum-exports.js`".to_string(),
            dismiss_on_select: true,
            ..Default::default()
        },
    ];
    let view = ListSelectionView::new(
        SelectionViewParams {
            title: Some("Approval".to_string()),
            items,
            ..Default::default()
        },
        tx,
    );

    let rendered = render_lines_with_width(&view, 60);
    let command_line = rendered
        .lines()
        .find(|line| line.contains("python -mpre_commit run"))
        .expect("rendered lines should include wrapped command");
    assert!(
        command_line.starts_with("     `python -mpre_commit run"),
        "wrapped command line should align under the numbered prefix:\n{rendered}"
    );
    assert!(
        rendered.contains("eslint-plugin/no-") && rendered.contains("mixed-const-enum-exports.js"),
        "long command should not be truncated even when wrapped:\n{rendered}"
    );
}

#[test]
fn width_changes_do_not_hide_rows() {
    let (tx_raw, _rx) = unbounded_channel::<AppEvent>();
    let tx = AppEventSender::new(tx_raw);
    let items = model_picker_items();
    let view = ListSelectionView::new(
        SelectionViewParams {
            title: Some("Select Model and Effort".to_string()),
            items,
            ..Default::default()
        },
        tx,
    );
    let mut missing: Vec<u16> = Vec::new();
    for width in 60..=90 {
        let rendered = render_lines_with_width(&view, width);
        if !rendered.contains("3.") {
            missing.push(width);
        }
    }
    assert!(
        missing.is_empty(),
        "third option missing at widths {missing:?}"
    );
}

#[test]
fn narrow_width_keeps_all_rows_visible() {
    let (tx_raw, _rx) = unbounded_channel::<AppEvent>();
    let tx = AppEventSender::new(tx_raw);
    let desc = "x".repeat(10);
    let items: Vec<SelectionItem> = (1..=3)
        .map(|idx| SelectionItem {
            name: format!("Item {idx}"),
            description: Some(desc.clone()),
            dismiss_on_select: true,
            ..Default::default()
        })
        .collect();
    let view = ListSelectionView::new(
        SelectionViewParams {
            title: Some("Debug".to_string()),
            items,
            ..Default::default()
        },
        tx,
    );
    let rendered = render_lines_with_width(&view, 24);
    assert!(
        rendered.contains("3."),
        "third option missing for width 24:\n{rendered}"
    );
}
