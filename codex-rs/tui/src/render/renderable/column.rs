use ratatui::buffer::Buffer;
use ratatui::layout::Rect;

use super::Renderable;
use super::RenderableItem;

pub struct ColumnRenderable<'a> {
    children: Vec<RenderableItem<'a>>,
}

impl Renderable for ColumnRenderable<'_> {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        let mut y = area.y;
        for child in &self.children {
            let child_area = Rect::new(area.x, y, area.width, child.desired_height(area.width))
                .intersection(area);
            if !child_area.is_empty() {
                child.render(child_area, buf);
            }
            y += child_area.height;
        }
    }

    fn desired_height(&self, width: u16) -> u16 {
        self.children
            .iter()
            .map(|child| child.desired_height(width))
            .sum()
    }

    /// Returns the cursor position of the first child that has a cursor position, offset by the
    /// child's position in the column.
    ///
    /// It is generally assumed that either zero or one child will have a cursor position.
    fn cursor_pos(&self, area: Rect) -> Option<(u16, u16)> {
        let mut y = area.y;
        for child in &self.children {
            let child_area = Rect::new(area.x, y, area.width, child.desired_height(area.width))
                .intersection(area);
            if !child_area.is_empty()
                && let Some((px, py)) = child.cursor_pos(child_area)
            {
                return Some((px, py));
            }
            y += child_area.height;
        }
        None
    }
}

impl<'a> ColumnRenderable<'a> {
    pub fn new() -> Self {
        Self { children: vec![] }
    }

    pub fn with<I, T>(children: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<RenderableItem<'a>>,
    {
        Self {
            children: children.into_iter().map(Into::into).collect(),
        }
    }

    pub fn push(&mut self, child: impl Into<Box<dyn Renderable + 'a>>) {
        self.children.push(RenderableItem::Owned(child.into()));
    }
}
