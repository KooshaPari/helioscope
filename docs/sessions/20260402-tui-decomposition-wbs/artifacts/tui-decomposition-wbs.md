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
- Extracted the reusable `chatwidget` test harness, rate-limit fixtures, and exec/history helper
  block into `codex-rs/tui/src/chatwidget/tests/support.rs` so future `chatwidget/tests/*` splits
  can share one stable helper surface.
- Extracted replayed-history and forked-thread coverage into
  `codex-rs/tui/src/chatwidget/tests/history_restore.rs`, leaving `chatwidget/tests.rs` smaller and
  making the remaining concern groups easier to peel off.
- Extracted rate-limit and Plan-mode prompt coverage into
  `codex-rs/tui/src/chatwidget/tests/rate_limits_and_plan.rs`, reducing
  `chatwidget/tests.rs` to about 7.0K LOC.
- Extracted the connectors/apps popup refresh coverage into
  `codex-rs/tui/src/chatwidget/tests/apps_and_popups.rs` and moved the shared
  `render_bottom_popup` helper into `codex-rs/tui/src/chatwidget/tests/support.rs`.
- Extracted the running-turn and exec/unified-exec coverage into
  `codex-rs/tui/src/chatwidget/tests/exec_and_running_turn.rs`, reducing
  `chatwidget/tests.rs` to about 5.6K LOC and isolating the keyboard-history/quit tests as a
  separate remaining concern.
- Extracted the snapshot-only layout/status modal coverage into
  `codex-rs/tui/src/chatwidget/tests/layout_snapshots.rs`, reducing
  `chatwidget/tests.rs` to about 5.5K LOC and leaving the remaining behavioral surface centered on
  slash/mode flows plus keyboard-history/quit handling.
- Extracted the remaining slash/mode behavior cluster into
  `codex-rs/tui/src/chatwidget/tests/modes_and_slash.rs`, reducing `chatwidget/tests.rs` to about
  5.0K LOC and leaving the smaller keyboard-history, quit, and copy tail as the main remaining
  test-side split candidate.
- Extracted the keyboard and simple command tail into
  `codex-rs/tui/src/chatwidget/tests/keyboard_and_commands.rs`, reducing `chatwidget/tests.rs` to
  about 4.5K LOC.
- `chat_composer.rs` now delegates pending-paste expansion, local image extraction, remote image
  row rendering, placeholder relabeling, and recent-submission image capture to that helper.
- `chat_composer.rs` also now delegates text trimming and pending-paste expansion to the shared
  `bottom_pane/text_manipulation.rs` utilities instead of carrying duplicate implementations.
- `chat_composer/submission.rs` now calls `chat_composer_images` and `text_manipulation`
  directly, removing the last thin submission wrappers from `chat_composer.rs` while leaving the
  popup `Enter` submit branch in place for a later popup-specific cut.
- Extracted the footer status-line orchestration into `codex-rs/tui/src/chatwidget/status.rs`,
  reducing `chatwidget.rs` to about 7.8K LOC while leaving live spinner/status-indicator behavior
  in the main file for a later turn-state split.
- Extracted the model-selection and reasoning/control popup cluster into the
  `codex-rs/tui/src/chatwidget/model_controls.rs` module family, reducing `chatwidget.rs` to about
  7.0K LOC while leaving realtime audio and permissions/sandbox flows in the main file.
- Extracted active-cell history insertion, session-header merge, and transcript live-tail helpers
  into `codex-rs/tui/src/chatwidget/history.rs`.
- Extracted turn lifecycle cleanup and Plan implementation prompt handling into
  `codex-rs/tui/src/chatwidget/turn_state.rs`, reducing `chatwidget.rs` to about 6.7K LOC.
- Extracted patch approval, patch apply begin/end, and view-image event handling into
  `codex-rs/tui/src/chatwidget/events/patch_media.rs`, keeping patch success/failure history and
  deferred interrupt ordering unchanged.
- Extracted feedback/app-link overlay opening into `codex-rs/tui/src/chatwidget/overlays.rs`,
  keeping the bottom-pane view constructors and redraw behavior unchanged.
- Extracted connector/app popup row construction into
  `codex-rs/tui/src/chatwidget/connectors_popup.rs`, keeping connector cache refresh and popup
  chrome in `chatwidget.rs`.
- Externalized `codex-rs/tui/src/bottom_pane/command_popup.rs` inline tests to
  `codex-rs/tui/src/bottom_pane/command_popup/tests.rs`.
