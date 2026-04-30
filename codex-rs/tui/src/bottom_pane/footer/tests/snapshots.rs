use super::*;

#[test]
fn footer_snapshots() {
    snapshot_footer(
        "footer_shortcuts_default",
        FooterProps {
            mode: FooterMode::ComposerEmpty,
            esc_backtrack_hint: false,
            use_shift_enter_hint: false,
            is_task_running: false,
            collaboration_modes_enabled: false,
            is_wsl: false,
            quit_shortcut_key: key_hint::ctrl(KeyCode::Char('c')),
            context_window_percent: None,
            context_window_used_tokens: None,
            status_line_value: None,
            status_line_enabled: false,
        },
    );

    snapshot_footer(
        "footer_shortcuts_shift_and_esc",
        FooterProps {
            mode: FooterMode::ShortcutOverlay,
            esc_backtrack_hint: true,
            use_shift_enter_hint: true,
            is_task_running: false,
            collaboration_modes_enabled: false,
            is_wsl: false,
            quit_shortcut_key: key_hint::ctrl(KeyCode::Char('c')),
            context_window_percent: None,
            context_window_used_tokens: None,
            status_line_value: None,
            status_line_enabled: false,
        },
    );

    snapshot_footer(
        "footer_shortcuts_collaboration_modes_enabled",
        FooterProps {
            mode: FooterMode::ShortcutOverlay,
            esc_backtrack_hint: false,
            use_shift_enter_hint: false,
            is_task_running: false,
            collaboration_modes_enabled: true,
            is_wsl: false,
            quit_shortcut_key: key_hint::ctrl(KeyCode::Char('c')),
            context_window_percent: None,
            context_window_used_tokens: None,
            status_line_value: None,
            status_line_enabled: false,
        },
    );

    snapshot_footer(
        "footer_ctrl_c_quit_idle",
        FooterProps {
            mode: FooterMode::QuitShortcutReminder,
            esc_backtrack_hint: false,
            use_shift_enter_hint: false,
            is_task_running: false,
            collaboration_modes_enabled: false,
            is_wsl: false,
            quit_shortcut_key: key_hint::ctrl(KeyCode::Char('c')),
            context_window_percent: None,
            context_window_used_tokens: None,
            status_line_value: None,
            status_line_enabled: false,
        },
    );

    snapshot_footer(
        "footer_ctrl_c_quit_running",
        FooterProps {
            mode: FooterMode::QuitShortcutReminder,
            esc_backtrack_hint: false,
            use_shift_enter_hint: false,
            is_task_running: true,
            collaboration_modes_enabled: false,
            is_wsl: false,
            quit_shortcut_key: key_hint::ctrl(KeyCode::Char('c')),
            context_window_percent: None,
            context_window_used_tokens: None,
            status_line_value: None,
            status_line_enabled: false,
        },
    );

    snapshot_footer(
        "footer_esc_hint_idle",
        FooterProps {
            mode: FooterMode::EscHint,
            esc_backtrack_hint: false,
            use_shift_enter_hint: false,
            is_task_running: false,
            collaboration_modes_enabled: false,
            is_wsl: false,
            quit_shortcut_key: key_hint::ctrl(KeyCode::Char('c')),
            context_window_percent: None,
            context_window_used_tokens: None,
            status_line_value: None,
            status_line_enabled: false,
        },
    );

    snapshot_footer(
        "footer_esc_hint_primed",
        FooterProps {
            mode: FooterMode::EscHint,
            esc_backtrack_hint: true,
            use_shift_enter_hint: false,
            is_task_running: false,
            collaboration_modes_enabled: false,
            is_wsl: false,
            quit_shortcut_key: key_hint::ctrl(KeyCode::Char('c')),
            context_window_percent: None,
            context_window_used_tokens: None,
            status_line_value: None,
            status_line_enabled: false,
        },
    );

    snapshot_footer(
        "footer_shortcuts_context_running",
        FooterProps {
            mode: FooterMode::ComposerEmpty,
            esc_backtrack_hint: false,
            use_shift_enter_hint: false,
            is_task_running: true,
            collaboration_modes_enabled: false,
            is_wsl: false,
            quit_shortcut_key: key_hint::ctrl(KeyCode::Char('c')),
            context_window_percent: Some(72),
            context_window_used_tokens: None,
            status_line_value: None,
            status_line_enabled: false,
        },
    );

    snapshot_footer(
        "footer_context_tokens_used",
        FooterProps {
            mode: FooterMode::ComposerEmpty,
            esc_backtrack_hint: false,
            use_shift_enter_hint: false,
            is_task_running: false,
            collaboration_modes_enabled: false,
            is_wsl: false,
            quit_shortcut_key: key_hint::ctrl(KeyCode::Char('c')),
            context_window_percent: None,
            context_window_used_tokens: Some(123_456),
            status_line_value: None,
            status_line_enabled: false,
        },
    );

    snapshot_footer(
        "footer_composer_has_draft_queue_hint_enabled",
        FooterProps {
            mode: FooterMode::ComposerHasDraft,
            esc_backtrack_hint: false,
            use_shift_enter_hint: false,
            is_task_running: true,
            collaboration_modes_enabled: false,
            is_wsl: false,
            quit_shortcut_key: key_hint::ctrl(KeyCode::Char('c')),
            context_window_percent: None,
            context_window_used_tokens: None,
            status_line_value: None,
            status_line_enabled: false,
        },
    );

    let props = FooterProps {
        mode: FooterMode::ComposerEmpty,
        esc_backtrack_hint: false,
        use_shift_enter_hint: false,
        is_task_running: false,
        collaboration_modes_enabled: true,
        is_wsl: false,
        quit_shortcut_key: key_hint::ctrl(KeyCode::Char('c')),
        context_window_percent: None,
        context_window_used_tokens: None,
        status_line_value: None,
        status_line_enabled: false,
    };

    snapshot_footer_with_mode_indicator(
        "footer_mode_indicator_wide",
        120,
        &props,
        Some(CollaborationModeIndicator::Plan),
    );

    snapshot_footer_with_mode_indicator(
        "footer_mode_indicator_narrow_overlap_hides",
        50,
        &props,
        Some(CollaborationModeIndicator::Plan),
    );

    let props = FooterProps {
        mode: FooterMode::ComposerEmpty,
        esc_backtrack_hint: false,
        use_shift_enter_hint: false,
        is_task_running: true,
        collaboration_modes_enabled: true,
        is_wsl: false,
        quit_shortcut_key: key_hint::ctrl(KeyCode::Char('c')),
        context_window_percent: None,
        context_window_used_tokens: None,
        status_line_value: None,
        status_line_enabled: false,
    };

    snapshot_footer_with_mode_indicator(
        "footer_mode_indicator_running_hides_hint",
        120,
        &props,
        Some(CollaborationModeIndicator::Plan),
    );

    let props = FooterProps {
        mode: FooterMode::ComposerEmpty,
        esc_backtrack_hint: false,
        use_shift_enter_hint: false,
        is_task_running: false,
        collaboration_modes_enabled: false,
        is_wsl: false,
        quit_shortcut_key: key_hint::ctrl(KeyCode::Char('c')),
        context_window_percent: None,
        context_window_used_tokens: None,
        status_line_value: Some(Line::from("Status line content".to_string())),
        status_line_enabled: true,
    };

    snapshot_footer("footer_status_line_overrides_shortcuts", props);

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
        status_line_value: None, // command timed out / empty
        status_line_enabled: true,
    };

    snapshot_footer_with_mode_indicator(
        "footer_status_line_enabled_mode_right",
        120,
        &props,
        Some(CollaborationModeIndicator::Plan),
    );

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
        status_line_value: None,
        status_line_enabled: false,
    };

    snapshot_footer_with_mode_indicator(
        "footer_status_line_disabled_context_right",
        120,
        &props,
        Some(CollaborationModeIndicator::Plan),
    );

    let props = FooterProps {
        mode: FooterMode::ComposerEmpty,
        esc_backtrack_hint: false,
        use_shift_enter_hint: false,
        is_task_running: false,
        collaboration_modes_enabled: false,
        is_wsl: false,
        quit_shortcut_key: key_hint::ctrl(KeyCode::Char('c')),
        context_window_percent: Some(50),
        context_window_used_tokens: None,
        status_line_value: None,
        status_line_enabled: true,
    };

    // has status line and no collaboration mode
    snapshot_footer_with_mode_indicator(
        "footer_status_line_enabled_no_mode_right",
        120,
        &props,
        None,
    );

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
            "Status line content that should truncate before the mode indicator".to_string(),
        )),
        status_line_enabled: true,
    };

    snapshot_footer_with_mode_indicator(
        "footer_status_line_truncated_with_gap",
        40,
        &props,
        Some(CollaborationModeIndicator::Plan),
    );
}
