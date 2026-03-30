# heliosCLI Implementation Plan

**Version:** 2.0 | **Status:** In Progress | **Date:** 2026-03-29

This plan maps the PRD user journeys and functional requirements to phased work packages (WPs), with explicit DAG dependencies and current implementation status. The system is built as a Rust/TypeScript hybrid with TUI, sandboxing, and multi-backend agent routing.

---

## Phase 0: Foundation & Scaffolding

**Status:** Mostly Complete

Core workspace setup, build infrastructure, and protocol layer.

| Task ID | Description | Depends On | Status |
|---------|-------------|------------|--------|
| P0.1 | Rust workspace with core, exec, protocol crates | — | Done |
| P0.2 | TypeScript CLI scaffold with command registry | — | Done |
| P0.3 | Bazel build rules for Rust and Node targets | P0.1, P0.2 | Done |
| P0.4 | Protobuf message definitions (Query, Response, Agent) | P0.1 | Done |
| P0.5 | Monorepo layout: `crates/`, `packages/`, `harnesses/` | — | Done |

**Deliverables:**
- Rust workspace builds cleanly with all 8 crates
- TypeScript package.json configured with TypeScript, vitest, biome
- Bazel targets for `cargo build`, `npm install`, cross-platform binaries

---

## Phase 1: Execution Core

**Status:** In Progress

Filesystem sandbox, network policies, timeout management, protocol serialization.

| Task ID | Description | Depends On | Status |
|---------|-------------|------------|--------|
| P1.1 | Filesystem sandbox with configurable allow/deny paths | P0.1 | Done |
| P1.2 | Network policy enforcement (domain-based allow/deny) | P0.1 | Done |
| P1.3 | Execution timeout and process management | P1.1 | Done |
| P1.4 | Protocol types with serde + TypeScript codegen | P0.1, P0.2 | Done |
| P1.5 | Sandbox policy validation and error messages | P1.1, P1.2 | Partial |
| P1.6 | macOS Seatbelt sandbox integration (Security framework) | P1.1 | Done |
| P1.7 | Linux seccomp/apparmor support | P1.1 | Partial |
| P1.8 | Windows sandbox (Job Objects) support | P1.1 | Planned |

**Deliverables:**
- `helios-exec` crate with sandbox API (allow/deny path lists)
- Network policy engine with domain whitelist/blacklist
- Process timeout enforcement with SIGTERM/SIGKILL
- Cross-platform sandbox implementations (macOS done, Linux partial)

---

## Phase 2: Agent Integration & Routing

**Status:** In Progress

Agent registry, dispatch layer, multi-backend routing (Claude, Codex, Gemini, Cursor, Copilot).

| Task ID | Description | Depends On | Status |
|---------|-------------|------------|--------|
| P2.1 | Agent registry and dispatch layer | P1.4 | Done |
| P2.2 | Claude API adapter (claude-3-opus, claude-3-sonnet) | P2.1 | Done |
| P2.3 | Codex API adapter (openai-compatible) | P2.1 | Partial |
| P2.4 | Gemini API adapter (Google AI) | P2.1 | Partial |
| P2.5 | Cursor API adapter (cursor-api backend) | P2.1 | Partial |
| P2.6 | Copilot adapter (GitHub Copilot Chat) | P2.1 | Planned |
| P2.7 | Provider fallback chains (cost/speed/quality Pareto) | P2.1-P2.6 | Partial |
| P2.8 | Model selection: default vs explicit --model flag | P2.1 | Done |
| P2.9 | Interactive REPL with streaming output | P2.1 | Done |
| P2.10 | Streaming response handler (SSE, token-by-token) | P2.9 | Done |

**Deliverables:**
- Agent dispatcher routing queries to configured backends
- 4 adapters functional (Claude, Codex, Gemini, Cursor)
- Fallback chains prevent single-provider outages
- Streaming TUI output with real-time token display

---

