use super::common::new_status_output;
use super::common::rate_limit_snapshot_display;
use super::common::render_snapshot;
use super::common::reset_at_from;
use super::common::test_auth_manager;
use super::common::test_config;
use super::common::token_info_for;
use chrono::Duration as ChronoDuration;
use chrono::TimeZone;
use codex_protocol::protocol::CreditsSnapshot;
use codex_protocol::protocol::RateLimitSnapshot;
use codex_protocol::protocol::RateLimitWindow;
use codex_protocol::protocol::TokenUsage;
use insta::assert_snapshot;
use std::path::PathBuf;
use tempfile::TempDir;

fn assert_status_snapshot(name: &str, rendered: String) {
    insta::with_settings!({
        snapshot_path => "../snapshots",
        prepend_module_to_snapshot => false,
    }, {
        assert_snapshot!(name, rendered);
    });
}

#[tokio::test]
async fn status_snapshot_shows_missing_limits_message() {
    let temp_home = TempDir::new().expect("temp home");
    let mut config = test_config(&temp_home).await;
    config.model = Some("gpt-5.1-codex-max".to_string());
    config.cwd = PathBuf::from("/workspace/tests");

    let auth_manager = test_auth_manager(&config);
    let usage = TokenUsage {
        input_tokens: 500,
        cached_input_tokens: 0,
        output_tokens: 250,
        reasoning_output_tokens: 0,
        total_tokens: 750,
    };

    let now = chrono::Local
        .with_ymd_and_hms(2024, 2, 3, 4, 5, 6)
        .single()
        .expect("timestamp");

    let model_slug = codex_core::test_support::get_model_offline(config.model.as_deref());
    let token_info = token_info_for(&model_slug, &config, &usage);
    let composite = new_status_output(
        &config,
        &auth_manager,
        Some(&token_info),
        &usage,
        &None,
        None,
        None,
        None,
        None,
        now,
        &model_slug,
        None,
        None,
    );
    let sanitized = render_snapshot(&composite, 80);
    assert_status_snapshot(
        "codex_tui__status__tests__status_snapshot_shows_missing_limits_message",
        sanitized,
    );
}

#[tokio::test]
async fn status_snapshot_includes_credits_and_limits() {
    let temp_home = TempDir::new().expect("temp home");
    let mut config = test_config(&temp_home).await;
    config.model = Some("gpt-5.1-codex".to_string());
    config.cwd = PathBuf::from("/workspace/tests");

    let auth_manager = test_auth_manager(&config);
    let usage = TokenUsage {
        input_tokens: 1_500,
        cached_input_tokens: 100,
        output_tokens: 600,
        reasoning_output_tokens: 0,
        total_tokens: 2_200,
    };

    let captured_at = chrono::Local
        .with_ymd_and_hms(2024, 7, 8, 9, 10, 11)
        .single()
        .expect("timestamp");
    let snapshot = RateLimitSnapshot {
        limit_id: None,
        limit_name: None,
        primary: Some(RateLimitWindow {
            used_percent: 45.0,
            window_minutes: Some(300),
            resets_at: Some(reset_at_from(&captured_at, 900)),
        }),
        secondary: Some(RateLimitWindow {
            used_percent: 30.0,
            window_minutes: Some(10_080),
            resets_at: Some(reset_at_from(&captured_at, 2_700)),
        }),
        credits: Some(CreditsSnapshot {
            has_credits: true,
            unlimited: false,
            balance: Some("37.5".to_string()),
        }),
        plan_type: None,
    };
    let rate_display = rate_limit_snapshot_display(&snapshot, captured_at);

    let model_slug = codex_core::test_support::get_model_offline(config.model.as_deref());
    let token_info = token_info_for(&model_slug, &config, &usage);
    let composite = new_status_output(
        &config,
        &auth_manager,
        Some(&token_info),
        &usage,
        &None,
        None,
        None,
        Some(&rate_display),
        None,
        captured_at,
        &model_slug,
        None,
        None,
    );
    let sanitized = render_snapshot(&composite, 80);
    assert_status_snapshot(
        "codex_tui__status__tests__status_snapshot_includes_credits_and_limits",
        sanitized,
    );
}

