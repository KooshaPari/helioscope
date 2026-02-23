# Merged Fragmented Markdown

## Source: research/harness-spec.md

# CLI/API/SDK Harness Spec (Phase-1 Draft)

## Purpose
Create a reusable, evidence-driven harness to evaluate OSS CLI/API/SDK repos on two axes:
- **Functionality**: install/build/test/runtime behavior.
- **Quality parity**: strictness and governance equivalence to project rules (`strict`, `strict-full`, `max-strictness`, `no-skip`).

## Target contract
The harness must support:
- CLI repos with mixed ecosystems (`go`, `rust`, `node`, `python`).
- API/SDK adjunct repos that ship CLI-compatible adapters.
- Parent/child repository dependency (e.g., child project missing but parent quality evidence exists).
- Non-blocking execution: strict failures escalate only when hard-gate semantics are clear.

## Canonical command profile model
For each repo, map observed commands into five buckets:
1. **Bootstrap**: clone/checkout/install/bootstrap.
2. **Static quality**: format/lint/typecheck/vet.
3. **Behavioral checks**: unit/integration tests.
4. **Build/packaging**: binary/build artifacts/publish staging.
5. **API artifacts**: schema/build docs/openapi/client generation.

### Strictness equivalence mapping
- `strict-full-noskip` (target): fail on any command or missing mandatory check; equivalent to repo-local `quality`, `task quality`, or enforced workflow matrix.
- `strict` (target): allow documented non-blocking external dependencies but no skipped mandatory checks.
- `strict-light`: warning-based when explicit evidence is incomplete.

## Harness architecture

### Modules
- `discoverer`
  - Detect repository root, toolchain, commands, workflow inventory, and AGENTS/contributing guidance.
- `runner`
  - Execute mapped commands in isolated profiles:
    - `bootstrap`
    - `quality`
    - `api`
    - `runtime`
- `quality-normalizer`
  - Normalize command naming into the canonical profile model.
  - Promote local project profiles:
    - `task quality`
    - `quality:strict`
    - `task test`
- `result-schema`
  - Emit evidence bundle with:
    - repo, branch, commit hash, remote
    - commands executed/failed
    - check timestamps
    - strictness level
    - skip rationale (if any)
- `comparator`
  - Score and rank for harness eligibility and shortlist.

### Data model (minimum fields)
- `repo_id`, `path`, `remote`, `branch`, `commit`, `toolchain`, `commands`, `quality_profile`, `quality_results`, `api_surface`, `mcp_surface`, `auth_profile`, `parent_repo`.
- `result_code`: PASS/WARN/FAIL, with reason code taxonomy.

## Multi-repo policy (parent + child)
- If child repo is missing/malformed, run parent repo quality profile and mark child fields as **inherited**.
- If both parent and child have quality evidence, both are recorded; child errors do not erase parent score.
- Parent fallback is allowed only with explicit justification in `result_code=WARN` unless a child hard-gate is explicitly required.

## Candidate onboarding workflow
1. `clone` and `status --short` each target.
2. Capture branch + commit + remote for traceability.
3. Detect quality commands and map into profile buckets.
4. Execute strict profile command set in canonical order:
   - bootstrap
   - quality
   - tests
   - build
   - api
5. Emit per-repo artifact under `artifacts/`.
6. Emit consolidated matrix and governance summary for phase handoff.

## Next phase design intent
Phase-2 should add:
- machine-readable schema for `harness-manifest.json`.
- CI dry-run runner with capped concurrency.
- strictness policy file (`harness/strictness.yaml`) to avoid manual interpretation.
- artifact contract tests (schema + command coverage).

---

## Source: research/helios-consolidated-evidence.md

# Helios Harness Non-Markdown Evidence Consolidated

> Generated: 2026-02-22T10:45:12Z