- Repointed the command, file-search, and skill popup render/height surface layer to the shared
  `selection_popup_common` menu surface without changing popup input routing or selection actions.
- Added focused file-search popup tests in `codex-rs/tui/src/bottom_pane/file_search_popup/tests.rs`
  for shared-surface height padding, inset rendering, and stale-result rejection.
- Extracted bottom-pane render/status layout into `codex-rs/tui/src/bottom_pane/status_layout.rs`,
  moving `as_renderable`, the `Renderable` impl, and status-line forwarding methods without
  changing key routing, modal lifecycle, or composer submission behavior.
- Extracted bottom-pane key/paste routing into `codex-rs/tui/src/bottom_pane/routing.rs`, moving
  `handle_key_event`, `on_ctrl_c`, `handle_paste`, paste-burst checks, and active-view completion
  without changing modal construction or container ownership.
- Extracted the `ChatComposer` image-state method surface into
  `codex-rs/tui/src/bottom_pane/chat_composer/images.rs`, leaving the pure helper functions in
  `chat_composer_images.rs` and keeping caller-facing method names unchanged.
- Extracted submission draft preparation and restore-state helpers into
  `codex-rs/tui/src/bottom_pane/chat_composer/submission/draft.rs`, reducing
  `submission.rs` below the 350-line target while preserving slash dispatch and submission
  behavior.
- Repaired the dirty markdown-render test split enough for TUI test-target compilation by adding
  explicit module paths in `codex-rs/tui/src/markdown_render_tests.rs`, avoiding an ambiguous
  parent `assert_eq` macro import, and restoring the missing `#[test]` marker for the empty
  markdown case.

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
  - extract the status-line cluster into `chatwidget/status.rs`
  - then take popup-control/model selection into `chatwidget/model_controls.rs`

### `chatwidget/tests.rs`

- Priority: `P1`
- First mandatory cut:
  - extracted shared harness/builders into `chatwidget/tests/support.rs`
- Coherent test groups:
  - `history_restore.rs`
  - `rate_limits_and_plan.rs`
  - `apps_and_popups.rs`
  - `exec_and_running_turn.rs`
  - `modes_and_slash.rs`
  - `layout_snapshots.rs`
  - `keyboard_and_commands.rs`
- Risks:
  - helper leakage from the current god-file pattern
  - snapshot churn if visual groups are interleaved instead of split by rendered surface
- Safest staged order:
1. `history_restore.rs` completed
  2. `rate_limits_and_plan.rs` completed
  3. `apps_and_popups.rs` completed
  4. `exec_and_running_turn.rs` completed
  5. `layout_snapshots.rs` completed
  6. `modes_and_slash.rs` completed
  7. `keyboard_and_commands.rs` completed

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
   - pure helpers are in `chat_composer_images.rs`
   - `ChatComposer` image-state methods are now in `chat_composer/images.rs`
2. Stabilize and validate the local `chat_composer/submission.rs` split.
   - depends on helper stabilization because submission must call the image helper cleanly
   - thin-wrapper cleanup completed
   - `submission/draft.rs` completed for draft prep and restore-state helpers
   - popup `Enter` submit path still anchors the remaining seam
3. Extract `chat_composer_popups.rs`.
   - depends on step 2 because popup submission paths share prompt expansion and queue logic
4. Extract `chat_composer_voice.rs`.
   - keep late because it is OS-gated and easier once core input routing is thinner
5. Create `chatwidget/tests/support.rs`.
   - completed; now also owns the shared `render_bottom_popup` helper
6. Split `chatwidget/tests.rs` by concern.
   - history/bootstrap, rate-limit/plan, and apps popup groups are now extracted
7. Extract `chatwidget/status.rs` and `chatwidget/model_controls.rs`.
   - `status.rs` completed
   - `status.rs` now also owns rate-limit snapshot ingestion
   - `status.rs` now also owns token/context-window indicator updates and pre-review token
     restoration
   - `status.rs` now also owns `/status` output generation
   - `model_controls.rs` completed
8. Extract `chatwidget/history.rs`.
   - completed
