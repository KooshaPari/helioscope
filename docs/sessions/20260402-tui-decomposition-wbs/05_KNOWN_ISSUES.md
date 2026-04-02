# Known Issues

## Current Blockers

- `cargo fmt --all` from `heliosCLI` is currently tripping over unrelated workspace/path issues on
  disk instead of just the TUI crate surface.
- Long-running `cargo test` / `cargo check` validations for the TUI crate are currently hitting
  infra timeouts during compile, including exact-test and lib-only runs attempted after the
  `chatwidget/tests/support.rs` extraction, so local structural changes need narrower validation
  steps or longer-running follow-up verification.
- There is a duplicate subagent-generated session draft at
  `docs/sessions/20260402-helioscli-tui-decomposition-wbs/`; this should not remain the canonical
  audit location.
