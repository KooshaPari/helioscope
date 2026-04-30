#[tokio::test]
async fn apps_popup_refreshes_when_connectors_snapshot_updates() {
    let (mut chat, _rx, _op_rx) = make_chatwidget_manual(None).await;
    chat.config.features.enable(Feature::Apps);
    chat.bottom_pane.set_connectors_enabled(true);
    let notion_id = "unit_test_apps_popup_refresh_connector_1";
    let linear_id = "unit_test_apps_popup_refresh_connector_2";

    chat.on_connectors_loaded(
        Ok(ConnectorsSnapshot {
            connectors: vec![codex_chatgpt::connectors::AppInfo {
                id: notion_id.to_string(),
                name: "Notion".to_string(),
                description: Some("Workspace docs".to_string()),
                logo_url: None,
                logo_url_dark: None,
                distribution_channel: None,
                branding: None,
                app_metadata: None,
                labels: None,
                install_url: Some("https://example.test/notion".to_string()),
                is_accessible: true,
                is_enabled: true,
            }],
        }),
        false,
    );
    chat.add_connectors_output();
    assert!(
        chat.connectors_prefetch_in_flight,
        "expected /apps to trigger a forced connectors refresh"
    );

    let before = render_bottom_popup(&chat, 80);
    assert!(
        before.contains("Installed 1 of 1 available apps."),
        "expected initial apps popup snapshot, got:\n{before}"
    );
    assert!(
        before.contains("Installed. Press Enter to open the app page"),
        "expected selected app description to explain the app page action, got:\n{before}"
    );

    chat.on_connectors_loaded(
        Ok(ConnectorsSnapshot {
            connectors: vec![
                codex_chatgpt::connectors::AppInfo {
                    id: notion_id.to_string(),
                    name: "Notion".to_string(),
                    description: Some("Workspace docs".to_string()),
                    logo_url: None,
                    logo_url_dark: None,
                    distribution_channel: None,
                    branding: None,
                    app_metadata: None,
                    labels: None,
                    install_url: Some("https://example.test/notion".to_string()),
                    is_accessible: true,
                    is_enabled: true,
                },
                codex_chatgpt::connectors::AppInfo {
                    id: linear_id.to_string(),
                    name: "Linear".to_string(),
                    description: Some("Project tracking".to_string()),
                    logo_url: None,
                    logo_url_dark: None,
                    distribution_channel: None,
                    branding: None,
                    app_metadata: None,
                    labels: None,
                    install_url: Some("https://example.test/linear".to_string()),
                    is_accessible: true,
                    is_enabled: true,
                },
            ],
        }),
        true,
    );

    let after = render_bottom_popup(&chat, 80);
    assert!(
        after.contains("Installed 2 of 2 available apps."),
        "expected refreshed apps popup snapshot, got:\n{after}"
    );
    assert!(
        after.contains("Linear"),
        "expected refreshed popup to include new connector, got:\n{after}"
    );
}

#[tokio::test]
async fn apps_refresh_failure_keeps_existing_full_snapshot() {
    let (mut chat, _rx, _op_rx) = make_chatwidget_manual(None).await;
    chat.config.features.enable(Feature::Apps);
    chat.bottom_pane.set_connectors_enabled(true);
    let notion_id = "unit_test_apps_refresh_failure_connector_1";
    let linear_id = "unit_test_apps_refresh_failure_connector_2";

    let full_connectors = vec![
        codex_chatgpt::connectors::AppInfo {
            id: notion_id.to_string(),
            name: "Notion".to_string(),
            description: Some("Workspace docs".to_string()),
            logo_url: None,
            logo_url_dark: None,
            distribution_channel: None,
            branding: None,
            app_metadata: None,
            labels: None,
            install_url: Some("https://example.test/notion".to_string()),
            is_accessible: true,
            is_enabled: true,
        },
        codex_chatgpt::connectors::AppInfo {
            id: linear_id.to_string(),
            name: "Linear".to_string(),
            description: Some("Project tracking".to_string()),
            logo_url: None,
            logo_url_dark: None,
            distribution_channel: None,
            branding: None,
            app_metadata: None,
            labels: None,
            install_url: Some("https://example.test/linear".to_string()),
            is_accessible: false,
            is_enabled: true,
        },
    ];
    chat.on_connectors_loaded(
        Ok(ConnectorsSnapshot {
            connectors: full_connectors.clone(),
        }),
        true,
    );

    chat.on_connectors_loaded(
        Ok(ConnectorsSnapshot {
            connectors: vec![codex_chatgpt::connectors::AppInfo {
                id: notion_id.to_string(),
                name: "Notion".to_string(),
                description: Some("Workspace docs".to_string()),
                logo_url: None,
                logo_url_dark: None,
                distribution_channel: None,
                branding: None,
                app_metadata: None,
                labels: None,
                install_url: Some("https://example.test/notion".to_string()),
                is_accessible: true,
                is_enabled: true,
            }],
        }),
        false,
    );
    chat.on_connectors_loaded(Err("failed to load apps".to_string()), true);

    assert_matches!(
        &chat.connectors_cache,
        ConnectorsCacheState::Ready(snapshot) if snapshot.connectors == full_connectors
    );

    chat.add_connectors_output();
    let popup = render_bottom_popup(&chat, 80);
    assert!(
        popup.contains("Installed 1 of 2 available apps."),
        "expected previous full snapshot to be preserved, got:\n{popup}"
    );
}

