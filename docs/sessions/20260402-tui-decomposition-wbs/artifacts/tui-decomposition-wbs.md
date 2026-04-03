# heliosCLI TUI Decomposition WBS

## Status

Audit complete enough to sequence the next decomposition wave.

## Hotspots

- `codex-rs/tui/src/bottom_pane/chat_composer.rs`: 8.5K+ LOC
- `codex-rs/tui/src/chatwidget.rs`: 7.6K+ LOC
- `codex-rs/tui/src/chatwidget/tests.rs`: 4.9K+ LOC
- `codex-rs/tui/src/bottom_pane/footer.rs`: 1.5K+ LOC
- `codex-rs/tui/src/bottom_pane/mod.rs`: 1.7K+ LOC

## Current Local Change

- Started the composer decomposition by extracting image and attachment helper logic into
  `codex-rs/tui/src/bottom_pane/chat_composer_images.rs`.
- Continued the composer split by extracting popup-specific slash completion and
  submit handling into `codex-rs/tui/src/bottom_pane/chat_composer/popup_submit.rs`.
- Continued the submission decomposition in `codex-rs/tui/src/bottom_pane/chat_composer/submission.rs`
  by centralizing draft restore-state capture and restore helpers there.
- Extracted the committed-history insertion surface into
  `codex-rs/tui/src/chatwidget/history.rs`.
- Extracted the turn lifecycle and event-domain bookkeeping surface into
  `codex-rs/tui/src/chatwidget/turn_state.rs`.
- Extracted the reusable `chatwidget` test harness, rate-limit fixtures, and exec/history helper
  block into `codex-rs/tui/src/chatwidget/tests/support.rs` so future `chatwidget/tests/*` splits
  can share one stable helper surface.
- Split the queued-message and status-row regression cluster into
  `codex-rs/tui/src/chatwidget/tests/status_and_queueing.rs`.
- Wired the prepared `chatwidget/tests/*` sibling modules into
  `codex-rs/tui/src/chatwidget/tests.rs` and pruned the duplicate root-file
  bodies for `history_restore.rs`, `rate_limits.rs`, `plan_implementation.rs`,
  `plan_reasoning.rs`, `apps.rs`, `approvals_and_reasoning_popups.rs`,
  `experimental_and_model_popups.rs`, `feedback_popups.rs`, and
  `exec_and_running_turn.rs`.
- Extracted the next live restore-focused test seam into
  `codex-rs/tui/src/chatwidget/tests/submission_restore.rs`.
- Extracted the slash / mode switching and slash-command regression cluster into
  `codex-rs/tui/src/chatwidget/tests/modes_and_slash.rs`.
- Extracted popup synchronization and mention popup plumbing into
  `codex-rs/tui/src/bottom_pane/chat_composer/popup_sync.rs`.
- Fixed duplicated module wiring in `codex-rs/tui/src/chatwidget/model_controls/mod.rs`.
- `chat_composer.rs` now delegates pending-paste expansion, local image extraction, remote image
  row rendering, placeholder relabeling, and recent-submission image capture to that helper.
- `chat_composer.rs` also now delegates text trimming and pending-paste expansion to the shared
  `bottom_pane/text_manipulation.rs` utilities instead of carrying duplicate implementations.

## Full Remaining Work Audit

### Lane A: `chat_composer` family

- Remaining seams:
  - mention token detection, insertion, and binding restore
  - voice / hold-to-talk / transcription placeholder flow
  - high-churn input routing and history navigation
- Recommended module targets:
  - `chat_composer_mentions.rs`
  - `chat_composer_voice.rs`
  - `chat_composer_input_router.rs`
- Dependency order:
  1. mention binding helpers after `popup_sync.rs`
  2. voice surface
  3. input router / history navigation
- Key risks:
  - popup state depends on textarea cursor state and footer mode
  - mention bindings, pending pastes, and image attachments interact during restore/submit
  - voice path is platform-gated and should stay isolated from popup work

### Lane B: `chatwidget` runtime family

- Remaining seams:
  - event-domain handling for exec / MCP / review / replay / agent stream
  - user-input bridge and history-submission / restore path
  - transcript overlay helpers and active-cell revision lifecycle
