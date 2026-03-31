# heliosCLI

Rust-based CLI for managing Helioscope applications with multi-backend support and sandboxing. A community fork of [OpenAI Codex CLI](https://github.com/openai/codex) with performance optimizations, a multi-crate harness system, and Phenotype governance integration.

<p align="center">
  <strong>helios</strong> — run AI coding agents locally with full control over execution, sandboxing, and model backends.
</p>

## Architecture Overview

heliosCLI is organized as **two Rust workspaces**:

### Root Workspace (`./Cargo.toml`)

The root workspace contains the **heliosHarness** system — a collection of 18 crates providing validation, caching, checkpointing, discovery, orchestration, and resilience for autonomous agent operations.

```
crates/
├── harness_cache/          # Persistent caching layer
├── harness_checkpoint/     # State checkpointing and recovery
├── harness_discoverer/     # Task and capability discovery
├── harness_elicitation/    # User input elicitation
├── harness_interfaces/     # Shared trait definitions
├── harness_normalizer/     # Input/output normalization
├── harness_orchestrator/   # Sub-agent orchestration
├── harness_pyo3/           # Python interop bindings
├── harness_queue/          # Task queue management
├── harness_rollback/       # Rollback and undo support
├── harness_runner/         # Task execution runner
├── harness_scaling/        # Dynamic scaling logic
├── harness_schema/         # Schema definitions
├── harness_spec/           # Specification parsing
├── harness_teammates/      # Multi-agent coordination
├── harness_utils/          # Shared utilities
└── harness_verify/         # Verification and validation
```

### Helios Workspace (`helios-rs/Cargo.toml`)

The `helios-rs/` workspace contains the **core CLI** and its 60+ crates, providing the full coding agent experience:

```
helios-rs/
├── cli/                    # CLI entry point (helios binary)
├── core/                   # Core agent logic and config
├── tui/                    # Terminal UI (ratatui-based)
├── exec/                   # Non-interactive execution mode
├── app-server/             # WebSocket/stdio app server
├── app-server-protocol/    # App server protocol definitions
├── protocol/               # Wire protocol types
├── config/                 # Configuration loading
├── execpolicy/             # Execution policy engine
├── linux-sandbox/          # Landlock+seccomp sandbox (Linux)
├── mcp-server/             # MCP server implementation
├── login/                  # Authentication (OAuth, API key)
├── secrets/                # Secure credential storage
├── hooks/                  # Pre/post execution hooks
├── skills/                 # Skill definitions
├── state/                  # Session state management
├── file-search/            # Codebase search (nucleo)
├── apply-patch/            # Diff application
├── feedback/               # User feedback collection
├── otel/                   # OpenTelemetry instrumentation
└── utils/                  # Shared utilities (20+ sub-crates)
```

### Key Crates and Responsibilities

| Crate                  | Responsibility                                                                        |
| ---------------------- | ------------------------------------------------------------------------------------- |
| `helios-cli`           | CLI entry point using clap; dispatches to subcommands (exec, tui, mcp, sandbox, etc.) |
| `helios-core`          | Agent core: config loading, feature flags, terminal detection, session management     |
| `helios-tui`           | Interactive terminal UI built on ratatui with streaming responses                     |
| `helios-exec`          | Non-interactive execution mode for scripted/CI usage                                  |
| `helios-app-server`    | Long-running server with stdio/WebSocket transport for IDE integration                |
| `helios-execpolicy`    | File-based execution policies controlling what commands the agent may run             |
| `helios-linux-sandbox` | Landlock + seccomp sandbox for safe command execution on Linux                        |
| `helios-mcp-server`    | Model Context Protocol server for external tool integration                           |
| `helios-config`        | TOML-based configuration with profile support and CLI overrides                       |
| `helios-protocol`      | Wire protocol types for client-server communication                                   |
| `helios-state`         | SQLite-backed session and conversation state persistence                              |
| `helios-login`         | OAuth device code flow and API key authentication                                     |

## Setup Instructions

### System Requirements

| Requirement | Details                                                              |
| ----------- | -------------------------------------------------------------------- |
| OS          | macOS 12+, Ubuntu 20.04+/Debian 10+, or Windows 11 via WSL2          |
| Rust        | Edition 2024 (helios-rs workspace), Edition 2021 (harness workspace) |
| RAM         | 4 GB minimum (8 GB recommended)                                      |
| Git         | 2.23+ for built-in PR helpers (optional)                             |

### Building from Source

```bash
# Clone the repository
git clone https://github.com/KooshaPari/helios-cli.git heliosCLI
cd heliosCLI

# Add upstream for tracking OpenAI Codex changes
git remote add upstream https://github.com/openai/codex.git

# Install Rust toolchain (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source "$HOME/.cargo/env"
rustup component add rustfmt
rustup component add clippy

# Install build helpers
cargo install just
cargo install --locked cargo-nextest  # optional

# Build the harness workspace
cargo build

# Build the main CLI workspace
cd helios-rs
cargo build

# Run the CLI
cargo run --bin helios -- --help
```

### Running Quality Checks

```bash
# Format check
cargo fmt --check

# Lint (zero warnings)
cargo clippy --all-targets -- -D warnings

# Run all tests
cargo test --all
```

### Syncing with Upstream

```bash
# Fetch upstream changes
git fetch upstream

# Sync main branch
git checkout main
git merge upstream/main
git push origin main

# Rebase a feature branch
git checkout helios-optimization
git rebase upstream/main
```

## CLI Usage Examples

### Interactive Mode (Default)

Launch the TUI with an optional prompt:

```bash
helios
helios "explain this codebase to me"
```

### Non-Interactive Execution

Run a single task without the TUI:

```bash
helios exec "add input validation to the login form"
helios exec --json "refactor the config loader"   # JSON output for scripting
```

### Code Review

Review a PR or branch non-interactively:

```bash
helios review --pr 42
```

### Session Management

Resume or fork a previous session:

```bash
helios resume                          # Pick from session list
helios resume --last                   # Resume most recent session
helios resume <session-id>             # Resume specific session
helios fork <session-id>               # Fork a previous session
```

### Authentication

```bash
helios login                           # OAuth device code flow
helios login --device-auth             # Explicit device auth
printenv OPENAI_API_KEY | helios login --with-api-key  # API key via stdin
helios login status                    # Check auth status
helios logout                          # Remove credentials
```

### Sandbox Execution

Run commands within a platform-specific sandbox:

```bash
# Linux (Landlock + seccomp)
helios sandbox linux -- "cat /etc/passwd"

# macOS (Seatbelt)
helios sandbox macos -- "ls -la"

# Windows (restricted token)
helios sandbox windows -- "dir"
```

### MCP Server Management

Manage external MCP (Model Context Protocol) servers:

```bash
helios mcp list
helios mcp add <name> <command>
helios mcp remove <name>
```

Run helios itself as an MCP server:

```bash
helios mcp-server
```

### Feature Flags

Inspect and toggle feature flags:

```bash
helios features list
helios features enable unified_exec
helios features disable shell_tool
```

Enable/disable features at runtime:

```bash
helios --enable web_search_request --disable unified_exec
```

### Shell Completions

Generate shell completions:

```bash
helios completion bash > ~/.local/share/bash-completion/completions/helios
helios completion zsh > ~/.zfunc/_helios
helios completion fish > ~/.config/fish/completions/helios.fish
```

### App Server (IDE Integration)

Run the app server for IDE extension connectivity:

```bash
helios app-server                           # stdio transport (default)
helios app-server --listen ws://127.0.0.1:4500  # WebSocket transport
```

### Apply Patches

Apply the latest diff produced by a helios agent session:

```bash
helios apply
```

## Configuration

Configuration lives in `~/.helios/config.toml` (or `~/.codex/config.toml` for compatibility). Supports profiles:

```toml
[profile.default]
model = "gpt-4o"
approval_policy = "on-request"
sandbox_mode = "workspace-write"

[profile.ci]
model = "gpt-4o-mini"
approval_policy = "auto-edit"
```

Override config from the CLI:

```bash
helios -c model=gpt-4o -c approval_policy=auto-edit
```

## Project Structure

```
heliosCLI/
├── Cargo.toml              # Root workspace (harness crates)
├── Cargo.lock
├── helios-rs/              # Main CLI workspace
│   ├── Cargo.toml
│   └── cli/src/main.rs     # CLI entry point
├── crates/                 # Harness + thegent utility crates
├── docs/                   # Documentation (VitePress site)
│   ├── adrs/               # Architecture decision records
│   ├── specs/              # Feature specifications
│   └── reference/          # Architecture guides
├── .github/workflows/      # CI/CD pipelines
├── AGENTS.md               # Agent operating instructions
└── justfile                # Build/dev task runner
```

## Performance Branches

| Branch           | Focus                |
| ---------------- | -------------------- |
| `helios-cpu-opt` | CPU optimization     |
| `helios-lat-opt` | Latency optimization |
| `helios-mem-opt` | Memory optimization  |

## License

This repository is licensed under the [Apache-2.0 License](LICENSE).
