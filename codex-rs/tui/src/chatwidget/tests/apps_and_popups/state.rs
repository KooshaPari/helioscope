#[tokio::test]
async fn apps_popup_shows_disabled_status_for_installed_but_disabled_apps() {
    let (mut chat, _rx, _op_rx) = make_chatwidget_manual(None).await;
    chat.config.features.enable(Feature::Apps);
    chat.bottom_pane.set_connectors_enabled(true);

    chat.on_connectors_loaded(
        Ok(ConnectorsSnapshot {
            connectors: vec![codex_chatgpt::connectors::AppInfo {
                id: "connector_1".to_string(),
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
                is_enabled: false,
            }],
        }),
        true,
    );

    chat.add_connectors_output();
    let popup = render_bottom_popup(&chat, 80);
    assert!(
        popup.contains("Installed · Disabled. Press Enter to open the app page"),
        "expected selected app description to include disabled status, got:\n{popup}"
    );
    assert!(
        popup.contains("enable/disable this app."),
        "expected selected app description to mention enable/disable action, got:\n{popup}"
    );
}

#[tokio::test]
async fn apps_initial_load_applies_enabled_state_from_config() {
    let (mut chat, _rx, _op_rx) = make_chatwidget_manual(None).await;
    chat.config.features.enable(Feature::Apps);
    chat.bottom_pane.set_connectors_enabled(true);

    let temp = tempdir().expect("tempdir");
    let config_toml_path =
        AbsolutePathBuf::try_from(temp.path().join("config.toml")).expect("absolute config path");
    let user_config = toml::from_str::<TomlValue>(
        "[apps.connector_1]\nenabled = false\ndisabled_reason = \"user\"\n",
    )
    .expect("apps config");
    chat.config.config_layer_stack = chat
        .config
        .config_layer_stack
        .with_user_config(&config_toml_path, user_config);

    chat.on_connectors_loaded(
        Ok(ConnectorsSnapshot {
            connectors: vec![codex_chatgpt::connectors::AppInfo {
                id: "connector_1".to_string(),
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
        true,
    );

    assert_matches!(
        &chat.connectors_cache,
        ConnectorsCacheState::Ready(snapshot)
            if snapshot
                .connectors
                .iter()
                .find(|connector| connector.id == "connector_1")
                .is_some_and(|connector| !connector.is_enabled)
    );
}

#[tokio::test]
async fn apps_refresh_preserves_toggled_enabled_state() {
    let (mut chat, _rx, _op_rx) = make_chatwidget_manual(None).await;
    chat.config.features.enable(Feature::Apps);
    chat.bottom_pane.set_connectors_enabled(true);

    chat.on_connectors_loaded(
        Ok(ConnectorsSnapshot {
            connectors: vec![codex_chatgpt::connectors::AppInfo {
                id: "connector_1".to_string(),
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
        true,
    );
    chat.update_connector_enabled("connector_1", false);

    chat.on_connectors_loaded(
        Ok(ConnectorsSnapshot {
            connectors: vec![codex_chatgpt::connectors::AppInfo {
                id: "connector_1".to_string(),
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
        true,
    );

    assert_matches!(
        &chat.connectors_cache,
        ConnectorsCacheState::Ready(snapshot)
            if snapshot
                .connectors
                .iter()
                .find(|connector| connector.id == "connector_1")
                .is_some_and(|connector| !connector.is_enabled)
    );

    chat.add_connectors_output();
    let popup = render_bottom_popup(&chat, 80);
    assert!(
        popup.contains("Installed · Disabled. Press Enter to open the app page"),
        "expected disabled status to persist after reload, got:\n{popup}"
    );
}

#[tokio::test]
async fn apps_popup_for_not_installed_app_uses_install_only_selected_description() {
    let (mut chat, _rx, _op_rx) = make_chatwidget_manual(None).await;
    chat.config.features.enable(Feature::Apps);
    chat.bottom_pane.set_connectors_enabled(true);

    chat.on_connectors_loaded(
        Ok(ConnectorsSnapshot {
            connectors: vec![codex_chatgpt::connectors::AppInfo {
                id: "connector_2".to_string(),
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
            }],
        }),
        true,
    );

    chat.add_connectors_output();
    let popup = render_bottom_popup(&chat, 80);
    assert!(
        popup.contains("Can be installed. Press Enter to open the app page to install"),
        "expected selected app description to be install-only for not-installed apps, got:\n{popup}"
    );
    assert!(
        !popup.contains("enable/disable this app."),
        "did not expect enable/disable text for not-installed apps, got:\n{popup}"
    );
}
