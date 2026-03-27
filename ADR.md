# Architecture Decision Records -- heliosCLI

## ADR-001: Bazel as Primary Build System

**Status**: Accepted
**Date**: 2026-03-27

**Context**: The monorepo contains Rust (`codex-rs`, `helios-rs`), TypeScript (`codex-cli`, `shell-tool-mcp`), Python (`harness_pyo3`), Mojo, and Zig components. These need unified dependency graphs, hermetic builds, remote caching, and cross-language test execution.

**Decision**: Use Bazel with `rules_rust`, `aspect_rules_js`, and `rules_python` as the primary build system. Cargo and pnpm serve as secondary entry points for single-language iteration.

**Rationale**:
- Bazel's hermetic execution and action-level caching reduce CI wall time on large Rust monorepos.
- Cross-language targets (generating TypeScript types from Rust schemas) require a unified build graph.
- Remote execution via `rbe.bzl` enables horizontal scale for CI.
- `rules_rust` provides first-class Bazel-native Rust support without requiring a custom build bridge.

**Alternatives Considered**:
- Cargo + pnpm separately: no unified dependency graph; cross-language integration requires ad-hoc scripts.
- Nix: hermetic but steep learning curve and fragile ecosystem integration for Rust nightly.

**Consequences**:
- `BUILD.bazel` files must be maintained alongside `Cargo.toml` and `package.json`.
- Contributors need Bazel familiarity. `justfile` provides shortcut commands.
- `build.rs` scripts that require network access must be replaced with Bazel `build_script` rules or vendored.

---

## ADR-002: Rust for Execution Core and Protocol Layer

**Status**: Accepted
**Date**: 2026-03-27

**Context**: The execution engine needs OS-level sandbox APIs (Landlock, Seatbelt, Windows job objects), low-latency async I/O for concurrent agent sessions, memory safety for security-critical sandbox enforcement, and the ability to generate typed schemas for TypeScript consumers.

**Decision**: Implement all execution, sandboxing, protocol, MCP server, TUI, and authentication logic in Rust, organized as a multi-crate workspace under `codex-rs/` and `helios-rs/`.

**Rationale**:
- Rust provides memory-safe FFI to OS sandbox APIs without GC pause risk on latency-sensitive execution paths.
- `tokio` async runtime handles hundreds of concurrent agent sessions with minimal overhead.
- `serde` + JSON schema generation enables typed protocol contracts shared with TypeScript via code generation.
- `clap` + `clap_complete` provides an ergonomic CLI with shell completion support.

**Alternatives Considered**:
- Go: GC pauses are unacceptable for sandbox enforcement; weaker OS sandbox API integration.
- TypeScript: insufficient access to OS-level APIs for Landlock/Seatbelt; no viable sandboxing path.

**Consequences**:
- Rust crate count is large (50+ crates). Crate granularity is managed via ADR-004.
- TypeScript types are derived from Rust schemas via `helios generate-ts`; the generation step must run before TypeScript builds.

---

## ADR-003: TypeScript CLI Frontend and MCP Package

**Status**: Accepted
**Date**: 2026-03-27

**Context**: The interactive CLI and `shell-tool-mcp` package benefit from the npm ecosystem for terminal UI libraries, provider SDKs (OpenAI, Anthropic), and rapid iteration.

**Decision**: Maintain `codex-cli` as a TypeScript CLI managed via pnpm workspace. The `shell-tool-mcp` TypeScript package exposes the shell execution MCP tool as an npm-publishable package.

**Rationale**:
- npm ecosystem provides ready-made SDKs for all major AI providers.
- TypeScript enforces protocol contracts generated from Rust schemas.
- `shell-tool-mcp` as an npm package is consumable by MCP clients without requiring a Rust build environment.

**Consequences**:
- Two language runtimes in CI. TypeScript tests run via vitest; linting via biome.
- Protocol type definitions must stay in sync with Rust schemas; `helios generate-ts` is the synchronization mechanism.
- Node version is pinned in `codex-rs/node-version.txt`; pnpm lockfile is committed.

---

## ADR-004: Fine-Grained Harness Crate Architecture

**Status**: Accepted
**Date**: 2026-03-27

