# Lane F Governance Mapping

Scope: Governance and quality mapping for CLI candidates (`codex`, `opencode`, `goose`, `kilocode`).
Date: 2026-02-22

## 1) Max-Strictness Equivalence

Use this equivalence when choosing which checks become the maximum gate for each repo.

| Repo (target) | Evidence source | Strictness level | Max-strictness definition (equivalent in harness) | Notes |
|---|---|---|---|---|
| codex | `heliosHarness/clones/codex/AGENTS.md`, `heliosHarness/clones/codex/.github/workflows/rust-ci.yml` | **High** | Equivalent of: format + lint+compile verification + multi-target test/build + release-hardening awareness | Rust CI includes changed-path detection, `cargo fmt --check`, `cargo test` across multiple OS/targets, and release-oriented build matrix controls. |
| opencode | `heliosHarness/clones/opencode/.github/workflows/build.yml`, `heliosHarness/clones/opencode/.github/workflows/release.yml`, `API/research/opencode/AGENTS.md` and workflows | **Medium** | Equivalent of: build/release pipeline + dependency hygiene, with explicit typecheck/tests in API parent but no direct lint/format gate in core opencode clone workflows | `clones/opencode` has release/build workflow only; parent AGENTS points to broader TypeScript quality checks not tightly enforced in clone CI. |
| goose | `API/research/goose/.github/workflows/ci.yml`, `API/research/goose/documentation/package.json`, `API/research/goose/ui/desktop/package.json` | **High** | Equivalent of: Rust format + Rust test matrix + clippy-style lint script + UI lint/test + doc/package checks | Core repo-level CI shows full Rust + frontend quality path with build/test/lint jobs. |
| kilocode | `heliosHarness/clones/kilocode/package.json`, `heliosHarness/clones/kilocode/.github/workflows/code-qa.yml` (mirrors `API/research/kilocode`) | **High** | Equivalent of: multi-surface install/format/lint/typecheck/test/CLI integration in a monorepo | Shared monorepo policy runs build, lint, typecheck, unit/e2e tests for extension, webview, cli, and packaging workflows. |

## 2) Command Mapping (evidence-backed)

### codex
- Install/bootstrap: project-specific dependency installation follows Rust-era tooling + monorepo workspace conventions; AGENTS emphasizes running `just`-based and targeted commands only when needed.
- Build/test quality: `cargo test` (platform matrix, dev+release profiles), `cargo fmt -- --config imports_granularity=Item --check`, `cargo clippy` via rust toolchain jobs.
- CI-only/strictness controls: `cargo-shear`, fail-fast override settings, cross-target matrix strategy.

### opencode
- Child baseline (clone): `go mod download`; `goreleaser build --snapshot --clean`; `goreleaser release --clean`.
- Parent API checks (secondary): `bun typecheck`; `bun turbo typecheck`; `bun turbo test` (workflow), plus CI-triggered test context.
- Governance mapping: build/release is baseline enforced; broader type/test checks from parent used as additional quality evidence where possible.

### goose
- Parent API baseline (clone `heliosHarness/clones/goose` is broken/unusable):
  - Rust path: `cargo fmt --check`; `cargo test -- --skip scenario_tests::scenarios::tests`; `cargo test --jobs 1 scenario_tests::scenarios::tests`; clippy path via `./scripts/clippy-lint.sh`.
  - Desktop/UI path: `npm run lint:check` and `npm run test:run` (working directory `ui/desktop`).
  - Docs/package path: `npm run typecheck` / `npm run build` in doc and app package scripts under parent repo.

### kilocode
- `pnpm install`
- `pnpm build`
- `pnpm check-types`
- `pnpm lint`
- `pnpm test`
- CLI-specific publish quality path: `pnpm run cli:bundle`, `pnpm --filter @kilocode/cli test:integration` (publish workflow), `npm pack`.

## 3) Parent/Child Repository Quality Fallback Policy

1. **Prefer child local source of truth first** (clone scope): child AGENTS + child workflows + child package scripts.
2. **Fallback to API/research parent only when**:
   - clone is incomplete/broken (e.g., `heliosHarness/clones/goose` broken metadata and missing usable AGENTS in cloned scope), or
   - child governance files are absent/missing for the requested question.
3. **Cross-validate on conflicts**:
   - If child and parent conflict on strictness evidence, keep the strictest executable interpretation that is directly evidenced.
4. **Confidence tagging**:
   - Mark any fallback-derived assertion as `degraded-confidence` so downstream scoring does not treat it as fully authoritative.
5. **Evidence precedence for lane F**:
   - `AGENTS.md` and workflow files are strongest for governance/operational rules.
   - package scripts are secondary for command surface.
   - status notes from discovery artifacts are tie-breakers for accessibility and stale snapshots.

## 4) Non-Blocking Failure Strategy

To keep lane F and downstream synthesis resilient, treat gate outcomes as follows:

- `PASS`: command exists and is executed successfully.
- `WARN`: command missing, command source absent in child, or command fails in soft-band area (tests/typecheck/lint when repo strictness is Medium or when using fallback).
- `BLOCK`: only when hard governance evidence is missing entirely (no reachable command source in either clone or parent), or when child/parent explicitly marks a command as mandatory and it fails.

Operationally:
- Execute checks with soft-fail semantics (`continue-on-error` style) unless a command is declared mandatory for the repo's max-strictness tier.
- Record failures in the evidence schema as structured, actionable deltas:
  - missing command
  - command timed out
  - command present but non-deterministic
  - command not yet mapped to parent fallback
- Promotion logic:
  - `High` strictness repos default to strict hard-stop on critical build/test commands,
  - `Medium` strictness repos default to warn-and-continue for CI quality shortfalls,
  - `High` fallback confidence requires no unclosed `WARN` before synthesis closeout.
