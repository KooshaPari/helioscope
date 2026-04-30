macro_rules! assert_chatwidget_snapshot_permissions {
    ($($tt:tt)*) => {
        insta::with_settings!({ snapshot_path => "../snapshots" }, {
            assert_snapshot!($($tt)*);
        });
    };
}

#[tokio::test]
async fn approvals_selection_popup_snapshot() {
    let (mut chat, _rx, _op_rx) = make_chatwidget_manual(None).await;

    chat.config.notices.hide_full_access_warning = None;
    chat.open_approvals_popup();

    let popup = render_bottom_popup(&chat, 80);
    #[cfg(target_os = "windows")]
    insta::with_settings!({ snapshot_suffix => "windows" }, {
        assert_chatwidget_snapshot_permissions!("approvals_selection_popup", popup);
    });
    #[cfg(not(target_os = "windows"))]
    assert_chatwidget_snapshot_permissions!("approvals_selection_popup", popup);
}

#[cfg(target_os = "windows")]
#[tokio::test]
#[serial]
async fn approvals_selection_popup_snapshot_windows_degraded_sandbox() {
    let (mut chat, _rx, _op_rx) = make_chatwidget_manual(None).await;

    chat.config.notices.hide_full_access_warning = None;
    chat.set_feature_enabled(Feature::WindowsSandbox, true);
    chat.set_feature_enabled(Feature::WindowsSandboxElevated, false);

    chat.open_approvals_popup();

    let popup = render_bottom_popup(&chat, 80);
    assert!(
        popup.contains("Default (non-admin sandbox)"),
        "expected degraded sandbox label in approvals popup: {popup}"
    );
    assert!(
        popup.contains("/setup-default-sandbox"),
        "expected setup hint in approvals popup: {popup}"
    );
    assert!(
        popup.contains("non-admin sandbox"),
        "expected degraded sandbox note in approvals popup: {popup}"
    );
}

#[tokio::test]
async fn preset_matching_requires_exact_workspace_write_settings() {
    let preset = builtin_approval_presets()
        .into_iter()
        .find(|p| p.id == "auto")
        .expect("auto preset exists");
    let current_sandbox = SandboxPolicy::WorkspaceWrite {
        writable_roots: vec![AbsolutePathBuf::try_from("C:\\extra").unwrap()],
        read_only_access: Default::default(),
        network_access: false,
        exclude_tmpdir_env_var: false,
        exclude_slash_tmp: false,
    };

    assert!(
        !ChatWidget::preset_matches_current(AskForApproval::OnRequest, &current_sandbox, &preset),
        "WorkspaceWrite with extra roots should not match the Default preset"
    );
    assert!(
        !ChatWidget::preset_matches_current(AskForApproval::Never, &current_sandbox, &preset),
        "approval mismatch should prevent matching the preset"
    );
}

#[tokio::test]
async fn full_access_confirmation_popup_snapshot() {
    let (mut chat, _rx, _op_rx) = make_chatwidget_manual(None).await;

    let preset = builtin_approval_presets()
        .into_iter()
        .find(|preset| preset.id == "full-access")
        .expect("full access preset");
    chat.open_full_access_confirmation(preset, false);

    let popup = render_bottom_popup(&chat, 80);
    assert_chatwidget_snapshot_permissions!("full_access_confirmation_popup", popup);
}

#[cfg(target_os = "windows")]
#[tokio::test]
async fn windows_auto_mode_prompt_requests_enabling_sandbox_feature() {
    let (mut chat, _rx, _op_rx) = make_chatwidget_manual(None).await;

    let preset = builtin_approval_presets()
        .into_iter()
        .find(|preset| preset.id == "auto")
        .expect("auto preset");
    chat.open_windows_sandbox_enable_prompt(preset);

    let popup = render_bottom_popup(&chat, 120);
    assert!(
        popup.contains("requires Administrator permissions"),
        "expected auto mode prompt to mention Administrator permissions, popup: {popup}"
    );
    assert!(
        popup.contains("Use non-admin sandbox"),
        "expected auto mode prompt to include non-admin fallback option, popup: {popup}"
    );
}

#[cfg(target_os = "windows")]
#[tokio::test]
async fn startup_prompts_for_windows_sandbox_when_agent_requested() {
    let (mut chat, _rx, _op_rx) = make_chatwidget_manual(None).await;

    chat.set_feature_enabled(Feature::WindowsSandbox, false);
    chat.set_feature_enabled(Feature::WindowsSandboxElevated, false);

    chat.maybe_prompt_windows_sandbox_enable(true);

    let popup = render_bottom_popup(&chat, 120);
    assert!(
        popup.contains("requires Administrator permissions"),
        "expected startup prompt to mention Administrator permissions: {popup}"
    );
    assert!(
        popup.contains("Set up default sandbox"),
        "expected startup prompt to offer default sandbox setup: {popup}"
    );
    assert!(
        popup.contains("Use non-admin sandbox"),
        "expected startup prompt to offer non-admin fallback: {popup}"
    );
    assert!(
        popup.contains("Quit"),
        "expected startup prompt to offer quit action: {popup}"
    );
}

