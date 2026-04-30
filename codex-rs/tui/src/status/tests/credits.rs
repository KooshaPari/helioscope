use super::common::new_status_output;
use super::common::rate_limit_snapshot_display;
use super::common::render_lines;
use super::common::test_auth_manager;
use super::common::test_config;
use super::common::token_info_for;
use crate::history_cell::HistoryCell;
use chrono::TimeZone;
use codex_protocol::protocol::CreditsSnapshot;
use codex_protocol::protocol::RateLimitSnapshot;
use codex_protocol::protocol::TokenUsage;
use tempfile::TempDir;

#[tokio::test]
async fn status_snapshot_shows_unlimited_credits() {
    let temp_home = TempDir::new().expect("temp home");
    let config = test_config(&temp_home).await;
    let auth_manager = test_auth_manager(&config);
    let usage = TokenUsage::default();
    let captured_at = chrono::Local
        .with_ymd_and_hms(2024, 2, 3, 4, 5, 6)
        .single()
        .expect("timestamp");
    let snapshot = RateLimitSnapshot {
        limit_id: None,
        limit_name: None,
        primary: None,
        secondary: None,
        credits: Some(CreditsSnapshot {
            has_credits: true,
            unlimited: true,
            balance: None,
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
    let rendered = render_lines(&composite.display_lines(120));
    assert!(
        rendered
            .iter()
            .any(|line| line.contains("Credits:") && line.contains("Unlimited")),
        "expected Credits: Unlimited line, got {rendered:?}"
    );
}

#[tokio::test]
async fn status_snapshot_shows_positive_credits() {
    let temp_home = TempDir::new().expect("temp home");
    let config = test_config(&temp_home).await;
    let auth_manager = test_auth_manager(&config);
    let usage = TokenUsage::default();
    let captured_at = chrono::Local
        .with_ymd_and_hms(2024, 3, 4, 5, 6, 7)
        .single()
        .expect("timestamp");
    let snapshot = RateLimitSnapshot {
        limit_id: None,
        limit_name: None,
        primary: None,
        secondary: None,
        credits: Some(CreditsSnapshot {
            has_credits: true,
            unlimited: false,
            balance: Some("12.5".to_string()),
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
    let rendered = render_lines(&composite.display_lines(120));
    assert!(
        rendered
            .iter()
            .any(|line| line.contains("Credits:") && line.contains("13 credits")),
        "expected Credits line with rounded credits, got {rendered:?}"
    );
}

#[tokio::test]
async fn status_snapshot_hides_zero_credits() {
    let temp_home = TempDir::new().expect("temp home");
    let config = test_config(&temp_home).await;
    let auth_manager = test_auth_manager(&config);
    let usage = TokenUsage::default();
    let captured_at = chrono::Local
        .with_ymd_and_hms(2024, 4, 5, 6, 7, 8)
        .single()
        .expect("timestamp");
    let snapshot = RateLimitSnapshot {
        limit_id: None,
        limit_name: None,
        primary: None,
        secondary: None,
        credits: Some(CreditsSnapshot {
            has_credits: true,
            unlimited: false,
            balance: Some("0".to_string()),
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
    let rendered = render_lines(&composite.display_lines(120));
    assert!(
        rendered.iter().all(|line| !line.contains("Credits:")),
        "expected no Credits line, got {rendered:?}"
    );
}

#[tokio::test]
async fn status_snapshot_hides_when_has_no_credits_flag() {
    let temp_home = TempDir::new().expect("temp home");
    let config = test_config(&temp_home).await;
    let auth_manager = test_auth_manager(&config);
    let usage = TokenUsage::default();
    let captured_at = chrono::Local
        .with_ymd_and_hms(2024, 5, 6, 7, 8, 9)
        .single()
        .expect("timestamp");
    let snapshot = RateLimitSnapshot {
        limit_id: None,
        limit_name: None,
        primary: None,
        secondary: None,
        credits: Some(CreditsSnapshot {
            has_credits: false,
            unlimited: true,
            balance: None,
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
    let rendered = render_lines(&composite.display_lines(120));
    assert!(
        rendered.iter().all(|line| !line.contains("Credits:")),
        "expected no Credits line when has_credits is false, got {rendered:?}"
    );
}
