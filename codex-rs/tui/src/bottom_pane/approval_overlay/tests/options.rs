use super::*;
use crate::bottom_pane::approval_overlay::options::exec_options;
use codex_protocol::models::FileSystemPermissions;
use codex_protocol::models::PermissionProfile;
use codex_protocol::protocol::NetworkApprovalContext;
use codex_protocol::protocol::NetworkApprovalProtocol;
use codex_protocol::protocol::NetworkPolicyAmendment;
use codex_protocol::protocol::NetworkPolicyRuleAction;
use pretty_assertions::assert_eq;

#[test]
fn network_exec_options_use_expected_labels_and_hide_execpolicy_amendment() {
    let network_context = NetworkApprovalContext {
        host: "example.com".to_string(),
        protocol: NetworkApprovalProtocol::Https,
    };
    let options = exec_options(
        &[
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
        Some(&network_context),
        None,
    );

    let labels: Vec<String> = options.into_iter().map(|option| option.label).collect();
    assert_eq!(
        labels,
        vec![
            "Yes, just this once".to_string(),
            "Yes, and allow this host for this conversation".to_string(),
            "Yes, and allow this host in the future".to_string(),
            "No, and tell Codex what to do differently".to_string(),
        ]
    );
}

#[test]
fn generic_exec_options_can_offer_allow_for_session() {
    let options = exec_options(
        &[
            ReviewDecision::Approved,
            ReviewDecision::ApprovedForSession,
            ReviewDecision::Abort,
        ],
        None,
        None,
    );

    let labels: Vec<String> = options.into_iter().map(|option| option.label).collect();
    assert_eq!(
        labels,
        vec![
            "Yes, proceed".to_string(),
            "Yes, and don't ask again for this command in this session".to_string(),
            "No, and tell Codex what to do differently".to_string(),
        ]
    );
}

#[test]
fn additional_permissions_exec_options_hide_execpolicy_amendment() {
    let additional_permissions = PermissionProfile {
        file_system: Some(FileSystemPermissions {
            read: Some(vec![absolute_path("/tmp/readme.txt")]),
            write: Some(vec![absolute_path("/tmp/out.txt")]),
        }),
        ..Default::default()
    };
    let options = exec_options(
        &[ReviewDecision::Approved, ReviewDecision::Abort],
        None,
        Some(&additional_permissions),
    );

    let labels: Vec<String> = options.into_iter().map(|option| option.label).collect();
    assert_eq!(
        labels,
        vec![
            "Yes, proceed".to_string(),
            "No, and tell Codex what to do differently".to_string(),
        ]
    );
}
