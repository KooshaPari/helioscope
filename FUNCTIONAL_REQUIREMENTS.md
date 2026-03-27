# Functional Requirements -- heliosCLI

## FR-CLI: Command-Line Interface

### FR-CLI-001: Multi-Backend Agent Dispatch
The `helios` binary SHALL dispatch to any registered AI backend via `--model` and `--provider` flags, with alias expansion for `oss-provider`/`local-provider` -> `oss_provider`.
Traces to: E1.1

### FR-CLI-002: Default Interactive TUI Launch
When stdin is a TTY and no subcommand is given, the binary SHALL launch the `helios-tui` interactive session.
Traces to: E1.2

### FR-CLI-003: Batch Non-Interactive Mode
When stdin is not a TTY, the binary SHALL process piped input without interactive prompts and exit with code 0 on success, non-zero on error.
Traces to: E1.3

### FR-CLI-004: Session Resume
`helios resume <session-id>` SHALL restore a prior session state from `$HELIOS_HOME/sessions/`.
Traces to: E1.2

### FR-CLI-005: Session Fork
`helios fork` SHALL create a new session branched from an existing session, inheriting conversation history up to the fork point.
Traces to: E1.2

### FR-CLI-006: Apply Patch
`helios apply <patch-file>` SHALL apply a unified diff to the working tree via the `ApplyCommand` in `helios-chatgpt`.
Traces to: E1.4

### FR-CLI-007: Shell Completion
`helios completion <shell>` SHALL emit shell completion scripts for Bash, Zsh, Fish, and PowerShell via `clap_complete`.
Traces to: E1.1

---

## FR-SBX: Sandboxed Execution

### FR-SBX-001: macOS Seatbelt Sandbox
`helios sandbox macos` SHALL configure and launch a Seatbelt (`sandbox-exec`) profile restricting writes to the designated workspace directory and enforcing network policy.
Traces to: E2.1

### FR-SBX-002: Linux Landlock Sandbox
`helios sandbox linux` SHALL configure and apply a Landlock ruleset restricting filesystem access and enforcing network policy via `helios-execpolicy`.
Traces to: E2.1

### FR-SBX-003: Windows Sandbox
`helios sandbox windows` SHALL configure Windows job object restrictions for filesystem and process isolation.
Traces to: E2.1

### FR-SBX-004: Write-Outside-Workspace Denial
All sandbox implementations SHALL deny write operations to paths outside the designated workspace directory and emit an explicit error message naming the denied path.
Traces to: E2.1

### FR-SBX-005: Configurable Network Policy
All sandbox implementations SHALL support a configurable network access policy with allow/deny rules per domain pattern, read from the helios config file.
Traces to: E2.1

### FR-SBX-006: Execution Timeout
All sandbox implementations SHALL enforce a configurable execution timeout; processes SHALL be terminated when the timeout elapses and the error SHALL include the timeout duration and process identity.
Traces to: E2.1

### FR-SBX-007: Execution Policy Check
`helios execpolicy check` SHALL evaluate the current execution policy configuration and report each policy assertion as pass/fail.
Traces to: E2.1

---

## FR-SRV: App Server and Protocol

### FR-SRV-001: App Server Lifecycle
`helios app-server start` SHALL launch the background app-server process; `helios app-server stop` SHALL terminate it; `helios app-server status` SHALL report health.
Traces to: E2.2

### FR-SRV-002: Typed Protocol Messages
All IPC messages between CLI and app-server SHALL use typed structs defined in `helios-app-server-protocol` with serde JSON serialization.
Traces to: E2.2

### FR-SRV-003: stdio-to-UDS Bridge
`helios stdio-to-uds <socket-path>` SHALL bridge the process's stdin/stdout to the specified Unix domain socket.
Traces to: E2.2

### FR-SRV-004: MCP Server Integration
`helios mcp` SHALL start the embedded MCP server exposing registered tools over the MCP protocol to external AI clients.
Traces to: E2.3

### FR-SRV-005: Shell Tool MCP Package
The `shell-tool-mcp` npm package SHALL expose a shell-execution MCP tool with a compliant JSON schema, tests, and sandbox policy enforcement.
Traces to: E5.3

---

## FR-AUTH: Authentication

### FR-AUTH-001: API Key Login
`helios login --api-key` SHALL accept an API key from stdin and store it via `helios-keyring-store` in the system keyring.
Traces to: E2.5

### FR-AUTH-002: Device-Code OAuth
`helios login` without flags SHALL initiate a device-code OAuth flow with the configured provider.
Traces to: E2.5

### FR-AUTH-003: ChatGPT Browser Login
`helios login chatgpt` SHALL initiate a browser-based ChatGPT authentication flow via `helios-chatgpt`.
Traces to: E2.5

### FR-AUTH-004: Login Status
`helios login status` SHALL display currently authenticated providers, token types, and expiry information.
Traces to: E2.5

### FR-AUTH-005: Logout
`helios logout` SHALL remove credentials for all providers from the system keyring.
Traces to: E2.5

---

## FR-CFG: Configuration

