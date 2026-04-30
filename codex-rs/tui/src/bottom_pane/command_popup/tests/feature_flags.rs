#[test]
fn collab_command_hidden_when_collaboration_modes_disabled() {
    let mut popup = CommandPopup::new(Vec::new(), CommandPopupFlags::default());
    popup.on_composer_text_change("/".to_string());

    let cmds: Vec<&str> = popup
        .filtered_items()
        .into_iter()
        .filter_map(|item| match item {
            CommandItem::Builtin(cmd) => Some(cmd.command()),
            CommandItem::UserPrompt(_) => None,
        })
        .collect();
    assert!(
        !cmds.contains(&"collab"),
        "expected '/collab' to be hidden when collaboration modes are disabled, got {cmds:?}"
    );
    assert!(
        !cmds.contains(&"plan"),
        "expected '/plan' to be hidden when collaboration modes are disabled, got {cmds:?}"
    );
}

#[test]
fn collab_command_visible_when_collaboration_modes_enabled() {
    let mut popup = CommandPopup::new(
        Vec::new(),
        CommandPopupFlags {
            collaboration_modes_enabled: true,
            connectors_enabled: false,
            personality_command_enabled: true,
            realtime_conversation_enabled: false,
            audio_device_selection_enabled: false,
            windows_degraded_sandbox_active: false,
        },
    );
    popup.on_composer_text_change("/collab".to_string());

    match popup.selected_item() {
        Some(CommandItem::Builtin(cmd)) => assert_eq!(cmd.command(), "collab"),
        other => panic!("expected collab to be selected for exact match, got {other:?}"),
    }
}

#[test]
fn plan_command_visible_when_collaboration_modes_enabled() {
    let mut popup = CommandPopup::new(
        Vec::new(),
        CommandPopupFlags {
            collaboration_modes_enabled: true,
            connectors_enabled: false,
            personality_command_enabled: true,
            realtime_conversation_enabled: false,
            audio_device_selection_enabled: false,
            windows_degraded_sandbox_active: false,
        },
    );
    popup.on_composer_text_change("/plan".to_string());

    match popup.selected_item() {
        Some(CommandItem::Builtin(cmd)) => assert_eq!(cmd.command(), "plan"),
        other => panic!("expected plan to be selected for exact match, got {other:?}"),
    }
}

#[test]
fn personality_command_hidden_when_disabled() {
    let mut popup = CommandPopup::new(
        Vec::new(),
        CommandPopupFlags {
            collaboration_modes_enabled: true,
            connectors_enabled: false,
            personality_command_enabled: false,
            realtime_conversation_enabled: false,
            audio_device_selection_enabled: false,
            windows_degraded_sandbox_active: false,
        },
    );
    popup.on_composer_text_change("/pers".to_string());

    let cmds: Vec<&str> = popup
        .filtered_items()
        .into_iter()
        .filter_map(|item| match item {
            CommandItem::Builtin(cmd) => Some(cmd.command()),
            CommandItem::UserPrompt(_) => None,
        })
        .collect();
    assert!(
        !cmds.contains(&"personality"),
        "expected '/personality' to be hidden when disabled, got {cmds:?}"
    );
}

#[test]
fn personality_command_visible_when_enabled() {
    let mut popup = CommandPopup::new(
        Vec::new(),
        CommandPopupFlags {
            collaboration_modes_enabled: true,
            connectors_enabled: false,
            personality_command_enabled: true,
            realtime_conversation_enabled: false,
            audio_device_selection_enabled: false,
            windows_degraded_sandbox_active: false,
        },
    );
    popup.on_composer_text_change("/personality".to_string());

    match popup.selected_item() {
        Some(CommandItem::Builtin(cmd)) => assert_eq!(cmd.command(), "personality"),
        other => panic!("expected personality to be selected for exact match, got {other:?}"),
    }
}

#[test]
fn settings_command_hidden_when_audio_device_selection_is_disabled() {
    let mut popup = CommandPopup::new(
        Vec::new(),
        CommandPopupFlags {
            collaboration_modes_enabled: false,
            connectors_enabled: false,
            personality_command_enabled: true,
            realtime_conversation_enabled: true,
            audio_device_selection_enabled: false,
            windows_degraded_sandbox_active: false,
        },
    );
    popup.on_composer_text_change("/aud".to_string());

    let cmds: Vec<&str> = popup
        .filtered_items()
        .into_iter()
        .filter_map(|item| match item {
            CommandItem::Builtin(cmd) => Some(cmd.command()),
            CommandItem::UserPrompt(_) => None,
        })
        .collect();

    assert!(
        !cmds.contains(&"settings"),
        "expected '/settings' to be hidden when audio device selection is disabled, got {cmds:?}"
    );
}
