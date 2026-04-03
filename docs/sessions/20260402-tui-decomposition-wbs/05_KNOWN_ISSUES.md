# Known Issues

## Current Blockers

- `cargo fmt --all` from `heliosCLI` is currently tripping over unrelated workspace/path issues on
  disk instead of just the TUI crate surface.
- Long-running `cargo test` / `cargo check` validations for the TUI crate are currently hitting
  infra timeouts during compile, including exact-test and lib-only runs attempted after the
  `chatwidget/tests/support.rs`, `chatwidget/tests/status_and_queueing.rs`,
  `chat_composer/popup_submit.rs`, `chatwidget/status.rs`, and
  `chatwidget/model_controls/{menu.rs,reasoning.rs,state.rs}` extractions, so
  local structural changes need narrower validation steps or longer-running
  follow-up verification.
- The prepared `chatwidget/tests/*` split modules are now wired, but the root
  `chatwidget/tests.rs` file is still large and carries the remaining live
  unsplit clusters such as approval decisions, session-history residue, and
  layout / snapshot residue.
- Crate-level validation is still environment-bound even after the latest
  `turn_state.rs`, `submission_restore.rs`, and `popup_sync.rs` cuts: narrow
  `cargo test` probes still time out, and at least one broader `cargo check`
  run was killed by a signal before proving the TUI crate green.
- There is a duplicate subagent-generated session draft at
  `docs/sessions/20260402-helioscli-tui-decomposition-wbs/`; this should not remain the canonical
  audit location.
