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