## Phase 3: CLI Interface & Session Management

**Status:** In Progress

Interactive TUI, batch mode, session persistence, REPL continuation.

| Task ID | Description | Depends On | Status |
|---------|-------------|------------|--------|
| P3.1 | Default interactive TUI launch (helios-tui crate) | P2.9 | Done |
| P3.2 | Batch non-interactive mode (stdin → JSON → stdout) | P2.1 | Done |
| P3.3 | Session persistence (JSONL append-only format) | P0.1 | Partial |
| P3.4 | Session resume/fork commands | P3.3 | Partial |
| P3.5 | Session history loading (conversation context) | P3.3 | Partial |
| P3.6 | Pagination and search within sessions | P3.3 | Planned |
| P3.7 | Shell completion generation (bash/zsh/fish/powershell) | P0.2 | Partial |
| P3.8 | TUI state management (input buffer, output scroll) | P3.1 | Done |

**Deliverables:**
- TUI launch with multi-turn conversation support
- Batch mode for CI/CD pipelines
- Session database at `$HELIOS_HOME/sessions/`
- Shell completion scripts for all major shells

---

## Phase 4: Sandboxing & Execution Policy

**Status:** In Progress

Execution policy validation, resource limits, evidence collection.

| Task ID | Description | Depends On | Status |
|---------|-------------|------------|--------|
| P4.1 | Execution policy model (allow/deny rules) | P1.1 | Partial |
| P4.2 | Write-outside-workspace denial | P1.1 | Done |
| P4.3 | Network policy validation at execution time | P1.2 | Done |
| P4.4 | Execution timeout enforcement | P1.3 | Done |
| P4.5 | Resource limits (memory, CPU, file descriptor caps) | P1.1 | Planned |
| P4.6 | Policy check command (execpolicy check) | P4.1 | Partial |
| P4.7 | Sandbox escape detection (heuristic analysis) | P1.1 | Planned |
| P4.8 | Audit trail: log all policy violations | P4.1 | Planned |

**Deliverables:**
- Execution policy enforced at runtime
- Clear error messages for policy violations
- `helios execpolicy check` CLI command validates config
- Audit logs for security review

---

## Phase 5: Harness System (Benchmarking)

**Status:** In Progress

Benchmark suite infrastructure, manifest parsing, result collection.

| Task ID | Description | Depends On | Status |
|---------|-------------|------------|--------|
| P5.1 | Benchmark suite manifest loader (TOML/YAML) | P0.1 | Done |
| P5.2 | harness_runner: sequential task execution | P5.1, P2.1 | Partial |
| P5.3 | harness_cache: result caching and invalidation | P5.2 | Partial |
| P5.4 | harness_scaling: concurrent N-session orchestration | P5.2 | Partial |
| P5.5 | harness_checkpoint: resumable benchmark runs | P5.3 | Partial |
| P5.6 | Structured JSON results with statistical summaries | P5.4 | Partial |
| P5.7 | harness_discoverer: auto-discover benchmark files | P5.1 | Partial |
| P5.8 | harness_normalizer: normalize results across agents | P5.6 | Planned |
| P5.9 | harness_verify: correctness verification | P5.2 | Planned |
| P5.10 | harness_schema: schema validation for manifests | P5.1 | Planned |

**Deliverables:**
- `harnesses/` directory with manifest-driven benchmarks
- Result aggregation with latency/throughput metrics
- Checkpoint recovery for long-running suites
- Cross-agent normalization for comparative analysis

---

## Phase 6: Authentication & Credential Management

**Status:** Planned

API key login, OAuth device-code flow, credential storage.

