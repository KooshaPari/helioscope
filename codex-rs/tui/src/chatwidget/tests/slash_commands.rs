use super::*;
use pretty_assertions::assert_eq;

#[tokio::test]
async fn slash_quit_requests_exit() {
    let (mut chat, mut rx, _op_rx) = make_chatwidget_manual(None).await;

    chat.dispatch_command(SlashCommand::Quit);

    assert_matches!(rx.try_recv(), Ok(AppEvent::Exit(ExitMode::ShutdownFirst)));
}

#[tokio::test]
async fn slash_copy_state_tracks_turn_complete_final_reply() {
    let (mut chat, _rx, _op_rx) = make_chatwidget_manual(None).await;

    chat.handle_codex_event(Event {
        id: "turn-1".into(),
        msg: EventMsg::TurnComplete(TurnCompleteEvent {
            turn_id: "turn-1".to_string(),
            last_agent_message: Some("Final reply **markdown**".to_string()),
        }),
    });

    assert_eq!(
        chat.last_copyable_output,
        Some("Final reply **markdown**".to_string())
    );
}

#[tokio::test]
async fn slash_copy_state_tracks_plan_item_completion() {
    let (mut chat, _rx, _op_rx) = make_chatwidget_manual(None).await;
    let plan_text = "## Plan\n\n1. Build it\n2. Test it".to_string();

    chat.handle_codex_event(Event {
        id: "item-plan".into(),
        msg: EventMsg::ItemCompleted(ItemCompletedEvent {
            thread_id: ThreadId::new(),
            turn_id: "turn-1".to_string(),
            item: TurnItem::Plan(PlanItem {
                id: "plan-1".to_string(),
                text: plan_text.clone(),
            }),
        }),
    });
    chat.handle_codex_event(Event {
        id: "turn-1".into(),
        msg: EventMsg::TurnComplete(TurnCompleteEvent {
            turn_id: "turn-1".to_string(),
            last_agent_message: None,
        }),
    });

    assert_eq!(chat.last_copyable_output, Some(plan_text));
}

#[tokio::test]
async fn slash_copy_reports_when_no_copyable_output_exists() {
    let (mut chat, mut rx, _op_rx) = make_chatwidget_manual(None).await;

    chat.dispatch_command(SlashCommand::Copy);

    let cells = drain_insert_history(&mut rx);
    assert_eq!(cells.len(), 1, "expected one info message");
    let rendered = lines_to_single_string(&cells[0]);
    assert_snapshot!("slash_copy_no_output_info_message", rendered);
    assert!(
        rendered.contains(
            "`/copy` is unavailable before the first Codex output or right after a rollback."
        ),
        "expected no-output message, got {rendered:?}"
    );
}

#[tokio::test]
async fn slash_copy_state_is_preserved_during_running_task() {
    let (mut chat, _rx, _op_rx) = make_chatwidget_manual(None).await;

    chat.handle_codex_event(Event {
        id: "turn-1".into(),
        msg: EventMsg::TurnComplete(TurnCompleteEvent {
            turn_id: "turn-1".to_string(),
            last_agent_message: Some("Previous completed reply".to_string()),
        }),
    });
    chat.on_task_started();

    assert_eq!(
        chat.last_copyable_output,
        Some("Previous completed reply".to_string())
    );
}

#[tokio::test]
async fn slash_copy_state_clears_on_thread_rollback() {
    let (mut chat, _rx, _op_rx) = make_chatwidget_manual(None).await;

    chat.handle_codex_event(Event {
        id: "turn-1".into(),
        msg: EventMsg::TurnComplete(TurnCompleteEvent {
            turn_id: "turn-1".to_string(),
            last_agent_message: Some("Reply that will be rolled back".to_string()),
        }),
    });
    chat.handle_codex_event(Event {
        id: "rollback-1".into(),
        msg: EventMsg::ThreadRolledBack(ThreadRolledBackEvent { num_turns: 1 }),
    });

    assert_eq!(chat.last_copyable_output, None);
}

