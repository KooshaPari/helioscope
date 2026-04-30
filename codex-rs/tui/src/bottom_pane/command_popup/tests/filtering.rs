#[test]
fn filter_includes_init_when_typing_prefix() {
    let mut popup = CommandPopup::new(Vec::new(), CommandPopupFlags::default());
    // Simulate the composer line starting with '/in' so the popup filters
    // matching commands by prefix.
    popup.on_composer_text_change("/in".to_string());

    // Access the filtered list via the selected command and ensure that
    // one of the matches is the new "init" command.
    let matches = popup.filtered_items();
    let has_init = matches.iter().any(|item| match item {
        CommandItem::Builtin(cmd) => cmd.command() == "init",
        CommandItem::UserPrompt(_) => false,
    });
    assert!(
        has_init,
        "expected '/init' to appear among filtered commands"
    );
}

#[test]
fn selecting_init_by_exact_match() {
    let mut popup = CommandPopup::new(Vec::new(), CommandPopupFlags::default());
    popup.on_composer_text_change("/init".to_string());

    // When an exact match exists, the selected command should be that
    // command by default.
    let selected = popup.selected_item();
    match selected {
        Some(CommandItem::Builtin(cmd)) => assert_eq!(cmd.command(), "init"),
        Some(CommandItem::UserPrompt(_)) => panic!("unexpected prompt selected for '/init'"),
        None => panic!("expected a selected command for exact match"),
    }
}

#[test]
fn model_is_first_suggestion_for_mo() {
    let mut popup = CommandPopup::new(Vec::new(), CommandPopupFlags::default());
    popup.on_composer_text_change("/mo".to_string());
    let matches = popup.filtered_items();
    match matches.first() {
        Some(CommandItem::Builtin(cmd)) => assert_eq!(cmd.command(), "model"),
        Some(CommandItem::UserPrompt(_)) => {
            panic!("unexpected prompt ranked before '/model' for '/mo'")
        }
        None => panic!("expected at least one match for '/mo'"),
    }
}

#[test]
fn filtered_commands_keep_presentation_order_for_prefix() {
    let mut popup = CommandPopup::new(Vec::new(), CommandPopupFlags::default());
    popup.on_composer_text_change("/m".to_string());

    let cmds: Vec<&str> = popup
        .filtered_items()
        .into_iter()
        .filter_map(|item| match item {
            CommandItem::Builtin(cmd) => Some(cmd.command()),
            CommandItem::UserPrompt(_) => None,
        })
        .collect();
    assert_eq!(cmds, vec!["model", "mention", "mcp"]);
}

#[test]
fn prefix_filter_limits_matches_for_ac() {
    let mut popup = CommandPopup::new(Vec::new(), CommandPopupFlags::default());
    popup.on_composer_text_change("/ac".to_string());

    let cmds: Vec<&str> = popup
        .filtered_items()
        .into_iter()
        .filter_map(|item| match item {
            CommandItem::Builtin(cmd) => Some(cmd.command()),
            CommandItem::UserPrompt(_) => None,
        })
        .collect();
    assert!(
        !cmds.contains(&"compact"),
        "expected prefix search for '/ac' to exclude 'compact', got {cmds:?}"
    );
}

#[test]
fn quit_hidden_in_empty_filter_but_shown_for_prefix() {
    let mut popup = CommandPopup::new(Vec::new(), CommandPopupFlags::default());
    popup.on_composer_text_change("/".to_string());
    let items = popup.filtered_items();
    assert!(!items.contains(&CommandItem::Builtin(SlashCommand::Quit)));

    popup.on_composer_text_change("/qu".to_string());
    let items = popup.filtered_items();
    assert!(items.contains(&CommandItem::Builtin(SlashCommand::Quit)));
}

#[test]
fn debug_commands_are_hidden_from_popup() {
    let popup = CommandPopup::new(Vec::new(), CommandPopupFlags::default());
    let cmds: Vec<&str> = popup
        .filtered_items()
        .into_iter()
        .filter_map(|item| match item {
            CommandItem::Builtin(cmd) => Some(cmd.command()),
            CommandItem::UserPrompt(_) => None,
        })
        .collect();

    assert!(
        !cmds.iter().any(|name| name.starts_with("debug")),
        "expected no /debug* command in popup menu, got {cmds:?}"
    );
}
