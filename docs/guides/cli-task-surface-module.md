# CLI Task Surface Module

## Purpose

`task_surface.sh` provides a standardized command surface for local developer and agent workflows.
It is a thin wrapper with explicit failure behavior and no fallback paths.

## Contract

Path: `scripts/task_surface.sh`

Usage:

```bash
scripts/task_surface.sh <lint|test|fmt|quality>
```

Supported commands:

- `lint`: Runs `cargo clippy --tests` from `codex-rs/`.
- `test`: Runs `cargo nextest run --no-fail-fast` from `codex-rs/`.
- `fmt`: Runs `cargo fmt -- --config imports_granularity=Item` from `codex-rs/`.
- `quality`: Runs `lint` then `test` sequentially.

## Failure Behavior

- Invalid command or argument count exits non-zero with usage text.
- Command failures are not suppressed.
- No fallback execution path is provided.
- Shell strict mode is enabled (`set -euo pipefail`).

## Just Integration

The `justfile` exposes wrapper-backed recipes:

- `surface-lint`
- `surface-test`
- `surface-fmt`
- `surface-quality`

Example:

```bash
just surface-quality
```

## Notes

- Existing recipes remain unchanged; these recipes add a stable task surface.
- The wrapper normalizes execution from repository root to `codex-rs/`.
