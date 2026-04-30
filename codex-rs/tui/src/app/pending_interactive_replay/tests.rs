use super::super::ThreadEventStore;
use codex_protocol::protocol::Event;
use codex_protocol::protocol::EventMsg;
use codex_protocol::protocol::Op;
use codex_protocol::protocol::TurnAbortReason;
use pretty_assertions::assert_eq;
use std::collections::HashMap;
use std::path::PathBuf;

#[test]
fn thread_event_snapshot_keeps_pending_request_user_input() {
    let mut store = ThreadEventStore::new(8);
    let request = Event {
        id: "ev-1".to_string(),
        msg: EventMsg::RequestUserInput(
            codex_protocol::request_user_input::RequestUserInputEvent {
                call_id: "call-1".to_string(),
                turn_id: "turn-1".to_string(),
                questions: Vec::new(),
            },
        ),
    };

    store.push_event(request);

    let snapshot = store.snapshot();
    assert_eq!(snapshot.events.len(), 1);
    assert!(matches!(
        snapshot.events.first().map(|event| &event.msg),
        Some(EventMsg::RequestUserInput(_))
    ));
}

#[test]
fn thread_event_snapshot_drops_resolved_request_user_input_after_user_answer() {
    let mut store = ThreadEventStore::new(8);
    store.push_event(Event {
        id: "ev-1".to_string(),
        msg: EventMsg::RequestUserInput(
            codex_protocol::request_user_input::RequestUserInputEvent {
                call_id: "call-1".to_string(),
                turn_id: "turn-1".to_string(),
                questions: Vec::new(),
            },
        ),
    });

    store.note_outbound_op(&Op::UserInputAnswer {
        id: "turn-1".to_string(),
        response: codex_protocol::request_user_input::RequestUserInputResponse {
            answers: HashMap::new(),
        },
    });

    let snapshot = store.snapshot();
    assert!(
        snapshot.events.is_empty(),
        "resolved request_user_input prompt should not replay on thread switch"
    );
}

#[test]
fn thread_event_snapshot_drops_resolved_exec_approval_after_outbound_approval_id() {
    let mut store = ThreadEventStore::new(8);
    store.push_event(Event {
        id: "ev-1".to_string(),
        msg: EventMsg::ExecApprovalRequest(codex_protocol::protocol::ExecApprovalRequestEvent {
            call_id: "call-1".to_string(),
            approval_id: Some("approval-1".to_string()),
            turn_id: "turn-1".to_string(),
            command: vec!["echo".to_string(), "hi".to_string()],
            cwd: PathBuf::from("/tmp"),
            reason: None,
            network_approval_context: None,
            proposed_execpolicy_amendment: None,
            proposed_network_policy_amendments: None,
            additional_permissions: None,
            available_decisions: None,
            parsed_cmd: Vec::new(),
        }),
    });

    store.note_outbound_op(&Op::ExecApproval {
        id: "approval-1".to_string(),
        turn_id: Some("turn-1".to_string()),
        decision: codex_protocol::protocol::ReviewDecision::Approved,
    });

    let snapshot = store.snapshot();
    assert!(
        snapshot.events.is_empty(),
        "resolved exec approval prompt should not replay on thread switch"
    );
}

#[test]
fn thread_event_snapshot_drops_answered_request_user_input_for_multi_prompt_turn() {
    let mut store = ThreadEventStore::new(8);
    store.push_event(Event {
        id: "ev-1".to_string(),
        msg: EventMsg::RequestUserInput(
            codex_protocol::request_user_input::RequestUserInputEvent {
                call_id: "call-1".to_string(),
                turn_id: "turn-1".to_string(),
                questions: Vec::new(),
            },
        ),
    });

    store.note_outbound_op(&Op::UserInputAnswer {
        id: "turn-1".to_string(),
        response: codex_protocol::request_user_input::RequestUserInputResponse {
            answers: HashMap::new(),
        },
    });

    store.push_event(Event {
        id: "ev-2".to_string(),
        msg: EventMsg::RequestUserInput(
            codex_protocol::request_user_input::RequestUserInputEvent {
                call_id: "call-2".to_string(),
                turn_id: "turn-1".to_string(),
                questions: Vec::new(),
            },
        ),
    });

    let snapshot = store.snapshot();
    assert_eq!(snapshot.events.len(), 1);
    assert!(matches!(
        snapshot.events.first().map(|event| &event.msg),
        Some(EventMsg::RequestUserInput(ev)) if ev.call_id == "call-2"
    ));
}

#[test]
fn thread_event_snapshot_keeps_newer_request_user_input_pending_when_same_turn_has_queue() {
    let mut store = ThreadEventStore::new(8);
    store.push_event(Event {
        id: "ev-1".to_string(),
        msg: EventMsg::RequestUserInput(
            codex_protocol::request_user_input::RequestUserInputEvent {
                call_id: "call-1".to_string(),
                turn_id: "turn-1".to_string(),
                questions: Vec::new(),
            },
        ),
    });
    store.push_event(Event {
        id: "ev-2".to_string(),
        msg: EventMsg::RequestUserInput(
            codex_protocol::request_user_input::RequestUserInputEvent {
                call_id: "call-2".to_string(),
                turn_id: "turn-1".to_string(),
                questions: Vec::new(),
            },
        ),
    });

    store.note_outbound_op(&Op::UserInputAnswer {
        id: "turn-1".to_string(),
        response: codex_protocol::request_user_input::RequestUserInputResponse {
            answers: HashMap::new(),
        },
    });

    let snapshot = store.snapshot();
    assert_eq!(snapshot.events.len(), 1);
    assert!(matches!(
        snapshot.events.first().map(|event| &event.msg),
        Some(EventMsg::RequestUserInput(ev)) if ev.call_id == "call-2"
    ));
}

