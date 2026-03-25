# Hybrid HMR + Incremental Dev Loop

This repo now uses `process-compose` as the orchestration layer and `just` as the top-level UX.

## Profiles

- `fast`: TypeScript watch loop + Rust incremental checks.
- `full`: `fast` plus Bazel incremental checks via `ibazel`.

## Commands

- Start fast loop: `just dev-fast`
- Start full loop: `just dev-full`
- Default loop: `just dev`
- Show process list: `just dev-status profile=fast`
- Stop loop: `just dev-down profile=fast`
- Validate prerequisites only: `just dev-preflight profile=fast`

## Design

- HMR-capable lane:
  - `pnpm --dir sdk/typescript build:watch`
- Rust incremental lane:
  - `watchexec ... cargo run -p codex-cli --bin codex -- --help`
- Bazel incremental lane (full profile):
  - `ibazel build //codex-rs/cli:codex`

## Rationale

- Keep `just` as command surface for humans and agents.
- Use `process-compose` for durable, multi-process orchestration and visibility.
- Use native incremental/watch capabilities per stack instead of a one-size-fits-all shell wrapper.
