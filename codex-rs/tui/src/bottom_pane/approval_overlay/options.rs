use crate::bottom_pane::approval_overlay::ApprovalRequest;
use crate::bottom_pane::approval_overlay::header::approval_footer_hint;
use crate::bottom_pane::list_selection_view::SelectionItem;
use crate::bottom_pane::list_selection_view::SelectionViewParams;
use crate::exec_command::strip_bash_lc_and_escape;
use crate::key_hint;
use crate::key_hint::KeyBinding;
use crate::render::renderable::ColumnRenderable;
use crate::render::renderable::Renderable;
use codex_core::features::Features;
use codex_protocol::models::PermissionProfile;
use codex_protocol::protocol::ElicitationAction;
use codex_protocol::protocol::NetworkApprovalContext;
use codex_protocol::protocol::NetworkPolicyRuleAction;
use codex_protocol::protocol::ReviewDecision;
use crossterm::event::KeyCode;
use ratatui::style::Stylize;
use ratatui::text::Line;

#[derive(Clone)]
pub(super) enum ApprovalDecision {
    Review(ReviewDecision),
    McpElicitation(ElicitationAction),
}

#[derive(Clone)]
pub(super) struct ApprovalOption {
    pub(super) label: String,
    pub(super) decision: ApprovalDecision,
    pub(super) display_shortcut: Option<KeyBinding>,
    pub(super) additional_shortcuts: Vec<KeyBinding>,
}

impl ApprovalOption {
    pub(super) fn shortcuts(&self) -> impl Iterator<Item = KeyBinding> + '_ {
        self.display_shortcut
            .into_iter()
            .chain(self.additional_shortcuts.iter().copied())
    }
}

pub(super) fn build_options(
    request: &ApprovalRequest,
    header: Box<dyn Renderable>,
    _features: &Features,
) -> (Vec<ApprovalOption>, SelectionViewParams) {
    let (options, title) = match request {
        ApprovalRequest::Exec {
            available_decisions,
            network_approval_context,
            additional_permissions,
            ..
        } => (
            exec_options(
                available_decisions,
                network_approval_context.as_ref(),
                additional_permissions.as_ref(),
            ),
            network_approval_context.as_ref().map_or_else(
                || "Would you like to run the following command?".to_string(),
                |network_approval_context| {
                    format!(
                        "Do you want to approve network access to \"{}\"?",
                        network_approval_context.host
                    )
                },
            ),
        ),
        ApprovalRequest::ApplyPatch { .. } => (
            patch_options(),
            "Would you like to make the following edits?".to_string(),
        ),
        ApprovalRequest::McpElicitation { server_name, .. } => (
            elicitation_options(),
            format!("{server_name} needs your approval."),
        ),
    };

    let header = Box::new(ColumnRenderable::with([
        Line::from(title.bold()).into(),
        Line::from("").into(),
        header,
    ]));

    let items = options
        .iter()
        .map(|opt| SelectionItem {
            name: opt.label.clone(),
            display_shortcut: opt
                .display_shortcut
                .or_else(|| opt.additional_shortcuts.first().copied()),
            dismiss_on_select: false,
            ..Default::default()
        })
        .collect();

    let params = SelectionViewParams {
        footer_hint: Some(approval_footer_hint(request)),
        items,
        header,
        ..Default::default()
    };

    (options, params)
}

