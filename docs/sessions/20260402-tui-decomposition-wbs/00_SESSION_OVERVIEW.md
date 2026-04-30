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
- The shared `chatwidget` test harness and exec/history helper block now lives in
  `codex-rs/tui/src/chatwidget/tests/support.rs`, with `chatwidget/tests.rs` delegating to it.
- The first concern-level `chatwidget` test split is now in
  `codex-rs/tui/src/chatwidget/tests/history_restore.rs`, covering replayed history and forked
  thread restoration flows.
- The second concern-level `chatwidget` test split is now in
  `codex-rs/tui/src/chatwidget/tests/rate_limits_and_plan.rs`, covering rate-limit prompts,
  plan implementation prompts, and Plan-mode reasoning scope flows.
- The third concern-level `chatwidget` test split is now in
  `codex-rs/tui/src/chatwidget/tests/apps_and_popups.rs`, covering connector/app popup refresh
  behavior and moving the shared popup renderer into `chatwidget/tests/support.rs`.
- The fourth concern-level `chatwidget` test split is now in
  `codex-rs/tui/src/chatwidget/tests/exec_and_running_turn.rs`, covering running-turn status,
  queued-follow-up handling, exec/unified-exec history cells, and background-terminal wait
  snapshots.
- The fifth concern-level `chatwidget` test split is now in
  `codex-rs/tui/src/chatwidget/tests/layout_snapshots.rs`, isolating the small-height and
  status/modal snapshot coverage from the remaining behavioral tests.
- The sixth concern-level `chatwidget` test split is now in
  `codex-rs/tui/src/chatwidget/tests/modes_and_slash.rs`, isolating review-popup command routing,
  collaboration-mode transitions, `/init`, `/plan`, and collaboration slash-command flows from the
  remaining keyboard-history, quit, and copy behavior.
- The seventh concern-level `chatwidget` test split is now in
  `codex-rs/tui/src/chatwidget/tests/keyboard_and_commands.rs`, covering queued-message edit
  bindings, prompt-history requeue behavior, Ctrl-C/Ctrl-D exit handling, copy state, and the
  remaining simple slash commands.
- The local `chat_composer/submission.rs` lane now calls the shared
  `chat_composer_images` and `text_manipulation` helpers directly, removing the last thin
  submission-normalization wrappers from `chat_composer.rs` without crossing the popup-submit
  boundary.
- The first runtime `chatwidget` extraction is now in `codex-rs/tui/src/chatwidget/status.rs`,
  which owns footer status-line setup, branch refresh orchestration, and status-line value
  formatting while leaving live spinner/status-indicator behavior in `chatwidget.rs`.
  It also now owns rate-limit snapshot ingestion, threshold warnings, and rate-limit
  model-switch prompt triggering, plus token/context-window indicator updates and pre-review
  token-info restoration. `/status` output generation now also lives in this status module.
- The second runtime `chatwidget` extraction is now in
  `codex-rs/tui/src/chatwidget/model_controls.rs` and its child modules, which own the model
  picker, reasoning picker, personality picker, collaboration-mode picker, and rate-limit
  model-switch prompt while leaving realtime audio and permissions/sandbox flows in
  `chatwidget.rs`.
- The third runtime `chatwidget` extraction is now in `codex-rs/tui/src/chatwidget/history.rs`,
  covering active-cell flushing, session header merge behavior, plain history insertion, and
  transcript live-tail helpers.
- The fourth runtime `chatwidget` extraction is now in `codex-rs/tui/src/chatwidget/turn_state.rs`,
  covering turn start/completion cleanup, interrupted-turn restoration, turn error finalization,
  queued-message lifecycle updates, and the Plan-mode implementation prompt.
- The first event-domain `chatwidget` extraction is now in
  `codex-rs/tui/src/chatwidget/events/status_lifecycle.rs`, covering warning/status lifecycle,
  MCP startup, undo, background, deprecation, turn-diff, and stream-error handlers.
- The second event-domain `chatwidget` extraction is now in
  `codex-rs/tui/src/chatwidget/events/web_search.rs`, covering web-search begin/end active-cell
  handling without touching exec or MCP interrupt ordering.
- The third event-domain `chatwidget` extraction is now in
  `codex-rs/tui/src/chatwidget/events/patch_media.rs`, covering patch approval, patch apply
  begin/end, and view-image event handling while preserving deferred interrupt ordering.
- The fourth event-domain `chatwidget` extraction is now in
  `codex-rs/tui/src/chatwidget/events/user_requests.rs`, covering elicitation and
  `request_user_input` event routing while leaving exec and MCP interrupt-ordering clusters in
  place.
- The fifth event-domain `chatwidget` extraction is now in
  `codex-rs/tui/src/chatwidget/events/agent_stream.rs`, covering agent message deltas, plan
  streaming/completion, reasoning deltas/finalization, reasoning section breaks, plan updates, and
  stream commit tick/finalization lifecycle. It also owns final `AgentMessage` rendering and
  `AgentMessageItem` completion status restoration.
