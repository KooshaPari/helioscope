# Architecture Decision Records -- heliosCLI

## ADR-001: Bazel as Primary Build System

**Status:** Accepted
**Context:** The monorepo contains Rust (codex-rs) and TypeScript (codex-cli) components that need unified builds, caching, and cross-language dependency management.
**Decision:** Use Bazel with rules_rust and rules_nodejs for hermetic, cacheable builds.
**Alternatives:** Cargo + pnpm separately (no unified graph), Nix (steeper learning curve).
**Consequences:** Build files must be kept in sync; contributors need Bazel familiarity.

## ADR-002: Rust for Execution Core

**Status:** Accepted
**Context:** The execution engine needs low-latency sandboxing, memory safety, and high concurrency for parallel agent sessions.
**Decision:** Implement core execution, protocol, and sandbox in Rust (codex-rs workspace).
**Alternatives:** Go (less control over memory), TypeScript (insufficient for sandboxing).
**Consequences:** Rust workspace with multiple crates; FFI boundary with TypeScript via protocol serialization.

## ADR-003: TypeScript CLI Frontend

**Status:** Accepted
**Context:** The user-facing CLI needs rapid iteration, rich terminal UI, and npm ecosystem access for agent SDKs.
**Decision:** codex-cli in TypeScript, managed via pnpm workspace.
**Consequences:** Two language runtimes in CI; protocol types generated from Rust schemas.

## ADR-004: Harness Crate Architecture

**Status:** Accepted
**Context:** The benchmark/harness system has multiple concerns: caching, checkpointing, discovery, orchestration, scaling, schema validation.
**Decision:** Decompose into fine-grained Rust crates under `crates/` (harness_cache, harness_runner, harness_scaling, etc.).
**Alternatives:** Single monolithic harness crate.
**Consequences:** Clear module boundaries; some inter-crate dependency management overhead.

## ADR-005: Measured Performance Over Language Novelty

**Status:** Accepted
**Context:** Mojo, Zig, and inline assembly were explored for hot-path optimization.
**Decision:** Prioritize profiling-driven optimization in Rust. Use Zig/Mojo only when benchmarks show a measurable win that Rust cannot achieve. Inline assembly only after exhausting compiler intrinsics.
**Consequences:** Mojo and Zig remain optional satellite components, not core runtime.
