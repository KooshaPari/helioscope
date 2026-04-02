# heliosCLI TUI Decomposition WBS

## Status

Audit complete enough to sequence the next decomposition wave.

## Hotspots

- `codex-rs/tui/src/bottom_pane/chat_composer.rs`: 9.4K+ LOC
- `codex-rs/tui/src/chatwidget.rs`: 8.1K+ LOC
- `codex-rs/tui/src/chatwidget/tests.rs`: 8.6K+ LOC
- `codex-rs/tui/src/bottom_pane/footer.rs`: 1.5K+ LOC
- `codex-rs/tui/src/bottom_pane/mod.rs`: 1.7K+ LOC

## Current Local Change

- Started the composer decomposition by extracting image and attachment helper logic into
  `codex-rs/tui/src/bottom_pane/chat_composer_images.rs`.
- Continued the submission decomposition in `codex-rs/tui/src/bottom_pane/chat_composer/submission.rs`
  by centralizing draft restore-state capture and restore helpers there.
- `chat_composer.rs` now delegates pending-paste expansion, local image extraction, remote image
  row rendering, placeholder relabeling, and recent-submission image capture to that helper.
- `chat_composer.rs` also now delegates text trimming and pending-paste expansion to the shared
  `bottom_pane/text_manipulation.rs` utilities instead of carrying duplicate implementations.

## Decomposition Matrix

### `chat_composer.rs`

- Priority: `P1`
- Best split seams:
  - attachment and remote-image management
  - submission pipeline
  - mention and popup synchronization
  - voice hold-to-talk and transcription placeholders
  - input routing and history navigation
- Target layout:
  - `chat_composer_history.rs`
  - `chat_composer_images.rs`
  - `chat_composer_submission.rs`
  - `chat_composer_popups.rs`
  - `chat_composer_voice.rs`
  - later: `chat_composer_input_router.rs`
- Risks:
  - submission mutates history, mention bindings, and image attachments together
  - popup sync depends on textarea cursor state and footer mode
  - voice code is platform-gated and easier to break with incidental moves
  - input routing is the highest-churn seam and should come after smaller helper extractions
- Safest next cut:
  - finish wiring the local `chat_composer/submission.rs` lane into a tracked, compile-validated
    module and then peel popup-specific submission branches away from the generic submission prep

### `chatwidget.rs`

- Priority: `P1`
- Best split seams:
  - status/model/config UI
  - transcript/history cell orchestration
  - turn and stream lifecycle
  - event ingestion and protocol dispatch
  - composer and user-input bridge
- Target layout:
  - `chatwidget/status.rs`
  - `chatwidget/model_controls.rs`
  - `chatwidget/history.rs`
  - `chatwidget/turn_state.rs`
  - `chatwidget/input.rs`
  - `chatwidget/events/{agent_stream.rs,exec.rs,mcp.rs,replay.rs,review.rs}`
- Risks:
  - `ChatWidget` is state-heavy, so naive extraction will create giant `&mut self` helper modules
  - event handlers mutate transcript cells, bottom pane, rate-limit state, and thread lifecycle
  - the file already has extracted modules (`agent.rs`, `interrupts.rs`, `realtime.rs`,
    `skills.rs`, `session_header.rs`) and the next cuts should extend that style instead of mixing
    patterns
- Safest first cut:
  - extract the status-line and popup-control cluster into `chatwidget/status.rs` and
    `chatwidget/model_controls.rs`

### `chatwidget/tests.rs`

- Priority: `P1`
- First mandatory cut:
  - extract shared harness/builders into `chatwidget/tests/support.rs`
- Coherent test groups:
  - `history_restore.rs`
  - `rate_limits_and_plan.rs`
  - `exec_and_running_turn.rs`
  - `modes_and_slash.rs`
  - `apps_and_popups.rs`
  - `layout_snapshots.rs`
- Risks:
  - helper leakage from the current god-file pattern
  - snapshot churn if visual groups are interleaved instead of split by rendered surface
