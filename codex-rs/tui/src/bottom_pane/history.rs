//! # History Navigation Trait
//!
//! This module defines the `HistoryNavigator` trait, which abstracts shell-style
//! history navigation (Up/Down) for the chat composer.
//!
//! The trait decouples the history state machine ([`ChatComposerHistory`]) from the
//! text editor widget, making it easier to:
//!
//! - Test history navigation logic in isolation
//! - Reuse the same navigation semantics in different contexts
//! - Mock history behavior for UI tests
//!
//! ## Usage
//!
//! Implementing types should:
//! 1. Determine if navigation should be handled via [`should_handle_history_navigation`](Self::should_handle_history_navigation)
//! 2. Call [`navigate_history_up`](Self::navigate_history_up) or [`navigate_history_down`](Self::navigate_history_down)
//! 3. Apply the returned entry to the UI state (text, elements, attachments)
//!
//! See [`ChatComposer`](super::chat_composer::ChatComposer) for a concrete implementation.

use super::chat_composer_history::HistoryEntry;
use crate::app_event_sender::AppEventSender;

/// Trait for history navigation in text editors that support shell-style Up/Down recall.
///
/// History navigation requires gating based on cursor position to avoid interfering
/// with normal multiline cursor movement. See [`should_handle_history_navigation`](Self::should_handle_history_navigation).
pub trait HistoryNavigator {
    /// Determine if Up/Down should navigate history for the current text state.
    ///
    /// This returns true when:
    /// - The text buffer is empty, OR
    /// - The text exactly matches the last recalled history entry AND the cursor is at a line boundary
    ///
    /// This boundary gate preserves shell-like history recall while keeping multiline
    /// cursor movement usable. If the cursor is in the middle of a recalled entry,
    /// the caller should treat Up/Down as normal cursor movement instead.
    fn should_handle_history_navigation(&self, text: &str, cursor: usize) -> bool;

    /// Navigate to the previous (older) history entry.
    ///
    /// Returns the entry to apply, or `None` if already at the oldest entry or
    /// the entry is still being fetched asynchronously.
    ///
    /// # Async Fetching
    ///
    /// For persistent (cross-session) history, fetching may be asynchronous.
    /// The implementation may send a request via `app_event_tx` and return `None`.
    /// When the response arrives, it will be fed back via [`on_history_entry_response`](Self::on_history_entry_response).
    fn navigate_history_up(&mut self, app_event_tx: &AppEventSender) -> Option<HistoryEntry>;

    /// Navigate to the next (newer) history entry.
    ///
    /// Returns the entry to apply, or `None` if already at the newest entry (beyond
    /// the most recent submission, i.e., the empty draft).
    fn navigate_history_down(&mut self, app_event_tx: &AppEventSender) -> Option<HistoryEntry>;

    /// Integrate an asynchronous response to an on-demand history lookup.
    ///
    /// Called when a persistent history entry fetch completes. The implementation
    /// should cache the entry and return it only if the cursor position still matches
    /// the request (to handle the case where the user moved on to a different entry
    /// while waiting for the fetch).
    fn on_history_entry_response(
        &mut self,
        log_id: u64,
        offset: usize,
        entry: Option<String>,
    ) -> Option<HistoryEntry>;

    /// Reset navigation tracking so the next Up key resumes from the latest entry.
    ///
    /// Called after a new submission is recorded, so the user can immediately
    /// press Up to recall what they just submitted.
    fn reset_history_navigation(&mut self);
}