9. Extract `chatwidget/turn_state.rs` and then event-domain modules.
   - `turn_state.rs` completed and now also owns interrupted-turn queued-draft restoration plus
     queued-message lifecycle updates
   - `events/agent_stream.rs` completed for agent message deltas, final agent messages, plan
     streaming, reasoning, plan updates, and stream commit/finalization lifecycle
   - `events/review.rs` completed for review-mode entry, exit, and output rendering
   - `events/status_lifecycle.rs` completed
   - `events/web_search.rs` completed
   - `events/patch_media.rs` completed
   - `events/user_requests.rs` completed for elicitation and `request_user_input` routing
   - `events/user_messages.rs` completed for replay/live user-message transcript rendering
   - `overlays.rs` completed for feedback note, feedback consent, and app-link overlay opening
   - `connectors_popup.rs` completed for connector/app selection row construction and labels
10. Externalize `bottom_pane/mod.rs` tests.
   - completed as `bottom_pane/tests.rs`
11. Split `footer.rs`.
   - completed as `footer/collapse.rs`, `footer/shortcuts.rs`, `footer/tests.rs`, and
     `footer/tests/snapshots.rs`
12. Split `list_selection_view.rs`.
   - completed as `list_selection_view/layout.rs`, `list_selection_view/render.rs`,
     `list_selection_view/input.rs`, `list_selection_view/tests.rs`, and
     `list_selection_view/tests/{support,behavior}.rs`
13. Repoint `command_popup` / `skill_popup` / `file_search_popup` to the cleaned popup substrate.
   - completed for the shared menu-surface render and vertical height-padding layer
   - `command_popup.rs` inline tests were externalized to `command_popup/tests.rs`
   - `file_search_popup.rs` now has focused surface/height/stale-result tests in
     `file_search_popup/tests.rs`
   - full `ListSelectionView` conversion remains intentionally deferred because popup input routing
     and side effects still live in `chat_composer.rs`
14. Split `bottom_pane/mod.rs` runtime into container, routing, and status layout.
   - `status_layout.rs` completed for render composition and status-line forwarding
   - `routing.rs` completed for Esc/Ctrl-C, paste, paste-burst, and active-view completion
   - container split remains deferred because moving `BottomPane` / `BottomPaneParams` would force
     field visibility and import churn across the extracted modules
15. Revisit `approval_overlay.rs` after the popup substrate stabilizes.

## Validation and Blockers

- `git diff --check` is clean for the current local TUI patch.
- File-level `rustfmt --edition 2024` passed on the touched TUI files.
- `cargo fmt --all` from `heliosCLI` is noisy because unrelated workspace state on disk pulls in
  broken external manifests.
- `rustfmt --edition 2024` passed on `tui/src/chatwidget/tests.rs` and
  `tui/src/chatwidget/tests/rate_limits_and_plan.rs`.
- `git diff --check` passed on the extracted test files.
- `rustfmt --edition 2024` also passed on `tui/src/chatwidget/tests/support.rs` and
  `tui/src/chatwidget/tests/apps_and_popups.rs`.
- `cargo check -p codex-tui --tests` was attempted with
  `CARGO_TARGET_DIR=/tmp/helioscli-tui-check CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0'`.
  That failure class has been cleared for the current decomposition lane.
- `cargo check -p codex-tui --tests` now passes with
  `CARGO_TARGET_DIR=/tmp/helioscli-tui-check3 CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0'`.
  A non-Windows test-constructor cfg mismatch in `tui/src/app.rs` was fixed during the latest
  validation pass so the package test compile now exits cleanly. The same gate also passed after
  extracting `chatwidget/history.rs`, `chatwidget/turn_state.rs`,
  `chatwidget/events/status_lifecycle.rs`, `chatwidget/events/web_search.rs`,
  `chatwidget/events/patch_media.rs`, `bottom_pane/tests.rs`, `bottom_pane/footer/*`,
  `bottom_pane/list_selection_view/*`, `bottom_pane/command_popup/tests.rs`, the shared popup
  surface repoint, `bottom_pane/status_layout.rs`, the markdown-render test split repair, and
  `chatwidget/tests/slash_commands.rs`.
- `cargo test -p codex-tui bottom_pane::footer::tests -- --nocapture` passes with
  `CARGO_TARGET_DIR=/tmp/helioscli-tui-check3 CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0'`.
- `cargo test -p codex-tui list_selection_view --lib -- --nocapture` passes with
  `CARGO_TARGET_DIR=/tmp/helioscli-tui-check3 CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0'`.
- `cargo test -p codex-tui command_popup --lib -- --nocapture` passes with
  `CARGO_TARGET_DIR=/tmp/helioscli-tui-check3 CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0'`.
