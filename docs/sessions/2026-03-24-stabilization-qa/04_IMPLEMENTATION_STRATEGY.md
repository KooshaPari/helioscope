# Implementation Strategy: heliosCLI Stabilization Session

## 1. Technical Approach

### 1.1 Multi-Stack Strategy
- JavaScript/TypeScript: pnpm workspace with Prettier
- Rust: Cargo workspaces (codex-rs, helios-rs)
- Unified quality gates across both stacks

### 1.2 Tool Selection
- **Prettier**: JS/TS formatting (root-level configuration)
- **rustfmt**: Rust formatting (nightly compatible)
- **clippy**: Rust linting (strict configuration)
- **pnpm**: Package management for monorepo

## 1.3 Current Execution Model

- Treat `docs/WORKLOG.md` as the live lane ledger for branch/worktree status.
- Keep `refactor/decouple-harness-crates` as the active root checkout unless a lane explicitly
  requires a new worktree.
- Reconcile or prune stale worktree metadata before starting new parallel lanes.

## 2. Architecture Decisions

### 2.1 Workspace Structure
```
heliosCLI/
├── packages/          # JS/TS packages
├── codex-rs/          # Rust workspace 1
├── helios-rs/         # Rust workspace 2
└── scripts/          # Build scripts
```

### 2.2 Quality Gate Configuration
- Prettier: Check on pre-commit
- rustfmt: Check on pre-commit
- clippy: Strict mode for CI
- Tests: 100% pass required

## 3. Performance Considerations
- Rust compilation: 65+ crates = ~10 min build time
- pnpm: Fast with workspace caching
- Prettier: <30s for full scan

## 4. Notes
- rustfmt imports_granularity warnings: Documented as ignorable per project config
- UTF-8 encoding verified across all files
- The stabilization session remains the canonical baseline, but not the current branch state.

## 5. 2026-04-01 Workspace Repair

- Removed nested `[workspace]` declarations from the harness crates under `crates/` so the root
  workspace is the only workspace authority.
- Moved the ignored `harness_runner` release profile to the root [Cargo.toml](/Users/kooshapari/CodeProjects/Phenotype/repos/heliosCLI/Cargo.toml)
  so release settings are applied instead of being silently ignored.
- Validation after the repair:
  - `cargo metadata --format-version 1`
  - `cargo check --workspace`
  - `cargo clippy --workspace --all-targets -- -D warnings`
  - `cargo test --workspace`
