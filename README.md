# heliosCLI

[![Build](https://img.shields.io/github/actions/workflow/status/KooshaPari/heliosCLI/ci.yml?branch=main&label=build)](https://github.com/KooshaPari/heliosCLI/actions)
[![Release](https://img.shields.io/github/v/release/KooshaPari/heliosCLI?include_prereleases&sort=semver)](https://github.com/KooshaPari/heliosCLI/releases)
[![License](https://img.shields.io/github/license/KooshaPari/heliosCLI)](LICENSE)
[![Phenotype](https://img.shields.io/badge/Phenotype-org-blueviolet)](https://github.com/KooshaPari)
[![AI Slop Inside](https://sladge.net/badge.svg)](https://sladge.net)

Rust-based CLI for managing Helioscope applications with multi-backend support and sandboxing. A community fork of [OpenAI Codex CLI](https://github.com/openai/codex) with performance optimizations, a multi-crate harness system, and Phenotype governance integration.

<p align="center">
  <strong>helios</strong> — run AI coding agents locally with full control over execution, sandboxing, and model backends.
</p>

## Architecture Overview

heliosCLI is organized around the active `codex-rs` Rust workspace, the `codex-cli` TypeScript CLI,
and Bazel monorepo rules:

### Rust Workspace (`codex-rs/Cargo.toml`)

The Rust workspace contains the core agent/runtime crates for validation, orchestration, and
resilience for autonomous agent operations.

```
crates/
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

**Note:** Additional crates in `crates/` (harness_cache, harness_checkpoint, harness_discoverer, harness_elicitation, harness_interfaces, harness_normalizer, harness_orchestrator, harness_pyo3, harness_mojo, harness_zig, thegent-*, adrs, api, arch_test, changes, governance) are utility, documentation, or test crates not included in the root workspace members list.

### CLI Workspace (`codex-cli/package.json`)

The `codex-cli/` workspace contains the TypeScript CLI and supporting tooling used for the
interactive agent experience:

```
codex-rs/
├── cli/                    # Rust CLI entry point
├── core/                   # Core agent logic and config
├── tui/                    # Terminal UI
├── exec/                   # Non-interactive execution mode
├── protocol/               # Wire protocol types
├── config/                 # Configuration loading
├── execpolicy/             # Execution policy engine
├── mcp-server/             # MCP server implementation
├── login/                  # Authentication (OAuth, API key)
├── secrets/                # Secure credential storage
├── hooks/                  # Pre/post execution hooks
├── state/                  # Session state management
├── file-search/            # Codebase search
├── apply-patch/            # Diff application
├── feedback/               # User feedback collection
└── utils/                  # Shared utilities
```

### Key Crates and Responsibilities

| Crate                  | Responsibility                                                                        |
| ---------------------- | ------------------------------------------------------------------------------------- |
| `codex-cli`            | TypeScript CLI entry point and command registry                                       |
| `codex-core`           | Agent core: config loading, terminal detection, session management                   |
| `codex-tui`            | Interactive terminal UI with streaming responses                                     |
| `codex-exec`           | Non-interactive execution mode for scripted/CI usage                                  |
| `codex-protocol`       | Wire protocol types for client-server communication                                   |
| `codex-state`          | Session and conversation state persistence                                            |
| `codex-login`          | OAuth device code flow and API key authentication                                     |

## Setup Instructions

### System Requirements

| Requirement | Details                                                              |
| ----------- | -------------------------------------------------------------------- |
| OS          | macOS 12+, Ubuntu 20.04+/Debian 10+, or Windows 11 via WSL2          |
| Rust        | Edition 2024 (codex-rs workspace), Edition 2021 (harness workspace) |
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

# Build the Rust workspace
cd codex-rs
cargo build

# Build the CLI workspace
cd ../codex-cli
npm install
npm run build
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
├── codex-rs/               # Main Rust workspace
│   ├── Cargo.toml
│   └── cli/src/main.rs     # CLI entry point
├── codex-cli/              # TypeScript CLI workspace
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