- Recommended module targets:
  - `chatwidget/events/{exec.rs,mcp.rs,review.rs,replay.rs,agent_stream.rs}`
  - `chatwidget/input.rs`
  - later, if still needed: a narrow transcript-overlay helper surface
- Dependency order:
  1. input / submission / restore bridge
  2. event-domain modules
  3. overlay-specific helpers only if the root file still stays oversized
- Key risks:
  - active-cell flush and revision behavior must stay coherent across history and turn-state seams
  - event handlers cross-cut transcript cells, bottom pane, rate-limit state, and thread lifecycle
  - review mode and replay paths can create subtle ordering regressions when split too early

### Lane C: `chatwidget/tests` family

- Remaining work is two kinds:
  - continue new concern splits from the remaining monolith
- Live wired modules:
  - `support.rs`
  - `history_restore.rs`
  - `submission_restore.rs`
  - `modes_and_slash.rs`
  - `rate_limits.rs`
  - `plan_implementation.rs`
  - `plan_reasoning.rs`
  - `apps.rs`
  - `approvals_and_reasoning_popups.rs`
  - `experimental_and_model_popups.rs`
  - `feedback_popups.rs`
  - `exec_and_running_turn.rs`
  - `status_and_queueing.rs`
- Remaining new split targets still inside `tests.rs`:
  - approval modal and history-decision cluster
  - layout / snapshot-only cluster
  - forked-thread / reconnect / session-history cluster
- Recommended module targets:
  - `approvals_and_decisions.rs`
  - `layout_snapshots.rs`
  - `session_history.rs`
- Dependency order:
  1. split approvals / decisions
  2. split session-history / reconnect residue
  3. finish layout / snapshot residue last
- Key risks:
  - snapshot names can drift if the module is wired after content changes
  - support helpers can regrow into a second monolith if each split leaks more harness code
  - the root file can still hide multiple coherent seams even after the prepared-module pass, so
    future cuts should stay concern-shaped rather than line-count-driven

### Lane D: shared `bottom_pane` family

- Remaining seams:
  - `bottom_pane/mod.rs` routing / container / status layout
  - `footer.rs` render / collapse / shortcut composition
  - `list_selection_view.rs` model / filtering / render / input
  - popup substrate unification across command / file search / skill / selection common
  - `approval_overlay.rs` after popup substrate stabilizes
- Recommended module targets:
  - `bottom_pane/{container.rs,routing.rs,status_layout.rs,tests.rs}`
  - `footer/{render.rs,collapse.rs,tests.rs}`
  - `list_selection_view/{model.rs,filtering.rs,render.rs,input.rs,tests.rs}`
  - popup-surface cleanup in existing popup modules before creating more files
- Dependency order:
  1. footer
  2. list selection view
  3. bottom pane root runtime split
  4. popup substrate cleanup
  5. approval overlay last
- Key risks:
  - footer and bottom-pane status rendering are tightly coupled to narrow-width behavior
  - popup family cleanup can create duplicate abstractions if done before the composer popup lane settles
  - list-selection snapshots are dense and should be staged in one pass

### Lane E: docs and validation family

- Remaining work:
  - keep `docs/sessions/20260402-tui-decomposition-wbs/*` aligned with live module wiring
  - keep `docs/tui-chat-composer.md` aligned whenever composer behavior changes
  - remove stale snapshot names when split modules are actually wired
  - convert validation from ad hoc timeouts into a repeatable narrow-probe sequence
- Validation backlog:
  - exact-test probes for each newly wired snapshot module
  - one or two higher-value lib probes after several seams land
  - longer-running compile warm-up outside the short shell window when needed
- Current blocker:
  - compile-bound cargo runs still time out before proving the TUI crate green

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
  - with `popup_submit.rs` and `popup_sync.rs` in place, peel the remaining
    mention-binding and input-router branches away from `chat_composer.rs`

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
  - `chatwidget/model_controls/{menu.rs,reasoning.rs,state.rs}`
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
  - `chatwidget/status.rs`, `chatwidget/model_controls/{menu.rs,reasoning.rs,state.rs}`,
    `chatwidget/history.rs`, and `chatwidget/turn_state.rs` are now live; the
    next runtime cut should be the input / submission / restore bridge or the
    next focused event-domain seam