## Scope
- Source roots: `wbs`, `research` subtrees under `heliosHarness`
- Included file types: `*.json`, `*.yml`, `*.yaml`, `*.txt`, `*.toml`, `*.csv`, `*.md` excluded here (already in `helios-consolidated.md`)

## Source Inventory

- `research/phase-2-reports/agent-a-core-repo-harden/artifacts/branch-topology.txt`
- `research/phase-2-reports/agent-a-core-repo-harden/artifacts/commit-provenance.txt`
- `research/phase-2-reports/agent-a-core-repo-harden/artifacts/final-cleanliness.txt`
- `research/phase-2-reports/agent-a-core-repo-harden/artifacts/final-closeout.txt`
- `research/phase-2-reports/agent-a-core-repo-harden/artifacts/lockfile-inventory.txt`
- `research/phase-2-reports/agent-a-core-repo-harden/artifacts/lockfile-parity.txt`
- `research/phase-2-reports/agent-a-core-repo-harden/artifacts/repo-manifest.txt`
- `research/phase-2-reports/agent-a-core-repo-harden/artifacts/strictness-surface.txt`
- `research/phase-2-reports/agent-a-core-repo-harden/artifacts/task-metadata.txt`
- `research/phase-2-reports/agent-a-core-repo-harden/artifacts/unresolved-risks.txt`
- `research/phase-2-reports/agent-d-harness-architecture/artifacts/discovery-codex.json`
- `research/phase-2-reports/agent-e-validation-automation/artifacts/e2-command-map-schema.yaml`
- `research/phase-2-reports/agent-e-validation-automation/artifacts/e6-failure-budget.yaml`
- `wbs/phase-1.json`
- `wbs/phase-2.json`

---

# Consolidated Evidence



<!-- SOURCE: research/phase-2-reports/agent-a-core-repo-harden/artifacts/branch-topology.txt -->

# branch-topology.txt

```text
== codex ==
origin	https://github.com/openai/codex.git (fetch)
origin	https://github.com/openai/codex.git (push)
main
55fc075

== opencode ==
origin	https://github.com/opencode-ai/opencode.git (fetch)
origin	https://github.com/opencode-ai/opencode.git (push)
main
73ee493

== kilocode ==
origin	/Users/kooshapari/temp-PRODVERCEL/485/API/research/kilocode (fetch)
origin	/Users/kooshapari/temp-PRODVERCEL/485/API/research/kilocode (push)
main
1440d1986d

== goose ==
origin	https://github.com/block/goose.git (fetch)
origin	https://github.com/block/goose.git (push)
main
66d075050ed

== cliproxyapi-plusplus ==
origin	https://github.com/router-for-me/CLIProxyAPI.git (fetch)
origin	https://github.com/router-for-me/CLIProxyAPI.git (push)
main
f80eed6


```



<!-- SOURCE: research/phase-2-reports/agent-a-core-repo-harden/artifacts/commit-provenance.txt -->

# commit-provenance.txt

```text
== codex ==
local: 55fc075 Send events to realtime api (#12423)
status: ## main...origin/main

== opencode ==
local: 73ee493 docs(readme): update archive note
status: ## main...origin/main

== kilocode ==
local: 1440d1986d Merge pull request #4300 from Kilo-Org/codingelves-update-kilo-naming
status: ## main...origin/main

== goose ==
local: 66d075050ed blog: typo fixes (#5896)
status: ## main...origin/main
 M AGENTS.md
 D documentation/AGENTS.md
?? CLAUDE.md
?? WARP.md

== cliproxyapi-plusplus ==
local: f80eed6 docs: Add CLIProxyAPI Phase 7.2 modifications audit report
status: ## main...origin/main [ahead 6, behind 62]
 D AUDIT_AND_FIXES_REPORT.md
 D AUGGIE_CLI_COMMAND_REFERENCE.md
 D AUGGIE_CURSOR_STREAMING_ANALYSIS.md
 D AUGGIE_EXECUTOR_IMPLEMENTATION_COMPLETE.md


```



