# Product Requirements Document -- heliosCLI

## Product Vision

heliosCLI is a multi-runtime AI coding assistant CLI that unifies multiple AI coding backends (Codex/OpenAI, Claude, Gemini, ChatGPT, local OSS providers) behind a single `helios` command. It is structured as a Bazel monorepo with three primary layers: a Rust core (`codex-rs`) providing sandboxed execution, protocol handling, MCP server hosting, and TUI; a Helios-specific Rust CLI layer (`helios-rs/cli`) that extends the core with login, app-server management, cloud tasks, and platform-specific sandbox controls; and a TypeScript CLI (`codex-cli`) as the user-facing command interface. A harness subsystem (`crates/harness_*`) provides benchmarking, scaling, and multi-agent orchestration for AI workload evaluation.

---

## E1: Multi-Runtime AI CLI

### E1.1: Unified Agent Dispatch

As a developer, I can invoke any supported AI backend through a single `helios` command with a consistent flag interface.

**Acceptance Criteria**:
- Default invocation (no subcommand) forwards options to the interactive TUI.
- `--model`, `--provider` flags select the backend; shorthand aliases `oss-provider` and `local-provider` map to `oss_provider`.
- Backend selection supports: OpenAI/Codex, Claude (Anthropic), Gemini, ChatGPT (via `helios-chatgpt`), local/OSS providers, LM Studio, Ollama.
- `helios completion <shell>` generates shell completions for Bash, Zsh, Fish, PowerShell.

### E1.2: Interactive TUI Mode

As a developer, I want an interactive terminal UI for ongoing coding conversations with an AI agent.

**Acceptance Criteria**:
- Default invocation (TTY detected, no subcommand) launches `helios-tui` with streaming output.
- TUI supports multi-turn conversation, file context injection, and tool-use display.
- `ExitReason` and `AppExitInfo` are surfaced to the parent process for scripting.
- `helios resume <session-id>` resumes a prior session.
- `helios fork` creates a new session branched from an existing one.

### E1.3: Batch / Non-Interactive Mode

As a CI script, I want to pipe prompts to helios and receive structured output with correct exit codes.

**Acceptance Criteria**:
- When stdin is not a TTY, `helios` operates in batch mode without interactive prompts.
- Exit code 0 on success; non-zero on agent error or sandbox violation.
- Output includes structured JSON when `--json` flag is set.

### E1.4: Apply Patch Command

As a developer, I can apply AI-generated patches to the working directory via `helios apply`.

**Acceptance Criteria**:
- `helios apply <patch-file>` applies a unified diff to the working tree.
- The `ApplyCommand` delegates to `helios-chatgpt`'s apply logic.
- Dry-run mode (`--dry-run`) reports changes without applying them.

---

## E2: Rust Execution Core (codex-rs)

### E2.1: Platform-Specific Sandboxed Execution

As a developer, AI-generated code executes in a sandboxed environment appropriate for the host OS.

**Acceptance Criteria**:
- macOS: uses Seatbelt (`sandbox-exec`) via `helios sandbox macos`.
- Linux: uses Landlock via `helios sandbox linux`.
- Windows: uses Windows job objects and restricted tokens via `helios sandbox windows`.
- All sandbox implementations enforce: write-outside-workspace denied; configurable network access; execution timeout.
- `helios execpolicy check` evaluates the current execution policy configuration.

### E2.2: Protocol and App-Server

As a developer, a local app-server provides a stable IPC channel between the CLI and background agent processes.

**Acceptance Criteria**:
- `helios app-server start` launches the background app-server process.
- `helios app-server status` reports health of the running server.
- Protocol messages are typed via `helios-app-server-protocol` crate; test client available via `helios-app-server-test-client`.
- `helios debug app-server send-message-v2` provides a debug hook for protocol testing.
- `helios stdio-to-uds` bridges stdio to a Unix domain socket for process connection.

### E2.3: MCP Server

As an agent tool author, I can expose tools to AI backends via the Model Context Protocol server built into helios.

**Acceptance Criteria**:
- `helios mcp` subcommand manages the embedded MCP server.
- MCP server exposes registered tools over the MCP wire protocol.
- Tool registration is driven by the `codex-rs/skills` and `codex-rs/mcp-server` crates.
- `shell-tool-mcp` TypeScript package provides a shell-execution MCP tool consumable by external clients.

### E2.4: Responses API Proxy

As a developer, I can route agent requests through a local responses-API-compatible proxy for observability and provider switching.

**Acceptance Criteria**:
- `helios-responses-api-proxy` crate exposes a local HTTP endpoint compatible with the OpenAI Responses API.
- Requests are forwarded to the configured backend provider.
- Proxy supports streaming SSE responses.

### E2.5: Login and Authentication

As a developer, I authenticate to AI providers via `helios login`.

