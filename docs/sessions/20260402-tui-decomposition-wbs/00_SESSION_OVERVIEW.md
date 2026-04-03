# Session Overview

Session: `20260402-tui-decomposition-wbs`

## Goal

Audit the `heliosCLI` TUI decomposition surface with six parallel subagents and turn the findings
into a dependency-aware work breakdown structure for non-PR local execution.

## Scope

- `codex-rs/tui/src/bottom_pane/chat_composer.rs`
- `codex-rs/tui/src/chatwidget.rs`
- `codex-rs/tui/src/chatwidget/tests.rs`
- shared `bottom_pane` helper modules and docs
- TUI validation and workspace blockers affecting safe refactors

## Deliverables

- Audit + WBS: [tui-decomposition-wbs.md](/Users/kooshapari/CodeProjects/Phenotype/repos/heliosCLI/docs/sessions/20260402-tui-decomposition-wbs/artifacts/tui-decomposition-wbs.md)
- Known issues: [05_KNOWN_ISSUES.md](/Users/kooshapari/CodeProjects/Phenotype/repos/heliosCLI/docs/sessions/20260402-tui-decomposition-wbs/05_KNOWN_ISSUES.md)

## Current State

- Initial TUI decomposition work has started in `chat_composer` with a new
  `bottom_pane/chat_composer_images.rs` helper module.
- The composer popup submit/completion branch now lives in
  `codex-rs/tui/src/bottom_pane/chat_composer/popup_submit.rs`, with
  `chat_composer.rs` routing the popup `Enter` and `Tab` command paths there.
- `chatwidget.rs` now delegates the low-risk popup/status runtime surfaces into
  `codex-rs/tui/src/chatwidget/model_controls/{menu.rs,reasoning.rs,state.rs}`
  and `codex-rs/tui/src/chatwidget/status.rs`.
- The committed-history insertion surface now lives in
  `codex-rs/tui/src/chatwidget/history.rs`, leaving `chatwidget.rs` as the
  runtime orchestrator around that history boundary instead of carrying the
  flush/add/session-info helpers inline.
- The turn lifecycle and event-domain bookkeeping surface now lives in
  `codex-rs/tui/src/chatwidget/turn_state.rs`, which removes task
  start/complete, plan prompt gating, rate-limit snapshot handling, warning /
  overload / startup updates, and interrupted-turn restore flow from the root
  runtime file.
- The shared `chatwidget` test harness and exec/history helper block now lives in
  `codex-rs/tui/src/chatwidget/tests/support.rs`, with `chatwidget/tests.rs` delegating to it.
- The next live `chatwidget/tests.rs` cut is now
  `codex-rs/tui/src/chatwidget/tests/status_and_queueing.rs`, which moves the
  queued-message editing, status-row restoration, and streamed follow-up queue
  regressions out of the root file.
- The prepared `chatwidget/tests/*` siblings are now wired from
  `codex-rs/tui/src/chatwidget/tests.rs`:
  `history_restore.rs`, `rate_limits.rs`, `plan_implementation.rs`,
  `plan_reasoning.rs`, `apps.rs`, `approvals_and_reasoning_popups.rs`,
  `experimental_and_model_popups.rs`, `feedback_popups.rs`, and
  `exec_and_running_turn.rs`.
- The next live `chatwidget/tests.rs` cut is now
  `codex-rs/tui/src/chatwidget/tests/submission_restore.rs`, which moves the
  blocked-image restore, queued restore, interrupted-turn resubmission, and
  placeholder remapping cases out of the root file.
- The slash / mode routing surface now lives in
  `codex-rs/tui/src/chatwidget/tests/modes_and_slash.rs`, with the related
  slash snapshots renamed to the module-qualified names.
- `codex-rs/tui/src/chatwidget/tests.rs` is now down to 4069 LOC after the
  staged wiring pass plus the `submission_restore.rs` and `modes_and_slash.rs`
  extractions.
- The next `chat_composer` runtime cut is now
  `codex-rs/tui/src/bottom_pane/chat_composer/popup_sync.rs`, which moves the
  popup synchronization and mention item plumbing out of the root composer file.
- The next `chatwidget/tests` work is no longer wiring staged files; it is
  extracting the remaining live clusters still in the root file, now starting
  with approvals / decisions and then session-history or layout residue.
- The next runtime work after `turn_state.rs` is the remaining input /
  submission / restore bridge and then narrower event-domain seams.
- The WBS artifact captures the next dependency-ordered cuts for `chat_composer`,
  `chatwidget`, `chatwidget/tests`, and shared `bottom_pane` surfaces.
- Full cargo validation remains open because targeted TUI crate builds exceeded the available shell
  execution window in this session.
