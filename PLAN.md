# heliosCLI Implementation Plan

**Status:** Active
**Stack:** Rust (codex-rs), TypeScript (codex-cli), Bazel monorepo

## Phase 0: Foundation

| Task | Description | Depends On |
|------|-------------|------------|
| P0.1 | Rust workspace with core, exec, protocol crates | -- |
| P0.2 | TypeScript CLI scaffold with command registry | -- |
| P0.3 | Bazel build rules for Rust and Node targets | P0.1, P0.2 |

## Phase 1: Execution Core

| Task | Description | Depends On |
|------|-------------|------------|
| P1.1 | Filesystem sandbox with configurable allow/deny paths | P0.1 |
| P1.2 | Network policy enforcement (domain-based allow/deny) | P0.1 |
| P1.3 | Execution timeout and process management | P1.1 |
| P1.4 | Protocol types with serde + TypeScript codegen | P0.1, P0.2 |

## Phase 2: Agent Integration

| Task | Description | Depends On |
|------|-------------|------------|
| P2.1 | Agent registry and dispatch layer | P1.4 |
| P2.2 | Codex agent adapter | P2.1 |
| P2.3 | Claude agent adapter | P2.1 |
| P2.4 | Gemini, Cursor, Copilot adapters | P2.1 |
| P2.5 | Interactive REPL with streaming output | P2.1 |

## Phase 3: Harness System

| Task | Description | Depends On |
|------|-------------|------------|
| P3.1 | Benchmark suite manifest loader (TOML/YAML) | P0.1 |
| P3.2 | harness_runner: sequential task execution | P3.1, P2.1 |
| P3.3 | harness_cache: result caching and invalidation | P3.2 |
| P3.4 | harness_scaling: concurrent N-session orchestration | P3.2 |
| P3.5 | harness_checkpoint: resumable benchmark runs | P3.3 |
| P3.6 | Structured JSON results with statistical summaries | P3.4 |

## Phase 4: Remaining Harness Crates

| Task | Description | Depends On |
|------|-------------|------------|
| P4.1 | harness_discoverer: auto-discover benchmark files | P3.1 |
| P4.2 | harness_normalizer: normalize results across agents | P3.6 |
| P4.3 | harness_verify: correctness verification against expected output | P3.2 |
| P4.4 | harness_schema: schema validation for manifests | P3.1 |
| P4.5 | harness_rollback: rollback failed benchmark state | P3.5 |

## Phase 5: CI and Release

| Task | Description | Depends On |
|------|-------------|------------|
| P5.1 | CI pipeline: Rust (clippy, fmt, test), TS (biome, vitest) | P0.3 |
| P5.2 | Policy gate composite action for PR validation | P5.1 |
| P5.3 | Bazel remote caching for CI builds | P5.1 |
| P5.4 | Release binary packaging (Linux, macOS) | P5.1 |