<!-- SOURCE: research/phase-2-reports/agent-a-core-repo-harden/artifacts/final-cleanliness.txt -->

# final-cleanliness.txt

```text
## main...origin/main
## main...origin/main
## main...origin/main
## main...origin/main
## main...origin/main [ahead 6, behind 62]

---

## Source: research/helios-consolidated-index.md

# Helios Harness Consolidation Index

> Regenerated: 2026-02-22T11:22:30Z

## Scope
- `research/helios-consolidated.md` (markdown artifacts)
- `research/helios-consolidated-evidence.md` (non-markdown evidence artifacts)

## Markdown Sources

- A1.md
  - Source: `A1.md` from `helios-consolidated.md` line ~161
  - Link: [A1.md](research/helios-consolidated.md#a1md)
- A10.md
  - Source: `A10.md` from `helios-consolidated.md` line ~175
  - Link: [A10.md](research/helios-consolidated.md#a10md)
- A2.md
  - Source: `A2.md` from `helios-consolidated.md` line ~189
  - Link: [A2.md](research/helios-consolidated.md#a2md)
- A3.md
  - Source: `A3.md` from `helios-consolidated.md` line ~203
  - Link: [A3.md](research/helios-consolidated.md#a3md)
- A4.md
  - Source: `A4.md` from `helios-consolidated.md` line ~217
  - Link: [A4.md](research/helios-consolidated.md#a4md)
- A5.md
  - Source: `A5.md` from `helios-consolidated.md` line ~231
  - Link: [A5.md](research/helios-consolidated.md#a5md)
- A6.md
  - Source: `A6.md` from `helios-consolidated.md` line ~245
  - Link: [A6.md](research/helios-consolidated.md#a6md)
- A7.md
  - Source: `A7.md` from `helios-consolidated.md` line ~259
  - Link: [A7.md](research/helios-consolidated.md#a7md)
- A8.md
  - Source: `A8.md` from `helios-consolidated.md` line ~273
  - Link: [A8.md](research/helios-consolidated.md#a8md)
- A9.md
  - Source: `A9.md` from `helios-consolidated.md` line ~287
  - Link: [A9.md](research/helios-consolidated.md#a9md)
- B1.md
  - Source: `B1.md` from `helios-consolidated.md` line ~301
  - Link: [B1.md](research/helios-consolidated.md#b1md)
- B10.md
  - Source: `B10.md` from `helios-consolidated.md` line ~315
  - Link: [B10.md](research/helios-consolidated.md#b10md)
- B2.md
  - Source: `B2.md` from `helios-consolidated.md` line ~329
  - Link: [B2.md](research/helios-consolidated.md#b2md)
- B3.md
  - Source: `B3.md` from `helios-consolidated.md` line ~343
  - Link: [B3.md](research/helios-consolidated.md#b3md)
- B4.md
  - Source: `B4.md` from `helios-consolidated.md` line ~357
  - Link: [B4.md](research/helios-consolidated.md#b4md)
- B5.md
  - Source: `B5.md` from `helios-consolidated.md` line ~371
  - Link: [B5.md](research/helios-consolidated.md#b5md)
- B6.md
  - Source: `B6.md` from `helios-consolidated.md` line ~385
  - Link: [B6.md](research/helios-consolidated.md#b6md)
- B7.md
  - Source: `B7.md` from `helios-consolidated.md` line ~399
  - Link: [B7.md](research/helios-consolidated.md#b7md)
- B8.md
  - Source: `B8.md` from `helios-consolidated.md` line ~413
  - Link: [B8.md](research/helios-consolidated.md#b8md)
- B9.md
  - Source: `B9.md` from `helios-consolidated.md` line ~427
  - Link: [B9.md](research/helios-consolidated.md#b9md)
- C1.md
  - Source: `C1.md` from `helios-consolidated.md` line ~441
  - Link: [C1.md](research/helios-consolidated.md#c1md)
- C10.md
  - Source: `C10.md` from `helios-consolidated.md` line ~455
  - Link: [C10.md](research/helios-consolidated.md#c10md)
- C2.md
  - Source: `C2.md` from `helios-consolidated.md` line ~469
  - Link: [C2.md](research/helios-consolidated.md#c2md)
- C3.md
  - Source: `C3.md` from `helios-consolidated.md` line ~483
  - Link: [C3.md](research/helios-consolidated.md#c3md)
- C4.md
  - Source: `C4.md` from `helios-consolidated.md` line ~497
  - Link: [C4.md](research/helios-consolidated.md#c4md)
- C5.md
  - Source: `C5.md` from `helios-consolidated.md` line ~511
  - Link: [C5.md](research/helios-consolidated.md#c5md)
- C6.md
  - Source: `C6.md` from `helios-consolidated.md` line ~525
  - Link: [C6.md](research/helios-consolidated.md#c6md)
- C7.md
  - Source: `C7.md` from `helios-consolidated.md` line ~539
  - Link: [C7.md](research/helios-consolidated.md#c7md)
- C8.md
  - Source: `C8.md` from `helios-consolidated.md` line ~553
  - Link: [C8.md](research/helios-consolidated.md#c8md)
- C9.md
  - Source: `C9.md` from `helios-consolidated.md` line ~567
  - Link: [C9.md](research/helios-consolidated.md#c9md)
- D1.md
  - Source: `D1.md` from `helios-consolidated.md` line ~581
  - Link: [D1.md](research/helios-consolidated.md#d1md)
- D10.md
  - Source: `D10.md` from `helios-consolidated.md` line ~595
  - Link: [D10.md](research/helios-consolidated.md#d10md)
- D2.md
  - Source: `D2.md` from `helios-consolidated.md` line ~609
  - Link: [D2.md](research/helios-consolidated.md#d2md)
- D3.md
  - Source: `D3.md` from `helios-consolidated.md` line ~623
  - Link: [D3.md](research/helios-consolidated.md#d3md)
- D4.md
  - Source: `D4.md` from `helios-consolidated.md` line ~637
  - Link: [D4.md](research/helios-consolidated.md#d4md)
- D5.md
  - Source: `D5.md` from `helios-consolidated.md` line ~651
  - Link: [D5.md](research/helios-consolidated.md#d5md)
- D6.md
  - Source: `D6.md` from `helios-consolidated.md` line ~665

---

## Source: research/helios-consolidated.md

# Helios Harness Research Consolidated

> Generated: 2026-02-22T10:42:45Z

## Source Inventory


### wbs/phase-1-reports

- `wbs/phase-1-reports/A1.md`
- `wbs/phase-1-reports/A10.md`
- `wbs/phase-1-reports/A2.md`
- `wbs/phase-1-reports/A3.md`
- `wbs/phase-1-reports/A4.md`
- `wbs/phase-1-reports/A5.md`
- `wbs/phase-1-reports/A6.md`
- `wbs/phase-1-reports/A7.md`
- `wbs/phase-1-reports/A8.md`
- `wbs/phase-1-reports/A9.md`
- `wbs/phase-1-reports/B1.md`
- `wbs/phase-1-reports/B10.md`
- `wbs/phase-1-reports/B2.md`
- `wbs/phase-1-reports/B3.md`
- `wbs/phase-1-reports/B4.md`
- `wbs/phase-1-reports/B5.md`
- `wbs/phase-1-reports/B6.md`
- `wbs/phase-1-reports/B7.md`
- `wbs/phase-1-reports/B8.md`
- `wbs/phase-1-reports/B9.md`
- `wbs/phase-1-reports/C1.md`
- `wbs/phase-1-reports/C10.md`
- `wbs/phase-1-reports/C2.md`
- `wbs/phase-1-reports/C3.md`
- `wbs/phase-1-reports/C4.md`
- `wbs/phase-1-reports/C5.md`
- `wbs/phase-1-reports/C6.md`
- `wbs/phase-1-reports/C7.md`
- `wbs/phase-1-reports/C8.md`
- `wbs/phase-1-reports/C9.md`
- `wbs/phase-1-reports/D1.md`
- `wbs/phase-1-reports/D10.md`
- `wbs/phase-1-reports/D2.md`
- `wbs/phase-1-reports/D3.md`
- `wbs/phase-1-reports/D4.md`
- `wbs/phase-1-reports/D5.md`
- `wbs/phase-1-reports/D6.md`
- `wbs/phase-1-reports/D7.md`
- `wbs/phase-1-reports/D8.md`
- `wbs/phase-1-reports/D9.md`
- `wbs/phase-1-reports/E1.md`
- `wbs/phase-1-reports/E10.md`
- `wbs/phase-1-reports/E2.md`
- `wbs/phase-1-reports/E3.md`
- `wbs/phase-1-reports/E4.md`
- `wbs/phase-1-reports/E5.md`
- `wbs/phase-1-reports/E6.md`
- `wbs/phase-1-reports/E7.md`
- `wbs/phase-1-reports/E8.md`
- `wbs/phase-1-reports/E9.md`
- `wbs/phase-1-reports/F1.md`
- `wbs/phase-1-reports/F10.md`
- `wbs/phase-1-reports/F2.md`
- `wbs/phase-1-reports/F3.md`
- `wbs/phase-1-reports/F4.md`
- `wbs/phase-1-reports/F5.md`
- `wbs/phase-1-reports/F6.md`
- `wbs/phase-1-reports/F7.md`
- `wbs/phase-1-reports/F8.md`
- `wbs/phase-1-reports/F9.md`
- `wbs/phase-1-reports/G1.md`
- `wbs/phase-1-reports/G10.md`
- `wbs/phase-1-reports/G2.md`
- `wbs/phase-1-reports/G3.md`
- `wbs/phase-1-reports/G4.md`
- `wbs/phase-1-reports/G5.md`
- `wbs/phase-1-reports/G6.md`
- `wbs/phase-1-reports/G7.md`
- `wbs/phase-1-reports/G8.md`
- `wbs/phase-1-reports/G9.md`
- `wbs/phase-1-reports/codex-api-matrix.md`
- `wbs/phase-1-reports/codex-dist-matrix.md`
- `wbs/phase-1-reports/codex-quality.md`
- `wbs/phase-1-reports/codex-readiness-score.md`
- `wbs/phase-1-reports/codex-repo-status.md`
- `wbs/phase-1-reports/codex-sdk-matrix.md`
- `wbs/phase-1-reports/codex-toolchain.md`
- `wbs/phase-1-reports/goose-config-model.md`
- `wbs/phase-1-reports/goose-install-matrix.md`
- `wbs/phase-1-reports/goose-mcp-api.md`
- `wbs/phase-1-reports/goose-quality.md`
- `wbs/phase-1-reports/goose-readiness-score.md`
- `wbs/phase-1-reports/goose-repo-status.md`
- `wbs/phase-1-reports/goose-runtime-profile.md`
- `wbs/phase-1-reports/goose-toolchain.md`
- `wbs/phase-1-reports/governance-mapping.md`
- `wbs/phase-1-reports/governance-risks.md`
- `wbs/phase-1-reports/governance-rubric.md`
- `wbs/phase-1-reports/harness-governance-rules.md`
- `wbs/phase-1-reports/kilo-api-matrix.md`
- `wbs/phase-1-reports/kilo-config-model.md`
- `wbs/phase-1-reports/kilo-extensions-matrix.md`
- `wbs/phase-1-reports/kilo-quality.md`
- `wbs/phase-1-reports/kilo-readiness-score.md`
- `wbs/phase-1-reports/kilo-repo-status.md`
- `wbs/phase-1-reports/kilo-runtime-profile.md`
- `wbs/phase-1-reports/kilo-toolchain.md`
- `wbs/phase-1-reports/opencode-api-matrix.md`
- `wbs/phase-1-reports/opencode-auth.md`
- `wbs/phase-1-reports/opencode-plugin-matrix.md`
- `wbs/phase-1-reports/opencode-quality.md`
- `wbs/phase-1-reports/opencode-readiness-score.md`
- `wbs/phase-1-reports/opencode-repo-status.md`
- `wbs/phase-1-reports/opencode-runtime-profile.md`
- `wbs/phase-1-reports/opencode-toolchain.md`
- `wbs/phase-1-reports/oss-cli-constraints.md`
- `wbs/phase-1-reports/oss-cli-discovery.md`
- `wbs/phase-1-reports/oss-cli-feasibility.md`
- `wbs/phase-1-reports/oss-cli-shortlist.md`
- `wbs/phase-1-reports/oss-cli-web-candidates.md`
- `wbs/phase-1-reports/phase-1-synthesis.md`

---

## Source: research/oss-cli-matrix.md

# OSS CLI Matrix (Phase-1)

## Scope and scoring baseline
- Scope: initial 4 core CLIs + additional OSS candidates discovered in `temp-PRODVERCEL/485`.
- Strictness scale (strictness equivalent):
  - **A (strict-full-noskip)**: full lint/test/build/typecheck, explicit fail on warnings/errors, CI-first workflow, explicit evidence artifacts.
  - **B (strict-partial)**: mostly complete quality checks with some manual/manual-like steps.
  - **C (pragmatic)**: lightweight local checks, sparse governance, or no central gate.
- Composite readiness score combines: `quality (40) + quality-coverage depth (20) + API/SDK maturity (20) + build health (20)`.

| Repo | Language | Install | Build | Test | Quality gates | Quality level | API / SDK surface | License | Notes |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `openai/codex` (core) | Rust + Node | `npm i -g @openai/codex`, brew, releases | Rust: `cargo build` via `rust-ci`; Node install workflows + npm publish staging paths | Rust tests in CI matrix + npm workspace checks for CLI/docs | `rust-ci`, `cargo-deny`, `codespell`, `ci.yml`, `sdk.yml`, `shell-tool-mcp-ci`, `shell-tool-mcp` | B+ | `codex` CLI; `codex-rs` crates; JS SDK in `sdk/typescript`; `shell-tool-mcp` package | Apache-2.0 | Strong engineering signals, mixed toolchain (Node + Rust). No single command named `quality:strict-full`; quality is distributed across workflows.
| `opencode` (archived) | Go | `go install` / goreleaser flow | `goreleaser build --snapshot` in `build.yml` | only release/build path currently visible; no broad unit-test matrix in CI workflow | `build.yml`, `release.yml` (publish); no strict lint/test stack visible | C | TUI binary + providers/plugins configured in codebase; config via JSON schema generator (`cmd/schema`) | Apache-2.0 | Active archival notice; project moved to Crush. Useful for legacy compatibility and provider flow examples.
| `goose` (API/research) | Rust + UI stack (Node) | `cargo build` target binaries, desktop bundles; docs-driven install scripts | `just` and/or `cargo build`; many CI jobs and release workflows | tests in Rust matrix + provider/recipe workflows | `ci.yml` + many release/publish workflows; `Justfile` exposes `check-everything`, `lint`, `generate-openapi` | A- | CLI + MCP crates (`crates/goose-mcp`, `goose-server` APIs, generated OpenAPI in `ui/desktop`) | Apache-2.0 | Mature strictness posture with explicit local check target; candidate strong for harness with API-first scoring.
| `kilocode` | Node TS + Rust | `pnpm install`, plugin/install scripts | `pnpm build` (`code-qa.yml`), turbo-based releases | `pnpm test` + package-level test steps in CI | `code-qa` (compile/build/typecheck/lint/test), Husky/pre-push style scripts | B | Extension+CLI surface in monorepo (packages + webview+jetbrains+cli); internal CLI run path appears in `packages/cli` | MIT | Very good workspace-quality story; heavy ecosystem-specific packaging but clean CI structure.
| `CLIProxyAPI` (local) | Go + Node docs + server | `go build` via CLI entrypoints | `go test` / local scripts; docs show release/build + docker variants | tests and integration docs; quality scripts not fully standardized | README-heavy governance, multiple workflow references from docs; no explicit all-in-one strict profile seen | C | Proxy + SDK (`sdk/`) + `cli-proxy-api` binary; strong auth/edge routing design | MIT | Strategic candidate due proxy value for multi-provider + API surface; quality confidence moderate because centralized strict gate evidence is weaker in-snapshot.
| `cliproxyapi++` (local) | Go + Node docs | `go build -o cliproxyapi++ ./cmd/server` (`Taskfile`), docker compose | `task build` plus docker targets + docs checks | `task test`, `go test ./...` | `task quality` => `gofmt` check + `go vet` + `golangci-lint` + `go test` + optional parent sibling check; README mentions `task quality` + gosec, actionlint in docs | A- | Proxy binary (`cli-proxy-api-plus-increment` docs), `pkg/llmproxy`, provider-first routing | MIT | Best concrete local strictness profile among local proxy forks; explicit quality alias in Taskfile aligns with strictness intent.
| `pluggedin-mcp-proxy` (zentest) | TypeScript | `npm/pnpm install` | `npm/pnpm run build` (tsdown) | `npm run test` + coverage command available | `lint` (`oxlint` + `eslint`), `lint:strict` available, build/test scripts explicit | A-/B | MCP proxy + CLI (`mcp-cli`) + API surface + integration simulator + codegen | MIT | Strong TS quality surface for MCP tooling; likely good candidate for API/SDK adapter testing pattern.
| `openai-codex-mcp` (zentest) | Python | `uv pip install .`/`setup_and_run.sh` | `uv` install + runtime entrypoint (`codex_server`) | no matrix evidence surfaced in-snapshot | no CI gate evidence collected; README focused on setup and MCP methods | C | MCP server bridging to codex CLI methods (`codex_completion`, `write_code`, etc.) | Apache-2.0 | Useful reference candidate for low-latency adapter path; quality posture should be improved before strict inclusion.

## Candidate shortlist rationale
### Priority-0 (pilot for this harness)
1. `goose` - strongest strictness, multi-surface CLI+MCP+server, explicit quality scripts.
2. `kilocode` - mature CI quality + typed monorepo test lanes.
3. `cliproxyapi++` - explicit local quality command and strong proxy API surface.
4. `pluggedin-mcp-proxy` - clear API+CLI tooling surface and strict lint/test scripts.

### Priority-1 (compare/future)
- `openai/codex`, `CLIProxyAPI`, `opencode`, `openai-codex-mcp`.

### Immediate gaps to resolve before final merge into strict harness
- normalize per-candidate quality command nomenclature (`quality`, `quality:strict-full`, `task quality`) to canonical `quality_profile=STRICT_FULL` mapping.
- collect commit-lock proof (`branch/main`, latest hash, remote URL, clean check) for each candidate in command artifacts.

---

## Source: research/wave1-agent1-discovery.md

# Wave 1 / Agent 1 Discovery

Scope: local clone/repo discovery for `codex`, `opencode`, `goose`, `kilo`.
Timestamp: 2026-02-22 (workspace local snapshot)

## Target Repo Discovery (Current)

### 1) `/Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/clones`

- `codex`
  - Path: `/Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/clones/codex`
  - Git: present
  - Status: **broken (no refs)**
  - `HEAD`: `ref: refs/heads/.invalid`
  - Branch: unusable (`detached_or_broken`)
  - `git status --short` count: `0`
  - Remote: `https://github.com/openai/codex.git`
  - Last commit: unavailable until clone is repaired

