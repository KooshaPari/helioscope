use super::*;
use crate::line_truncation::truncate_line_with_ellipsis_if_overflow;
use crate::test_backend::VT100Backend;
use insta::assert_snapshot;
use pretty_assertions::assert_eq;
use ratatui::Terminal;
use ratatui::backend::Backend;
use ratatui::backend::TestBackend;

fn snapshot_footer(name: &str, props: FooterProps) {
    snapshot_footer_with_mode_indicator(name, 80, &props, None);
}

fn draw_footer_frame<B: Backend>(
    terminal: &mut Terminal<B>,
    height: u16,
    props: &FooterProps,
    collaboration_mode_indicator: Option<CollaborationModeIndicator>,
) {
    terminal
        .draw(|f| {
            let area = Rect::new(0, 0, f.area().width, height);
            let show_cycle_hint = !props.is_task_running;
            let show_shortcuts_hint = match props.mode {
                FooterMode::ComposerEmpty => true,
                FooterMode::QuitShortcutReminder
                | FooterMode::ShortcutOverlay
                | FooterMode::EscHint
                | FooterMode::ComposerHasDraft => false,
            };
            let show_queue_hint = match props.mode {
                FooterMode::ComposerHasDraft => props.is_task_running,
                FooterMode::QuitShortcutReminder
                | FooterMode::ComposerEmpty
                | FooterMode::ShortcutOverlay
                | FooterMode::EscHint => false,
            };
            let left_mode_indicator = if props.status_line_enabled {
                None
            } else {
                collaboration_mode_indicator
            };
            let available_width = area.width.saturating_sub(FOOTER_INDENT_COLS as u16) as usize;
            let mut truncated_status_line = if props.status_line_enabled
                && matches!(
                    props.mode,
                    FooterMode::ComposerEmpty | FooterMode::ComposerHasDraft
                ) {
                props
                    .status_line_value
                    .as_ref()
                    .map(|line| line.clone().dim())
                    .map(|line| truncate_line_with_ellipsis_if_overflow(line, available_width))
            } else {
                None
            };
            let mut left_width = if props.status_line_enabled {
                truncated_status_line
                    .as_ref()
                    .map(|line| line.width() as u16)
                    .unwrap_or(0)
            } else {
                footer_line_width(
                    props,
                    left_mode_indicator,
                    show_cycle_hint,
                    show_shortcuts_hint,
                    show_queue_hint,
                )
            };
            let right_line = if props.status_line_enabled {
                let full = mode_indicator_line(collaboration_mode_indicator, show_cycle_hint);
                let compact = mode_indicator_line(collaboration_mode_indicator, false);
                let full_width = full.as_ref().map(|line| line.width() as u16).unwrap_or(0);
                if can_show_left_with_context(area, left_width, full_width) {
                    full
                } else {
                    compact
                }
            } else {
                Some(context_window_line(
                    props.context_window_percent,
                    props.context_window_used_tokens,
                ))
            };
            let right_width = right_line
                .as_ref()
                .map(|line| line.width() as u16)
                .unwrap_or(0);
            if props.status_line_enabled
                && let Some(max_left) = max_left_width_for_right(area, right_width)
                && left_width > max_left
                && let Some(line) = props
                    .status_line_value
                    .as_ref()
                    .map(|line| line.clone().dim())
                    .map(|line| truncate_line_with_ellipsis_if_overflow(line, max_left as usize))
            {
                left_width = line.width() as u16;
                truncated_status_line = Some(line);
            }
            let can_show_left_and_context =
                can_show_left_with_context(area, left_width, right_width);
            if matches!(
                props.mode,
                FooterMode::ComposerEmpty | FooterMode::ComposerHasDraft
            ) {
                let (summary_left, show_context) = single_line_footer_layout(
                    area,
                    right_width,
                    left_mode_indicator,
                    show_cycle_hint,
                    show_shortcuts_hint,
                    show_queue_hint,
                );
                match summary_left {
                    SummaryLeft::Default => {
                        if props.status_line_enabled {
                            if let Some(line) = truncated_status_line.clone() {
                                render_footer_line(area, f.buffer_mut(), line);
                            }
                        } else {
                            render_footer_from_props(
                                area,
                                f.buffer_mut(),
                                props,
                                left_mode_indicator,
                                show_cycle_hint,
                                show_shortcuts_hint,
                                show_queue_hint,
                            );
                        }
                    }
                    SummaryLeft::Custom(line) => {
                        render_footer_line(area, f.buffer_mut(), line);
                    }
                    SummaryLeft::None => {}
                }
                if show_context && let Some(line) = &right_line {
                    render_context_right(area, f.buffer_mut(), line);
                }
            } else {
                render_footer_from_props(
                    area,
                    f.buffer_mut(),
                    props,
                    left_mode_indicator,
                    show_cycle_hint,
                    show_shortcuts_hint,
                    show_queue_hint,
                );
                let show_context = can_show_left_and_context
                    && !matches!(
                        props.mode,
                        FooterMode::EscHint
                            | FooterMode::QuitShortcutReminder
                            | FooterMode::ShortcutOverlay
                    );
                if show_context && let Some(line) = &right_line {
                    render_context_right(area, f.buffer_mut(), line);
                }
            }
        })
        .unwrap();
}

