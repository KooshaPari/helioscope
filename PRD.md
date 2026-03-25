# Product Requirements Document -- heliosCLI

## Product Vision

heliosCLI is a multi-runtime AI coding CLI that unifies Codex, Claude, Gemini, Cursor, and Copilot agents behind a single command-line interface. Built as a Bazel monorepo with a Rust execution core (`codex-rs`) and TypeScript CLI frontend (`codex-cli`), it provides sandboxed code execution, multi-agent orchestration, and a harness system for benchmarking and scaling AI workloads.

## E1: Multi-Agent CLI

### E1.1: Unified Agent Interface

As a developer, I can invoke any supported AI agent (Codex, Claude, Gemini, Cursor, Copilot) through a single `helios` command.

**Acceptance Criteria:**
- `helios run --agent <name>` dispatches to the correct backend
- Agent list is discoverable via `helios agents list`
- Common flags (model, temperature, max-tokens) work across all agents

### E1.2: Interactive and Batch Modes

As a developer, I can use helios interactively (REPL) or in batch mode (piped input, scripts).

**Acceptance Criteria:**
- `helios chat` opens interactive session with selected agent
- `echo "fix bug" | helios run` processes batch input
- Exit codes reflect success/failure for CI integration

## E2: Rust Execution Core (codex-rs)

### E2.1: Sandboxed Code Execution

As a developer, I can execute AI-generated code in a sandboxed environment with filesystem and network restrictions.

**Acceptance Criteria:**
- Sandbox prevents writes outside designated directories
- Network access is configurable (allow/deny per domain)
- Execution timeout with configurable limits

### E2.2: Protocol Layer

As a developer, the CLI communicates with agents via a typed protocol with structured request/response messages.

**Acceptance Criteria:**
- Protocol defined as Rust types with serde serialization
- Schema artifacts generated for TypeScript consumers
- Backward-compatible versioning for protocol changes

## E3: Harness System

### E3.1: Benchmark Harness

As a developer, I can benchmark AI agent performance across standardized coding tasks.

**Acceptance Criteria:**
- Define benchmark suites as TOML/YAML manifests
- Run benchmarks with `helios bench run <suite>`
- Results output as structured JSON with timing, token usage, and correctness metrics

### E3.2: Scaling and Orchestration

As an operator, I can scale harness workloads across multiple concurrent agent sessions.

**Acceptance Criteria:**
- Configurable concurrency (N parallel agent sessions)
- Queue-based task distribution across sessions
- Aggregated results with statistical summaries

## E4: Build and CI

### E4.1: Bazel Monorepo Build

As a contributor, I can build and test all components (Rust, TypeScript) via Bazel.

**Acceptance Criteria:**
- `bazel build //...` succeeds for all targets
- `bazel test //...` runs all test suites
- Incremental builds cache correctly

### E4.2: CI Pipeline

As a contributor, PRs are validated by automated CI checks.

**Acceptance Criteria:**
- Rust: clippy, fmt, cargo test
- TypeScript: biome lint, vitest
- Bazel: build and test all targets
- Policy gate enforces PR conventions
