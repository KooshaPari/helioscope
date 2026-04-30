#[tokio::test]
async fn rate_limit_switch_prompt_skips_when_on_lower_cost_model() {
    let (mut chat, _, _) = make_chatwidget_manual(Some(NUDGE_MODEL_SLUG)).await;
    chat.auth_manager = codex_core::test_support::auth_manager_from_auth(
        CodexAuth::create_dummy_chatgpt_auth_for_testing(),
    );

    chat.on_rate_limit_snapshot(Some(snapshot(95.0)));

    assert!(matches!(
        chat.rate_limit_switch_prompt,
        RateLimitSwitchPromptState::Idle
    ));
}

#[tokio::test]
async fn rate_limit_switch_prompt_skips_non_codex_limit() {
    let auth = CodexAuth::create_dummy_chatgpt_auth_for_testing();
    let (mut chat, _, _) = make_chatwidget_manual(Some("gpt-5")).await;
    chat.auth_manager = codex_core::test_support::auth_manager_from_auth(auth);

    chat.on_rate_limit_snapshot(Some(RateLimitSnapshot {
        limit_id: Some("codex_other".to_string()),
        limit_name: Some("codex_other".to_string()),
        primary: Some(RateLimitWindow {
            used_percent: 95.0,
            window_minutes: Some(60),
            resets_at: None,
        }),
        secondary: None,
        credits: None,
        plan_type: None,
    }));

    assert!(matches!(
        chat.rate_limit_switch_prompt,
        RateLimitSwitchPromptState::Idle
    ));
}

#[tokio::test]
async fn rate_limit_switch_prompt_shows_once_per_session() {
    let auth = CodexAuth::create_dummy_chatgpt_auth_for_testing();
    let (mut chat, _, _) = make_chatwidget_manual(Some("gpt-5")).await;
    chat.auth_manager = codex_core::test_support::auth_manager_from_auth(auth);

    chat.on_rate_limit_snapshot(Some(snapshot(90.0)));
    assert!(
        chat.rate_limit_warnings.primary_index >= 1,
        "warnings not emitted"
    );
    chat.maybe_show_pending_rate_limit_prompt();
    assert!(matches!(
        chat.rate_limit_switch_prompt,
        RateLimitSwitchPromptState::Shown
    ));

    chat.on_rate_limit_snapshot(Some(snapshot(95.0)));
    assert!(matches!(
        chat.rate_limit_switch_prompt,
        RateLimitSwitchPromptState::Shown
    ));
}

#[tokio::test]
async fn rate_limit_switch_prompt_respects_hidden_notice() {
    let auth = CodexAuth::create_dummy_chatgpt_auth_for_testing();
    let (mut chat, _, _) = make_chatwidget_manual(Some("gpt-5")).await;
    chat.auth_manager = codex_core::test_support::auth_manager_from_auth(auth);
    chat.config.notices.hide_rate_limit_model_nudge = Some(true);

    chat.on_rate_limit_snapshot(Some(snapshot(95.0)));

    assert!(matches!(
        chat.rate_limit_switch_prompt,
        RateLimitSwitchPromptState::Idle
    ));
}

#[tokio::test]
async fn rate_limit_switch_prompt_defers_until_task_complete() {
    let auth = CodexAuth::create_dummy_chatgpt_auth_for_testing();
    let (mut chat, _, _) = make_chatwidget_manual(Some("gpt-5")).await;
    chat.auth_manager = codex_core::test_support::auth_manager_from_auth(auth);

    chat.bottom_pane.set_task_running(true);
    chat.on_rate_limit_snapshot(Some(snapshot(90.0)));
    assert!(matches!(
        chat.rate_limit_switch_prompt,
        RateLimitSwitchPromptState::Pending
    ));

    chat.bottom_pane.set_task_running(false);
    chat.maybe_show_pending_rate_limit_prompt();
    assert!(matches!(
        chat.rate_limit_switch_prompt,
        RateLimitSwitchPromptState::Shown
    ));
}
