# Lane A Report: codex

Scope: local clone evidence from `/Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/clones/codex` (branch `main`, commit `55fc075723abd039476f4918941570e8a0308a93`).

## 1) Repo constraints and toolchain
- Node and package manager constraints are pinned in `package.json` (`node >=22`, `pnpm >=10.29.3`, pinned `pnpm@10.29.3+...`). (`heliosHarness/clones/codex/package.json:20-24`)
- Rust toolchain is pinned in `codex-rs/rust-toolchain.toml` to version `1.93.0` with `clippy`, `rustfmt`, and `rust-src`. (`heliosHarness/clones/codex/codex-rs/rust-toolchain.toml:1-3`)
- Main Cargo workspace metadata includes resolver/workspace members and global `edition = "2024"` in `codex-rs/Cargo.toml`. (`heliosHarness/clones/codex/codex-rs/Cargo.toml:1-75`)
- Workspace package set includes `codex-cli`, `codex-rs/responses-api-proxy/npm`, `sdk/typescript`, `shell-tool-mcp` in `pnpm-workspace.yaml`. (`heliosHarness/clones/codex/pnpm-workspace.yaml:1-12`)
- Environment minimums and bootstrap steps are documented in `docs/install.md` (OS + RAM + rustup/tooling install). (`heliosHarness/clones/codex/docs/install.md:3-10`, `20-30`)
- Local AGENTS workflow requirements are explicit: scoped `just fmt`, scoped tests, `just fix -p <crate>`, and avoiding routine `--all-features`. (`heliosHarness/clones/codex/AGENTS.md:24-30`, `27-31`)

## 2) Quality command surface
- Rust workflow commands (`codex-rs/justfile`):
  - `just fmt` (`cargo fmt -- --config imports_granularity=Item`). (`codex-rs/justfile:26-28`)
  - `just fix -p <crate>` (`cargo clippy --fix --tests`). (`codex-rs/justfile:30-31`)
  - `just clippy` (`cargo clippy --tests`). (`codex-rs/justfile:33-34`)
  - `just test` (`cargo nextest run --no-fail-fast`). (`codex-rs/justfile:46-47`)
- Root JS checks (`package.json`): `pnpm run format` and `pnpm run format:fix`. (`heliosHarness/clones/codex/package.json:6-8`)
- SDK package checks (`sdk/typescript/package.json`): `lint`, `test`, `build`, `format`, `format:fix`. (`heliosHarness/clones/codex/sdk/typescript/package.json:34-45`)
- Shell MCP checks (`shell-tool-mcp/package.json`): `format`, `test`, `build`, `format:fix`. (`heliosHarness/clones/codex/shell-tool-mcp/package.json:22-30`)
- AGENTS adds API/schema lifecycle checks for API surface changes: `just write-config-schema`, `just write-app-server-schema`, plus tests in `codex-app-server-protocol`. (`heliosHarness/clones/codex/AGENTS.md:17`, `77-83`, `167-171`)

## 3) CI quality gates and strictness posture
- `ci.yml`: frozen dependency install, README ASCII/ToC checks, prettier format check. (`.github/workflows/ci.yml:27-30`, `55-66`)
- `rust-ci.yml`:
  - change-set detection with conditional jobs (`general`, `cargo_shear`, `lint_build`, `tests`). (`.github/workflows/rust-ci.yml:12-18`, `52-58`, `86-101`, `453-458`)
  - formatter check, lint/build, and tests are matrixed across OS/targets with hard failure defaults and `-D warnings`. (`.github/workflows/rust-ci.yml:65-67`, `390-392`, `595-598`)
  - required aggregator job is explicit (`results` gate). (`.github/workflows/rust-ci.yml:653-680`)
- `shell-tool-mcp-ci.yml`: package format/test/build. (`.github/workflows/shell-tool-mcp-ci.yml:41-48`)
- `sdk.yml`: Rust binary build + SDK build/lint/test. (`.github/workflows/sdk.yml:34-50`)
- `cargo-deny.yml`: dependency/license policy checks. (`.github/workflows/cargo-deny.yml:22-24`)
- `codespell.yml`: spelling check gate with problem matcher/action. (`.github/workflows/codespell.yml:14-27`)

