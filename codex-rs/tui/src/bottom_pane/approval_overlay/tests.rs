mod actions;
mod options;

use crate::app_event::AppEvent;
use codex_protocol::ThreadId;
use codex_protocol::models::FileSystemPermissions;
use codex_protocol::models::PermissionProfile;
use codex_protocol::protocol::NetworkApprovalContext;
use codex_protocol::protocol::NetworkApprovalProtocol;
use codex_protocol::protocol::NetworkPolicyAmendment;
use codex_protocol::protocol::NetworkPolicyRuleAction;
use codex_utils_absolute_path::AbsolutePathBuf;
use insta::assert_snapshot;
use pretty_assertions::assert_eq;
use tokio::sync::mpsc::unbounded_channel;

pub(super) use super::*;

fn absolute_path(path: &str) -> AbsolutePathBuf {
    AbsolutePathBuf::from_absolute_path(path).expect("absolute path")
}

fn render_overlay_lines(view: &ApprovalOverlay, width: u16) -> String {
    let height = view.desired_height(width);
    let mut buf = Buffer::empty(Rect::new(0, 0, width, height));
    view.render(Rect::new(0, 0, width, height), &mut buf);
    (0..buf.area.height)
        .map(|row| {
            (0..buf.area.width)
                .map(|col| buf[(col, row)].symbol().to_string())
                .collect::<String>()
                .trim_end()
                .to_string()
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn normalize_snapshot_paths(rendered: String) -> String {
    [
        (absolute_path("/tmp/readme.txt"), "/tmp/readme.txt"),
        (absolute_path("/tmp/out.txt"), "/tmp/out.txt"),
    ]
    .into_iter()
    .fold(rendered, |rendered, (path, normalized)| {
        rendered.replace(&path.display().to_string(), normalized)
    })
}

fn make_exec_request() -> ApprovalRequest {
    ApprovalRequest::Exec {
        thread_id: ThreadId::new(),
        thread_label: None,
        id: "test".to_string(),
        command: vec!["echo".to_string(), "hi".to_string()],
        reason: Some("reason".to_string()),
        available_decisions: vec![ReviewDecision::Approved, ReviewDecision::Abort],
        network_approval_context: None,
        additional_permissions: None,
    }
}

#[test]
fn cross_thread_footer_hint_mentions_o_shortcut() {
    let (tx, _rx) = unbounded_channel::<AppEvent>();
    let tx = AppEventSender::new(tx);
    let view = ApprovalOverlay::new(
        ApprovalRequest::Exec {
            thread_id: ThreadId::new(),
            thread_label: Some("Robie [explorer]".to_string()),
            id: "test".to_string(),
            command: vec!["echo".to_string(), "hi".to_string()],
            reason: None,
            available_decisions: vec![ReviewDecision::Approved, ReviewDecision::Abort],
            network_approval_context: None,
            additional_permissions: None,
        },
        tx,
        Features::with_defaults(),
    );

    insta::with_settings!({ snapshot_path => "../snapshots" }, {
        assert_snapshot!(
            "approval_overlay_cross_thread_prompt",
            render_overlay_lines(&view, 80)
        );
    });
}

#[test]
fn header_includes_command_snippet() {
    let (tx, _rx) = unbounded_channel::<AppEvent>();
    let tx = AppEventSender::new(tx);
    let command = vec!["echo".into(), "hello".into(), "world".into()];
    let exec_request = ApprovalRequest::Exec {
        thread_id: ThreadId::new(),
        thread_label: None,
        id: "test".into(),
        command,
        reason: None,
        available_decisions: vec![ReviewDecision::Approved, ReviewDecision::Abort],
        network_approval_context: None,
        additional_permissions: None,
    };

    let view = ApprovalOverlay::new(exec_request, tx, Features::with_defaults());
    let mut buf = Buffer::empty(Rect::new(0, 0, 80, view.desired_height(80)));
    view.render(Rect::new(0, 0, 80, view.desired_height(80)), &mut buf);

    let rendered: Vec<String> = (0..buf.area.height)
        .map(|row| {
            (0..buf.area.width)
                .map(|col| buf[(col, row)].symbol().to_string())
                .collect()
        })
        .collect();
    assert!(
        rendered
            .iter()
            .any(|line| line.contains("echo hello world")),
        "expected header to include command snippet, got {rendered:?}"
    );
}

#[test]
fn additional_permissions_prompt_shows_permission_rule_line() {
    let (tx, _rx) = unbounded_channel::<AppEvent>();
    let tx = AppEventSender::new(tx);
    let exec_request = ApprovalRequest::Exec {
        thread_id: ThreadId::new(),
        thread_label: None,
        id: "test".into(),
        command: vec!["cat".into(), "/tmp/readme.txt".into()],
        reason: None,
        available_decisions: vec![ReviewDecision::Approved, ReviewDecision::Abort],
        network_approval_context: None,
        additional_permissions: Some(PermissionProfile {
            file_system: Some(FileSystemPermissions {
                read: Some(vec![absolute_path("/tmp/readme.txt")]),
                write: Some(vec![absolute_path("/tmp/out.txt")]),
            }),
            ..Default::default()
        }),
    };

    let view = ApprovalOverlay::new(exec_request, tx, Features::with_defaults());
    let mut buf = Buffer::empty(Rect::new(0, 0, 120, view.desired_height(120)));
    view.render(Rect::new(0, 0, 120, view.desired_height(120)), &mut buf);

    let rendered: Vec<String> = (0..buf.area.height)
        .map(|row| {
            (0..buf.area.width)
                .map(|col| buf[(col, row)].symbol().to_string())
                .collect()
        })
        .collect();

    assert!(
        rendered
            .iter()
            .any(|line| line.contains("Permission rule:")),
        "expected permission-rule line, got {rendered:?}"
    );
}

#[test]
fn additional_permissions_prompt_snapshot() {
    let (tx, _rx) = unbounded_channel::<AppEvent>();
    let tx = AppEventSender::new(tx);
    let exec_request = ApprovalRequest::Exec {
        thread_id: ThreadId::new(),
        thread_label: None,
        id: "test".into(),
        command: vec!["cat".into(), "/tmp/readme.txt".into()],
        reason: Some("need filesystem access".into()),
        available_decisions: vec![ReviewDecision::Approved, ReviewDecision::Abort],
        network_approval_context: None,
        additional_permissions: Some(PermissionProfile {
            file_system: Some(FileSystemPermissions {
                read: Some(vec![absolute_path("/tmp/readme.txt")]),
                write: Some(vec![absolute_path("/tmp/out.txt")]),
            }),
            ..Default::default()
        }),
    };

    let view = ApprovalOverlay::new(exec_request, tx, Features::with_defaults());
    insta::with_settings!({ snapshot_path => "../snapshots" }, {
        assert_snapshot!(
            "approval_overlay_additional_permissions_prompt",
            normalize_snapshot_paths(render_overlay_lines(&view, 120))
        );
    });
}

#[test]
fn network_exec_prompt_title_includes_host() {
    let (tx, _rx) = unbounded_channel::<AppEvent>();
    let tx = AppEventSender::new(tx);
    let exec_request = ApprovalRequest::Exec {
        thread_id: ThreadId::new(),
        thread_label: None,
        id: "test".into(),
        command: vec!["curl".into(), "https://example.com".into()],
        reason: Some("network request blocked".into()),
        available_decisions: vec![
            ReviewDecision::Approved,
            ReviewDecision::ApprovedForSession,
            ReviewDecision::NetworkPolicyAmendment {
                network_policy_amendment: NetworkPolicyAmendment {
                    host: "example.com".to_string(),
                    action: NetworkPolicyRuleAction::Allow,
                },
            },
            ReviewDecision::Abort,
        ],
        network_approval_context: Some(NetworkApprovalContext {
            host: "example.com".to_string(),
            protocol: NetworkApprovalProtocol::Https,
        }),
        additional_permissions: None,
    };

    let view = ApprovalOverlay::new(exec_request, tx, Features::with_defaults());
    let mut buf = Buffer::empty(Rect::new(0, 0, 100, view.desired_height(100)));
    view.render(Rect::new(0, 0, 100, view.desired_height(100)), &mut buf);
    insta::with_settings!({ snapshot_path => "../snapshots" }, {
        assert_snapshot!("network_exec_prompt", format!("{buf:?}"));
    });

    let rendered: Vec<String> = (0..buf.area.height)
        .map(|row| {
            (0..buf.area.width)
                .map(|col| buf[(col, row)].symbol().to_string())
                .collect()
        })
        .collect();

    assert!(
        rendered.iter().any(|line| {
            line.contains("Do you want to approve network access to \"example.com\"?")
        }),
        "expected network title to include host, got {rendered:?}"
    );
    assert!(
        !rendered.iter().any(|line| line.contains("$ curl")),
        "network prompt should not show command line, got {rendered:?}"
    );
    assert!(
        !rendered.iter().any(|line| line.contains("don't ask again")),
        "network prompt should not show execpolicy option, got {rendered:?}"
    );
}

#[test]
fn exec_history_cell_wraps_with_two_space_indent() {
    let command = vec![
        "/bin/zsh".into(),
        "-lc".into(),
        "git add tui/src/render/mod.rs tui/src/render/renderable.rs".into(),
    ];
    let cell = history_cell::new_approval_decision_cell(command, ReviewDecision::Approved);
    let lines = cell.display_lines(28);
    let rendered: Vec<String> = lines
        .iter()
        .map(|line| {
            line.spans
                .iter()
                .map(|span| span.content.as_ref())
                .collect::<String>()
        })
        .collect();
    let expected = vec![
        "✔ You approved codex to run".to_string(),
        "  git add tui/src/render/".to_string(),
        "  mod.rs tui/src/render/".to_string(),
        "  renderable.rs this time".to_string(),
    ];
    assert_eq!(rendered, expected);
}
