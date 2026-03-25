# Claude AI Agent Guide - heliosCLI

heliosCLI is a multi-runtime AI coding CLI (Codex, Claude, Gemini, Cursor, Copilot) built with a Bazel monorepo, Rust core (`codex-rs`), and TypeScript CLI (`codex-cli`). It integrates with `thegent` for agent orchestration.

**Authority and Scope**
- This file is the canonical contract for all agent behavior in this repository.
- Act autonomously; only pause when blocked by missing secrets, external access, or truly destructive actions.

---

## Quick Start

```bash
# Build everything
bazel build //...

# Run Rust tests
cargo test --workspace

# Run TypeScript CLI tests
pnpm --filter codex-cli test

# Run a specific Bazel target
bazel run //codex-cli:codex -- --help
```

---

## 1. Core Expectations for Agents

### Autonomous Operation

**Proceed without asking:**
- Implementation details and technical approach
- Adding new CLI flags, commands, or agent integrations
- Refactoring and optimization within existing patterns
- Bug fixes and test improvements
- Documentation updates

**Only ask when blocked by:**
- Missing API keys or secrets
- External service access permissions
- Genuine product ambiguity
- Destructive operations (production data, forced pushes)

### Optionality and Failure Behavior

- **Fail clearly, not silently.** Use explicit failures—not silent degradation or logging-only warnings.
- **Force requirements where they belong.** If a service or config is required for correctness, fail when it is missing.
- **Graceful only via:** retries with visible feedback; error messages listing each failing item; actionable, non-obscure stack traces.

---

## 2. Repository Structure

```
heliosCLI/
├── codex-rs/           # Rust core (exec engine, protocol, sandbox)
│   ├── core/           # Core types, models, config
│   ├── exec/           # Execution engine
│   └── ...
├── codex-cli/          # TypeScript CLI (user-facing commands)
├── helios-rs/          # Helios-specific Rust extensions
├── sdk/                # SDKs for agent integration
├── scripts/            # Dev and CI scripts
├── docs/               # Documentation
├── BUILD.bazel         # Root Bazel build
├── MODULE.bazel        # Bazel module deps
├── Cargo.toml          # Rust workspace
└── pnpm-workspace.yaml # Node workspace
```

---

## 3. Build System (Bazel)

heliosCLI uses Bazel as the primary build system with Cargo and pnpm as secondary.

```bash
# Build all targets
bazel build //...

# Test all targets
bazel test //...

# Build specific target
bazel build //codex-rs/core:core

# Run specific binary
bazel run //codex-cli:codex

# Query targets
bazel query //...
```

### Bazel Rules

- Rust targets use `rules_rust`
- Node targets use `rules_nodejs` / `aspect_rules_js`
- Do not add raw `build.rs` files that bypass Bazel; use `build_script` rules
- Keep `BUILD.bazel` files in sync when adding new source files

---

## 4. Rust (codex-rs)

### Key Patterns

```rust
// Error handling: use anyhow for application code
use anyhow::{Context, Result};

fn example() -> Result<()> {
    let val = operation().context("failed to run operation")?;
    Ok(())
}

// Async: tokio runtime
#[tokio::main]
async fn main() -> anyhow::Result<()> { ... }
```

### Running Rust Checks

```bash
cargo build --workspace
cargo test --workspace
cargo clippy --workspace -- -D warnings
cargo fmt --check
```

---

## 5. TypeScript CLI (codex-cli)

### Key Patterns

```typescript
// Commands use a command registry pattern
// Add new commands in codex-cli/src/commands/

// Error handling: throw with descriptive messages, never swallow
throw new Error(`Failed to connect to agent: ${err.message}`);
```

### Running Node Checks

```bash
pnpm --filter codex-cli build
pnpm --filter codex-cli test
pnpm --filter codex-cli lint
```

---

## 6. CI / Workflows

Key workflows in `.github/workflows/`:

| Workflow | Purpose |
|----------|---------|
| `policy-gate.yml` | PR policy enforcement (composite action) |
| `rust-ci.yml` | Rust lint, test, build |
| `bazel.yml` | Bazel build and test |
| `stage-gates.yml` | Stage-based release gates |
| `ci.yml` | Main CI pipeline |

**Do not inline policy logic in workflows.** Use `KooshaPari/phenotypeActions/actions/policy-gate@main`.

---

## 7. Documentation Organization

Follow `AGENTS.md` for file placement:

| Pattern | Location |
|---------|----------|
| `*QUICK_START*.md` | `docs/guides/quick-start/` |
| `*GUIDE*.md` | `docs/guides/` |
| `*SUMMARY*.md`, `*REPORT*.md`, `PHASE_*.md` | `docs/reports/` |
| `*INDEX*.md`, `*RESEARCH*.md` | `docs/research/` |
| `*CHECKLIST*.md` | `docs/checklists/` |
| `*QUICK_REFERENCE*.md` | `docs/reference/` |

Root-level markdown: only `README.md`, `CHANGELOG.md`, `AGENTS.md`, `CLAUDE.md`.

---

## 8. Worktree Discipline

- Feature work goes in `heliosCLI-wtrees/<topic>/` or `PROJECT-wtrees/<topic>/`
- Canonical `heliosCLI/` stays on `main`
- Never commit feature branches directly to canonical `main`

---

## Quick Reference

| Command | Purpose |
|---------|---------|
| `bazel build //...` | Build all Bazel targets |
| `bazel test //...` | Test all Bazel targets |
| `cargo test --workspace` | Run all Rust tests |
| `cargo clippy --workspace` | Rust linting |
| `pnpm --filter codex-cli test` | TypeScript CLI tests |
