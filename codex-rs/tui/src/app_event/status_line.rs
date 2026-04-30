use std::path::PathBuf;

use crate::bottom_pane::StatusLineItem;

#[derive(Debug)]
pub(crate) enum StatusLineEvent {
    /// Async update of the current git branch for status line rendering.
    BranchUpdated {
        cwd: PathBuf,
        branch: Option<String>,
    },

    /// Apply a user-confirmed status-line item ordering/selection.
    Setup { items: Vec<StatusLineItem> },

    /// Dismiss the status-line setup UI without changing config.
    SetupCancelled,
}