**Acceptance Criteria**:
- `helios login` supports: API key from stdin (`--api-key`), device-code OAuth flow, ChatGPT browser-based login.
- `helios login status` shows currently authenticated providers and token validity.
- `helios logout` removes stored credentials.
- Credentials are stored via `helios-keyring-store` (system keyring integration).

### E2.6: Configuration Management

As a developer, I manage helios configuration via `helios config`.

**Acceptance Criteria**:
- Config is read from `$HELIOS_HOME` (default: `~/.helios`).
- `helios config edit` opens config in the user's editor.
- `helios config set <key> <value>` applies a single config override.
- `helios generate-ts` generates TypeScript type definitions from the config JSON schema.
- Feature flags are managed via `helios-core/features` with `Stage` gating.

---

## E3: Harness System

### E3.1: Benchmark Suite Definition and Loading

As a developer, I can define benchmark suites as structured manifests and run them against any configured AI backend.

**Acceptance Criteria**:
- Harness suite manifests are loaded by `harness_spec` and `harness_schema` crates.
- Each task in a suite defines: prompt, expected outputs, success criteria, timeout.
- `harness_discoverer` scans configured directories for suite manifests.
- `harness_elicitation` handles prompting the AI backend and capturing raw responses.

### E3.2: Multi-Agent Orchestration and Scaling

As an operator, I can run harness workloads across N concurrent agent sessions with queue-based task distribution.

**Acceptance Criteria**:
- `harness_orchestrator` manages the execution lifecycle: queue, dispatch, collect.
- `harness_queue` provides FIFO task queuing with concurrency control.
- `harness_scaling` dynamically adjusts the number of active agent sessions based on load.
- `harness_runner` executes individual tasks and captures results.
- `harness_teammates` supports multi-agent collaboration on a single task.

### E3.3: Results, Verification, and Rollback

As a developer, harness results are verified, normalized, and stored with rollback capability.

**Acceptance Criteria**:
- `harness_verify` validates task outputs against defined success criteria.
- `harness_normalizer` normalizes outputs for consistent comparison across agents.
- `harness_checkpoint` snapshots harness state for resumability after failure.
- `harness_rollback` restores prior state when a harness run is aborted.
- `harness_cache` caches LLM responses for deterministic replay and cost reduction.

### E3.4: Results Output and Reporting

As a developer, harness results are exported as structured JSON with timing, token usage, and correctness metrics.

**Acceptance Criteria**:
- Results include per-task fields: agent, model, latency, token count, correctness verdict, raw output.
- Aggregated summary includes pass rate, P50/P90/P99 latency, cost estimate.
- `thegent-benchmark` crate integrates with the `thegent` orchestration layer for cross-repo benchmarking.

### E3.5: Python and Native Bindings

As a data scientist, I can run harness workloads from Python via pyo3 bindings.

**Acceptance Criteria**:
- `harness_pyo3` exposes the harness runner and orchestrator to Python.
- `harness-native` crate (the Cargo workspace member) provides native extension build artifacts.
- Mojo (`harness_mojo`) and Zig (`harness_zig`) satellite extensions are available for high-performance task execution.

---

## E4: Cloud Tasks

### E4.1: Remote Task Dispatch

As an operator, I can dispatch long-running AI tasks to a cloud backend and poll for results.

**Acceptance Criteria**:
- `helios cloud-tasks submit <task>` dispatches a task to the cloud backend.
- `helios cloud-tasks status <task-id>` polls task completion.
- `helios-cloud-tasks` crate provides typed request/response structs with serde serialization.
- `helios-cloud-tasks-client` handles HTTP transport and retry logic.

---

## E5: Build and CI

### E5.1: Bazel Monorepo Build

As a contributor, all components build via `bazel build //...` with hermetic, cached builds.

**Acceptance Criteria**:
- `bazel build //...` succeeds for all Rust and TypeScript targets.
- `bazel test //...` runs all test suites.
- Incremental builds use action cache; clean builds are hermetic.
- `rules_rust` manages Rust toolchain; `aspect_rules_js` manages Node toolchain.

### E5.2: CI Pipeline

As a contributor, PRs trigger automated CI checks across Rust, TypeScript, and Bazel layers.

**Acceptance Criteria**:
- Rust: `clippy --workspace -D warnings`, `fmt --check`, `cargo test --workspace`.
- TypeScript: biome lint, vitest test suite.
- Bazel: build and test all targets.
- Policy gate (`KooshaPari/phenotypeActions/actions/policy-gate`) enforces PR conventions.
- Stage gates evaluated on merge to `main`.

### E5.3: shell-tool-mcp TypeScript Package

As an agent tool consumer, I can install `shell-tool-mcp` as an MCP server that executes shell commands in a sandboxed environment.

**Acceptance Criteria**:
- Package is publishable from `shell-tool-mcp/`.
- Provides MCP-compliant tool schema for shell execution.
- Integrates with helios sandbox policies for network and filesystem restrictions.
- Tests in `shell-tool-mcp/tests/` cover tool invocation and error paths.
