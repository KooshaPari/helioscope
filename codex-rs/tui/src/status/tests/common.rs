pub(super) use super::super::new_status_output;
pub(super) use super::super::rate_limit_snapshot_display;

use crate::history_cell::HistoryCell;
use chrono::Duration as ChronoDuration;
use chrono::Utc;
use codex_core::AuthManager;
use codex_core::config::Config;
use codex_core::config::ConfigBuilder;
use codex_protocol::protocol::TokenUsage;
use codex_protocol::protocol::TokenUsageInfo;
use ratatui::prelude::Line;
use tempfile::TempDir;

pub(super) async fn test_config(temp_home: &TempDir) -> Config {
    ConfigBuilder::default()
        .codex_home(temp_home.path().to_path_buf())
        .build()
        .await
        .expect("load config")
}

pub(super) fn test_auth_manager(config: &Config) -> AuthManager {
    AuthManager::new(
        config.codex_home.clone(),
        false,
        config.cli_auth_credentials_store_mode,
    )
}

pub(super) fn token_info_for(
    model_slug: &str,
    config: &Config,
    usage: &TokenUsage,
) -> TokenUsageInfo {
    let context_window =
        codex_core::test_support::construct_model_info_offline(model_slug, config).context_window;
    TokenUsageInfo {
        total_token_usage: usage.clone(),
        last_token_usage: usage.clone(),
        model_context_window: context_window,
    }
}

pub(super) fn render_lines(lines: &[Line<'static>]) -> Vec<String> {
    lines
        .iter()
        .map(|line| {
            line.spans
                .iter()
                .map(|span| span.content.as_ref())
                .collect::<String>()
        })
        .collect()
}

fn sanitize_directory(lines: Vec<String>) -> Vec<String> {
    lines
        .into_iter()
        .map(|line| {
            if let (Some(dir_pos), Some(pipe_idx)) = (line.find("Directory: "), line.rfind('│')) {
                let prefix = &line[..dir_pos + "Directory: ".len()];
                let suffix = &line[pipe_idx..];
                let content_width = pipe_idx.saturating_sub(dir_pos + "Directory: ".len());
                let replacement = "[[workspace]]";
                let mut rebuilt = prefix.to_string();
                rebuilt.push_str(replacement);
                if content_width > replacement.len() {
                    rebuilt.push_str(&" ".repeat(content_width - replacement.len()));
                }
                rebuilt.push_str(suffix);
                rebuilt
            } else {
                line
            }
        })
        .collect()
}

pub(super) fn reset_at_from(captured_at: &chrono::DateTime<chrono::Local>, seconds: i64) -> i64 {
    (*captured_at + ChronoDuration::seconds(seconds))
        .with_timezone(&Utc)
        .timestamp()
}

pub(super) fn render_snapshot(cell: &impl HistoryCell, width: u16) -> String {
    let mut rendered_lines = render_lines(&cell.display_lines(width));
    if cfg!(windows) {
        for line in &mut rendered_lines {
            *line = line.replace('\\', "/");
        }
    }
    sanitize_directory(rendered_lines).join("\n")
}
