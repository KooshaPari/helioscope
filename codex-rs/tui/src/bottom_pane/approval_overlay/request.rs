use std::collections::HashMap;
use std::path::PathBuf;

use codex_protocol::ThreadId;
use codex_protocol::mcp::RequestId;
use codex_protocol::models::PermissionProfile;
use codex_protocol::protocol::FileChange;
use codex_protocol::protocol::NetworkApprovalContext;
use codex_protocol::protocol::ReviewDecision;

/// Request coming from the agent that needs user approval.
#[derive(Clone, Debug)]
pub(crate) enum ApprovalRequest {
    Exec {
        thread_id: ThreadId,
        thread_label: Option<String>,
        id: String,
        command: Vec<String>,
        reason: Option<String>,
        available_decisions: Vec<ReviewDecision>,
        network_approval_context: Option<NetworkApprovalContext>,
        additional_permissions: Option<PermissionProfile>,
    },
    ApplyPatch {
        thread_id: ThreadId,
        thread_label: Option<String>,
        id: String,
        reason: Option<String>,
        cwd: PathBuf,
        changes: HashMap<PathBuf, FileChange>,
    },
    McpElicitation {
        thread_id: ThreadId,
        thread_label: Option<String>,
        server_name: String,
        request_id: RequestId,
        message: String,
    },
}

impl ApprovalRequest {
    pub(super) fn thread_id(&self) -> ThreadId {
        match self {
            ApprovalRequest::Exec { thread_id, .. }
            | ApprovalRequest::ApplyPatch { thread_id, .. }
            | ApprovalRequest::McpElicitation { thread_id, .. } => *thread_id,
        }
    }

    pub(super) fn thread_label(&self) -> Option<&str> {
        match self {
            ApprovalRequest::Exec { thread_label, .. }
            | ApprovalRequest::ApplyPatch { thread_label, .. }
            | ApprovalRequest::McpElicitation { thread_label, .. } => thread_label.as_deref(),
        }
    }
}