pub(super) fn exec_options(
    available_decisions: &[ReviewDecision],
    network_approval_context: Option<&NetworkApprovalContext>,
    additional_permissions: Option<&PermissionProfile>,
) -> Vec<ApprovalOption> {
    available_decisions
        .iter()
        .filter_map(|decision| match decision {
            ReviewDecision::Approved => Some(ApprovalOption {
                label: if network_approval_context.is_some() {
                    "Yes, just this once".to_string()
                } else {
                    "Yes, proceed".to_string()
                },
                decision: ApprovalDecision::Review(ReviewDecision::Approved),
                display_shortcut: None,
                additional_shortcuts: vec![key_hint::plain(KeyCode::Char('y'))],
            }),
            ReviewDecision::ApprovedExecpolicyAmendment {
                proposed_execpolicy_amendment,
            } => {
                let rendered_prefix =
                    strip_bash_lc_and_escape(proposed_execpolicy_amendment.command());
                if rendered_prefix.contains('\n') || rendered_prefix.contains('\r') {
                    return None;
                }

                Some(ApprovalOption {
                    label: format!(
                        "Yes, and don't ask again for commands that start with `{rendered_prefix}`"
                    ),
                    decision: ApprovalDecision::Review(
                        ReviewDecision::ApprovedExecpolicyAmendment {
                            proposed_execpolicy_amendment: proposed_execpolicy_amendment.clone(),
                        },
                    ),
                    display_shortcut: None,
                    additional_shortcuts: vec![key_hint::plain(KeyCode::Char('p'))],
                })
            }
            ReviewDecision::ApprovedForSession => Some(ApprovalOption {
                label: if network_approval_context.is_some() {
                    "Yes, and allow this host for this conversation".to_string()
                } else if additional_permissions.is_some() {
                    "Yes, and allow these permissions for this session".to_string()
                } else {
                    "Yes, and don't ask again for this command in this session".to_string()
                },
                decision: ApprovalDecision::Review(ReviewDecision::ApprovedForSession),
                display_shortcut: None,
                additional_shortcuts: vec![key_hint::plain(KeyCode::Char('a'))],
            }),
            ReviewDecision::NetworkPolicyAmendment {
                network_policy_amendment,
            } => {
                let (label, shortcut) = match network_policy_amendment.action {
                    NetworkPolicyRuleAction::Allow => (
                        "Yes, and allow this host in the future".to_string(),
                        KeyCode::Char('p'),
                    ),
                    NetworkPolicyRuleAction::Deny => (
                        "No, and block this host in the future".to_string(),
                        KeyCode::Char('d'),
                    ),
                };
                Some(ApprovalOption {
                    label,
                    decision: ApprovalDecision::Review(ReviewDecision::NetworkPolicyAmendment {
                        network_policy_amendment: network_policy_amendment.clone(),
                    }),
                    display_shortcut: None,
                    additional_shortcuts: vec![key_hint::plain(shortcut)],
                })
            }
            ReviewDecision::Denied => Some(ApprovalOption {
                label: "No, continue without running it".to_string(),
                decision: ApprovalDecision::Review(ReviewDecision::Denied),
                display_shortcut: None,
                additional_shortcuts: vec![key_hint::plain(KeyCode::Char('d'))],
            }),
            ReviewDecision::Abort => Some(ApprovalOption {
                label: "No, and tell Codex what to do differently".to_string(),
                decision: ApprovalDecision::Review(ReviewDecision::Abort),
                display_shortcut: Some(key_hint::plain(KeyCode::Esc)),
                additional_shortcuts: vec![key_hint::plain(KeyCode::Char('n'))],
            }),
        })
        .collect()
}

pub(super) fn patch_options() -> Vec<ApprovalOption> {
    vec![
        ApprovalOption {
            label: "Yes, proceed".to_string(),
            decision: ApprovalDecision::Review(ReviewDecision::Approved),
            display_shortcut: None,
            additional_shortcuts: vec![key_hint::plain(KeyCode::Char('y'))],
        },
        ApprovalOption {
            label: "Yes, and don't ask again for these files".to_string(),
            decision: ApprovalDecision::Review(ReviewDecision::ApprovedForSession),
            display_shortcut: None,
            additional_shortcuts: vec![key_hint::plain(KeyCode::Char('a'))],
        },
        ApprovalOption {
            label: "No, and tell Codex what to do differently".to_string(),
            decision: ApprovalDecision::Review(ReviewDecision::Abort),
            display_shortcut: Some(key_hint::plain(KeyCode::Esc)),
            additional_shortcuts: vec![key_hint::plain(KeyCode::Char('n'))],
        },
    ]
}

pub(super) fn elicitation_options() -> Vec<ApprovalOption> {
    vec![
        ApprovalOption {
            label: "Yes, provide the requested info".to_string(),
            decision: ApprovalDecision::McpElicitation(ElicitationAction::Accept),
            display_shortcut: None,
            additional_shortcuts: vec![key_hint::plain(KeyCode::Char('y'))],
        },
        ApprovalOption {
            label: "No, but continue without it".to_string(),
            decision: ApprovalDecision::McpElicitation(ElicitationAction::Decline),
            display_shortcut: None,
            additional_shortcuts: vec![key_hint::plain(KeyCode::Char('n'))],
        },
        ApprovalOption {
            label: "Cancel this request".to_string(),
            decision: ApprovalDecision::McpElicitation(ElicitationAction::Cancel),
            display_shortcut: Some(key_hint::plain(KeyCode::Esc)),
            additional_shortcuts: vec![key_hint::plain(KeyCode::Char('c'))],
        },
    ]
}
