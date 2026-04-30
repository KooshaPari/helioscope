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
