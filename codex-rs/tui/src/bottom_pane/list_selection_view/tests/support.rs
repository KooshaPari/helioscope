use super::*;
pub(super) use crate::app_event::AppEvent;
pub(super) use crate::bottom_pane::popup_consts::standard_popup_hint_line;
pub(super) use crossterm::event::KeyCode;
pub(super) use ratatui::buffer::Buffer;
pub(super) use ratatui::layout::Rect;
pub(super) use ratatui::style::Color;
pub(super) use ratatui::style::Style;
pub(super) use tokio::sync::mpsc::unbounded_channel;

pub(super) struct MarkerRenderable {
    pub(super) marker: &'static str,
    pub(super) height: u16,
}

impl Renderable for MarkerRenderable {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        for y in area.y..area.y.saturating_add(area.height) {
            for x in area.x..area.x.saturating_add(area.width) {
                if x < buf.area().width && y < buf.area().height {
                    buf[(x, y)].set_symbol(self.marker);
                }
            }
        }
    }

    fn desired_height(&self, _width: u16) -> u16 {
        self.height
    }
}

pub(super) struct StyledMarkerRenderable {
    pub(super) marker: &'static str,
    pub(super) style: Style,
    pub(super) height: u16,
}

impl Renderable for StyledMarkerRenderable {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        for y in area.y..area.y.saturating_add(area.height) {
            for x in area.x..area.x.saturating_add(area.width) {
                if x < buf.area().width && y < buf.area().height {
                    buf[(x, y)].set_symbol(self.marker).set_style(self.style);
                }
            }
        }
    }

    fn desired_height(&self, _width: u16) -> u16 {
        self.height
    }
}

pub(super) fn make_selection_view(subtitle: Option<&str>) -> ListSelectionView {
    let (tx_raw, _rx) = unbounded_channel::<AppEvent>();
    let tx = AppEventSender::new(tx_raw);
    let items = vec![
        SelectionItem {
            name: "Read Only".to_string(),
            description: Some("Codex can read files".to_string()),
            is_current: true,
            dismiss_on_select: true,
            ..Default::default()
        },
        SelectionItem {
            name: "Full Access".to_string(),
            description: Some("Codex can edit files".to_string()),
            is_current: false,
            dismiss_on_select: true,
            ..Default::default()
        },
    ];
    ListSelectionView::new(
        SelectionViewParams {
            title: Some("Select Approval Mode".to_string()),
            subtitle: subtitle.map(str::to_string),
            footer_hint: Some(standard_popup_hint_line()),
            items,
            ..Default::default()
        },
        tx,
    )
}

pub(super) fn render_lines(view: &ListSelectionView) -> String {
    render_lines_with_width(view, 48)
}

pub(super) fn render_lines_with_width(view: &ListSelectionView, width: u16) -> String {
    render_lines_in_area(view, width, view.desired_height(width))
}

pub(super) fn render_lines_in_area(view: &ListSelectionView, width: u16, height: u16) -> String {
    let area = Rect::new(0, 0, width, height);
    let mut buf = Buffer::empty(area);
    view.render(area, &mut buf);

    let lines: Vec<String> = (0..area.height)
        .map(|row| {
            let mut line = String::new();
            for col in 0..area.width {
                let symbol = buf[(area.x + col, area.y + row)].symbol();
                if symbol.is_empty() {
                    line.push(' ');
                } else {
                    line.push_str(symbol);
                }
            }
            line
        })
        .collect();
    lines.join("\n")
}

pub(super) fn description_col(rendered: &str, item_marker: &str, description: &str) -> usize {
    let line = rendered
        .lines()
        .find(|line| line.contains(item_marker) && line.contains(description))
        .expect("expected rendered line to contain row marker and description");
    line.find(description)
        .expect("expected rendered line to contain description")
}

pub(super) fn make_scrolling_width_items() -> Vec<SelectionItem> {
    let mut items: Vec<SelectionItem> = (1..=8)
        .map(|idx| SelectionItem {
            name: format!("Item {idx}"),
            description: Some(format!("desc {idx}")),
            dismiss_on_select: true,
            ..Default::default()
        })
        .collect();
    items.push(SelectionItem {
        name: "Item 9 with an intentionally much longer name".to_string(),
        description: Some("desc 9".to_string()),
        dismiss_on_select: true,
        ..Default::default()
    });
    items
}

pub(super) fn model_picker_items() -> Vec<SelectionItem> {
    vec![
        SelectionItem {
            name: "gpt-5.1-codex".to_string(),
            description: Some(
                "Optimized for Codex. Balance of reasoning quality and coding ability.".to_string(),
            ),
            is_current: true,
            dismiss_on_select: true,
            ..Default::default()
        },
        SelectionItem {
            name: "gpt-5.1-codex-mini".to_string(),
            description: Some(
                "Optimized for Codex. Cheaper, faster, but less capable.".to_string(),
            ),
            dismiss_on_select: true,
            ..Default::default()
        },
        SelectionItem {
            name: "gpt-4.1-codex".to_string(),
            description: Some(
                "Legacy model. Use when you need compatibility with older automations.".to_string(),
            ),
            dismiss_on_select: true,
            ..Default::default()
        },
    ]
}

pub(super) fn render_before_after_scroll_snapshot(
    col_width_mode: ColumnWidthMode,
    width: u16,
) -> String {
    let (tx_raw, _rx) = unbounded_channel::<AppEvent>();
    let tx = AppEventSender::new(tx_raw);
    let mut view = ListSelectionView::new(
        SelectionViewParams {
            title: Some("Debug".to_string()),
            items: make_scrolling_width_items(),
            col_width_mode,
            ..Default::default()
        },
        tx,
    );

    let before_scroll = render_lines_with_width(&view, width);
    for _ in 0..8 {
        view.handle_key_event(KeyEvent::from(KeyCode::Down));
    }
    let after_scroll = render_lines_with_width(&view, width);

    format!("before scroll:\n{before_scroll}\n\nafter scroll:\n{after_scroll}")
}
