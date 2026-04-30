use super::*;

// Snapshot test: ChatWidget at very small heights (idle)
// Ensures overall layout behaves when terminal height is extremely constrained.
#[tokio::test]
async fn ui_snapshots_small_heights_idle() {
    use ratatui::Terminal;
    use ratatui::backend::TestBackend;
    let (chat, _rx, _op_rx) = make_chatwidget_manual(None).await;
    for h in [1u16, 2, 3] {
        let name = format!("chat_small_idle_h{h}");
        let mut terminal = Terminal::new(TestBackend::new(40, h)).expect("create terminal");
        terminal
            .draw(|f| chat.render(f.area(), f.buffer_mut()))
            .expect("draw chat idle");
        assert_snapshot!(name, terminal.backend());
    }
}

// Snapshot test: ChatWidget at very small heights (task running)
// Validates how status + composer are presented within tight space.
#[tokio::test]
async fn ui_snapshots_small_heights_task_running() {
    use ratatui::Terminal;
    use ratatui::backend::TestBackend;
    let (mut chat, _rx, _op_rx) = make_chatwidget_manual(None).await;
    // Activate status line
    chat.handle_codex_event(Event {
        id: "task-1".into(),
        msg: EventMsg::TurnStarted(TurnStartedEvent {
            turn_id: "turn-1".to_string(),
            model_context_window: None,
            collaboration_mode_kind: ModeKind::Default,
        }),
    });
    chat.handle_codex_event(Event {
        id: "task-1".into(),
        msg: EventMsg::AgentReasoningDelta(AgentReasoningDeltaEvent {
            delta: "**Thinking**".into(),
        }),
    });
    for h in [1u16, 2, 3] {
        let name = format!("chat_small_running_h{h}");
        let mut terminal = Terminal::new(TestBackend::new(40, h)).expect("create terminal");
        terminal
            .draw(|f| chat.render(f.area(), f.buffer_mut()))
            .expect("draw chat running");
        assert_snapshot!(name, terminal.backend());
    }
}

// Snapshot test: status widget + approval modal active together
// The modal takes precedence visually; this captures the layout with a running
// task (status indicator active) while an approval request is shown.
#[tokio::test]
async fn status_widget_and_approval_modal_snapshot() {
    use codex_protocol::protocol::ExecApprovalRequestEvent;

    let (mut chat, _rx, _op_rx) = make_chatwidget_manual(None).await;
    // Begin a running task so the status indicator would be active.
    chat.handle_codex_event(Event {
        id: "task-1".into(),
        msg: EventMsg::TurnStarted(TurnStartedEvent {
            turn_id: "turn-1".to_string(),
            model_context_window: None,
            collaboration_mode_kind: ModeKind::Default,
        }),
    });
    // Provide a deterministic header for the status line.
    chat.handle_codex_event(Event {
        id: "task-1".into(),
        msg: EventMsg::AgentReasoningDelta(AgentReasoningDeltaEvent {
            delta: "**Analyzing**".into(),
        }),
    });

    // Now show an approval modal (e.g. exec approval).
    let ev = ExecApprovalRequestEvent {
        call_id: "call-approve-exec".into(),
        approval_id: Some("call-approve-exec".into()),
        turn_id: "turn-approve-exec".into(),
        command: vec!["echo".into(), "hello world".into()],
        cwd: PathBuf::from("/tmp"),
        reason: Some(
            "this is a test reason such as one that would be produced by the model".into(),
        ),
        network_approval_context: None,
        proposed_execpolicy_amendment: Some(ExecPolicyAmendment::new(vec![
            "echo".into(),
            "hello world".into(),
        ])),
        proposed_network_policy_amendments: None,
        additional_permissions: None,
        available_decisions: None,
        parsed_cmd: vec![],
    };
    chat.handle_codex_event(Event {
        id: "sub-approve-exec".into(),
        msg: EventMsg::ExecApprovalRequest(ev),
    });

    // Render at the widget's desired height and snapshot.
    let width: u16 = 100;
    let height = chat.desired_height(width);
    let mut terminal = ratatui::Terminal::new(ratatui::backend::TestBackend::new(width, height))
        .expect("create terminal");
    terminal
        .draw(|f| chat.render(f.area(), f.buffer_mut()))
        .expect("draw status + approval modal");
    assert_snapshot!("status_widget_and_approval_modal", terminal.backend());
}

// Snapshot test: status widget active (StatusIndicatorView)
// Ensures the VT100 rendering of the status indicator is stable when active.
#[tokio::test]
async fn status_widget_active_snapshot() {
    let (mut chat, _rx, _op_rx) = make_chatwidget_manual(None).await;
    // Activate the status indicator by simulating a task start.
    chat.handle_codex_event(Event {
        id: "task-1".into(),
        msg: EventMsg::TurnStarted(TurnStartedEvent {
            turn_id: "turn-1".to_string(),
            model_context_window: None,
            collaboration_mode_kind: ModeKind::Default,
        }),
    });
    // Provide a deterministic header via a bold reasoning chunk.
    chat.handle_codex_event(Event {
        id: "task-1".into(),
        msg: EventMsg::AgentReasoningDelta(AgentReasoningDeltaEvent {
            delta: "**Analyzing**".into(),
        }),
    });
    // Render and snapshot.
    let height = chat.desired_height(80);
    let mut terminal = ratatui::Terminal::new(ratatui::backend::TestBackend::new(80, height))
        .expect("create terminal");
    terminal
        .draw(|f| chat.render(f.area(), f.buffer_mut()))
        .expect("draw status widget");
    assert_snapshot!("status_widget_active", terminal.backend());
}

#[tokio::test]
async fn mcp_startup_header_booting_snapshot() {
    let (mut chat, _rx, _op_rx) = make_chatwidget_manual(None).await;
    chat.show_welcome_banner = false;

    chat.handle_codex_event(Event {
        id: "mcp-1".into(),
        msg: EventMsg::McpStartupUpdate(McpStartupUpdateEvent {
            server: "alpha".into(),
            status: McpStartupStatus::Starting,
        }),
    });

    let height = chat.desired_height(80);
    let mut terminal = ratatui::Terminal::new(ratatui::backend::TestBackend::new(80, height))
        .expect("create terminal");
    terminal
        .draw(|f| chat.render(f.area(), f.buffer_mut()))
        .expect("draw chat widget");
    assert_snapshot!("mcp_startup_header_booting", terminal.backend());
}