#[tokio::test]
async fn apps_refresh_failure_with_cached_snapshot_triggers_pending_force_refetch() {
    let (mut chat, _rx, _op_rx) = make_chatwidget_manual(None).await;
    chat.config.features.enable(Feature::Apps);
    chat.bottom_pane.set_connectors_enabled(true);
    chat.connectors_prefetch_in_flight = true;
    chat.connectors_force_refetch_pending = true;

    let full_connectors = vec![codex_chatgpt::connectors::AppInfo {
        id: "unit_test_apps_refresh_failure_pending_connector".to_string(),
        name: "Notion".to_string(),
        description: Some("Workspace docs".to_string()),
        logo_url: None,
        logo_url_dark: None,
        distribution_channel: None,
        branding: None,
        app_metadata: None,
        labels: None,
        install_url: Some("https://example.test/notion".to_string()),
        is_accessible: true,
        is_enabled: true,
    }];
    chat.connectors_cache = ConnectorsCacheState::Ready(ConnectorsSnapshot {
        connectors: full_connectors.clone(),
    });

    chat.on_connectors_loaded(Err("failed to load apps".to_string()), true);

    assert!(chat.connectors_prefetch_in_flight);
    assert!(!chat.connectors_force_refetch_pending);
    assert_matches!(
        &chat.connectors_cache,
        ConnectorsCacheState::Ready(snapshot) if snapshot.connectors == full_connectors
    );
}

#[tokio::test]
async fn apps_partial_refresh_uses_same_filtering_as_full_refresh() {
    let (mut chat, _rx, _op_rx) = make_chatwidget_manual(None).await;
    chat.config.features.enable(Feature::Apps);
    chat.bottom_pane.set_connectors_enabled(true);

    let full_connectors = vec![
        codex_chatgpt::connectors::AppInfo {
            id: "unit_test_connector_1".to_string(),
            name: "Notion".to_string(),
            description: Some("Workspace docs".to_string()),
            logo_url: None,
            logo_url_dark: None,
            distribution_channel: None,
            branding: None,
            app_metadata: None,
            labels: None,
            install_url: Some("https://example.test/notion".to_string()),
            is_accessible: true,
            is_enabled: true,
        },
        codex_chatgpt::connectors::AppInfo {
            id: "unit_test_connector_2".to_string(),
            name: "Linear".to_string(),
            description: Some("Project tracking".to_string()),
            logo_url: None,
            logo_url_dark: None,
            distribution_channel: None,
            branding: None,
            app_metadata: None,
            labels: None,
            install_url: Some("https://example.test/linear".to_string()),
            is_accessible: false,
            is_enabled: true,
        },
    ];
    chat.on_connectors_loaded(
        Ok(ConnectorsSnapshot {
            connectors: full_connectors.clone(),
        }),
        true,
    );
    chat.add_connectors_output();

    chat.on_connectors_loaded(
        Ok(ConnectorsSnapshot {
            connectors: vec![
                codex_chatgpt::connectors::AppInfo {
                    id: "unit_test_connector_1".to_string(),
                    name: "Notion".to_string(),
                    description: Some("Workspace docs".to_string()),
                    logo_url: None,
                    logo_url_dark: None,
                    distribution_channel: None,
                    branding: None,
                    app_metadata: None,
                    labels: None,
                    install_url: Some("https://example.test/notion".to_string()),
                    is_accessible: true,
                    is_enabled: true,
                },
                codex_chatgpt::connectors::AppInfo {
                    id: "connector_openai_hidden".to_string(),
                    name: "Hidden OpenAI".to_string(),
                    description: Some("Should be filtered".to_string()),
                    logo_url: None,
                    logo_url_dark: None,
                    distribution_channel: None,
                    branding: None,
                    app_metadata: None,
                    labels: None,
                    install_url: Some("https://example.test/hidden-openai".to_string()),
                    is_accessible: true,
                    is_enabled: true,
                },
            ],
        }),
        false,
    );

    assert_matches!(
        &chat.connectors_cache,
        ConnectorsCacheState::Ready(snapshot) if snapshot.connectors == full_connectors
    );

    let popup = render_bottom_popup(&chat, 80);
    assert!(
        popup.contains("Installed 1 of 1 available apps."),
        "expected partial refresh popup to use filtered connectors, got:\n{popup}"
    );
    assert!(
        !popup.contains("Hidden OpenAI"),
        "expected disallowed connector to be filtered from partial refresh popup, got:\n{popup}"
    );
}