#[tokio::test]
async fn slash_copy_is_unavailable_when_legacy_agent_message_is_not_repeated_on_turn_complete() {
    let (mut chat, mut rx, _op_rx) = make_chatwidget_manual(None).await;

    chat.handle_codex_event(Event {
        id: "turn-1".into(),
        msg: EventMsg::AgentMessage(AgentMessageEvent {
            message: "Legacy final message".into(),
            phase: None,
        }),
    });
    let _ = drain_insert_history(&mut rx);
    chat.handle_codex_event(Event {
        id: "turn-1".into(),
        msg: EventMsg::TurnComplete(TurnCompleteEvent {
            turn_id: "turn-1".to_string(),
            last_agent_message: None,
        }),
    });
    let _ = drain_insert_history(&mut rx);

    chat.dispatch_command(SlashCommand::Copy);

    let cells = drain_insert_history(&mut rx);
    assert_eq!(cells.len(), 1, "expected one info message");
    let rendered = lines_to_single_string(&cells[0]);
    assert!(
        rendered.contains(
            "`/copy` is unavailable before the first Codex output or right after a rollback."
        ),
        "expected unavailable message, got {rendered:?}"
    );
}

#[tokio::test]
async fn slash_copy_is_unavailable_when_legacy_agent_message_item_is_not_repeated_on_turn_complete()
{
    let (mut chat, mut rx, _op_rx) = make_chatwidget_manual(None).await;

    complete_assistant_message(&mut chat, "msg-1", "Legacy item final message", None);
    let _ = drain_insert_history(&mut rx);
    chat.handle_codex_event(Event {
        id: "turn-1".into(),
        msg: EventMsg::TurnComplete(TurnCompleteEvent {
            turn_id: "turn-1".to_string(),
            last_agent_message: None,
        }),
    });
    let _ = drain_insert_history(&mut rx);

    chat.dispatch_command(SlashCommand::Copy);

    let cells = drain_insert_history(&mut rx);
    assert_eq!(cells.len(), 1, "expected one info message");
    let rendered = lines_to_single_string(&cells[0]);
    assert!(
        rendered.contains(
            "`/copy` is unavailable before the first Codex output or right after a rollback."
        ),
        "expected unavailable message, got {rendered:?}"
    );
}

#[tokio::test]
async fn slash_copy_does_not_return_stale_output_after_thread_rollback() {
    let (mut chat, mut rx, _op_rx) = make_chatwidget_manual(None).await;

    chat.handle_codex_event(Event {
        id: "turn-1".into(),
        msg: EventMsg::TurnComplete(TurnCompleteEvent {
            turn_id: "turn-1".to_string(),
            last_agent_message: Some("Reply that will be rolled back".to_string()),
        }),
    });
    let _ = drain_insert_history(&mut rx);

    chat.handle_codex_event(Event {
        id: "rollback-1".into(),
        msg: EventMsg::ThreadRolledBack(ThreadRolledBackEvent { num_turns: 1 }),
    });
    let _ = drain_insert_history(&mut rx);

    chat.dispatch_command(SlashCommand::Copy);

    let cells = drain_insert_history(&mut rx);
    assert_eq!(cells.len(), 1, "expected one info message");
    let rendered = lines_to_single_string(&cells[0]);
    assert!(
        rendered.contains(
            "`/copy` is unavailable before the first Codex output or right after a rollback."
        ),
        "expected rollback-cleared copy state message, got {rendered:?}"
    );
}

#[tokio::test]
async fn slash_exit_requests_exit() {
    let (mut chat, mut rx, _op_rx) = make_chatwidget_manual(None).await;

    chat.dispatch_command(SlashCommand::Exit);

    assert_matches!(rx.try_recv(), Ok(AppEvent::Exit(ExitMode::ShutdownFirst)));
}

