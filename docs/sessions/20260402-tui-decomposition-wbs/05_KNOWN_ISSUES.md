# Known Issues

## Current Blockers

- `cargo fmt --all` from `heliosCLI` is currently tripping over unrelated workspace/path issues on
  disk instead of just the TUI crate surface.
- Full `cargo test` remains intentionally unclaimed in this lane because it is more expensive than
  the current structural WBS requires. Narrow package compile validation is green:
  `CARGO_TARGET_DIR=/tmp/helioscli-tui-check3 CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo check -p codex-tui --tests`.
- File-level `rustfmt --edition 2024` succeeds on the touched TUI files, but the stable toolchain
  emits warnings about the repo's unstable `imports_granularity = Item` rustfmt setting.
- There is a duplicate subagent-generated session draft at
  `docs/sessions/20260402-helioscli-tui-decomposition-wbs/`; this should not remain the canonical
  audit location.