- The sixth event-domain `chatwidget` extraction is now in
  `codex-rs/tui/src/chatwidget/events/review.rs`, covering review-mode entry/exit banners,
  review output rendering, and pre-review token-info restoration.
- The seventh event-domain `chatwidget` extraction is now in
  `codex-rs/tui/src/chatwidget/events/user_messages.rs`, covering replay/live user-message
  transcript rendering and separator reset.
- `codex-rs/tui/src/chatwidget/overlays.rs` now owns feedback note, feedback consent, and app-link
  overlay opening while preserving bottom-pane view ownership in `chatwidget.rs`.
- `codex-rs/tui/src/chatwidget/connectors_popup.rs` now owns connector/app selection row
  construction and connector status/description labels while leaving popup refresh and cache state
  in `chatwidget.rs`.
- `codex-rs/tui/src/chatwidget/tests/keyboard_and_commands.rs` was split again so slash-command
  dispatch coverage now lives in `codex-rs/tui/src/chatwidget/tests/slash_commands.rs`; both files
  are under the 500-line hard limit.
- `codex-rs/tui/src/bottom_pane/mod.rs` now externalizes its inline tests to
  `codex-rs/tui/src/bottom_pane/tests.rs`, preserving the `bottom_pane::tests` module path while
  reducing the runtime module footprint.
- `codex-rs/tui/src/bottom_pane/footer.rs` is now split into
  `footer/collapse.rs`, `footer/shortcuts.rs`, `footer/tests.rs`, and
  `footer/tests/snapshots.rs`; all footer modules are under the 500-line hard limit.
- `codex-rs/tui/src/bottom_pane/list_selection_view.rs` is now split into
  `list_selection_view/layout.rs`, `list_selection_view/render.rs`,
  `list_selection_view/input.rs`, `list_selection_view/tests.rs`, and
  `list_selection_view/tests/{support,behavior}.rs`; all list-selection modules are under the
  500-line hard limit.
- `codex-rs/tui/src/bottom_pane/command_popup.rs` now externalizes its inline tests to
  `codex-rs/tui/src/bottom_pane/command_popup/tests.rs`, preserving command popup behavior
  coverage while keeping the runtime module under the hard limit.
- `command_popup`, `file_search_popup`, and `skill_popup` now render through the shared
  `selection_popup_common` menu surface and include its vertical padding in height calculations.
- `codex-rs/tui/src/bottom_pane/file_search_popup.rs` now externalizes focused tests to
  `codex-rs/tui/src/bottom_pane/file_search_popup/tests.rs`, covering shared-surface height
  padding, inset rendering, and stale-result rejection.
- `codex-rs/tui/src/bottom_pane/status_layout.rs` now owns the bottom-pane render composition,
  `Renderable` impl, and status-line forwarding methods, leaving key routing and modal lifecycle in
  `bottom_pane/mod.rs`.
- `codex-rs/tui/src/bottom_pane/routing.rs` now owns the bottom-pane key/paste routing surface:
  `handle_key_event`, `on_ctrl_c`, `handle_paste`, paste-burst checks, and active-view completion.
  `bottom_pane/mod.rs` still owns the container fields and modal construction.
- `codex-rs/tui/src/bottom_pane/chat_composer/images.rs` now owns the `ChatComposer` image-state
  method surface for local image paths, attachment insertion, recent submission images, remote
  image rows, selected remote image removal, and local placeholder relabeling.
- `codex-rs/tui/src/bottom_pane/chat_composer/submission/draft.rs` now owns submission draft
  preparation and restore-state capture/restore helpers, keeping `submission.rs` focused on the
  submission state machine and slash dispatch.
- The dirty markdown-render test split was made test-compile-safe by adding explicit child-module
  paths in `codex-rs/tui/src/markdown_render_tests.rs`, removing the ambiguous parent
  `assert_eq` macro import, and restoring the missing `#[test]` marker in
  `codex-rs/tui/src/markdown_render_tests/basics.rs`.
- The WBS artifact captures the next dependency-ordered cuts for `chat_composer`,
  `chatwidget`, `chatwidget/tests`, and shared `bottom_pane` surfaces.
- Narrow cargo validation for the TUI lane is green again:
  `CARGO_TARGET_DIR=/tmp/helioscli-tui-check3 CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo check -p codex-tui --tests`
  now passes. The latest pass includes the `history.rs`, `turn_state.rs`,
  `events/status_lifecycle.rs`, `events/web_search.rs`, `events/patch_media.rs`,
  `bottom_pane/tests.rs`, `bottom_pane/footer/*`, `bottom_pane/list_selection_view/*`,
  `bottom_pane/command_popup/tests.rs`, the shared popup-surface repoint, and
  `bottom_pane/status_layout.rs`, and `slash_commands.rs` extractions plus the non-Windows
  test-constructor cfg fix around Windows sandbox state in `codex-rs/tui/src/app.rs`.