- Safest staged order:
  1. `support.rs`
  2. `history_restore.rs`
  3. `rate_limits_and_plan.rs`
  4. `apps_and_popups.rs`
  5. `exec_and_running_turn.rs`
  6. `layout_snapshots.rs`

### Shared `bottom_pane` surfaces

- Priority: `P2`
- Key targets:
  - `bottom_pane/mod.rs`
  - `footer.rs`
  - `list_selection_view.rs`
  - popup family: `command_popup.rs`, `file_search_popup.rs`, `skill_popup.rs`,
    `selection_popup_common.rs`
- Recommended boundaries:
  - `bottom_pane/mod.rs`: `container.rs`, `routing.rs`, `status_layout.rs`, `tests.rs`
  - `footer.rs`: `model.rs`, `collapse.rs`, `render.rs`, `tests.rs`
  - `list_selection_view.rs`: `model.rs`, `filtering.rs`, `render.rs`, `input.rs`, `tests.rs`
- Leave alone for now:
  - `request_user_input/`
  - `chat_composer_history.rs`
  - `paste_burst.rs`
  - `chat_composer_images.rs`
  - `scroll_state.rs`
  - `popup_consts.rs`
  - `status_line_setup.rs`
- Docs and snapshot obligations:
  - if paste-burst or composer behavior changes, keep `chat_composer.rs`, `paste_burst.rs`, and
    `docs/tui-chat-composer.md` in sync
  - keep top-of-file narrative docs aligned in `bottom_pane/mod.rs`, `footer.rs`,
    `list_selection_view.rs`, and `request_user_input/mod.rs`
  - stage snapshot-heavy splits carefully

## Dependency-Aware WBS

1. Stabilize the helper extraction pattern in `chat_composer`.
   - already in place: `chat_composer_history.rs`
   - started now: `chat_composer_images.rs`
2. Stabilize and validate the local `chat_composer/submission.rs` split.
   - depends on helper stabilization because submission must call the image helper cleanly
3. Extract `chat_composer_popups.rs`.
   - depends on step 2 because popup submission paths share prompt expansion and queue logic
4. Extract `chat_composer_voice.rs`.
   - keep late because it is OS-gated and easier once core input routing is thinner
5. Create `chatwidget/tests/support.rs`.
   - required before splitting `chatwidget/tests.rs` to avoid duplicate harness code
6. Split `chatwidget/tests.rs` by concern.
   - start with history/bootstrap support, then rate-limit and popup groups
7. Extract `chatwidget/status.rs` and `chatwidget/model_controls.rs`.
   - low behavioral risk and a good precedent for larger runtime extractions
8. Extract `chatwidget/history.rs`.
9. Extract `chatwidget/turn_state.rs` and then event-domain modules.
10. Externalize `bottom_pane/mod.rs` tests.
11. Split `footer.rs`.
12. Split `list_selection_view.rs`.
13. Repoint `command_popup` / `skill_popup` / `file_search_popup` to the cleaned popup substrate.
14. Split `bottom_pane/mod.rs` runtime into container, routing, and status layout.
15. Revisit `approval_overlay.rs` after the popup substrate stabilizes.

## Validation and Blockers

- `git diff --check` is clean for the current local TUI patch.
- File-level `rustfmt --edition 2024` passed on the touched TUI files.
- `cargo fmt --all` from `heliosCLI` is noisy because unrelated workspace state on disk pulls in
  broken external manifests.
- Targeted `cargo test` / `cargo check` runs for `codex-rs/tui` exceeded the shell execution
  window in this session before returning, so compile validation remains open.

## Recommended Next Execution

1. Finish the `chat_composer_images` extraction by moving any remaining remote-image-only methods
   into the helper or a dedicated remote-image helper.
2. Finish and validate the local `chat_composer/submission.rs` extraction with no behavior changes.
3. Add `chatwidget/tests/support.rs`.
4. Split `chatwidget/tests.rs` starting with history/bootstrap coverage.
5. Extract `chatwidget/status.rs` and `chatwidget/model_controls.rs`.
