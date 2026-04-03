use super::*;

#[tokio::test]
async fn experimental_features_popup_snapshot() {
    let (mut chat, _rx, _op_rx) = make_chatwidget_manual(None).await;

    let features = vec![
        ExperimentalFeatureItem {
            feature: Feature::GhostCommit,
            name: "Ghost snapshots".to_string(),
            description: "Capture undo snapshots each turn.".to_string(),
            enabled: false,
        },
        ExperimentalFeatureItem {
            feature: Feature::ShellTool,
            name: "Shell tool".to_string(),
            description: "Allow the model to run shell commands.".to_string(),
            enabled: true,
        },
    ];
    let view = ExperimentalFeaturesView::new(features, chat.app_event_tx.clone());
    chat.bottom_pane.show_view(Box::new(view));

    let popup = render_bottom_popup(&chat, 80);
    assert_snapshot!("experimental_features_popup", popup);
}

#[tokio::test]
async fn experimental_features_toggle_saves_on_exit() {
    let (mut chat, mut rx, _op_rx) = make_chatwidget_manual(None).await;

    let expected_feature = Feature::GhostCommit;
    let view = ExperimentalFeaturesView::new(
        vec![ExperimentalFeatureItem {
            feature: expected_feature,
            name: "Ghost snapshots".to_string(),
            description: "Capture undo snapshots each turn.".to_string(),
            enabled: false,
        }],
        chat.app_event_tx.clone(),
    );
    chat.bottom_pane.show_view(Box::new(view));

    chat.handle_key_event(KeyEvent::new(KeyCode::Char(' '), KeyModifiers::NONE));

    assert!(
        rx.try_recv().is_err(),
        "expected no updates until saving the popup"
    );

    chat.handle_key_event(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));

    let mut updates = None;
    while let Ok(event) = rx.try_recv() {
        if let AppEvent::UpdateFeatureFlags {
            updates: event_updates,
        } = event
        {
            updates = Some(event_updates);
            break;
        }
    }

    let updates = updates.expect("expected UpdateFeatureFlags event");
    assert_eq!(updates, vec![(expected_feature, true)]);
}

#[tokio::test]
async fn experimental_popup_shows_js_repl_node_requirement() {
    let (mut chat, _rx, _op_rx) = make_chatwidget_manual(None).await;

    let js_repl_description = FEATURES
        .iter()
        .find(|spec| spec.id == Feature::JsRepl)
        .and_then(|spec| spec.stage.experimental_menu_description())
        .expect("expected js_repl experimental description");
    let node_requirement = js_repl_description
        .split(". ")
        .find(|sentence| sentence.starts_with("Requires Node >= v"))
        .map(|sentence| sentence.trim_end_matches(" installed."))
        .expect("expected js_repl description to mention the Node requirement");

    chat.open_experimental_popup();

    let popup = render_bottom_popup(&chat, 120);
    assert!(
        popup.contains(node_requirement),
        "expected js_repl feature description to mention the required Node version, got:\n{popup}"
    );
}

#[tokio::test]
async fn model_selection_popup_snapshot() {
    let (mut chat, _rx, _op_rx) = make_chatwidget_manual(Some("gpt-5-codex")).await;
    chat.thread_id = Some(ThreadId::new());
    chat.open_model_popup();

    let popup = render_bottom_popup(&chat, 80);
    assert_snapshot!("model_selection_popup", popup);
}

#[tokio::test]
async fn personality_selection_popup_snapshot() {
    let (mut chat, _rx, _op_rx) = make_chatwidget_manual(Some("gpt-5.2-codex")).await;
    chat.thread_id = Some(ThreadId::new());
    chat.open_personality_popup();

    let popup = render_bottom_popup(&chat, 80);
    assert_snapshot!("personality_selection_popup", popup);
}

#[cfg(all(not(target_os = "linux"), feature = "voice-input"))]
#[tokio::test]
async fn realtime_audio_selection_popup_snapshot() {
    let (mut chat, _rx, _op_rx) = make_chatwidget_manual(Some("gpt-5.2-codex")).await;
    chat.open_realtime_audio_popup();

    let popup = render_bottom_popup(&chat, 80);
    assert_snapshot!("realtime_audio_selection_popup", popup);
}