- `cargo test -p codex-tui skill_popup --lib -- --nocapture` passes with
  `CARGO_TARGET_DIR=/tmp/helioscli-tui-check3 CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0'`.
- `cargo test -p codex-tui file_search_popup --lib -- --nocapture` passes with
  `CARGO_TARGET_DIR=/tmp/helioscli-tui-check3 CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0'`.
- `cargo test -p codex-tui view_image_tool_call_adds_history_cell --lib -- --nocapture` and
  `cargo test -p codex-tui apply_patch_events_emit_history_cells --lib -- --nocapture` pass with
  `CARGO_TARGET_DIR=/tmp/helioscli-tui-check3 CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0'`.
- `cargo test -p codex-tui bottom_pane::tests --lib -- --nocapture` passes with
  `CARGO_TARGET_DIR=/tmp/helioscli-tui-check3 CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0'`.
- After the routing split, `cargo test -p codex-tui --lib --no-run --message-format short` passes
  with `AR=/tmp/ar-no-d CARGO_TARGET_DIR=/tmp/helioscli-tui-routing2 CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0'`.
  Direct execution of
  `/tmp/helioscli-tui-routing2/debug/deps/codex_tui-ce71fcc8ef954efa 'bottom_pane::tests::' --nocapture`
  passes 17 bottom-pane tests. The `AR=/tmp/ar-no-d` wrapper strips Apple-unsupported deterministic
  archive flags from C build scripts in this local toolchain.
- After the `chat_composer/images.rs` split, `cargo check -p codex-tui --tests --message-format short`
  and `cargo test -p codex-tui --lib --no-run --message-format short` pass with
  `AR=/tmp/ar-no-d CARGO_TARGET_DIR=/tmp/helioscli-tui-routing2 CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0'`.
  Direct test-binary filters also pass for `attach_image`, `remote_image`, and `local_image`.
- After the `submission/draft.rs` split, direct test-binary filters pass for `submission`,
  `custom_prompt`, and `slash` using
  `/tmp/helioscli-tui-routing2/debug/deps/codex_tui-ce71fcc8ef954efa`.
- After the `events/user_requests.rs` split,
  `AR=/tmp/ar-no-d CARGO_TARGET_DIR=/tmp/helioscli-tui-routing2 CARGO_BUILD_JOBS=2 CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo check -p codex-tui --tests --message-format short`
  passes. Direct execution of
  `/tmp/helioscli-tui-routing2/debug/deps/codex_tui-ce71fcc8ef954efa request_user_input --nocapture`
  passes 57 tests, and
  `/tmp/helioscli-tui-routing2/debug/deps/codex_tui-ce71fcc8ef954efa elicitation --nocapture`
  passes the elicitation replay test.
- After moving `on_rate_limit_snapshot` into `chatwidget/status.rs`, the same `cargo check` gate
  passes and direct execution of
  `/tmp/helioscli-tui-routing2/debug/deps/codex_tui-ce71fcc8ef954efa rate_limit --nocapture`
  passes 40 tests.
- After the `events/agent_stream.rs` split, the same `cargo check` gate passes. Direct execution of
  `/tmp/helioscli-tui-routing2/debug/deps/codex_tui-ce71fcc8ef954efa plan --nocapture`
  passes 48 tests, and
  `/tmp/helioscli-tui-routing2/debug/deps/codex_tui-ce71fcc8ef954efa reasoning --nocapture`
  passes 28 tests.
- After the `events/review.rs` split, the same `cargo check` gate passes and direct execution of
  `/tmp/helioscli-tui-routing2/debug/deps/codex_tui-ce71fcc8ef954efa review --nocapture`
  passes 17 tests.
- After moving interrupted-turn restore handling into `chatwidget/turn_state.rs`, the same
  `cargo check` gate passes. Direct execution of
  `/tmp/helioscli-tui-routing2/debug/deps/codex_tui-ce71fcc8ef954efa interrupted --nocapture`
  passes 5 tests, and
  `/tmp/helioscli-tui-routing2/debug/deps/codex_tui-ce71fcc8ef954efa queued --nocapture`
  passes 26 tests.
- After moving token/context-window status handling into `chatwidget/status.rs`, the same
  `cargo check` gate passes. Direct execution of
  `/tmp/helioscli-tui-routing2/debug/deps/codex_tui-ce71fcc8ef954efa token --nocapture`
  passes 31 tests,
  `/tmp/helioscli-tui-routing2/debug/deps/codex_tui-ce71fcc8ef954efa context_window --nocapture`
  passes 2 tests,
  `/tmp/helioscli-tui-routing2/debug/deps/codex_tui-ce71fcc8ef954efa rate_limit --nocapture`
  passes 40 tests, and
  `/tmp/helioscli-tui-routing2/debug/deps/codex_tui-ce71fcc8ef954efa review --nocapture`
  passes 17 tests.