## 4) Governance status
- Contribution model is invitation-only; unsolicited PRs are explicitly rejected in `docs/contributing.md`. (`heliosHarness/clones/codex/docs/contributing.md:3-18`)
- Invited contributors are still required to run repo checks and tests before PR and include behavior/UX notes for user-visible changes. (`heliosHarness/clones/codex/docs/contributing.md:31-60`)
- CLA flow is enforced with `contributor-assistant` workflow and explicit `CLA.md` terms. (`heliosHarness/clones/codex/.github/workflows/cla.yml:14-47`, `heliosHarness/clones/codex/docs/CLA.md:5-12`)
- Security policy references direct reporting and Bugcrowd program. (`heliosHarness/clones/codex/SECURITY.md:5-13`)

## 5) API surface and protocol contracts
- Rust CLI core surface includes `codex exec`, sandbox modes, MCP entrypoints (`codex mcp`, `codex mcp-server`) and major crate decomposition. (`heliosHarness/clones/codex/codex-rs/README.md:21-45`, `90-99`)
- App-server defines JSON-RPC 2.0 with stdio/websocket transport and method naming conventions (`<resource>/<method>`). (`heliosHarness/clones/codex/codex-rs/app-server/README.md:22-30`, `138-149`)
- App API method footprint includes thread/turn/model/skills/config/command/mcp hooks and explicit notification/event behavior. (`heliosHarness/clones/codex/codex-rs/app-server/README.md:120-160`)
- Schema/test lifecycle for app-server protocol changes is documented (`generate-ts`, `generate-json-schema`, schema fixtures + tests). (`heliosHarness/clones/codex/codex-rs/app-server/README.md:44-49`, `168-171`)
- TypeScript SDK wraps CLI transport and exposes thread run/streamed events plus structured output. (`heliosHarness/clones/codex/sdk/typescript/README.md:5-13`, `34-51`, `53-70`)
- HTTP proxy package is explicitly constrained to `POST /v1/responses` only. (`heliosHarness/clones/codex/codex-rs/responses-api-proxy/README.md:3-4`, `35-37`, `63-64`)

## 6) Strictness mapping and task-quality comparison

### Task-Quality Level 1 — Localized/Dev Surface
- Scope: docs, isolated package changes, non-API refactors.
- Required checks: `just fmt`, scoped tests/build for touched package, optional formatter only.
- Evidence source: AGENTS local workflow guidance and package scripts. (`heliosHarness/clones/codex/AGENTS.md:24-30`, `codex-rs/justfile:26-47`, `heliosHarness/clones/codex/sdk/typescript/package.json:34-45`)

### Task-Quality Level 2 — Surface-Coupled/API-Aware
- Scope: non-breaking API surface updates, config/schema changes, or behavior changes with cross-package impact.
- Required checks: Level 1 + schema update tasks + relevant protocol/SDK tests.
- Evidence source: AGENTS schema tasks and app-server protocol docs. (`heliosHarness/clones/codex/AGENTS.md:16-17`, `77-83`, `167-171`; `heliosHarness/clones/codex/codex-rs/app-server/README.md:44-49`)

### Task-Quality Level 3 — High-Risk/Release Critical
- Scope: protocol mutation, core protocol/engine changes, dependency updates, release-impacting paths.
- Required checks: `just fmt`, `just fix`, scoped or full `cargo clippy` as applicable, full `cargo nextest` matrix, CI required jobs (`format/cargo_shear/lint_build/tests`) and merge-gate `results`.
- Evidence source: justfile + rust-ci matrix + required results job. (`heliosHarness/clones/codex/codex-rs/justfile:27-47`, `heliosHarness/clones/codex/.github/workflows/rust-ci.yml:86-92`, `heliosHarness/clones/codex/.github/workflows/rust-ci.yml:520-598`, `heliosHarness/clones/codex/.github/workflows/rust-ci.yml:653-680`)

### Level comparison against Codex lane behavior
- This lane most reliably supports **L3** for protocol and core engine changes.
- For isolated package changes, **L1** is usually sufficient if scoped checks pass.
- For schema/config/API coupling work, require **L2**.

## 7) Readiness score
- **84 / 100**
- Strengths: explicit pinned toolchain, strict lint/format/test CI matrix, API naming/schema conventions, and explicit security/CLA governance.
- Gaps: strictness is highly area-dependent; if a change is mis-classified below L3, enforcement can be unintentionally under-applied.
