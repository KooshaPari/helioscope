use super::super::*;

pub(crate) fn next_submit_op(op_rx: &mut tokio::sync::mpsc::UnboundedReceiver<Op>) -> Op {
    loop {
        match op_rx.try_recv() {
            Ok(op @ Op::UserTurn { .. }) => return op,
            Ok(_) => continue,
            Err(TryRecvError::Empty) => panic!("expected a submit op but queue was empty"),
            Err(TryRecvError::Disconnected) => panic!("expected submit op but channel closed"),
        }
    }
}

pub(crate) fn assert_no_submit_op(op_rx: &mut tokio::sync::mpsc::UnboundedReceiver<Op>) {
    while let Ok(op) = op_rx.try_recv() {
        assert!(
            !matches!(op, Op::UserTurn { .. }),
            "unexpected submit op: {op:?}"
        );
    }
}

pub(crate) fn begin_exec_with_source(
    chat: &mut ChatWidget,
    call_id: &str,
    raw_cmd: &str,
    source: ExecCommandSource,
) -> ExecCommandBeginEvent {
    let command = vec!["bash".to_string(), "-lc".to_string(), raw_cmd.to_string()];
    let parsed_cmd: Vec<ParsedCommand> =
        codex_shell_command::parse_command::parse_command(&command);
    let cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let interaction_input = None;
    let event = ExecCommandBeginEvent {
        call_id: call_id.to_string(),
        process_id: None,
        turn_id: "turn-1".to_string(),
        command,
        cwd,
        parsed_cmd,
        source,
        interaction_input,
    };
    chat.handle_codex_event(Event {
        id: call_id.to_string(),
        msg: EventMsg::ExecCommandBegin(event.clone()),
    });
    event
}

pub(crate) fn begin_unified_exec_startup(
    chat: &mut ChatWidget,
    call_id: &str,
    process_id: &str,
    raw_cmd: &str,
) -> ExecCommandBeginEvent {
    let command = vec!["bash".to_string(), "-lc".to_string(), raw_cmd.to_string()];
    let cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let event = ExecCommandBeginEvent {
        call_id: call_id.to_string(),
        process_id: Some(process_id.to_string()),
        turn_id: "turn-1".to_string(),
        command,
        cwd,
        parsed_cmd: Vec::new(),
        source: ExecCommandSource::UnifiedExecStartup,
        interaction_input: None,
    };
    chat.handle_codex_event(Event {
        id: call_id.to_string(),
        msg: EventMsg::ExecCommandBegin(event.clone()),
    });
    event
}

pub(crate) fn terminal_interaction(
    chat: &mut ChatWidget,
    call_id: &str,
    process_id: &str,
    stdin: &str,
) {
    chat.handle_codex_event(Event {
        id: call_id.to_string(),
        msg: EventMsg::TerminalInteraction(TerminalInteractionEvent {
            call_id: call_id.to_string(),
            process_id: process_id.to_string(),
            stdin: stdin.to_string(),
        }),
    });
}

pub(crate) fn complete_assistant_message(
    chat: &mut ChatWidget,
    item_id: &str,
    text: &str,
    phase: Option<MessagePhase>,
) {
    chat.handle_codex_event(Event {
        id: format!("raw-{item_id}"),
        msg: EventMsg::ItemCompleted(ItemCompletedEvent {
            thread_id: ThreadId::new(),
            turn_id: "turn-1".to_string(),
            item: TurnItem::AgentMessage(AgentMessageItem {
                id: item_id.to_string(),
                content: vec![AgentMessageContent::Text {
                    text: text.to_string(),
                }],
                phase,
            }),
        }),
    });
}

pub(crate) fn begin_exec(
    chat: &mut ChatWidget,
    call_id: &str,
    raw_cmd: &str,
) -> ExecCommandBeginEvent {
    begin_exec_with_source(chat, call_id, raw_cmd, ExecCommandSource::Agent)
}

pub(crate) fn end_exec(
    chat: &mut ChatWidget,
    begin_event: ExecCommandBeginEvent,
    stdout: &str,
    stderr: &str,
    exit_code: i32,
) {
    let aggregated = if stderr.is_empty() {
        stdout.to_string()
    } else {
        format!("{stdout}{stderr}")
    };
    let ExecCommandBeginEvent {
        call_id,
        turn_id,
        command,
        cwd,
        parsed_cmd,
        source,
        interaction_input,
        process_id,
    } = begin_event;
    chat.handle_codex_event(Event {
        id: call_id.clone(),
        msg: EventMsg::ExecCommandEnd(ExecCommandEndEvent {
            call_id,
            process_id,
            turn_id,
            command,
            cwd,
            parsed_cmd,
            source,
            interaction_input,
            stdout: stdout.to_string(),
            stderr: stderr.to_string(),
            aggregated_output: aggregated.clone(),
            exit_code,
            duration: std::time::Duration::from_millis(5),
            formatted_output: aggregated,
            status: if exit_code == 0 {
                CoreExecCommandStatus::Completed
            } else {
                CoreExecCommandStatus::Failed
            },
        }),
    });
}