### `chatwidget/tests.rs`

- Priority: `P1`
- First mandatory cut:
  - extracted shared harness/builders into `chatwidget/tests/support.rs`
- Important live gap:
  - the staged split modules are wired now, but `tests.rs` still contains the
    remaining approval decision, session-history, and layout residue clusters
- Coherent test groups:
  - `status_and_queueing.rs`
  - `history_restore.rs`
  - `rate_limits.rs`
  - `plan_implementation.rs`
  - `plan_reasoning.rs`
  - `exec_and_running_turn.rs`
  - `modes_and_slash.rs`
  - `apps.rs`
  - `approvals_and_reasoning_popups.rs`
  - `experimental_and_model_popups.rs`
  - `feedback_popups.rs`
  - `submission_restore.rs`
  - `modes_and_slash.rs`
  - `approvals_and_decisions.rs`
  - `session_history.rs`
  - `layout_snapshots.rs`
- Risks:
  - helper leakage from the current god-file pattern
  - snapshot churn if visual groups are interleaved instead of split by rendered surface
- Safest staged order:
  1. split `approvals_and_decisions.rs`
  2. split `session_history.rs`
  3. finish `layout_snapshots.rs`

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
   - completed; future test splits should import from the shared helper surface
6. Split `chatwidget/tests.rs` by concern.
  - status/queueing cluster completed in `status_and_queueing.rs`
  - prepared sibling wiring and duplicate pruning completed for the staged
    `history_restore`, `rate_limits`, `plan_implementation`, `plan_reasoning`,
    `apps`, `approvals_and_reasoning_popups`, `experimental_and_model_popups`,
    `feedback_popups`, and `exec_and_running_turn` modules
  - `submission_restore.rs` now owns the blocked-image restore, queued restore,
    interrupted-turn resubmission, and placeholder remapping cases
  - `modes_and_slash.rs` now owns the slash / mode switching and slash-command
    routing regressions
  - the next new split should be approvals / decisions, then session-history
7. Extract `chatwidget/status.rs` and `chatwidget/model_controls.rs`.
   - low behavioral risk and a good precedent for larger runtime extractions
8. Extract `chatwidget/history.rs`.
   - completed; committed-history insertion now lives in `chatwidget/history.rs`
9. Extract `chatwidget/turn_state.rs` and then event-domain modules.
10. Continue the `chatwidget/tests.rs` tail with slash/mode, approvals, and session-history cuts.
11. Externalize `bottom_pane/mod.rs` tests.
12. Split `footer.rs`.
13. Split `list_selection_view.rs`.
14. Repoint `command_popup` / `skill_popup` / `file_search_popup` to the cleaned popup substrate.
15. Split `bottom_pane/mod.rs` runtime into container, routing, and status layout.
16. Revisit `approval_overlay.rs` after the popup substrate stabilizes.

## Validation and Blockers

- `git diff --check` is clean for the current local TUI patch.
- File-level `rustfmt --edition 2024` passed on the touched TUI files.
- `cargo fmt --all` from `heliosCLI` is noisy because unrelated workspace state on disk pulls in
  broken external manifests.
- Targeted `cargo test` / `cargo check` runs for `codex-rs/tui` still do not
  return a clean fast verdict in this environment: narrow exact-test probes time
  out, and at least one broader `cargo check` run was killed by a signal, so
  compile validation remains open.

## Recommended Next Execution

1. Validate the live `submission_restore.rs`, `modes_and_slash.rs`,
   `turn_state.rs`, and `popup_sync.rs`
   splits with fast hygiene and a narrow cargo probe.
2. Split the approvals / decisions cluster out of `chatwidget/tests.rs`.
3. Extract the `chatwidget` input / submission / restore bridge or the next
   focused event-domain seam now that `turn_state.rs` is live.
4. Peel the remaining mention-binding and input-router branches away from
   `chat_composer.rs`.
5. Split the next session-history or layout residue out of `chatwidget/tests.rs`.