| Task ID | Description | Depends On | Status |
|---------|-------------|------------|--------|
| P6.1 | API key login (helios login --api-key) | P0.2 | Planned |
| P6.2 | System keyring integration (macOS Keychain, Linux Secret Service) | P6.1 | Planned |
| P6.3 | OAuth device-code flow (provider.com/device) | P0.2 | Planned |
| P6.4 | Token refresh logic (auto-renew before expiry) | P6.3 | Planned |
| P6.5 | Session token storage (secure, encrypted at rest) | P6.2, P6.3 | Planned |
| P6.6 | Logout command (revoke tokens) | P6.2 | Planned |
| P6.7 | Multi-account support (switch providers/accounts) | P6.1 | Planned |

**Deliverables:**
- Secure credential storage with no plaintext in git
- Device-code OAuth flow for user-friendly login
- API key support for automation
- Token refresh automatic on session start

---

## Phase 7: App Server & IPC

**Status:** Planned

Background service mode, Unix socket IPC, stdio-to-UDS bridge.

| Task ID | Description | Depends On | Status |
|---------|-------------|------------|--------|
| P7.1 | App server lifecycle (start, status, stop) | P2.1 | Planned |
| P7.2 | Unix socket IPC (stdio-to-UDS bridge) | P7.1 | Planned |
| P7.3 | Typed protocol for socket messages (serde JSON) | P0.4 | Planned |
| P7.4 | Connection pooling and session management | P7.2 | Planned |
| P7.5 | Graceful shutdown (drain in-flight requests) | P7.1 | Planned |
| P7.6 | Process supervisor (auto-restart on crash) | P7.1 | Planned |

**Deliverables:**
- `helios app-server` command for background service
- IPC protocol for multi-client communication
- Persistent state across server restarts

---

## Phase 8: MCP Server Integration

**Status:** Planned

Model Context Protocol server, tool exposure, external LLM client support.

| Task ID | Description | Depends On | Status |
|---------|-------------|------------|--------|
| P8.1 | MCP server scaffolding (listen on port) | P0.4 | Planned |
| P8.2 | Tool exposure (shell-tool, apply-patch, query-knowledge) | P8.1 | Planned |
| P8.3 | Resource exposure (spec docs, session logs) | P8.1 | Planned |
| P8.4 | Schema validation for tool inputs (JSONSchema) | P8.2 | Planned |
| P8.5 | Sampling support (multi-turn tool invocation) | P8.1 | Planned |
| P8.6 | Error handling and response formatting | P8.1 | Planned |

**Deliverables:**
- MCP server on configurable port with >5 tools
- External LLM clients can discover and call tools
- Schema-validated inputs and outputs

---

## Phase 9: Documentation & Examples

**Status:** Planned

User guide, API reference, example workflows, troubleshooting.

| Task ID | Description | Depends On | Status |
|---------|-------------|------------|--------|
| P9.1 | Quick start guide | P3.1 | Planned |
| P9.2 | CLI reference (all commands and flags) | P0.2 | Planned |
| P9.3 | Sandbox policy configuration guide | P4.1 | Planned |
| P9.4 | Benchmark manifest examples | P5.1 | Planned |
| P9.5 | Multi-backend routing examples | P2.7 | Planned |
| P9.6 | Troubleshooting & common errors | P3.1, P4.1 | Planned |
| P9.7 | VitePress docsite with embedded examples | P9.1-P9.6 | Planned |

**Deliverables:**
- Complete user documentation
- API reference auto-generated from code
- 10+ example workflows
- Troubleshooting guide with common issues

---

## Phase 10: Testing & Quality

**Status:** Planned

Integration tests, property-based tests, fuzz testing.

| Task ID | Description | Depends On | Status |
|---------|-------------|------------|--------|
| P10.1 | Unit tests for all crates | P0.1 | Partial |
| P10.2 | Integration tests (TUI, batch, agent routing) | P3.1, P2.1 | Partial |
| P10.3 | Property-based tests (sandbox correctness) | P1.1 | Planned |
| P10.4 | Fuzz testing (protocol parsing) | P0.4 | Planned |
| P10.5 | Benchmarks (startup time, query latency) | P5.1 | Planned |
| P10.6 | Coverage gates (80% for core modules) | P10.1-P10.5 | Planned |
| P10.7 | Test-driven development for all new features | — | Ongoing |

