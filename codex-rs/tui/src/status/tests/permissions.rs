use super::common::new_status_output;
use super::common::render_lines;
use super::common::test_auth_manager;
use super::common::test_config;
use crate::history_cell::HistoryCell;
use chrono::TimeZone;
use codex_protocol::protocol::AskForApproval;
use codex_protocol::protocol::SandboxPolicy;
use codex_protocol::protocol::TokenUsage;
use pretty_assertions::assert_eq;
use std::path::PathBuf;
use tempfile::TempDir;

#[tokio::test]
async fn status_permissions_non_default_workspace_write_is_custom() {
    let temp_home = TempDir::new().expect("temp home");
    let mut config = test_config(&temp_home).await;
    config.model = Some("gpt-5.1-codex-max".to_string());
    config.model_provider_id = "openai".to_string();
    config
        .permissions
        .approval_policy
        .set(AskForApproval::OnRequest)
        .expect("set approval policy");
    config
        .permissions
        .sandbox_policy
        .set(SandboxPolicy::WorkspaceWrite {
            writable_roots: Vec::new(),
            read_only_access: Default::default(),
            network_access: true,
            exclude_tmpdir_env_var: false,
            exclude_slash_tmp: false,
        })
        .expect("set sandbox policy");
    config.cwd = PathBuf::from("/workspace/tests");

    let auth_manager = test_auth_manager(&config);
    let usage = TokenUsage::default();
    let captured_at = chrono::Local
        .with_ymd_and_hms(2024, 1, 2, 3, 4, 5)
        .single()
        .expect("timestamp");
    let model_slug = codex_core::test_support::get_model_offline(config.model.as_deref());

    let composite = new_status_output(
        &config,
        &auth_manager,
        None,
        &usage,
        &None,
        None,
        None,
        None,
        None,
        captured_at,
        &model_slug,
        None,
        None,
    );
    let rendered_lines = render_lines(&composite.display_lines(80));
    let permissions_line = rendered_lines
        .iter()
        .find(|line| line.contains("Permissions:"))
        .expect("permissions line");
    let permissions_text = permissions_line
        .split("Permissions:")
        .nth(1)
        .map(str::trim)
        .map(|text| text.trim_end_matches('│'))
        .map(str::trim);

    assert_eq!(
        permissions_text,
        Some("Custom (workspace-write with network access, on-request)")
    );
}