#[tokio::test]
async fn slash_clean_submits_background_terminal_cleanup() {
    let (mut chat, mut rx, mut op_rx) = make_chatwidget_manual(None).await;

    chat.dispatch_command(SlashCommand::Clean);

    assert_matches!(op_rx.try_recv(), Ok(Op::CleanBackgroundTerminals));
    let cells = drain_insert_history(&mut rx);
    assert_eq!(cells.len(), 1, "expected cleanup confirmation message");
    let rendered = lines_to_single_string(&cells[0]);
    assert!(
        rendered.contains("Stopping all background terminals."),
        "expected cleanup confirmation, got {rendered:?}"
    );
}

#[tokio::test]
async fn slash_clear_requests_ui_clear_when_idle() {
    let (mut chat, mut rx, _op_rx) = make_chatwidget_manual(None).await;

    chat.dispatch_command(SlashCommand::Clear);

    assert_matches!(rx.try_recv(), Ok(AppEvent::ClearUi));
}

#[tokio::test]
async fn slash_clear_is_disabled_while_task_running() {
    let (mut chat, mut rx, _op_rx) = make_chatwidget_manual(None).await;
    chat.bottom_pane.set_task_running(true);

    chat.dispatch_command(SlashCommand::Clear);

    let event = rx.try_recv().expect("expected disabled command error");
    match event {
        AppEvent::InsertHistoryCell(cell) => {
            let rendered = lines_to_single_string(&cell.display_lines(80));
            assert!(
                rendered.contains("'/clear' is disabled while a task is in progress."),
                "expected /clear task-running error, got {rendered:?}"
            );
        }
        other => panic!("expected InsertHistoryCell error, got {other:?}"),
    }
    assert!(rx.try_recv().is_err(), "expected no follow-up events");
}

#[tokio::test]
async fn slash_memory_drop_submits_drop_memories_op() {
    let (mut chat, _rx, mut op_rx) = make_chatwidget_manual(None).await;

    chat.dispatch_command(SlashCommand::MemoryDrop);

    assert_matches!(op_rx.try_recv(), Ok(Op::DropMemories));
}

#[tokio::test]
async fn slash_memory_update_submits_update_memories_op() {
    let (mut chat, _rx, mut op_rx) = make_chatwidget_manual(None).await;

    chat.dispatch_command(SlashCommand::MemoryUpdate);

    assert_matches!(op_rx.try_recv(), Ok(Op::UpdateMemories));
}

#[tokio::test]
async fn slash_resume_opens_picker() {
    let (mut chat, mut rx, _op_rx) = make_chatwidget_manual(None).await;

    chat.dispatch_command(SlashCommand::Resume);

    assert_matches!(rx.try_recv(), Ok(AppEvent::OpenResumePicker));
}

#[tokio::test]
async fn slash_fork_requests_current_fork() {
    let (mut chat, mut rx, _op_rx) = make_chatwidget_manual(None).await;

    chat.dispatch_command(SlashCommand::Fork);

    assert_matches!(rx.try_recv(), Ok(AppEvent::ForkCurrentSession));
}

#[tokio::test]
async fn slash_rollout_displays_current_path() {
    let (mut chat, mut rx, _op_rx) = make_chatwidget_manual(None).await;
    let rollout_path = PathBuf::from("/tmp/codex-test-rollout.jsonl");
    chat.current_rollout_path = Some(rollout_path.clone());

    chat.dispatch_command(SlashCommand::Rollout);

    let cells = drain_insert_history(&mut rx);
    assert_eq!(cells.len(), 1, "expected info message for rollout path");
    let rendered = lines_to_single_string(&cells[0]);
    assert!(
        rendered.contains(&rollout_path.display().to_string()),
        "expected rollout path to be shown: {rendered}"
    );
}

#[tokio::test]
async fn slash_rollout_handles_missing_path() {
    let (mut chat, mut rx, _op_rx) = make_chatwidget_manual(None).await;

    chat.dispatch_command(SlashCommand::Rollout);

    let cells = drain_insert_history(&mut rx);
    assert_eq!(
        cells.len(),
        1,
        "expected info message explaining missing path"
    );
    let rendered = lines_to_single_string(&cells[0]);
    assert!(
        rendered.contains("not available"),
        "expected missing rollout path message: {rendered}"
    );
}
