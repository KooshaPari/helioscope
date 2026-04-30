use std::path::PathBuf;

use codex_utils_approval_presets::ApprovalPreset;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum WindowsSandboxEnableMode {
    Elevated,
    Legacy,
}

#[derive(Debug)]
pub(crate) enum WindowsSandboxEvent {
    /// Open the Windows world-writable directories warning.
    /// If `preset` is `Some`, the confirmation will apply the provided
    /// approval/sandbox configuration on Continue; if `None`, it performs no
    /// policy change and only acknowledges/dismisses the warning.
    OpenWorldWritableWarningConfirmation {
        preset: Option<ApprovalPreset>,
        /// Up to 3 sample world-writable directories to display in the warning.
        sample_paths: Vec<String>,
        /// If there are more than `sample_paths`, this carries the remaining count.
        extra_count: usize,
        /// True when the scan failed (e.g. ACL query error) and protections could not be verified.
        failed_scan: bool,
    },

    /// Prompt to enable the Windows sandbox feature before using Agent mode.
    OpenWindowsSandboxEnablePrompt { preset: ApprovalPreset },

    /// Open the Windows sandbox fallback prompt after declining or failing elevation.
    OpenWindowsSandboxFallbackPrompt { preset: ApprovalPreset },

    /// Begin the elevated Windows sandbox setup flow.
    BeginWindowsSandboxElevatedSetup { preset: ApprovalPreset },

    /// Begin the non-elevated Windows sandbox setup flow.
    BeginWindowsSandboxLegacySetup { preset: ApprovalPreset },

    /// Begin a non-elevated grant of read access for an additional directory.
    BeginWindowsSandboxGrantReadRoot { path: String },

    /// Result of attempting to grant read access for an additional directory.
    WindowsSandboxGrantReadRootCompleted {
        path: PathBuf,
        error: Option<String>,
    },

    /// Enable the Windows sandbox feature and switch to Agent mode.
    EnableWindowsSandboxForAgentMode {
        preset: ApprovalPreset,
        mode: WindowsSandboxEnableMode,
    },

    /// Update whether the world-writable directories warning has been acknowledged.
    UpdateWorldWritableWarningAcknowledged(bool),

    /// Persist the acknowledgement flag for the world-writable directories warning.
    PersistWorldWritableWarningAcknowledged,

    /// Skip the next world-writable scan (one-shot) after a user-confirmed continue.
    SkipNextWorldWritableScan,
}
