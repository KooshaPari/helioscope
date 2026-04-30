use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::text::Line;

use crate::render::renderable::FlexRenderable;
use crate::render::renderable::Renderable;
use crate::render::renderable::RenderableItem;

use super::BottomPane;

impl BottomPane {
    fn as_renderable(&'_ self) -> RenderableItem<'_> {
        if let Some(view) = self.active_view() {
            return RenderableItem::Borrowed(view);
        }

        let mut inline_stack = FlexRenderable::new();
        if let Some(status) = &self.status {
            inline_stack.push(0, RenderableItem::Borrowed(status));
        }
        // Avoid double-surfacing the same summary and avoid adding an extra
        // row while the status line is already visible.
        if self.status.is_none() && !self.unified_exec_footer.is_empty() {
            inline_stack.push(0, RenderableItem::Borrowed(&self.unified_exec_footer));
        }

        let has_pending_thread_approvals = !self.pending_thread_approvals.is_empty();
        let has_queued_messages = !self.queued_user_messages.messages.is_empty();
        let has_status_or_footer = self.status.is_some() || !self.unified_exec_footer.is_empty();
        let has_inline_previews = has_pending_thread_approvals || has_queued_messages;
        if has_inline_previews && has_status_or_footer {
            inline_stack.push(0, RenderableItem::Owned("".into()));
        }
        inline_stack.push(1, RenderableItem::Borrowed(&self.pending_thread_approvals));
        if has_pending_thread_approvals && has_queued_messages {
            inline_stack.push(0, RenderableItem::Owned("".into()));
        }
        inline_stack.push(1, RenderableItem::Borrowed(&self.queued_user_messages));
        if !has_inline_previews && has_status_or_footer {
            inline_stack.push(0, RenderableItem::Owned("".into()));
        }

        let mut pane = FlexRenderable::new();
        pane.push(1, RenderableItem::Owned(inline_stack.into()));
        pane.push(0, RenderableItem::Borrowed(&self.composer));
        RenderableItem::Owned(Box::new(pane))
    }

    pub(crate) fn set_status_line(&mut self, status_line: Option<Line<'static>>) {
        if self.composer.set_status_line(status_line) {
            self.request_redraw();
        }
    }

    pub(crate) fn set_status_line_enabled(&mut self, enabled: bool) {
        if self.composer.set_status_line_enabled(enabled) {
            self.request_redraw();
        }
    }
}

impl Renderable for BottomPane {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        self.as_renderable().render(area, buf);
    }

    fn desired_height(&self, width: u16) -> u16 {
        self.as_renderable().desired_height(width)
    }

    fn cursor_pos(&self, area: Rect) -> Option<(u16, u16)> {
        self.as_renderable().cursor_pos(area)
    }
}
