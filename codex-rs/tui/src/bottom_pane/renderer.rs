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
        if !self.input_enabled || self.selected_remote_image_index.is_some() {
            return None;
        }

        let [_, _, textarea_rect, _] = self.layout_areas(area);
        let state = *self.textarea_state.borrow();
        self.textarea.cursor_pos_with_state(textarea_rect, state)
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
        self.textarea.desired_height(inner_width)
            + remote_images_height
            + remote_images_separator
            + 2
            + match &self.active_popup {
                super::chat_composer::ActivePopup::None => footer_total_height,
                super::chat_composer::ActivePopup::Command(c) => {
                    c.calculate_required_height(width)
                }
                super::chat_composer::ActivePopup::File(c) => c.calculate_required_height(),
                super::chat_composer::ActivePopup::Skill(c) => {
                    c.calculate_required_height(width)
                }
            }
    }

    /// Renders the composer and all sub-components to the given buffer.
    ///
    /// This delegates to [`ChatComposer::render_with_mask`] with no mask character.
    fn render(&self, area: Rect, buf: &mut Buffer) {
        self.render_with_mask(area, buf, None);
    }
}

impl ChatComposer {
    /// Computes the vertical spacing (in rows) between the composer and footer.
    ///
    /// Returns 0 if the footer height is 0 (i.e., no footer), otherwise returns
    /// the configured [`FOOTER_SPACING_HEIGHT`] constant.
    pub(crate) fn footer_spacing(footer_hint_height: u16) -> u16 {
        const FOOTER_SPACING_HEIGHT: u16 = 0;
        if footer_hint_height == 0 {
            0
        } else {
            FOOTER_SPACING_HEIGHT
        }
    }
}
