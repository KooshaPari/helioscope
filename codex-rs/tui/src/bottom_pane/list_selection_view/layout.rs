/// Minimum list width (in content columns) required before the side-by-side
/// layout is activated. Keeps the list usable even when sharing horizontal
/// space with the side content panel.
const MIN_LIST_WIDTH_FOR_SIDE: u16 = 40;

/// Horizontal gap (in columns) between the list area and the side content
/// panel when side-by-side layout is active.
pub(super) const SIDE_CONTENT_GAP: u16 = 2;

/// Shared menu-surface horizontal inset (2 cells per side) used by selection popups.
const MENU_SURFACE_HORIZONTAL_INSET: u16 = 4;

/// Controls how the side content panel is sized relative to the popup width.
///
/// When the computed side width falls below `side_content_min_width` or the
/// remaining list area would be narrower than [`MIN_LIST_WIDTH_FOR_SIDE`], the
/// side-by-side layout is abandoned and the stacked fallback is used instead.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum SideContentWidth {
    /// Fixed number of columns.  `Fixed(0)` disables side content entirely.
    Fixed(u16),
    /// Exact 50/50 split of the content area (minus the inter-column gap).
    Half,
}

impl Default for SideContentWidth {
    fn default() -> Self {
        Self::Fixed(0)
    }
}

/// Returns the popup content width after subtracting the shared menu-surface
/// horizontal inset (2 columns on each side).
pub(crate) fn popup_content_width(total_width: u16) -> u16 {
    total_width.saturating_sub(MENU_SURFACE_HORIZONTAL_INSET)
}

/// Returns side-by-side layout widths as `(list_width, side_width)` when the
/// layout can fit. Returns `None` when the side panel is disabled/too narrow or
/// when the remaining list width would become unusably small.
pub(crate) fn side_by_side_layout_widths(
    content_width: u16,
    side_content_width: SideContentWidth,
    side_content_min_width: u16,
) -> Option<(u16, u16)> {
    let side_width = match side_content_width {
        SideContentWidth::Fixed(0) => return None,
        SideContentWidth::Fixed(width) => width,
        SideContentWidth::Half => content_width.saturating_sub(SIDE_CONTENT_GAP) / 2,
    };
    if side_width < side_content_min_width {
        return None;
    }
    let list_width = content_width.saturating_sub(SIDE_CONTENT_GAP + side_width);
    (list_width >= MIN_LIST_WIDTH_FOR_SIDE).then_some((list_width, side_width))
}
