use super::support::*;
use super::*;
use pretty_assertions::assert_eq;

#[test]
fn auto_all_rows_col_width_does_not_shift_when_scrolling() {
    let (tx_raw, _rx) = unbounded_channel::<AppEvent>();
    let tx = AppEventSender::new(tx_raw);

    let mut view = ListSelectionView::new(
        SelectionViewParams {
            title: Some("Debug".to_string()),
            items: make_scrolling_width_items(),
            col_width_mode: ColumnWidthMode::AutoAllRows,
            ..Default::default()
        },
        tx,
    );

    let before_scroll = render_lines_with_width(&view, 96);
    for _ in 0..8 {
        view.handle_key_event(KeyEvent::from(KeyCode::Down));
    }
    let after_scroll = render_lines_with_width(&view, 96);

    assert!(
        after_scroll.contains("9. Item 9 with an intentionally much longer name"),
        "expected the scrolled view to include the longer row:\n{after_scroll}"
    );

    let before_col = description_col(&before_scroll, "8. Item 8", "desc 8");
    let after_col = description_col(&after_scroll, "8. Item 8", "desc 8");
    assert_eq!(
        before_col, after_col,
        "description column changed across scroll:\nbefore:\n{before_scroll}\nafter:\n{after_scroll}"
    );
}

#[test]
fn side_layout_width_half_uses_exact_split() {
    let (tx_raw, _rx) = unbounded_channel::<AppEvent>();
    let tx = AppEventSender::new(tx_raw);
    let view = ListSelectionView::new(
        SelectionViewParams {
            items: vec![SelectionItem {
                name: "Item 1".to_string(),
                dismiss_on_select: true,
                ..Default::default()
            }],
            side_content: Box::new(MarkerRenderable {
                marker: "W",
                height: 1,
            }),
            side_content_width: SideContentWidth::Half,
            side_content_min_width: 10,
            ..Default::default()
        },
        tx,
    );

    let content_width: u16 = 120;
    let expected = content_width.saturating_sub(SIDE_CONTENT_GAP) / 2;
    assert_eq!(view.side_layout_width(content_width), Some(expected));
}

#[test]
fn side_layout_width_half_falls_back_when_list_would_be_too_narrow() {
    let (tx_raw, _rx) = unbounded_channel::<AppEvent>();
    let tx = AppEventSender::new(tx_raw);
    let view = ListSelectionView::new(
        SelectionViewParams {
            items: vec![SelectionItem {
                name: "Item 1".to_string(),
                dismiss_on_select: true,
                ..Default::default()
            }],
            side_content: Box::new(MarkerRenderable {
                marker: "W",
                height: 1,
            }),
            side_content_width: SideContentWidth::Half,
            side_content_min_width: 50,
            ..Default::default()
        },
        tx,
    );

    assert_eq!(view.side_layout_width(80), None);
}

#[test]
fn stacked_side_content_is_used_when_side_by_side_does_not_fit() {
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
            side_content: Box::new(MarkerRenderable {
                marker: "W",
                height: 1,
            }),
            stacked_side_content: Some(Box::new(MarkerRenderable {
                marker: "N",
                height: 1,
            })),
            side_content_width: SideContentWidth::Half,
            side_content_min_width: 60,
            ..Default::default()
        },
        tx,
    );

    let rendered = render_lines_with_width(&view, 70);
    assert!(
        rendered.contains('N'),
        "expected stacked marker to be rendered:\n{rendered}"
    );
    assert!(
        !rendered.contains('W'),
        "wide marker should not render in stacked mode:\n{rendered}"
    );
}

#[test]
fn side_content_clearing_resets_symbols_and_style() {
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
            side_content: Box::new(MarkerRenderable {
                marker: "W",
                height: 1,
            }),
            side_content_width: SideContentWidth::Half,
            side_content_min_width: 10,
            ..Default::default()
        },
        tx,
    );

    let width = 120;
    let height = view.desired_height(width);
    let area = Rect::new(0, 0, width, height);
    let mut buf = Buffer::empty(area);
    for y in 0..height {
        for x in 0..width {
            buf[(x, y)]
                .set_symbol("X")
                .set_style(Style::default().bg(Color::Red));
        }
    }
    view.render(area, &mut buf);

    let cell = &buf[(width - 1, 0)];
    assert_eq!(cell.symbol(), " ");
    let style = cell.style();
    assert_eq!(style.fg, Some(Color::Reset));
    assert_eq!(style.bg, Some(Color::Reset));
    assert_eq!(style.underline_color, Some(Color::Reset));

    let mut saw_marker = false;
    for y in 0..height {
        for x in 0..width {
            let cell = &buf[(x, y)];
            if cell.symbol() == "W" {
                saw_marker = true;
                assert_eq!(cell.style().bg, Some(Color::Reset));
            }
        }
    }
    assert!(
        saw_marker,
        "expected side marker renderable to draw into buffer"
    );
}

#[test]
fn side_content_clearing_handles_non_zero_buffer_origin() {
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
            side_content: Box::new(MarkerRenderable {
                marker: "W",
                height: 1,
            }),
            side_content_width: SideContentWidth::Half,
            side_content_min_width: 10,
            ..Default::default()
        },
        tx,
    );

    let width = 120;
    let height = view.desired_height(width);
    let area = Rect::new(0, 20, width, height);
    let mut buf = Buffer::empty(area);
    for y in area.y..area.y + height {
        for x in area.x..area.x + width {
            buf[(x, y)]
                .set_symbol("X")
                .set_style(Style::default().bg(Color::Red));
        }
    }
    view.render(area, &mut buf);

    let cell = &buf[(area.x + width - 1, area.y)];
    assert_eq!(cell.symbol(), " ");
    assert_eq!(cell.style().bg, Some(Color::Reset));
}
