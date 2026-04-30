use super::*;
use crate::app_event::AppEvent;
use crate::render::renderable::Renderable;
use crate::status_indicator_widget::STATUS_DETAILS_DEFAULT_MAX_LINES;
use crate::status_indicator_widget::StatusDetailsCapitalization;
use codex_protocol::protocol::Op;
use codex_protocol::protocol::SkillScope;
use crossterm::event::KeyCode;
use crossterm::event::KeyEvent;
use crossterm::event::KeyEventKind;
use crossterm::event::KeyModifiers;
use insta::assert_snapshot;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use std::cell::Cell;
use std::path::PathBuf;
use std::rc::Rc;
use tokio::sync::mpsc::unbounded_channel;

fn snapshot_buffer(buf: &Buffer) -> String {
    let mut lines = Vec::new();
    for y in 0..buf.area().height {
        let mut row = String::new();
        for x in 0..buf.area().width {
            row.push(buf[(x, y)].symbol().chars().next().unwrap_or(' '));
        }
        lines.push(row);
    }
    lines.join("\n")
}

fn render_snapshot(pane: &BottomPane, area: Rect) -> String {
    let mut buf = Buffer::empty(area);
    pane.render(area, &mut buf);
    snapshot_buffer(&buf)
}

fn exec_request() -> ApprovalRequest {
    ApprovalRequest::Exec {
        thread_id: codex_protocol::ThreadId::new(),
        thread_label: None,
        id: "1".to_string(),
        command: vec!["echo".into(), "ok".into()],
        reason: None,
        available_decisions: vec![
            codex_protocol::protocol::ReviewDecision::Approved,
            codex_protocol::protocol::ReviewDecision::Abort,
        ],
        network_approval_context: None,
        additional_permissions: None,
    }
}

include!("tests/status.rs");
include!("tests/attachments.rs");
include!("tests/routing.rs");
