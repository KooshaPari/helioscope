# Functional Requirements -- heliosCLI

## FR-CLI: Command-Line Interface

### FR-CLI-001: Agent Dispatch
The CLI SHALL dispatch commands to any registered agent backend (Codex, Claude, Gemini, Cursor, Copilot) via `helios run --agent <name>`.
**Traces to:** E1.1

### FR-CLI-002: Interactive Mode
The CLI SHALL provide an interactive REPL via `helios chat` with streaming output from the selected agent.
**Traces to:** E1.2

### FR-CLI-003: Batch Mode
The CLI SHALL accept piped stdin input and return structured output with appropriate exit codes.
**Traces to:** E1.2

## FR-EXC: Execution Core

### FR-EXC-001: Filesystem Sandbox
The execution engine SHALL restrict AI-generated code to designated directories; writes outside the sandbox SHALL fail with an explicit error.
**Traces to:** E2.1

### FR-EXC-002: Network Policy
The execution engine SHALL support configurable network access policies (allow/deny per domain pattern).
**Traces to:** E2.1

### FR-EXC-003: Execution Timeout
The execution engine SHALL enforce configurable timeouts and terminate runaway processes.
**Traces to:** E2.1

### FR-EXC-004: Protocol Serialization
The protocol layer SHALL use typed Rust structs with serde for serialization and generate TypeScript type definitions.
**Traces to:** E2.2

## FR-HAR: Harness System

### FR-HAR-001: Benchmark Suite Definition
The harness SHALL load benchmark suites from TOML/YAML manifest files specifying tasks, agents, and success criteria.
**Traces to:** E3.1

### FR-HAR-002: Results Output
The harness SHALL produce structured JSON results with timing, token usage, and correctness metrics per task.
**Traces to:** E3.1

### FR-HAR-003: Concurrent Execution
The harness SHALL support configurable concurrency for running N parallel agent sessions with queue-based task distribution.
**Traces to:** E3.2

## FR-BLD: Build and CI

### FR-BLD-001: Bazel Build
All Rust and TypeScript targets SHALL build successfully via `bazel build //...`.
**Traces to:** E4.1

### FR-BLD-002: CI Checks
PRs SHALL pass: Rust clippy + fmt + tests, TypeScript lint + tests, Bazel build + test, and policy gate validation.
**Traces to:** E4.2
