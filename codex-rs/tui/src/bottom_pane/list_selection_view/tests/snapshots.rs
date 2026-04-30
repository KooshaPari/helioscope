macro_rules! assert_list_selection_snapshot {
    ($name:expr, $value:expr) => {
        insta::with_settings!({ snapshot_path => "../snapshots" }, {
            assert_snapshot!($name, $value);
        });
    };
}

#[test]
fn renders_blank_line_between_title_and_items_without_subtitle() {
    let view = make_selection_view(None);
    assert_list_selection_snapshot!(
        "list_selection_spacing_without_subtitle",
        render_lines(&view)
    );
}

#[test]
fn renders_blank_line_between_subtitle_and_items() {
    let view = make_selection_view(Some("Switch between Codex approval presets"));
    assert_list_selection_snapshot!("list_selection_spacing_with_subtitle", render_lines(&view));
}

#[test]
fn snapshot_footer_note_wraps() {
    let (tx_raw, _rx) = unbounded_channel::<AppEvent>();
    let tx = AppEventSender::new(tx_raw);
    let items = vec![SelectionItem {
        name: "Read Only".to_string(),
        description: Some("Codex can read files".to_string()),
        is_current: true,
        dismiss_on_select: true,
        ..Default::default()
    }];
    let footer_note = Line::from(vec![
        "Note: ".dim(),
        "Use /setup-default-sandbox".cyan(),
        " to allow network access.".dim(),
    ]);
    let view = ListSelectionView::new(
        SelectionViewParams {
            title: Some("Select Approval Mode".to_string()),
            footer_note: Some(footer_note),
            footer_hint: Some(standard_popup_hint_line()),
            items,
            ..Default::default()
        },
        tx,
    );
    assert_list_selection_snapshot!(
        "list_selection_footer_note_wraps",
        render_lines_with_width(&view, 40)
    );
}

#[test]
fn snapshot_model_picker_width_80() {
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
    assert_list_selection_snapshot!(
        "list_selection_model_picker_width_80",
        render_lines_with_width(&view, 80)
    );
}

#[test]
fn snapshot_narrow_width_preserves_third_option() {
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
    assert_list_selection_snapshot!(
        "list_selection_narrow_width_preserves_rows",
        render_lines_with_width(&view, 24)
    );
}

#[test]
fn snapshot_auto_visible_col_width_mode_scroll_behavior() {
    assert_list_selection_snapshot!(
        "list_selection_col_width_mode_auto_visible_scroll",
        render_before_after_scroll_snapshot(ColumnWidthMode::AutoVisible, 96)
    );
}

#[test]
fn snapshot_auto_all_rows_col_width_mode_scroll_behavior() {
    assert_list_selection_snapshot!(
        "list_selection_col_width_mode_auto_all_rows_scroll",
        render_before_after_scroll_snapshot(ColumnWidthMode::AutoAllRows, 96)
    );
}