#[cfg(all(not(target_os = "linux"), feature = "voice-input"))]
#[tokio::test]
async fn realtime_audio_selection_popup_narrow_snapshot() {
    let (mut chat, _rx, _op_rx) = make_chatwidget_manual(Some("gpt-5.2-codex")).await;
    chat.open_realtime_audio_popup();

    let popup = render_bottom_popup(&chat, 56);
    assert_snapshot!("realtime_audio_selection_popup_narrow", popup);
}

#[cfg(all(not(target_os = "linux"), feature = "voice-input"))]
#[tokio::test]
async fn realtime_microphone_picker_popup_snapshot() {
    let (mut chat, _rx, _op_rx) = make_chatwidget_manual(Some("gpt-5.2-codex")).await;
    chat.config.realtime_audio.microphone = Some("Studio Mic".to_string());
    chat.open_realtime_audio_device_selection_with_names(
        RealtimeAudioDeviceKind::Microphone,
        vec!["Built-in Mic".to_string(), "USB Mic".to_string()],
    );

    let popup = render_bottom_popup(&chat, 80);
    assert_snapshot!("realtime_microphone_picker_popup", popup);
}

#[cfg(all(not(target_os = "linux"), feature = "voice-input"))]
#[tokio::test]
async fn realtime_audio_picker_emits_persist_event() {
    let (mut chat, mut rx, _op_rx) = make_chatwidget_manual(Some("gpt-5.2-codex")).await;
    chat.open_realtime_audio_device_selection_with_names(
        RealtimeAudioDeviceKind::Speaker,
        vec!["Desk Speakers".to_string(), "Headphones".to_string()],
    );

    chat.handle_key_event(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));
    chat.handle_key_event(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));
    chat.handle_key_event(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));

    assert_matches!(
        rx.try_recv(),
        Ok(AppEvent::PersistRealtimeAudioDeviceSelection {
            kind: RealtimeAudioDeviceKind::Speaker,
            name: Some(name),
        }) if name == "Headphones"
    );
}

#[tokio::test]
async fn model_picker_hides_show_in_picker_false_models_from_cache() {
    let (mut chat, _rx, _op_rx) = make_chatwidget_manual(Some("test-visible-model")).await;
    chat.thread_id = Some(ThreadId::new());
    let preset = |slug: &str, show_in_picker: bool| ModelPreset {
        id: slug.to_string(),
        model: slug.to_string(),
        display_name: slug.to_string(),
        description: format!("{slug} description"),
        default_reasoning_effort: ReasoningEffortConfig::Medium,
        supported_reasoning_efforts: vec![ReasoningEffortPreset {
            effort: ReasoningEffortConfig::Medium,
            description: "medium".to_string(),
        }],
        supports_personality: false,
        is_default: false,
        upgrade: None,
        show_in_picker,
        availability_nux: None,
        supported_in_api: true,
        input_modalities: default_input_modalities(),
    };

    chat.open_model_popup_with_presets(vec![
        preset("test-visible-model", true),
        preset("test-hidden-model", false),
    ]);
    let popup = render_bottom_popup(&chat, 80);
    assert_snapshot!("model_picker_filters_hidden_models", popup);
    assert!(
        popup.contains("test-visible-model"),
        "expected visible model to appear in picker:\n{popup}"
    );
    assert!(
        !popup.contains("test-hidden-model"),
        "expected hidden model to be excluded from picker:\n{popup}"
    );
}

#[tokio::test]
async fn server_overloaded_error_does_not_switch_models() {
    let (mut chat, mut rx, mut op_rx) = make_chatwidget_manual(Some("gpt-5.2-codex")).await;
    chat.set_model("gpt-5.2-codex");
    while rx.try_recv().is_ok() {}
    while op_rx.try_recv().is_ok() {}

    chat.handle_codex_event(Event {
        id: "err-1".to_string(),
        msg: EventMsg::Error(ErrorEvent {
            message: "server overloaded".to_string(),
            codex_error_info: Some(CodexErrorInfo::ServerOverloaded),
        }),
    });

    while let Ok(event) = rx.try_recv() {
        if let AppEvent::UpdateModel(model) = event {
            assert_eq!(
                model, "gpt-5.2-codex",
                "did not expect model switch on server-overloaded error"
            );
        }
    }

    while let Ok(event) = op_rx.try_recv() {
        if let Op::OverrideTurnContext { model, .. } = event {
            assert!(
                model.is_none(),
                "did not expect OverrideTurnContext model update on server-overloaded error"
            );
        }
    }
}