- After the `events/user_messages.rs` split, the same `cargo check` gate passes. Direct execution of
  `/tmp/helioscli-tui-routing2/debug/deps/codex_tui-ce71fcc8ef954efa history_restore --nocapture`
  passes 7 tests,
  `/tmp/helioscli-tui-routing2/debug/deps/codex_tui-ce71fcc8ef954efa user_message --nocapture`
  passes 18 tests, and
  `/tmp/helioscli-tui-routing2/debug/deps/codex_tui-ce71fcc8ef954efa realtime --nocapture`
  passes 6 tests.
- After moving stream commit and finalization helpers into `events/agent_stream.rs`, the same
  `cargo check` gate passes. Direct execution of
  `/tmp/helioscli-tui-routing2/debug/deps/codex_tui-ce71fcc8ef954efa plan --nocapture`
  passes 48 tests,
  `/tmp/helioscli-tui-routing2/debug/deps/codex_tui-ce71fcc8ef954efa reasoning --nocapture`
  passes 28 tests, and
  `/tmp/helioscli-tui-routing2/debug/deps/codex_tui-ce71fcc8ef954efa commit --nocapture`
  passes 11 tests.
- After moving final agent-message handlers into `events/agent_stream.rs`, the same `cargo check`
  gate passes. Direct execution of
  `/tmp/helioscli-tui-routing2/debug/deps/codex_tui-ce71fcc8ef954efa plan --nocapture`
  passes 48 tests,
  `/tmp/helioscli-tui-routing2/debug/deps/codex_tui-ce71fcc8ef954efa reasoning --nocapture`
  passes 28 tests, and
  `/tmp/helioscli-tui-routing2/debug/deps/codex_tui-ce71fcc8ef954efa message --nocapture`
  passes 52 tests.
- After moving `/status` output generation into `chatwidget/status.rs`, the same `cargo check`
  gate passes. Direct execution of
  `/tmp/helioscli-tui-routing2/debug/deps/codex_tui-ce71fcc8ef954efa status --nocapture`
  passes 55 tests,
  `/tmp/helioscli-tui-routing2/debug/deps/codex_tui-ce71fcc8ef954efa token --nocapture`
  passes 31 tests, and
  `/tmp/helioscli-tui-routing2/debug/deps/codex_tui-ce71fcc8ef954efa rate_limit --nocapture`
  passes 40 tests.
- After moving queued-message lifecycle helpers into `chatwidget/turn_state.rs`, the same
  `cargo check` gate passes. Direct execution of
  `/tmp/helioscli-tui-routing2/debug/deps/codex_tui-ce71fcc8ef954efa queued --nocapture`
  passes 26 tests,
  `/tmp/helioscli-tui-routing2/debug/deps/codex_tui-ce71fcc8ef954efa interrupted --nocapture`
  passes 5 tests, and
  `/tmp/helioscli-tui-routing2/debug/deps/codex_tui-ce71fcc8ef954efa message --nocapture`
  passes 52 tests.
- After the `chatwidget/overlays.rs` split, the same `cargo check` gate passes. Direct execution of
  `/tmp/helioscli-tui-routing2/debug/deps/codex_tui-ce71fcc8ef954efa feedback --nocapture`
  passes 8 tests, and
  `/tmp/helioscli-tui-routing2/debug/deps/codex_tui-ce71fcc8ef954efa app_link --nocapture`
  passes 4 tests.
- After the `chatwidget/connectors_popup.rs` split, the same `cargo check` gate passes. Direct
  execution of
  `/tmp/helioscli-tui-routing2/debug/deps/codex_tui-ce71fcc8ef954efa apps_popup --nocapture`
  and
  `/tmp/helioscli-tui-routing2/debug/deps/codex_tui-ce71fcc8ef954efa apps_refresh --nocapture`
  passes.

## Recommended Next Execution

1. Defer the `bottom_pane` container split until enough field access can be reduced without broad
   visibility churn.
2. Continue with the remaining non-exec `chatwidget` seams, with permissions popups and model-state
   helpers preferred before any MCP/exec interrupt-ordering extraction.