#[tokio::test]
async fn status_snapshot_shows_empty_limits_message() {
    let temp_home = TempDir::new().expect("temp home");
    let mut config = test_config(&temp_home).await;
    config.model = Some("gpt-5.1-codex-max".to_string());
    config.cwd = PathBuf::from("/workspace/tests");

    let auth_manager = test_auth_manager(&config);
    let usage = TokenUsage {
        input_tokens: 500,
        cached_input_tokens: 0,
        output_tokens: 250,
        reasoning_output_tokens: 0,
        total_tokens: 750,
    };

    let snapshot = RateLimitSnapshot {
        limit_id: None,
        limit_name: None,
        primary: None,
        secondary: None,
        credits: None,
        plan_type: None,
    };
    let captured_at = chrono::Local
        .with_ymd_and_hms(2024, 6, 7, 8, 9, 10)
        .single()
        .expect("timestamp");
    let rate_display = rate_limit_snapshot_display(&snapshot, captured_at);

    let model_slug = codex_core::test_support::get_model_offline(config.model.as_deref());
    let token_info = token_info_for(&model_slug, &config, &usage);
    let composite = new_status_output(
        &config,
        &auth_manager,
        Some(&token_info),
        &usage,
        &None,
        None,
        None,
        Some(&rate_display),
        None,
        captured_at,
        &model_slug,
        None,
        None,
    );
    let sanitized = render_snapshot(&composite, 80);
    assert_status_snapshot(
        "codex_tui__status__tests__status_snapshot_shows_empty_limits_message",
        sanitized,
    );
}

#[tokio::test]
async fn status_snapshot_shows_stale_limits_message() {
    let temp_home = TempDir::new().expect("temp home");
    let mut config = test_config(&temp_home).await;
    config.model = Some("gpt-5.1-codex-max".to_string());
    config.cwd = PathBuf::from("/workspace/tests");

    let auth_manager = test_auth_manager(&config);
    let usage = TokenUsage {
        input_tokens: 1_200,
        cached_input_tokens: 200,
        output_tokens: 900,
        reasoning_output_tokens: 150,
        total_tokens: 2_250,
    };

    let captured_at = chrono::Local
        .with_ymd_and_hms(2024, 1, 2, 3, 4, 5)
        .single()
        .expect("timestamp");
    let snapshot = RateLimitSnapshot {
        limit_id: None,
        limit_name: None,
        primary: Some(RateLimitWindow {
            used_percent: 72.5,
            window_minutes: Some(300),
            resets_at: Some(reset_at_from(&captured_at, 600)),
        }),
        secondary: Some(RateLimitWindow {
            used_percent: 40.0,
            window_minutes: Some(10_080),
            resets_at: Some(reset_at_from(&captured_at, 1_800)),
        }),
        credits: None,
        plan_type: None,
    };
    let rate_display = rate_limit_snapshot_display(&snapshot, captured_at);
    let now = captured_at + ChronoDuration::minutes(20);

    let model_slug = codex_core::test_support::get_model_offline(config.model.as_deref());
    let token_info = token_info_for(&model_slug, &config, &usage);
    let composite = new_status_output(
        &config,
        &auth_manager,
        Some(&token_info),
        &usage,
        &None,
        None,
        None,
        Some(&rate_display),
        None,
        now,
        &model_slug,
        None,
        None,
    );
    let sanitized = render_snapshot(&composite, 80);
    assert_status_snapshot(
        "codex_tui__status__tests__status_snapshot_shows_stale_limits_message",
        sanitized,
    );
}

#[tokio::test]
async fn status_snapshot_cached_limits_hide_credits_without_flag() {
    let temp_home = TempDir::new().expect("temp home");
    let mut config = test_config(&temp_home).await;
    config.model = Some("gpt-5.1-codex".to_string());
    config.cwd = PathBuf::from("/workspace/tests");

    let auth_manager = test_auth_manager(&config);
    let usage = TokenUsage {
        input_tokens: 900,
        cached_input_tokens: 200,
        output_tokens: 350,
        reasoning_output_tokens: 0,
        total_tokens: 1_450,
    };

    let captured_at = chrono::Local
        .with_ymd_and_hms(2024, 9, 10, 11, 12, 13)
        .single()
        .expect("timestamp");
    let snapshot = RateLimitSnapshot {
        limit_id: None,
        limit_name: None,
        primary: Some(RateLimitWindow {
            used_percent: 60.0,
            window_minutes: Some(300),
            resets_at: Some(reset_at_from(&captured_at, 1_200)),
        }),
        secondary: Some(RateLimitWindow {
            used_percent: 35.0,
            window_minutes: Some(10_080),
            resets_at: Some(reset_at_from(&captured_at, 2_400)),
        }),
        credits: Some(CreditsSnapshot {
            has_credits: false,
            unlimited: false,
            balance: Some("80".to_string()),
        }),
        plan_type: None,
    };
    let rate_display = rate_limit_snapshot_display(&snapshot, captured_at);
    let now = captured_at + ChronoDuration::minutes(20);

    let model_slug = codex_core::test_support::get_model_offline(config.model.as_deref());
    let token_info = token_info_for(&model_slug, &config, &usage);
    let composite = new_status_output(
        &config,
        &auth_manager,
        Some(&token_info),
        &usage,
        &None,
        None,
        None,
        Some(&rate_display),
        None,
        now,
        &model_slug,
        None,
        None,
    );
    let sanitized = render_snapshot(&composite, 80);
    assert_status_snapshot(
        "codex_tui__status__tests__status_snapshot_cached_limits_hide_credits_without_flag",
        sanitized,
    );
}
