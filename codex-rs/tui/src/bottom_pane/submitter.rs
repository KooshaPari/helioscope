//! Submitter trait and validation logic for chat composer message submission.
//!
//! This module encapsulates the core submission workflow:
//! - Submit validation: ensuring the input is valid before dispatch
//! - Message construction: preparing the final text and elements
//! - Send triggers: coordinating the actual submission and queuing mechanics

use codex_protocol::user_input::TextElement;

/// Result of a submission attempt.
#[derive(Debug, Clone, PartialEq)]
pub enum SubmissionResult {
    /// Successfully prepared for immediate submission or queuing.
    Ready {
        text: String,
        text_elements: Vec<TextElement>,
    },
    /// Submission was rejected; the draft should remain unchanged.
    Rejected,
}

/// Core trait for submission validation, message preparation, and send triggers.
///
/// Implementers handle:
/// - Validating whether a submission is allowed (e.g., constraints, empty checks)
/// - Preparing the final message text and text elements
/// - Coordinating the submission trigger (Submit vs Queue)
pub trait Submitter {
    /// Validate that a submission attempt is allowed.
    ///
    /// Returns `true` if submission may proceed, `false` if it should be rejected.
    /// Implementers may emit diagnostic messages (e.g., error history cells) before returning.
    fn validate_submission(&mut self) -> bool;

    /// Prepare the message text and elements for submission.
    ///
    /// This is called after validation passes. It should:
    /// - Expand any pending placeholders (pasted content)
    /// - Trim whitespace and rebase text elements
    /// - Validate special cases (slash commands, custom prompts)
    /// - Check size limits
    ///
    /// Returns `Some(text, elements)` if preparation succeeds, `None` if the draft
    /// should be restored (e.g., validation failure or oversized input).
    fn prepare_message(&mut self) -> SubmissionResult;

    /// Handle a submit trigger (Enter key).
    ///
    /// The message has been prepared and is ready to dispatch.
    /// This method records the submission in history and clears temporary state.
    fn on_submit(&mut self, text: String, text_elements: Vec<TextElement>);

    /// Handle a queue trigger (Tab key when a task is running).
    ///
    /// The message has been prepared and will be queued for later dispatch.
    /// This method records the submission in history and clears temporary state.
    fn on_queue(&mut self, text: String, text_elements: Vec<TextElement>);
}

/// Helper trait for detecting and aborting submission in edge cases.
///
/// Used during submission preparation to detect invalid states and
/// restore the original draft when needed.
pub trait SubmissionGuard {
    /// Check whether submission should proceed despite unusual conditions.
    ///
    /// Returns `true` to proceed, `false` to abort and restore the original draft.
    fn should_proceed(&self) -> bool;

    /// Restore the original draft state if submission is aborted.
    fn restore_draft(&mut self);
}
