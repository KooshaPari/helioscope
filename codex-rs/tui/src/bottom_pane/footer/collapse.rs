use super::CollaborationModeIndicator;
use crate::key_hint;
use crate::ui_consts::FOOTER_INDENT_COLS;
use crossterm::event::KeyCode;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::Stylize;
use ratatui::text::Line;

const FOOTER_CONTEXT_GAP_COLS: u16 = 1;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum SummaryHintKind {
    None,
    Shortcuts,
    QueueMessage,
    QueueShort,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct LeftSideState {
    pub(super) hint: SummaryHintKind,
    pub(super) show_cycle_hint: bool,
}

pub(super) fn left_side_line(
    collaboration_mode_indicator: Option<CollaborationModeIndicator>,
    state: LeftSideState,
) -> Line<'static> {
    let mut line = Line::from("");
    match state.hint {
        SummaryHintKind::None => {}
        SummaryHintKind::Shortcuts => {
            line.push_span(key_hint::plain(KeyCode::Char('?')));
            line.push_span(" for shortcuts".dim());
        }
        SummaryHintKind::QueueMessage => {
            line.push_span(key_hint::plain(KeyCode::Tab));
            line.push_span(" to queue message".dim());
        }
        SummaryHintKind::QueueShort => {
            line.push_span(key_hint::plain(KeyCode::Tab));
            line.push_span(" to queue".dim());
        }
    };

    if let Some(collaboration_mode_indicator) = collaboration_mode_indicator {
        if !matches!(state.hint, SummaryHintKind::None) {
            line.push_span(" · ".dim());
        }
        line.push_span(collaboration_mode_indicator.styled_span(state.show_cycle_hint));
    }

    line
}

pub(crate) enum SummaryLeft {
    Default,
    Custom(Line<'static>),
    None,
}

/// Compute the single-line footer layout and whether the right-side context
/// indicator can be shown alongside it.
pub(crate) fn single_line_footer_layout(
    area: Rect,
    context_width: u16,
    collaboration_mode_indicator: Option<CollaborationModeIndicator>,
    show_cycle_hint: bool,
    show_shortcuts_hint: bool,
    show_queue_hint: bool,
) -> (SummaryLeft, bool) {
    let hint_kind = if show_queue_hint {
        SummaryHintKind::QueueMessage
    } else if show_shortcuts_hint {
        SummaryHintKind::Shortcuts
    } else {
        SummaryHintKind::None
    };
    let default_state = LeftSideState {
        hint: hint_kind,
        show_cycle_hint,
    };
    let default_line = left_side_line(collaboration_mode_indicator, default_state);
    let default_width = default_line.width() as u16;
    if default_width > 0 && can_show_left_with_context(area, default_width, context_width) {
        return (SummaryLeft::Default, true);
    }

    let state_line = |state: LeftSideState| -> Line<'static> {
        if state == default_state {
            default_line.clone()
        } else {
            left_side_line(collaboration_mode_indicator, state)
        }
    };
    let state_width = |state: LeftSideState| -> u16 { state_line(state).width() as u16 };
    // When the mode cycle hint is applicable (idle, non-queue mode), only show
    // the right-side context indicator if the "(shift+tab to cycle)" variant
    // can also fit.
    let context_requires_cycle_hint = show_cycle_hint && !show_queue_hint;

    if show_queue_hint {
        // In queue mode, prefer dropping context before dropping the queue hint.
        let queue_states = [
            default_state,
            LeftSideState {
                hint: SummaryHintKind::QueueMessage,
                show_cycle_hint: false,
            },
            LeftSideState {
                hint: SummaryHintKind::QueueShort,
                show_cycle_hint: false,
            },
        ];

        // Pass 1: keep the right-side context indicator if any queue variant
        // can fit alongside it. We skip adjacent duplicates because
        // `default_state` can already be the no-cycle queue variant.
        let mut previous_state: Option<LeftSideState> = None;
        for state in queue_states {
            if previous_state == Some(state) {
                continue;
            }
            previous_state = Some(state);
            let width = state_width(state);
            if width > 0 && can_show_left_with_context(area, width, context_width) {
                if state == default_state {
                    return (SummaryLeft::Default, true);
                }
                return (SummaryLeft::Custom(state_line(state)), true);
            }
        }

        // Pass 2: if context cannot fit, drop it before dropping the queue
        // hint. Reuse the same dedupe so we do not try equivalent states twice.
        let mut previous_state: Option<LeftSideState> = None;
        for state in queue_states {
            if previous_state == Some(state) {
                continue;
            }
            previous_state = Some(state);
            let width = state_width(state);
            if width > 0 && left_fits(area, width) {
                if state == default_state {
                    return (SummaryLeft::Default, false);
                }
                return (SummaryLeft::Custom(state_line(state)), false);
            }
        }
    } else if collaboration_mode_indicator.is_some() {
        if show_cycle_hint {
            // First fallback: drop shortcut hint but keep the cycle
            // hint on the mode label if it can fit.
            let cycle_state = LeftSideState {
                hint: SummaryHintKind::None,
                show_cycle_hint: true,
            };
            let cycle_width = state_width(cycle_state);
            if cycle_width > 0 && can_show_left_with_context(area, cycle_width, context_width) {
                return (SummaryLeft::Custom(state_line(cycle_state)), true);
            }
            if cycle_width > 0 && left_fits(area, cycle_width) {
                return (SummaryLeft::Custom(state_line(cycle_state)), false);
            }
        }

        // Next fallback: mode label only. If the cycle hint is applicable but
        // cannot fit, we also suppress context so the right side does not
        // outlive "(shift+tab to cycle)" on the left.
        let mode_only_state = LeftSideState {
            hint: SummaryHintKind::None,
            show_cycle_hint: false,
        };
        let mode_only_width = state_width(mode_only_state);
        if !context_requires_cycle_hint
            && mode_only_width > 0
            && can_show_left_with_context(area, mode_only_width, context_width)
        {
            return (
                SummaryLeft::Custom(state_line(mode_only_state)),
                true, // show_context
            );
        }
        if mode_only_width > 0 && left_fits(area, mode_only_width) {
            return (
                SummaryLeft::Custom(state_line(mode_only_state)),
                false, // show_context
            );
        }
    }