#[test]
fn thread_event_snapshot_drops_resolved_patch_approval_after_outbound_approval() {
    let mut store = ThreadEventStore::new(8);
    store.push_event(Event {
        id: "ev-1".to_string(),
        msg: EventMsg::ApplyPatchApprovalRequest(
            codex_protocol::protocol::ApplyPatchApprovalRequestEvent {
                call_id: "call-1".to_string(),
                turn_id: "turn-1".to_string(),
                changes: HashMap::new(),
                reason: None,
                grant_root: None,
            },
        ),
    });

    store.note_outbound_op(&Op::PatchApproval {
        id: "call-1".to_string(),
        decision: codex_protocol::protocol::ReviewDecision::Approved,
    });

    let snapshot = store.snapshot();
    assert!(
        snapshot.events.is_empty(),
        "resolved patch approval prompt should not replay on thread switch"
    );
}

#[test]
fn thread_event_snapshot_drops_pending_approvals_when_turn_aborts() {
    let mut store = ThreadEventStore::new(8);
    store.push_event(Event {
        id: "ev-1".to_string(),
        msg: EventMsg::ExecApprovalRequest(codex_protocol::protocol::ExecApprovalRequestEvent {
            call_id: "exec-call-1".to_string(),
            approval_id: Some("approval-1".to_string()),
            turn_id: "turn-1".to_string(),
            command: vec!["echo".to_string(), "hi".to_string()],
            cwd: PathBuf::from("/tmp"),
            reason: None,
            network_approval_context: None,
            proposed_execpolicy_amendment: None,
            proposed_network_policy_amendments: None,
            additional_permissions: None,
            available_decisions: None,
            parsed_cmd: Vec::new(),
        }),
    });
    store.push_event(Event {
        id: "ev-2".to_string(),
        msg: EventMsg::ApplyPatchApprovalRequest(
            codex_protocol::protocol::ApplyPatchApprovalRequestEvent {
                call_id: "patch-call-1".to_string(),
                turn_id: "turn-1".to_string(),
                changes: HashMap::new(),
                reason: None,
                grant_root: None,
            },
        ),
    });
    store.push_event(Event {
        id: "ev-3".to_string(),
        msg: EventMsg::TurnAborted(codex_protocol::protocol::TurnAbortedEvent {
            turn_id: Some("turn-1".to_string()),
            reason: TurnAbortReason::Replaced,
        }),
    });

    let snapshot = store.snapshot();
    assert!(snapshot.events.iter().all(|event| {
        !matches!(
            &event.msg,
            EventMsg::ExecApprovalRequest(_) | EventMsg::ApplyPatchApprovalRequest(_)
        )
    }));
}

#[test]
fn thread_event_snapshot_drops_resolved_elicitation_after_outbound_resolution() {
    let mut store = ThreadEventStore::new(8);
    let request_id = codex_protocol::mcp::RequestId::String("request-1".to_string());
    store.push_event(Event {
        id: "ev-1".to_string(),
        msg: EventMsg::ElicitationRequest(codex_protocol::approvals::ElicitationRequestEvent {
            server_name: "server-1".to_string(),
            id: request_id.clone(),
            message: "Please confirm".to_string(),
        }),
    });

    store.note_outbound_op(&Op::ResolveElicitation {
        server_name: "server-1".to_string(),
        request_id,
        decision: codex_protocol::approvals::ElicitationAction::Accept,
    });

    let snapshot = store.snapshot();
    assert!(
        snapshot.events.is_empty(),
        "resolved elicitation prompt should not replay on thread switch"
    );
}

#[test]
fn thread_event_store_reports_pending_thread_approvals() {
    let mut store = ThreadEventStore::new(8);
    assert_eq!(store.has_pending_thread_approvals(), false);

    store.push_event(Event {
        id: "ev-1".to_string(),
        msg: EventMsg::ExecApprovalRequest(codex_protocol::protocol::ExecApprovalRequestEvent {
            call_id: "call-1".to_string(),
            approval_id: None,
            turn_id: "turn-1".to_string(),
            command: vec!["echo".to_string(), "hi".to_string()],
            cwd: PathBuf::from("/tmp"),
            reason: None,
            network_approval_context: None,
            proposed_execpolicy_amendment: None,
            proposed_network_policy_amendments: None,
            additional_permissions: None,
            available_decisions: None,
            parsed_cmd: Vec::new(),
        }),
    });

    assert_eq!(store.has_pending_thread_approvals(), true);

    store.note_outbound_op(&Op::ExecApproval {
        id: "call-1".to_string(),
        turn_id: Some("turn-1".to_string()),
        decision: codex_protocol::protocol::ReviewDecision::Approved,
    });

    assert_eq!(store.has_pending_thread_approvals(), false);
}

#[test]
fn request_user_input_does_not_count_as_pending_thread_approval() {
    let mut store = ThreadEventStore::new(8);
    store.push_event(Event {
        id: "ev-1".to_string(),
        msg: EventMsg::RequestUserInput(
            codex_protocol::request_user_input::RequestUserInputEvent {
                call_id: "call-1".to_string(),
                turn_id: "turn-1".to_string(),
                questions: Vec::new(),
            },
        ),
    });

    assert_eq!(store.has_pending_thread_approvals(), false);
}