- Targeted footer tests also pass:
  `CARGO_TARGET_DIR=/tmp/helioscli-tui-check3 CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-tui bottom_pane::footer::tests -- --nocapture`.
- Targeted list-selection tests also pass:
  `CARGO_TARGET_DIR=/tmp/helioscli-tui-check3 CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-tui list_selection_view --lib -- --nocapture`.
- Targeted command popup tests also pass:
  `CARGO_TARGET_DIR=/tmp/helioscli-tui-check3 CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-tui command_popup --lib -- --nocapture`.
- Targeted file-search popup tests also pass:
  `CARGO_TARGET_DIR=/tmp/helioscli-tui-check3 CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-tui file_search_popup --lib -- --nocapture`.
- Targeted patch/media event tests also pass:
  `CARGO_TARGET_DIR=/tmp/helioscli-tui-check3 CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-tui view_image_tool_call_adds_history_cell --lib -- --nocapture`
  and
  `CARGO_TARGET_DIR=/tmp/helioscli-tui-check3 CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-tui apply_patch_events_emit_history_cells --lib -- --nocapture`.
- Targeted bottom-pane tests also pass:
  `CARGO_TARGET_DIR=/tmp/helioscli-tui-routing2 CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-tui --lib --no-run`
  followed by direct execution of
  `/tmp/helioscli-tui-routing2/debug/deps/codex_tui-ce71fcc8ef954efa 'bottom_pane::tests::' --nocapture`.
  Direct execution passed 17 bottom-pane tests; the direct binary path avoided Cargo wrapper
  timeout/build-lock noise in this dirty checkout.
- Image-focused direct test execution also passes after compiling the lib test binary:
  `/tmp/helioscli-tui-routing2/debug/deps/codex_tui-ce71fcc8ef954efa attach_image --nocapture`,
  `/tmp/helioscli-tui-routing2/debug/deps/codex_tui-ce71fcc8ef954efa remote_image --nocapture`,
  and
  `/tmp/helioscli-tui-routing2/debug/deps/codex_tui-ce71fcc8ef954efa local_image --nocapture`.
- Submission-focused direct test execution also passes:
  `/tmp/helioscli-tui-routing2/debug/deps/codex_tui-ce71fcc8ef954efa submission --nocapture`,
  `/tmp/helioscli-tui-routing2/debug/deps/codex_tui-ce71fcc8ef954efa custom_prompt --nocapture`,
  and `/tmp/helioscli-tui-routing2/debug/deps/codex_tui-ce71fcc8ef954efa slash --nocapture`.
- After the `events/user_requests.rs` split,
  `AR=/tmp/ar-no-d CARGO_TARGET_DIR=/tmp/helioscli-tui-routing2 CARGO_BUILD_JOBS=2 CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo check -p codex-tui --tests --message-format short`
  passes, and direct test-binary filters pass for `request_user_input` and `elicitation`.
- After moving `on_rate_limit_snapshot` into `chatwidget/status.rs`, the same `cargo check` gate
  passes and the direct `rate_limit` test-binary filter passes 40 tests.
- After the `events/agent_stream.rs` split, the same `cargo check` gate passes and direct
  test-binary filters pass for `plan` and `reasoning`.
- After the `events/review.rs` split, the same `cargo check` gate passes and the direct `review`
  test-binary filter passes 17 tests.
- After moving interrupted-turn restore handling into `chatwidget/turn_state.rs`, the same
  `cargo check` gate passes and direct test-binary filters pass for `interrupted` and `queued`.
- After moving token/context-window status handling into `chatwidget/status.rs`, the same
  `cargo check` gate passes and direct test-binary filters pass for `token`, `context_window`,
  `rate_limit`, and `review`.
- After the `events/user_messages.rs` split, the same `cargo check` gate passes and direct
  test-binary filters pass for `history_restore`, `user_message`, and `realtime`.
- After moving stream commit and finalization helpers into `events/agent_stream.rs`, the same
  `cargo check` gate passes and direct test-binary filters pass for `plan`, `reasoning`, and
  `commit`.
- After moving final agent-message handlers into `events/agent_stream.rs`, the same `cargo check`
  gate passes and direct test-binary filters pass for `plan`, `reasoning`, and `message`.
- After moving `/status` output generation into `chatwidget/status.rs`, the same `cargo check`
  gate passes and direct test-binary filters pass for `status`, `token`, and `rate_limit`.
- After moving queued-message lifecycle helpers into `chatwidget/turn_state.rs`, the same
  `cargo check` gate passes and direct test-binary filters pass for `queued`, `interrupted`, and
  `message`.
- After the `chatwidget/overlays.rs` split, the same `cargo check` gate passes and direct
  test-binary filters pass for `feedback` and `app_link`.
- After the `chatwidget/connectors_popup.rs` split, the same `cargo check` gate passes and direct
  test-binary filters pass for `apps_popup` and `apps_refresh`.
