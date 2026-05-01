use super::support::*;
use super::*;
use pretty_assertions::assert_eq;

#[tokio::test]
async fn prefetch_rate_limits_is_gated_on_chatgpt_auth_provider() {
    let (mut chat, _rx, _op_rx) = make_chatwidget_manual(None).await;

    assert!(!chat.should_prefetch_rate_limits());

    set_chatgpt_auth(&mut chat);
    assert!(chat.should_prefetch_rate_limits());

    chat.config.model_provider.requires_openai_auth = false;
    assert!(!chat.should_prefetch_rate_limits());

    chat.prefetch_rate_limits();
    assert!(chat.rate_limit_poller.is_none());
}

#[tokio::test]
async fn worked_elapsed_from_resets_when_timer_restarts() {
    let (mut chat, _rx, _op_rx) = make_chatwidget_manual(None).await;
    assert_eq!(chat.worked_elapsed_from(5), 5);
    assert_eq!(chat.worked_elapsed_from(9), 4);
    // Simulate status timer resetting (e.g., status indicator recreated for a new task).
    assert_eq!(chat.worked_elapsed_from(3), 3);
    assert_eq!(chat.worked_elapsed_from(7), 4);
}

#[tokio::test]
async fn rate_limit_warnings_emit_thresholds() {
    let mut state = RateLimitWarningState::default();
    let mut warnings: Vec<String> = Vec::new();

    warnings.extend(state.take_warnings(Some(10.0), Some(10079), Some(55.0), Some(299)));
    warnings.extend(state.take_warnings(Some(55.0), Some(10081), Some(10.0), Some(299)));
    warnings.extend(state.take_warnings(Some(10.0), Some(10081), Some(80.0), Some(299)));
    warnings.extend(state.take_warnings(Some(80.0), Some(10081), Some(10.0), Some(299)));
    warnings.extend(state.take_warnings(Some(10.0), Some(10081), Some(95.0), Some(299)));
    warnings.extend(state.take_warnings(Some(95.0), Some(10079), Some(10.0), Some(299)));

    assert_eq!(
        warnings,
        vec![
            String::from(
                "Heads up, you have less than 25% of your 5h limit left. Run /status for a breakdown."
            ),
            String::from(
                "Heads up, you have less than 25% of your weekly limit left. Run /status for a breakdown.",
            ),
            String::from(
                "Heads up, you have less than 5% of your 5h limit left. Run /status for a breakdown."
            ),
            String::from(
                "Heads up, you have less than 5% of your weekly limit left. Run /status for a breakdown.",
            ),
        ],
        "expected one warning per limit for the highest crossed threshold"
    );
}

#[tokio::test]
async fn test_rate_limit_warnings_monthly() {
    let mut state = RateLimitWarningState::default();
    let mut warnings: Vec<String> = Vec::new();

    warnings.extend(state.take_warnings(Some(75.0), Some(43199), None, None));
    assert_eq!(
        warnings,
        vec![String::from(
            "Heads up, you have less than 25% of your monthly limit left. Run /status for a breakdown.",
        ),],
        "expected one warning per limit for the highest crossed threshold"
    );
}

#[tokio::test]
async fn rate_limit_snapshot_keeps_prior_credits_when_missing_from_headers() {
    let (mut chat, _rx, _op_rx) = make_chatwidget_manual(None).await;

    chat.on_rate_limit_snapshot(Some(RateLimitSnapshot {
        limit_id: None,
        limit_name: None,
        primary: None,
        secondary: None,
        credits: Some(CreditsSnapshot {
            has_credits: true,
            unlimited: false,
            balance: Some("17.5".to_string()),
        }),
        plan_type: None,
    }));
    let initial_balance = chat
        .rate_limit_snapshots_by_limit_id
        .get("codex")
        .and_then(|snapshot| snapshot.credits.as_ref())
        .and_then(|credits| credits.balance.as_deref());
    assert_eq!(initial_balance, Some("17.5"));

    chat.on_rate_limit_snapshot(Some(RateLimitSnapshot {
        limit_id: None,
        limit_name: None,
        primary: Some(RateLimitWindow {
            used_percent: 80.0,
            window_minutes: Some(60),
            resets_at: Some(123),
        }),
        secondary: None,
        credits: None,
        plan_type: None,
    }));

    let display = chat
        .rate_limit_snapshots_by_limit_id
        .get("codex")
        .expect("rate limits should be cached");
    let credits = display
        .credits
        .as_ref()
        .expect("credits should persist when headers omit them");

    assert_eq!(credits.balance.as_deref(), Some("17.5"));
    assert!(!credits.unlimited);
    assert_eq!(
        display.primary.as_ref().map(|window| window.used_percent),
        Some(80.0)
    );
}