    // Final fallback: if queue variants (or other earlier states) could not fit
    // at all, drop every hint and try to show just the mode label.
    if let Some(collaboration_mode_indicator) = collaboration_mode_indicator {
        let mode_only_state = LeftSideState {
            hint: SummaryHintKind::None,
            show_cycle_hint: false,
        };
        // Compute the width without going through `state_line` so we do not
        // depend on `default_state` (which may still be a queue variant).
        let mode_only_width =
            left_side_line(Some(collaboration_mode_indicator), mode_only_state).width() as u16;
        if !context_requires_cycle_hint
            && can_show_left_with_context(area, mode_only_width, context_width)
        {
            return (
                SummaryLeft::Custom(left_side_line(
                    Some(collaboration_mode_indicator),
                    mode_only_state,
                )),
                true, // show_context
            );
        }
        if left_fits(area, mode_only_width) {
            return (
                SummaryLeft::Custom(left_side_line(
                    Some(collaboration_mode_indicator),
                    mode_only_state,
                )),
                false, // show_context
            );
        }
    }

    (SummaryLeft::None, true)
}

fn left_fits(area: Rect, left_width: u16) -> bool {
    let max_width = area.width.saturating_sub(FOOTER_INDENT_COLS as u16);
    left_width <= max_width
}

fn right_aligned_x(area: Rect, content_width: u16) -> Option<u16> {
    if area.is_empty() {
        return None;
    }

    let right_padding = FOOTER_INDENT_COLS as u16;
    let max_width = area.width.saturating_sub(right_padding);
    if content_width == 0 || max_width == 0 {
        return None;
    }

    if content_width >= max_width {
        return Some(area.x.saturating_add(right_padding));
    }

    Some(
        area.x
            .saturating_add(area.width)
            .saturating_sub(content_width)
            .saturating_sub(right_padding),
    )
}

pub(crate) fn max_left_width_for_right(area: Rect, right_width: u16) -> Option<u16> {
    let context_x = right_aligned_x(area, right_width)?;
    let left_start = area.x + FOOTER_INDENT_COLS as u16;

    // minimal one column gap between left and right
    let gap = FOOTER_CONTEXT_GAP_COLS;

    if context_x <= left_start + gap {
        return Some(0);
    }

    Some(context_x.saturating_sub(left_start + gap))
}

pub(crate) fn can_show_left_with_context(area: Rect, left_width: u16, context_width: u16) -> bool {
    let Some(context_x) = right_aligned_x(area, context_width) else {
        return true;
    };
    if left_width == 0 {
        return true;
    }
    let left_extent = FOOTER_INDENT_COLS as u16 + left_width + FOOTER_CONTEXT_GAP_COLS;
    left_extent <= context_x.saturating_sub(area.x)
}

pub(crate) fn render_context_right(area: Rect, buf: &mut Buffer, line: &Line<'static>) {
    if area.is_empty() {
        return;
    }

    let context_width = line.width() as u16;
    let Some(mut x) = right_aligned_x(area, context_width) else {
        return;
    };
    let y = area.y + area.height.saturating_sub(1);
    let max_x = area.x.saturating_add(area.width);

    for span in &line.spans {
        if x >= max_x {
            break;
        }
        let span_width = span.width() as u16;
        if span_width == 0 {
            continue;
        }
        let remaining = max_x.saturating_sub(x);
        let draw_width = span_width.min(remaining);
        buf.set_span(x, y, span, draw_width);
        x = x.saturating_add(span_width);
    }
}
