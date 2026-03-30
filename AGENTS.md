<!-- Base: platforms/thegent/governance/AGENTS.base.md -->
<!-- Last synced: 2026-03-29 -->

# AGENTS.md — heliosCLI

Extends thegent governance base. See `platforms/thegent/governance/AGENTS.base.md` for canonical definitions of agent expectations, testing requirements, research patterns, and standard operating procedures.

## Project Identity & Work Management

### Project Overview

- **Name**: heliosCLI
- **Description**: Rust-based CLI for managing Helioscope applications with multi-backend support and sandboxing
- **Location**: `/Users/kooshapari/CodeProjects/Phenotype/repos/heliosCLI`
- **Language Stack**: Rust (edition 2021)
- **Published**: Internal

### AgilePlus Integration

All work MUST be tracked in AgilePlus:
- Reference: `/Users/kooshapari/CodeProjects/Phenotype/repos/AgilePlus`
- CLI: `cd AgilePlus && agileplus <command>`
- Specs: `AgilePlus/kitty-specs/<feature-id>/`
- Worklog: `AgilePlus/.work-audit/worklog.md`

**Requirements**:
1. Check for AgilePlus spec before implementing
2. Create spec for new work: `agileplus specify --title "<feature>"`
3. Update work package status as work progresses
4. No code without corresponding AgilePlus spec

---

## Repository Mental Model

### Project Structure

```
src/
  main.rs               # CLI entrypoint using clap
  commands/             # Command handlers by domain
    mod.rs              # Command registry
    app.rs              # App management commands
    config.rs           # Configuration commands
    execution.rs        # Execution backend commands
  config/               # Configuration loading and parsing
    mod.rs              # Config loader
  execution/            # Backend executors
    mod.rs              # Executor trait
    docker.rs           # Docker executor
    kubernetes.rs       # Kubernetes executor
    local.rs            # Local executor
    sandbox.rs          # Sandbox executor
  app/                  # Core application logic
  models/               # Data models (App, Config, etc.)
  utils/                # Shared utilities
  errors.rs             # Error types using thiserror

tests/                  # Integration tests
docs/
  adr/                  # Architecture decision records
  sessions/             # Session-based work documentation
  reference/            # Architecture docs and quick references
```

### Style Constraints

- **Line length**: 100 characters (Rust convention)
- **Formatter**: `cargo fmt` (mandatory)
- **Type checker**: Rust compiler (strict)
- **Linter**: `cargo clippy` with `-- -D warnings` (zero warnings)
- **File size target**: ≤350 lines per source file, hard limit ≤500 lines
- **Typing**: Full type annotations required; no `impl Trait` in public APIs

### Key Constraints

- All CLI commands must use `clap` for argument parsing
- All backends must implement a common executor trait
- Error handling via `thiserror` with clear error types
- Async code uses `tokio` runtime
- No global state; dependency injection for configuration
- Tests verify both happy path and error conditions

---

## Session Documentation

All agents MUST maintain session documentation for research, decisions, and findings:

### Location

- Default: `docs/sessions/<session-id>/`

### Standard Session Structure

```
docs/sessions/<session-id>/
├── README.md           # Overview and context
├── 01_RESEARCH.md      # Findings and analysis
├── 02_PLAN.md          # Design and approach
├── 03_IMPLEMENTATION.md # Code changes and rationale
├── 04_VALIDATION.md    # Tests and verification
└── 05_KNOWN_ISSUES.md  # Blockers and follow-ups
```

---

## Quality Standards

### Code Quality Mandate

- **All linters must pass**: `cargo clippy --all-targets -- -D warnings`
- **All tests must pass**: `cargo test --all`
- **No AI slop**: Avoid placeholder TODOs, lorem ipsum, generic comments
- **Backwards incompatibility**: No shims, full migrations, clean breaks

### Test-First Mandate

- **For NEW modules**: test file MUST exist before implementation file
- **For BUG FIXES**: failing test MUST be written before the fix
- **For REFACTORS**: existing tests must pass before AND after

### FR Traceability

All tests MUST reference a Functional Requirement (FR):

```rust
// Traces to: FR-HELIOS-NNN
#[test]
fn test_feature_name() {
    // Test body
}
```

---

## Governance Reference

See thegent governance base for complete guidance on:

1. **Core Agent Expectations** — Autonomous operation, when to ask vs. decide
2. **Standard Operating Loop (SWE Autopilot)** — Review, Research, Plan, Execute, Size-Check, Test, Review & Polish, Repeat
3. **File Size & Modularity Mandate** — ≤500 line hard limit, decomposition patterns
4. **Research-First Development** — Codebase research, web research, documentation
5. **Branch Discipline** — Worktree usage, PR workflow, git best practices
6. **Child-Agent and Delegation Policy** — When to spawn subagents, parallel vs. sequential
7. **Tool Usage & CLI Priority** — CLI as primary interface, read-only tools first
8. **Naming Conventions** — Session naming, file naming, branch naming

Location: `platforms/thegent/governance/AGENTS.base.md`

---

## Quick Reference Commands

```bash
# Run all quality checks
cargo test --all
cargo clippy --all-targets -- -D warnings
cargo fmt --check

# Auto-format code
cargo fmt

# Run specific test
cargo test <test_name>

# Build and run
cargo build
./target/debug/helios-cli --help

# Build release
cargo build --release

# View documentation
cargo doc --open
```
