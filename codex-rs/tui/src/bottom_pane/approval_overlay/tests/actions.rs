use super::*;
use crate::app_event::AppEvent;
use codex_protocol::ThreadId;
use codex_protocol::protocol::ExecPolicyAmendment;
use codex_protocol::protocol::NetworkApprovalContext;
use codex_protocol::protocol::NetworkApprovalProtocol;
use codex_protocol::protocol::NetworkPolicyAmendment;
use codex_protocol::protocol::NetworkPolicyRuleAction;
use codex_protocol::protocol::Op;
use crossterm::event::KeyCode;
use crossterm::event::KeyEvent;
use crossterm::event::KeyModifiers;
use pretty_assertions::assert_eq;
use tokio::sync::mpsc::unbounded_channel;

#[test]
fn ctrl_c_aborts_and_clears_queue() {
    let (tx, _rx) = unbounded_channel::<AppEvent>();
    let tx = AppEventSender::new(tx);
    let mut view = ApprovalOverlay::new(make_exec_request(), tx, Features::with_defaults());
    view.enqueue_request(make_exec_request());
    assert_eq!(CancellationEvent::Handled, view.on_ctrl_c());
    assert!(view.queue.is_empty());
    assert!(view.is_complete());
}

#[test]
fn shortcut_triggers_selection() {
    let (tx, mut rx) = unbounded_channel::<AppEvent>();
    let tx = AppEventSender::new(tx);
    let mut view = ApprovalOverlay::new(make_exec_request(), tx, Features::with_defaults());
    assert!(!view.is_complete());
    view.handle_key_event(KeyEvent::new(KeyCode::Char('y'), KeyModifiers::NONE));
    // We expect at least one thread-scoped approval op message in the queue.
    let mut saw_op = false;
    while let Ok(ev) = rx.try_recv() {
        if matches!(ev, AppEvent::SubmitThreadOp { .. }) {
            saw_op = true;
            break;
        }
    }
    assert!(saw_op, "expected approval decision to emit an op");
}

#[test]
fn o_opens_source_thread_for_cross_thread_approval() {
    let (tx, mut rx) = unbounded_channel::<AppEvent>();
    let tx = AppEventSender::new(tx);
    let thread_id = ThreadId::new();
    let mut view = ApprovalOverlay::new(
        ApprovalRequest::Exec {
            thread_id,
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

    view.handle_key_event(KeyEvent::new(KeyCode::Char('o'), KeyModifiers::NONE));

    let event = rx.try_recv().expect("expected select-agent-thread event");
    assert_eq!(
        matches!(event, AppEvent::SelectAgentThread(id) if id == thread_id),
        true
    );
}

#[test]
fn exec_prefix_option_emits_execpolicy_amendment() {
    let (tx, mut rx) = unbounded_channel::<AppEvent>();
    let tx = AppEventSender::new(tx);
    let mut view = ApprovalOverlay::new(
        ApprovalRequest::Exec {
            thread_id: ThreadId::new(),
            thread_label: None,
            id: "test".to_string(),
            command: vec!["echo".to_string()],
            reason: None,
            available_decisions: vec![
                ReviewDecision::Approved,
                ReviewDecision::ApprovedExecpolicyAmendment {
                    proposed_execpolicy_amendment: ExecPolicyAmendment::new(vec![
                        "echo".to_string(),
                    ]),
                },
                ReviewDecision::Abort,
            ],
            network_approval_context: None,
            additional_permissions: None,
        },
        tx,
        Features::with_defaults(),
    );
    view.handle_key_event(KeyEvent::new(KeyCode::Char('p'), KeyModifiers::NONE));
    let mut saw_op = false;
    while let Ok(ev) = rx.try_recv() {
        if let AppEvent::SubmitThreadOp {
            op: Op::ExecApproval { decision, .. },
            ..
        } = ev
        {
            assert_eq!(
                decision,
                ReviewDecision::ApprovedExecpolicyAmendment {
                    proposed_execpolicy_amendment: ExecPolicyAmendment::new(vec![
                        "echo".to_string()
                    ])
                }
            );
            saw_op = true;
            break;
        }
    }
    assert!(
        saw_op,
        "expected approval decision to emit an op with command prefix"
    );
}

#[test]
fn network_deny_forever_shortcut_is_not_bound() {
    let (tx, mut rx) = unbounded_channel::<AppEvent>();
    let tx = AppEventSender::new(tx);
    let mut view = ApprovalOverlay::new(
        ApprovalRequest::Exec {
            thread_id: ThreadId::new(),
            thread_label: None,
            id: "test".to_string(),
            command: vec!["curl".to_string(), "https://example.com".to_string()],
            reason: None,
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
        },
        tx,
        Features::with_defaults(),
    );
    view.handle_key_event(KeyEvent::new(KeyCode::Char('d'), KeyModifiers::NONE));

    assert!(
        rx.try_recv().is_err(),
        "unexpected approval event emitted for hidden network deny shortcut"
    );
}

#[test]
fn enter_sets_last_selected_index_without_dismissing() {
    let (tx_raw, mut rx) = unbounded_channel::<AppEvent>();
    let tx = AppEventSender::new(tx_raw);
    let mut view = ApprovalOverlay::new(make_exec_request(), tx, Features::with_defaults());
    view.handle_key_event(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));

    assert!(
        view.is_complete(),
        "exec approval should complete without queued requests"
    );

    let mut decision = None;
    while let Ok(ev) = rx.try_recv() {
        if let AppEvent::SubmitThreadOp {
            op: Op::ExecApproval { decision: d, .. },
            ..
        } = ev
        {
            decision = Some(d);
            break;
        }
    }
    assert_eq!(decision, Some(ReviewDecision::Approved));
}
