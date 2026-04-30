use crate::bottom_pane::approval_overlay::ApprovalRequest;
use crate::diff_render::DiffSummary;
use crate::exec_command::strip_bash_lc_and_escape;
use crate::key_hint;
use crate::render::highlight::highlight_bash_to_lines;
use crate::render::renderable::ColumnRenderable;
use crate::render::renderable::Renderable;
use codex_protocol::models::PermissionProfile;
use crossterm::event::KeyCode;
use ratatui::style::Stylize;
use ratatui::text::Line;
use ratatui::text::Span;
use ratatui::widgets::Paragraph;
use ratatui::widgets::Wrap;

pub(super) fn approval_footer_hint(request: &ApprovalRequest) -> Line<'static> {
    let mut spans = vec![
        "Press ".into(),
        key_hint::plain(KeyCode::Enter).into(),
        " to confirm or ".into(),
        key_hint::plain(KeyCode::Esc).into(),
        " to cancel".into(),
    ];
    if request.thread_label().is_some() {
        spans.extend([
            " or ".into(),
            key_hint::plain(KeyCode::Char('o')).into(),
            " to open thread".into(),
        ]);
    }
    Line::from(spans)
}

pub(super) fn build_header(request: &ApprovalRequest) -> Box<dyn Renderable> {
    match request {
        ApprovalRequest::Exec {
            thread_label,
            reason,
            command,
            network_approval_context,
            additional_permissions,
            ..
        } => {
            let mut header: Vec<Line<'static>> = Vec::new();
            if let Some(thread_label) = thread_label {
                header.push(Line::from(vec![
                    "Thread: ".into(),
                    thread_label.clone().bold(),
                ]));
                header.push(Line::from(""));
            }
            if let Some(reason) = reason {
                header.push(Line::from(vec!["Reason: ".into(), reason.clone().italic()]));
                header.push(Line::from(""));
            }
            if let Some(additional_permissions) = additional_permissions
                && let Some(rule_line) = format_additional_permissions_rule(additional_permissions)
            {
                header.push(Line::from(vec![
                    "Permission rule: ".into(),
                    rule_line.cyan(),
                ]));
                header.push(Line::from(""));
            }
            let full_cmd = strip_bash_lc_and_escape(command);
            let mut full_cmd_lines = highlight_bash_to_lines(&full_cmd);
            if let Some(first) = full_cmd_lines.first_mut() {
                first.spans.insert(0, Span::from("$ "));
            }
            if network_approval_context.is_none() {
                header.extend(full_cmd_lines);
            }
            Box::new(Paragraph::new(header).wrap(Wrap { trim: false }))
        }
        ApprovalRequest::ApplyPatch {
            thread_label,
            reason,
            cwd,
            changes,
            ..
        } => {
            let mut header: Vec<Box<dyn Renderable>> = Vec::new();
            if let Some(thread_label) = thread_label {
                header.push(Box::new(Line::from(vec![
                    "Thread: ".into(),
                    thread_label.clone().bold(),
                ])));
                header.push(Box::new(Line::from("")));
            }
            if let Some(reason) = reason
                && !reason.is_empty()
            {
                header.push(Box::new(
                    Paragraph::new(Line::from_iter([
                        "Reason: ".into(),
                        reason.clone().italic(),
                    ]))
                    .wrap(Wrap { trim: false }),
                ));
                header.push(Box::new(Line::from("")));
            }
            header.push(DiffSummary::new(changes.clone(), cwd.clone()).into());
            Box::new(ColumnRenderable::with(header))
        }
        ApprovalRequest::McpElicitation {
            thread_label,
            server_name,
            message,
            ..
        } => {
            let mut lines = Vec::new();
            if let Some(thread_label) = thread_label {
                lines.push(Line::from(vec![
                    "Thread: ".into(),
                    thread_label.clone().bold(),
                ]));
                lines.push(Line::from(""));
            }
            lines.extend([
                Line::from(vec!["Server: ".into(), server_name.clone().bold()]),
                Line::from(""),
                Line::from(message.clone()),
            ]);
            let header = Paragraph::new(lines).wrap(Wrap { trim: false });
            Box::new(header)
        }
    }
}

fn format_additional_permissions_rule(
    additional_permissions: &PermissionProfile,
) -> Option<String> {
    let mut parts = Vec::new();
    if let Some(file_system) = additional_permissions.file_system.as_ref() {
        if let Some(read) = file_system.read.as_ref() {
            let reads = read
                .iter()
                .map(|path| format!("`{}`", path.display()))
                .collect::<Vec<_>>()
                .join(", ");
            parts.push(format!("read {reads}"));
        }
        if let Some(write) = file_system.write.as_ref() {
            let writes = write
                .iter()
                .map(|path| format!("`{}`", path.display()))
                .collect::<Vec<_>>()
                .join(", ");
            parts.push(format!("write {writes}"));
        }
    }

    if parts.is_empty() {
        None
    } else {
        Some(parts.join("; "))
    }
}
