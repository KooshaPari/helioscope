#[test]
fn prompt_discovery_lists_custom_prompts() {
    let prompts = vec![
        CustomPrompt {
            name: "foo".to_string(),
            path: "/tmp/foo.md".to_string().into(),
            content: "hello from foo".to_string(),
            description: None,
            argument_hint: None,
        },
        CustomPrompt {
            name: "bar".to_string(),
            path: "/tmp/bar.md".to_string().into(),
            content: "hello from bar".to_string(),
            description: None,
            argument_hint: None,
        },
    ];
    let popup = CommandPopup::new(prompts, CommandPopupFlags::default());
    let items = popup.filtered_items();
    let mut prompt_names: Vec<String> = items
        .into_iter()
        .filter_map(|it| match it {
            CommandItem::UserPrompt(i) => popup.prompt(i).map(|p| p.name.clone()),
            _ => None,
        })
        .collect();
    prompt_names.sort();
    assert_eq!(prompt_names, vec!["bar".to_string(), "foo".to_string()]);
}

#[test]
fn prompt_name_collision_with_builtin_is_ignored() {
    // Create a prompt named like a builtin (e.g. "init").
    let popup = CommandPopup::new(
        vec![CustomPrompt {
            name: "init".to_string(),
            path: "/tmp/init.md".to_string().into(),
            content: "should be ignored".to_string(),
            description: None,
            argument_hint: None,
        }],
        CommandPopupFlags::default(),
    );
    let items = popup.filtered_items();
    let has_collision_prompt = items.into_iter().any(|it| match it {
        CommandItem::UserPrompt(i) => popup.prompt(i).is_some_and(|p| p.name == "init"),
        _ => false,
    });
    assert!(
        !has_collision_prompt,
        "prompt with builtin name should be ignored"
    );
}

#[test]
fn prompt_description_uses_frontmatter_metadata() {
    let popup = CommandPopup::new(
        vec![CustomPrompt {
            name: "draftpr".to_string(),
            path: "/tmp/draftpr.md".to_string().into(),
            content: "body".to_string(),
            description: Some("Create feature branch, commit and open draft PR.".to_string()),
            argument_hint: None,
        }],
        CommandPopupFlags::default(),
    );
    let rows = popup.rows_from_matches(vec![(CommandItem::UserPrompt(0), None)]);
    let description = rows.first().and_then(|row| row.description.as_deref());
    assert_eq!(
        description,
        Some("Create feature branch, commit and open draft PR.")
    );
}

#[test]
fn prompt_description_falls_back_when_missing() {
    let popup = CommandPopup::new(
        vec![CustomPrompt {
            name: "foo".to_string(),
            path: "/tmp/foo.md".to_string().into(),
            content: "body".to_string(),
            description: None,
            argument_hint: None,
        }],
        CommandPopupFlags::default(),
    );
    let rows = popup.rows_from_matches(vec![(CommandItem::UserPrompt(0), None)]);
    let description = rows.first().and_then(|row| row.description.as_deref());
    assert_eq!(description, Some("send saved prompt"));
}