### FR-CFG-001: Config Home Resolution
The binary SHALL resolve `$HELIOS_HOME` from the environment; if unset, default to `~/.helios`. Config files SHALL be read from this directory.
Traces to: E2.6

### FR-CFG-002: Config Edit
`helios config edit` SHALL open the config file in `$EDITOR` or a fallback editor.
Traces to: E2.6

### FR-CFG-003: Config Override
`helios config set <key> <value>` SHALL apply a single config key-value pair by writing through `ConfigEditsBuilder`.
Traces to: E2.6

### FR-CFG-004: TypeScript Schema Generation
`helios generate-ts` SHALL emit TypeScript type definitions derived from the Rust config JSON schema to a configured output path.
Traces to: E2.6

### FR-CFG-005: Feature Flags
The config SHALL support feature-flag entries gated by `Stage` (alpha/beta/stable). Features not at or above the configured stage SHALL be disabled at runtime.
Traces to: E2.6

---

## FR-RESP: Responses API Proxy

### FR-RESP-001: Local Proxy Endpoint
`helios-responses-api-proxy` SHALL expose a local HTTP server endpoint compatible with the OpenAI Responses API format.
Traces to: E2.4

### FR-RESP-002: Streaming SSE
The proxy SHALL forward streamed responses from the backend as Server-Sent Events on the local endpoint.
Traces to: E2.4

---

## FR-HAR: Harness System

### FR-HAR-001: Suite Manifest Loading
`harness_spec` SHALL load suite manifests from TOML or YAML files; `harness_discoverer` SHALL recursively scan configured directories.
Traces to: E3.1

### FR-HAR-002: Task Execution
`harness_runner` SHALL execute each harness task by sending the task prompt to the configured agent and capturing the raw response.
Traces to: E3.1

### FR-HAR-003: Concurrency Control
`harness_queue` SHALL maintain a FIFO task queue; `harness_orchestrator` SHALL dispatch tasks to a configurable number of concurrent `harness_runner` instances.
Traces to: E3.2

### FR-HAR-004: Dynamic Scaling
`harness_scaling` SHALL adjust the pool of active runners based on queue depth and configurable scaling rules.
Traces to: E3.2

### FR-HAR-005: Output Verification
`harness_verify` SHALL evaluate each task output against success criteria defined in the suite manifest and assign a correctness verdict (pass/fail/partial).
Traces to: E3.3

### FR-HAR-006: Output Normalization
`harness_normalizer` SHALL normalize raw agent outputs to a canonical form before verification and comparison.
Traces to: E3.3

### FR-HAR-007: Checkpoint and Rollback
`harness_checkpoint` SHALL persist harness state after each completed task. `harness_rollback` SHALL restore from the most recent checkpoint on abort or crash.
Traces to: E3.3

### FR-HAR-008: Response Cache
`harness_cache` SHALL cache LLM responses keyed by (model, prompt hash); cached responses SHALL be replayed on re-run when the cache hit matches.
Traces to: E3.3

### FR-HAR-009: Results JSON Output
The harness SHALL produce a structured JSON results file with per-task fields: agent, model, latency_ms, prompt_tokens, completion_tokens, correctness_verdict, raw_output. Aggregate fields: pass_rate, p50/p90/p99 latency, estimated_cost.
Traces to: E3.4

### FR-HAR-010: Python Bindings
`harness_pyo3` SHALL expose `HarnessRunner` and `HarnessOrchestrator` classes to Python via pyo3, installable as a native Python extension.
Traces to: E3.5

---

## FR-CT: Cloud Tasks

### FR-CT-001: Task Submission
`helios cloud-tasks submit` SHALL serialize a task request via `helios-cloud-tasks` and dispatch it to the configured cloud endpoint.
Traces to: E4.1

### FR-CT-002: Task Status Polling
`helios cloud-tasks status <task-id>` SHALL poll the cloud endpoint and display the current task state until completion or timeout.
Traces to: E4.1

### FR-CT-003: HTTP Transport with Retry
`helios-cloud-tasks-client` SHALL implement exponential-backoff retry on 5xx responses and network errors, with configurable max retries.
Traces to: E4.1

---

## FR-BUILD: Build and CI

### FR-BUILD-001: Hermetic Bazel Build
`bazel build //...` SHALL produce all Rust and TypeScript artifacts without network access after the initial dependency fetch.
Traces to: E5.1

### FR-BUILD-002: Full Test Suite
`bazel test //...` SHALL run all test targets. `cargo test --workspace` and `pnpm --filter codex-cli test` SHALL be equivalent secondary entry points.
Traces to: E5.2

### FR-BUILD-003: Rust Quality Gates
All Rust code SHALL pass `cargo clippy --workspace -- -D warnings` and `cargo fmt --check` with zero errors.
Traces to: E5.2

### FR-BUILD-004: Policy Gate Integration
All PRs SHALL invoke `KooshaPari/phenotypeActions/actions/policy-gate@v0` to enforce namespace ownership, merge-commit, and layered-fix policies.
Traces to: E5.2