#[cfg(target_os = "windows")]
#[tokio::test]
async fn startup_does_not_prompt_for_windows_sandbox_when_not_requested() {
    let (mut chat, _rx, _op_rx) = make_chatwidget_manual(None).await;

    chat.set_feature_enabled(Feature::WindowsSandbox, false);
    chat.set_feature_enabled(Feature::WindowsSandboxElevated, false);
    chat.maybe_prompt_windows_sandbox_enable(false);

    assert!(
        chat.bottom_pane.no_modal_or_popup_active(),
        "expected no startup sandbox NUX popup when startup trigger is false"
    );
}

#[tokio::test]
async fn model_reasoning_selection_popup_snapshot() {
    let (mut chat, _rx, _op_rx) = make_chatwidget_manual(Some("gpt-5.1-codex-max")).await;

    set_chatgpt_auth(&mut chat);
    chat.set_reasoning_effort(Some(ReasoningEffortConfig::High));

    let preset = get_available_model(&chat, "gpt-5.1-codex-max");
    chat.open_reasoning_popup(preset);

    let popup = render_bottom_popup(&chat, 80);
    assert_chatwidget_snapshot_permissions!("model_reasoning_selection_popup", popup);
}

#[tokio::test]
async fn model_reasoning_selection_popup_extra_high_warning_snapshot() {
    let (mut chat, _rx, _op_rx) = make_chatwidget_manual(Some("gpt-5.1-codex-max")).await;

    set_chatgpt_auth(&mut chat);
    chat.set_reasoning_effort(Some(ReasoningEffortConfig::XHigh));

    let preset = get_available_model(&chat, "gpt-5.1-codex-max");
    chat.open_reasoning_popup(preset);

    let popup = render_bottom_popup(&chat, 80);
    assert_chatwidget_snapshot_permissions!(
        "model_reasoning_selection_popup_extra_high_warning",
        popup
    );
}

#[tokio::test]
async fn reasoning_popup_shows_extra_high_with_space() {
    let (mut chat, _rx, _op_rx) = make_chatwidget_manual(Some("gpt-5.1-codex-max")).await;

    set_chatgpt_auth(&mut chat);

    let preset = get_available_model(&chat, "gpt-5.1-codex-max");
    chat.open_reasoning_popup(preset);

    let popup = render_bottom_popup(&chat, 120);
    assert!(
        popup.contains("Extra high"),
        "expected popup to include 'Extra high'; popup: {popup}"
    );
    assert!(
        !popup.contains("Extrahigh"),
        "expected popup not to include 'Extrahigh'; popup: {popup}"
    );
}

#[tokio::test]
async fn single_reasoning_option_skips_selection() {
    let (mut chat, mut rx, _op_rx) = make_chatwidget_manual(None).await;

    let single_effort = vec![ReasoningEffortPreset {
        effort: ReasoningEffortConfig::High,
        description: "Greater reasoning depth for complex or ambiguous problems".to_string(),
    }];
    let preset = ModelPreset {
        id: "model-with-single-reasoning".to_string(),
        model: "model-with-single-reasoning".to_string(),
        display_name: "model-with-single-reasoning".to_string(),
        description: "".to_string(),
        default_reasoning_effort: ReasoningEffortConfig::High,
        supported_reasoning_efforts: single_effort,
        supports_personality: false,
        is_default: false,
        upgrade: None,
        show_in_picker: true,
        availability_nux: None,
        supported_in_api: true,
        input_modalities: default_input_modalities(),
    };
    chat.open_reasoning_popup(preset);

    let popup = render_bottom_popup(&chat, 80);
    assert!(
        !popup.contains("Select Reasoning Level"),
        "expected reasoning selection popup to be skipped"
    );

    let mut events = Vec::new();
    while let Ok(ev) = rx.try_recv() {
        events.push(ev);
    }

    assert!(
        events
            .iter()
            .any(|ev| matches!(ev, AppEvent::UpdateReasoningEffort(Some(effort)) if *effort == ReasoningEffortConfig::High)),
        "expected reasoning effort to be applied automatically; events: {events:?}"
    );
}

#[tokio::test]
async fn feedback_selection_popup_snapshot() {
    let (mut chat, _rx, _op_rx) = make_chatwidget_manual(None).await;

    // Open the feedback category selection popup via slash command.
    chat.dispatch_command(SlashCommand::Feedback);

    let popup = render_bottom_popup(&chat, 80);
    assert_chatwidget_snapshot_permissions!("feedback_selection_popup", popup);
}

#[tokio::test]
async fn feedback_upload_consent_popup_snapshot() {
    let (mut chat, _rx, _op_rx) = make_chatwidget_manual(None).await;

    // Open the consent popup directly for a chosen category.
    chat.open_feedback_consent(crate::app_event::FeedbackCategory::Bug);

    let popup = render_bottom_popup(&chat, 80);
    assert_chatwidget_snapshot_permissions!("feedback_upload_consent_popup", popup);
}

#[tokio::test]
async fn reasoning_popup_escape_returns_to_model_popup() {
    let (mut chat, _rx, _op_rx) = make_chatwidget_manual(Some("gpt-5.1-codex-max")).await;
    chat.thread_id = Some(ThreadId::new());
    chat.open_model_popup();

    let preset = get_available_model(&chat, "gpt-5.1-codex-max");
    chat.open_reasoning_popup(preset);

    let before_escape = render_bottom_popup(&chat, 80);
    assert!(before_escape.contains("Select Reasoning Level"));

    chat.handle_key_event(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE));

    let after_escape = render_bottom_popup(&chat, 80);
    assert!(after_escape.contains("Select Model"));
    assert!(!after_escape.contains("Select Reasoning Level"));
}