#[tokio::test]
async fn rate_limit_snapshot_updates_and_retains_plan_type() {
    let (mut chat, _rx, _op_rx) = make_chatwidget_manual(None).await;

    chat.on_rate_limit_snapshot(Some(RateLimitSnapshot {
        limit_id: None,
        limit_name: None,
        primary: Some(RateLimitWindow {
            used_percent: 10.0,
            window_minutes: Some(60),
            resets_at: None,
        }),
        secondary: Some(RateLimitWindow {
            used_percent: 5.0,
            window_minutes: Some(300),
            resets_at: None,
        }),
        credits: None,
        plan_type: Some(PlanType::Plus),
    }));
    assert_eq!(chat.plan_type, Some(PlanType::Plus));

    chat.on_rate_limit_snapshot(Some(RateLimitSnapshot {
        limit_id: None,
        limit_name: None,
        primary: Some(RateLimitWindow {
            used_percent: 25.0,
            window_minutes: Some(30),
            resets_at: Some(123),
        }),
        secondary: Some(RateLimitWindow {
            used_percent: 15.0,
            window_minutes: Some(300),
            resets_at: Some(234),
        }),
        credits: None,
        plan_type: Some(PlanType::Pro),
    }));
    assert_eq!(chat.plan_type, Some(PlanType::Pro));

    chat.on_rate_limit_snapshot(Some(RateLimitSnapshot {
        limit_id: None,
        limit_name: None,
        primary: Some(RateLimitWindow {
            used_percent: 30.0,
            window_minutes: Some(60),
            resets_at: Some(456),
        }),
        secondary: Some(RateLimitWindow {
            used_percent: 18.0,
            window_minutes: Some(300),
            resets_at: Some(567),
        }),
        credits: None,
        plan_type: None,
    }));
    assert_eq!(chat.plan_type, Some(PlanType::Pro));
}

#[tokio::test]
async fn rate_limit_snapshots_keep_separate_entries_per_limit_id() {
    let (mut chat, _rx, _op_rx) = make_chatwidget_manual(None).await;

    chat.on_rate_limit_snapshot(Some(RateLimitSnapshot {
        limit_id: Some("codex".to_string()),
        limit_name: Some("codex".to_string()),
        primary: Some(RateLimitWindow {
            used_percent: 20.0,
            window_minutes: Some(300),
            resets_at: Some(100),
        }),
        secondary: None,
        credits: Some(CreditsSnapshot {
            has_credits: true,
            unlimited: false,
            balance: Some("5.00".to_string()),
        }),
        plan_type: Some(PlanType::Pro),
    }));

    chat.on_rate_limit_snapshot(Some(RateLimitSnapshot {
        limit_id: Some("codex_other".to_string()),
        limit_name: Some("codex_other".to_string()),
        primary: Some(RateLimitWindow {
            used_percent: 90.0,
            window_minutes: Some(60),
            resets_at: Some(200),
        }),
        secondary: None,
        credits: None,
        plan_type: Some(PlanType::Pro),
    }));

    let codex = chat
        .rate_limit_snapshots_by_limit_id
        .get("codex")
        .expect("codex snapshot should exist");
    let other = chat
        .rate_limit_snapshots_by_limit_id
        .get("codex_other")
        .expect("codex_other snapshot should exist");

    assert_eq!(codex.primary.as_ref().map(|w| w.used_percent), Some(20.0));
    assert_eq!(
        codex
            .credits
            .as_ref()
            .and_then(|credits| credits.balance.as_deref()),
        Some("5.00")
    );
    assert_eq!(other.primary.as_ref().map(|w| w.used_percent), Some(90.0));
    assert!(other.credits.is_none());
}

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