**Context**: The harness system has distinct concerns: caching, checkpointing, discovery, elicitation, interfaces, normalization, orchestration, queue management, rollback, running, scaling, schema validation, task specs, and teammate collaboration. A single harness crate would have high internal coupling.

**Decision**: Decompose the harness into ~15 fine-grained crates under `crates/harness_*`, each with a single responsibility. `harness_orchestrator` is the integration point that composes the others.

**Rationale**:
- Fine-grained crates enforce explicit dependency relationships; circular dependencies fail at compile time.
- Each crate is testable in isolation with minimal mocking surface.
- New scaling strategies or caching backends can be swapped by replacing a single crate.
- Mojo (`harness_mojo`) and Zig (`harness_zig`) satellite crates remain isolated from the Rust hot path.

**Alternatives Considered**:
- Single monolithic harness crate: simpler dependency management but degrades into a ball of mud under active development.

**Consequences**:
- Inter-crate dependency management requires Cargo.toml edits for each new harness concern.
- `harness_interfaces` crate provides shared trait definitions to prevent circular dependencies.

---

## ADR-005: Profiling-Driven Optimization; Mojo/Zig as Satellites

**Status**: Accepted
**Date**: 2026-03-27

**Context**: Mojo and Zig were explored as alternatives for performance-critical harness paths. Inline assembly was considered for hot-path optimization.

**Decision**: Default to Rust with profiling-driven optimization. Adopt Mojo or Zig only when profiling demonstrates a measurable improvement that Rust cannot achieve. Inline assembly is last resort after exhausting compiler intrinsics.

**Rationale**:
- Premature optimization in non-Rust languages adds build complexity before there is evidence of a bottleneck.
- `perf-results/` tracks benchmark history to provide objective evidence for any future language adoption.
- Mojo and Zig toolchains are less mature; introducing them as build dependencies risks build system instability.

**Consequences**:
- `harness_mojo` and `harness_zig` remain satellite experiments and are not in the critical path.
- Performance regressions are caught by the `heliosBench` benchmark suite run in CI.

---

## ADR-006: Platform-Native Sandbox per OS

**Status**: Accepted
**Date**: 2026-03-27

**Context**: Available sandbox mechanisms differ across macOS (Seatbelt/sandbox-exec), Linux (Landlock, seccomp, namespaces), and Windows (job objects, restricted tokens). A unified abstraction would result in the lowest common denominator.

**Decision**: Implement a separate sandbox module per platform (`helios-linux-sandbox`, `helios-windows-sandbox-rs`, `SeatbeltCommand`). The `helios sandbox <platform>` subcommand dispatches to the platform-appropriate implementation. A common `helios-execpolicy` crate defines the policy abstraction shared across platforms.

**Rationale**:
- Platform-specific implementations can use each OS's strongest available mechanism.
- The `execpolicy` abstraction layer allows policy definitions (allow/deny lists, timeouts) to be expressed once and enforced natively.
- `helios sandbox linux` falls back to `execpolicy-legacy` on kernels without full Landlock support.

**Consequences**:
- Platform-specific crates must be conditionally compiled (`#[cfg(target_os = "macos")]`).
- CI must test on all three platforms; the build matrix from `policy-gate` manages platform-aware testing.

---

## ADR-007: App-Server for Long-Running Agent Sessions

**Status**: Accepted
**Date**: 2026-03-27

**Context**: AI coding sessions may run for minutes to hours. CLI processes are not robust to terminal disconnection. A background service model allows sessions to continue independently of the terminal.

**Decision**: Run a background `app-server` process that manages agent session state. The `helios` CLI connects to it via Unix domain socket (or named pipe on Windows). `helios stdio-to-uds` provides a bridge for tools that communicate via stdio.

**Rationale**:
- Session state (conversation history, file edits, tool outputs) is durable across terminal reconnects.
- Multiple CLI processes can observe the same session.
- The typed `helios-app-server-protocol` is versionable; breaking changes require a server restart, not a full session restart.

**Consequences**:
- `helios app-server start/stop/status` lifecycle commands must be documented.
- The server PID must be tracked in `$HELIOS_HOME` to prevent duplicate instances.
- `helios debug app-server send-message-v2` provides a development escape hatch for protocol testing.