fn snapshot_footer_with_mode_indicator(
    name: &str,
    width: u16,
    props: &FooterProps,
    collaboration_mode_indicator: Option<CollaborationModeIndicator>,
) {
    let height = footer_height(props).max(1);
    let mut terminal = Terminal::new(TestBackend::new(width, height)).unwrap();
    draw_footer_frame(&mut terminal, height, props, collaboration_mode_indicator);
    assert_snapshot!(name, terminal.backend());
}

fn render_footer_with_mode_indicator(
    width: u16,
    props: &FooterProps,
    collaboration_mode_indicator: Option<CollaborationModeIndicator>,
) -> String {
    let height = footer_height(props).max(1);
    let mut terminal = Terminal::new(VT100Backend::new(width, height)).expect("terminal");
    draw_footer_frame(&mut terminal, height, props, collaboration_mode_indicator);
    terminal.backend().vt100().screen().contents()
}

mod snapshots;

#[test]
fn footer_status_line_truncates_to_keep_mode_indicator() {
    let props = FooterProps {
        mode: FooterMode::ComposerEmpty,
        esc_backtrack_hint: false,
        use_shift_enter_hint: false,
        is_task_running: false,
        collaboration_modes_enabled: true,
        is_wsl: false,
        quit_shortcut_key: key_hint::ctrl(KeyCode::Char('c')),
        context_window_percent: Some(50),
        context_window_used_tokens: None,
        status_line_value: Some(Line::from(
            "Status line content that is definitely too long to fit alongside the mode label"
                .to_string(),
        )),
        status_line_enabled: true,
    };

    let screen =
        render_footer_with_mode_indicator(80, &props, Some(CollaborationModeIndicator::Plan));
    let collapsed = screen.split_whitespace().collect::<Vec<_>>().join(" ");
    assert!(
        collapsed.contains("Plan mode"),
        "mode indicator should remain visible"
    );
    assert!(
        !collapsed.contains("shift+tab to cycle"),
        "compact mode indicator should be used when space is tight"
    );
    assert!(
        screen.contains('…'),
        "status line should be truncated with ellipsis to keep mode indicator"
    );
}

#[test]
fn paste_image_shortcut_prefers_ctrl_alt_v_under_wsl() {
    let descriptor = SHORTCUTS
        .iter()
        .find(|descriptor| descriptor.id == ShortcutId::PasteImage)
        .expect("paste image shortcut");

    let is_wsl = {
        #[cfg(target_os = "linux")]
        {
            crate::clipboard_paste::is_probably_wsl()
        }
        #[cfg(not(target_os = "linux"))]
        {
            false
        }
    };

    let expected_key = if is_wsl {
        key_hint::ctrl_alt(KeyCode::Char('v'))
    } else {
        key_hint::ctrl(KeyCode::Char('v'))
    };

    let actual_key = descriptor
        .binding_for(ShortcutsState {
            use_shift_enter_hint: false,
            esc_backtrack_hint: false,
            is_wsl,
            collaboration_modes_enabled: false,
        })
        .expect("shortcut binding")
        .key;

    assert_eq!(actual_key, expected_key);
}
