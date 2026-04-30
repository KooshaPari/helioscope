use std::sync::Arc;

use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::text::Line;
use ratatui::text::Span;
use ratatui::widgets::Paragraph;
use ratatui::widgets::Widget;
use ratatui::widgets::WidgetRef;

use crate::render::Insets;
use crate::render::RectExt as _;

mod column;
mod flex;

pub use column::ColumnRenderable;
pub use flex::FlexRenderable;

pub trait Renderable {
    fn render(&self, area: Rect, buf: &mut Buffer);
    fn desired_height(&self, width: u16) -> u16;
    fn cursor_pos(&self, _area: Rect) -> Option<(u16, u16)> {
        None
    }
}

pub enum RenderableItem<'a> {
    Owned(Box<dyn Renderable + 'a>),
    Borrowed(&'a dyn Renderable),
}

impl<'a> Renderable for RenderableItem<'a> {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        match self {
            RenderableItem::Owned(child) => child.render(area, buf),
            RenderableItem::Borrowed(child) => child.render(area, buf),
        }
    }

    fn desired_height(&self, width: u16) -> u16 {
        match self {
            RenderableItem::Owned(child) => child.desired_height(width),
            RenderableItem::Borrowed(child) => child.desired_height(width),
        }
    }

    fn cursor_pos(&self, area: Rect) -> Option<(u16, u16)> {
        match self {
            RenderableItem::Owned(child) => child.cursor_pos(area),
            RenderableItem::Borrowed(child) => child.cursor_pos(area),
        }
    }
}

impl<'a> From<Box<dyn Renderable + 'a>> for RenderableItem<'a> {
    fn from(value: Box<dyn Renderable + 'a>) -> Self {
        RenderableItem::Owned(value)
    }
}

impl<'a, R> From<R> for Box<dyn Renderable + 'a>
where
    R: Renderable + 'a,
{
    fn from(value: R) -> Self {
        Box::new(value)
    }
}

impl Renderable for () {
    fn render(&self, _area: Rect, _buf: &mut Buffer) {}
    fn desired_height(&self, _width: u16) -> u16 {
        0
    }
}

impl Renderable for &str {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        self.render_ref(area, buf);
    }
    fn desired_height(&self, _width: u16) -> u16 {
        1
    }
}

impl Renderable for String {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        self.render_ref(area, buf);
    }
    fn desired_height(&self, _width: u16) -> u16 {
        1
    }
}

impl<'a> Renderable for Span<'a> {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        self.render_ref(area, buf);
    }
    fn desired_height(&self, _width: u16) -> u16 {
        1
    }
}

impl<'a> Renderable for Line<'a> {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        Widget::render(self, area, buf);
    }
    fn desired_height(&self, _width: u16) -> u16 {
        1
    }
}

impl<'a> Renderable for Paragraph<'a> {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        self.render_ref(area, buf);
    }
    fn desired_height(&self, width: u16) -> u16 {
        self.line_count(width) as u16
    }
}

impl<R: Renderable> Renderable for Option<R> {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        if let Some(renderable) = self {
            renderable.render(area, buf);
        }
    }

    fn desired_height(&self, width: u16) -> u16 {
        if let Some(renderable) = self {
            renderable.desired_height(width)
        } else {
            0
        }
    }
}

impl<R: Renderable> Renderable for Arc<R> {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        self.as_ref().render(area, buf);
    }
    fn desired_height(&self, width: u16) -> u16 {
        self.as_ref().desired_height(width)
    }
}

pub struct RowRenderable<'a> {
    children: Vec<(u16, RenderableItem<'a>)>,
}

impl Renderable for RowRenderable<'_> {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        let mut x = area.x;
        for (width, child) in &self.children {
            let available_width = area.width.saturating_sub(x - area.x);
            let child_area = Rect::new(x, area.y, (*width).min(available_width), area.height);
            if child_area.is_empty() {
                break;
            }
            child.render(child_area, buf);
            x = x.saturating_add(*width);
        }
    }
    fn desired_height(&self, width: u16) -> u16 {
        let mut max_height = 0;
        let mut width_remaining = width;
        for (child_width, child) in &self.children {
            let w = (*child_width).min(width_remaining);
            if w == 0 {
                break;
            }
            let height = child.desired_height(w);
            if height > max_height {
                max_height = height;
            }
            width_remaining = width_remaining.saturating_sub(w);
        }
        max_height
    }

    fn cursor_pos(&self, area: Rect) -> Option<(u16, u16)> {
        let mut x = area.x;
        for (width, child) in &self.children {
            let available_width = area.width.saturating_sub(x - area.x);
            let child_area = Rect::new(x, area.y, (*width).min(available_width), area.height);
            if !child_area.is_empty()
                && let Some(pos) = child.cursor_pos(child_area)
            {
                return Some(pos);
            }
            x = x.saturating_add(*width);
        }
        None
    }
}

impl<'a> RowRenderable<'a> {
    pub fn new() -> Self {
        Self { children: vec![] }
    }

    pub fn push(&mut self, width: u16, child: impl Into<Box<dyn Renderable>>) {
        self.children
            .push((width, RenderableItem::Owned(child.into())));
    }
}

pub struct InsetRenderable<'a> {
    child: RenderableItem<'a>,
    insets: Insets,
}

impl<'a> Renderable for InsetRenderable<'a> {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        self.child.render(area.inset(self.insets), buf);
    }
    fn desired_height(&self, width: u16) -> u16 {
        self.child
            .desired_height(width - self.insets.left - self.insets.right)
            + self.insets.top
            + self.insets.bottom
    }
    fn cursor_pos(&self, area: Rect) -> Option<(u16, u16)> {
        self.child.cursor_pos(area.inset(self.insets))
    }
}

impl<'a> InsetRenderable<'a> {
    pub fn new(child: impl Into<RenderableItem<'a>>, insets: Insets) -> Self {
        Self {
            child: child.into(),
            insets,
        }
    }
}

pub trait RenderableExt<'a> {
    fn inset(self, insets: Insets) -> RenderableItem<'a>;
}

impl<'a, R> RenderableExt<'a> for R
where
    R: Renderable + 'a,
{
    fn inset(self, insets: Insets) -> RenderableItem<'a> {
        let child: RenderableItem<'a> =
            RenderableItem::Owned(Box::new(self) as Box<dyn Renderable + 'a>);
        RenderableItem::Owned(Box::new(InsetRenderable { child, insets }))
    }
}
