//! Rendering logic and layout computation for the bottom pane chat composer.
//!
//! This module encapsulates the [`Renderable`] trait implementation for [`ChatComposer`],
//! providing cursor positioning and desired height computation for dynamic layout.
//!
//! The actual layout computation and rendering implementation is coordinated through
//! [`ChatComposer::layout_areas`] and [`ChatComposer::render_with_mask`].

use crate::render::renderable::Renderable;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;

use super::chat_composer::ChatComposer;

impl Renderable for ChatComposer {
    /// Returns the current cursor position within the textarea, if input is enabled.
    ///
    /// The cursor position accounts for the layout offset (left margin for the prompt character).
    /// Returns `None` if input is disabled or if a remote image is currently selected.
    fn cursor_pos(&self, area: Rect) -> Option<(u16, u16)> {
        if !self.is_cursor_visible() {
            return None;
        }

        let [_, _, textarea_rect, _] = self.layout_areas(area);
        self.textarea_cursor_pos(textarea_rect)
    }

    /// Computes the total height required to render the composer given a maximum width.
    ///
    /// Height includes:
    /// - Remote image rows (if any)
    /// - Textarea (with dynamic height based on content)
    /// - Border and padding (2 rows)
    /// - Popup height (footer or active modal)
    fn desired_height(&self, width: u16) -> u16 {
        let footer_props = self.footer_props();
        let footer_hint_height = self
            .custom_footer_height()
            .unwrap_or_else(|| super::footer::footer_height(&footer_props));
        let footer_spacing = Self::footer_spacing(footer_hint_height);
        let footer_total_height = footer_hint_height + footer_spacing;
        const COLS_WITH_MARGIN: u16 = 3; // LIVE_PREFIX_COLS + 1
        let inner_width = width.saturating_sub(COLS_WITH_MARGIN);
        let remote_images_height: u16 = self
            .remote_images_lines(inner_width)
            .len()
            .try_into()
            .unwrap_or(u16::MAX);
        let remote_images_separator = u16::from(remote_images_height > 0);
        self.textarea_desired_height(inner_width)
            + remote_images_height
            + remote_images_separator
            + 2
            + self.popup_or_footer_height(width, footer_total_height)
    }

    /// Renders the composer and all sub-components to the given buffer.
    ///
    /// This delegates to [`ChatComposer::render_with_mask`] with no mask character.
    fn render(&self, area: Rect, buf: &mut Buffer) {
        self.render_with_mask(area, buf, None);
    }
}