- `opencode`
  - Path: `/Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/clones/opencode`
  - Git: healthy
  - Branch: `main`
  - `git status --short` count: `0`
  - Remote: `https://github.com/opencode-ai/opencode.git`
  - Last commit: `73ee493` (`docs(readme): update archive note`, `2025-09-17`)

- `goose`
  - Path: `/Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/clones/goose`
  - Git: present
  - Status: **broken (no refs)**
  - `HEAD`: `ref: refs/heads/.invalid`
  - Branch: unusable (`detached_or_broken`)
  - `git status --short` count: `0`
  - Remote: `https://github.com/block/goose.git`
  - Last commit: unavailable until clone is repaired

- `kilocode` (alias `kilo`)
  - Path: `/Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/clones/kilocode`
  - Status: **missing**

### 2) Additional local matches (same target stack)

- `API/codex-upstream`
  - Branch: `main`
  - Remote: `https://github.com/openai/codex.git`
  - Last commit: `2d6757430` (`plan mode prompt (#10308)`, `2026-01-31`)
  - `git status --short` count: `0`

- `API/research/opencode`
  - Branch: `dev`
  - Remote: `https://github.com/sst/opencode.git`
  - Last commit: `070ced0b3` (`fix: revert hook try/catch that surpressed errors`, `2025-12-10`)
  - `git status --short` count: `0`