**Deliverables:**
- >80% coverage for core modules
- All tests passing in CI
- Performance benchmarks tracked over time

---

## Phase 11: Release & Deployment

**Status:** Planned

CI/CD pipeline, binary packaging, release notes.

| Task ID | Description | Depends On | Status |
|---------|-------------|------------|--------|
| P11.1 | CI pipeline (Rust: clippy, fmt, test) | P0.3 | Done |
| P11.2 | CI pipeline (TypeScript: biome, vitest) | P0.3 | Done |
| P11.3 | Policy gate composite action (PR validation) | P11.1, P11.2 | Partial |
| P11.4 | Bazel remote caching for CI builds | P11.1 | Planned |
| P11.5 | Cross-platform binary packaging (Linux, macOS, Windows) | P11.1 | Partial |
| P11.6 | Release notes generation (conventional commits) | P11.5 | Planned |
| P11.7 | Version bumping strategy (semver) | P11.5 | Planned |
| P11.8 | Distribution (GitHub Releases, crates.io, Homebrew) | P11.5 | Planned |

**Deliverables:**
- Automated CI checks for all PRs
- Binary packages for Linux, macOS (Windows planned)
- Automated release pipeline to GitHub Releases
- crates.io publication for Rust libraries

---

## Phase 12: Multi-Backend Optimization

**Status:** Planned

Cost optimization, latency reduction, provider selection heuristics.

| Task ID | Description | Depends On | Status |
|---------|-------------|------------|--------|
| P12.1 | Cost estimation per provider (token pricing) | P2.1-P2.6 | Planned |
| P12.2 | Latency measurement and tracking | P2.1 | Planned |
| P12.3 | Provider selection heuristics (cost vs speed) | P12.1, P12.2 | Planned |
| P12.4 | User preferences config (preferred providers) | P2.1 | Planned |
| P12.5 | A/B testing framework for agent comparison | P5.1 | Planned |
| P12.6 | Fallback quality metrics (success rate tracking) | P2.7 | Planned |

**Deliverables:**
- Cost-aware provider selection
- Automatic fallback based on success rates
- User-configurable provider preferences

---

## Success Metrics

| Metric | Target | Timeline | Status |
|--------|--------|----------|--------|
| TUI Launch | 0-config interactive session | Phase 1-3 | ✅ Done |
| Batch Mode | Process stdin to JSON output | Phase 1-3 | ✅ Done |
| Agent Support | 4+ backends (Claude, Codex, Gemini, Cursor) | Phase 2 | ⚠️ Partial |
| Sandboxing | Filesystem/network policies enforced | Phase 1-4 | ⚠️ Partial |
| Session Persistence | JSONL storage with resume/fork | Phase 3 | ⚠️ Partial |
| Benchmark Harness | Manifest-driven suite runner | Phase 5 | ⚠️ Partial |
| Test Coverage | >80% for core modules | Phase 10 | 🔲 Planned |
| Documentation | Complete user guide + examples | Phase 9 | 🔲 Planned |
| Release | Binary packages on GitHub Releases | Phase 11 | 🔲 Planned |

---

## User Journey Coverage

| Journey | Phases | Status |
|---------|--------|--------|
| UJ-001: Interactive Session | Phase 1, 3, 9 | ✅ Core done, docs planned |
| UJ-002: Batch Processing | Phase 1, 3 | ✅ Done |
| UJ-003: Session Resume | Phase 3 | ⚠️ Partial |
| UJ-004: Sandboxed Execution | Phase 1, 4 | ⚠️ Partial |
| UJ-005: Shell Completion | Phase 3 | ⚠️ Partial |
| UJ-006: App Server IPC | Phase 7 | 🔲 Planned |
| UJ-007: MCP Integration | Phase 8 | 🔲 Planned |
| UJ-008: Multi-Backend Routing | Phase 2, 12 | ⚠️ Partial |
| UJ-009: API Key Auth | Phase 6 | 🔲 Planned |
| UJ-010: OAuth Device Flow | Phase 6 | 🔲 Planned |