- `API/research/goose`
  - Branch: `main`
  - Remote: `https://github.com/block/goose.git`
  - Last commit: `66d075050ed` (`blog: typo fixes (#5896)`, `2025-11-26`)
  - `git status --short`: 6 changes (local edits/untracked)

- `API/research/kilocode`
  - Branch: `main`
  - Remote: `https://github.com/Kilo-Org/kilocode.git`
  - Last commit: `1440d1986d` (`Merge pull request #4300 ...`, `2025-12-09`)
  - `git status --short` count: `0`

## Toolchain discovery

- `codex` (openai)
  - Root `package.json` says `node >=22`, `pnpm >=10.28.0` and `packageManager: pnpm@10.28.2+sha512...`
  - Rust workspace detected: many `codex-rs/*/Cargo.toml` manifests
  - `pnpm-lock.yaml` present

- `opencode` (`API/research/opencode` and `heliosHarness/clones/opencode`)
  - `go.mod` exists with `go 1.24.0` (Go toolchain)
  - No project-level `package.json` at `heliosHarness/clones/opencode`

- `goose` (`API/research/goose`)
  - Root `Cargo.toml`: Rust workspace with `resolver = "2"`, `edition = "2021"`
  - UI/docs tooling present with `package.json` and Node workflows (no single repo-level version pin seen)

- `kilo` (`API/research/kilocode`)
  - Root `package.json`: `packageManager: pnpm@10.8.1`, `engines.node: 20.19.2`
  - `.tool-versions`: `nodejs 20.19.2`
  - `.nvmrc`: `v20.19.2`

## Clone checklist

1. Repair broken/empty existing clones under `heliosHarness/clones`:
   - `rm -rf /Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/clones/codex`
   - `rm -rf /Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/clones/goose`
2. Re-clone targets for Wave 1 prep (into `heliosHarness/clones`):
   - `git clone https://github.com/openai/codex.git /Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/clones/codex`
   - `git clone https://github.com/opencode-ai/opencode.git /Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/clones/opencode`
   - `git clone https://github.com/block/goose.git /Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/clones/goose`
   - `git clone https://github.com/Kilo-Org/kilocode.git /Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/clones/kilocode`
3. Verification commands:
   - `git -C <repo> remote -v`
   - `git -C <repo> status --short`
   - `git -C <repo> log -1 --oneline`

---