**Coverage:** 7/10 journeys have some implementation (2 complete, 5 partial, 3 planned)

---

## Dependency Graph (DAG)

```
Phase 0 (Foundation):
  P0.1 → P0.3, P0.4, P1.1, P1.2, P1.4, P5.1
  P0.2 → P0.3, P0.4, P2.1, P3.7
  P0.3 → P11.1

Phase 1 (Execution Core):
  P1.1 → P1.6, P1.7, P1.8, P4.1-P4.8
  P1.2 → P4.3, P4.4
  P1.3 → P4.4
  P1.4 → P2.1, P7.3

Phase 2 (Agent Integration):
  P2.1 → P2.2-P2.10, P5.2, P6.1
  P2.1-P2.6 → P2.7

Phase 3 (CLI):
  P2.9 → P3.1, P3.8
  P0.2 → P3.7
  P3.3 → P3.4, P3.5, P3.6

Phase 4 (Sandboxing):
  P1.1, P1.2 → P4.1-P4.8

Phase 5 (Harness):
  P0.1 → P5.1
  P5.1 → P5.2, P5.7, P5.10
  P5.2 → P5.3, P5.4, P5.6
  P5.3 → P5.5
  P5.4 → P5.6
  P5.6 → P5.8, P5.9

Phase 6 (Auth):
  P0.2 → P6.1, P6.3

Phase 7 (App Server):
  P2.1 → P7.1, P7.2
  P0.4 → P7.3

Phase 8 (MCP):
  P0.4 → P8.1, P8.3
  P8.1 → P8.2-P8.6

Phase 9 (Docs):
  P3.1, P4.1, P5.1, P2.7 → P9.1-P9.7

Phase 10 (Testing):
  P0.1 → P10.1
  P3.1, P2.1 → P10.2
  P1.1 → P10.3
  P0.4 → P10.4
  P5.1 → P10.5

Phase 11 (Release):
  P0.3 → P11.1, P11.2
  P11.1, P11.2 → P11.3
  P11.3 → P11.4-P11.8

Phase 12 (Optimization):
  P2.1-P2.6 → P12.1, P12.3, P12.4
  P2.1 → P12.2
  P5.1 → P12.5
  P2.7 → P12.6
```

---

## Critical Path to MVP

**Minimum Viable Product (MVP) Definition:** Interactive TUI, batch mode, sandboxed execution, agent routing to 2+ backends, session persistence.

**Critical Path (dependencies only):**
1. P0.1-P0.3 (Foundation) — ~5 tool calls
2. P1.1-P1.4 (Execution Core) — ~8 tool calls
3. P2.1-P2.4 (Agent Integration: Claude, Codex, Gemini) — ~10 tool calls
4. P3.1-P3.3 (TUI, Batch, Session Persistence) — ~8 tool calls
5. P4.1-P4.4 (Sandbox Policies) — ~5 tool calls

**Total Critical Path:** ~36 tool calls, ~12-15 min wall-clock time with 2-3 parallel agents.

---

## Notes

- All phases follow hexagonal architecture with clean domain/ports/adapters separation
- Sandboxing is platform-specific (macOS Seatbelt done, Linux seccomp partial, Windows planned)
- Agent routing uses provider fallback chains to prevent single-provider outages
- Session persistence uses append-only JSONL format for auditability
- All code is Rust (8 crates) except CLI wrapper (TypeScript optional)
- Tests are mandatory for all new features (TDD mandate)
- Documentation includes user journeys, quick starts, and troubleshooting
- CI/CD pipeline enforces lint, format, test checks before merge
