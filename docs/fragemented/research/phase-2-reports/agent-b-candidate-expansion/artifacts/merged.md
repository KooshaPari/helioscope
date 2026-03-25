# Merged Fragmented Markdown

## Source: research/phase-2-reports/agent-b-candidate-expansion/artifacts/b-local-scan.md

# Lane B Execution Evidence

## B2 Local Candidate Enumeration (command surface)
- Source roots: `/Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/clones/*`, `/Users/kooshapari/temp-PRODVERCEL/485/API/research/CLIProxyAPI`, `/Users/kooshapari/temp-PRODVERCEL/485/zentest/*`
- Files used:
  - `research/oss-cli-matrix.md`
  - `research/phase-1-reports/agent-e-discovery.md`

## B3 Filtered short-list
- Included in candidate set: `codex`, `opencode`, `goose`, `kilocode`, `cliproxyapi-plusplus`, `pluggedin-mcp-proxy`, `openai-codex-mcp`, `claude-code-flow`, `CLIProxyAPI`.

## B4 License evidence
- `codex`: `LICENSE`/`NOTICE` under clone root.
- `goose`: `LICENSE`, plus `Rust` and `Node` lock metadata.
- `kilocode`: `LICENSE` + `NOTICE`.
- `cliproxyapi-plusplus`: `LICENSE` in source tree.
- `opencode`: `LICENSE`.
- `pluggedin-mcp-proxy`: `LICENSE` from `zentest` tree.
- `openai-codex-mcp` and `claude-code-flow`: `LICENSE` / SPDX headers as published in repository.

## B5 Install surface evidence
- `codex`: `AGENTS.md`, `README.md` + npm/CI install references.
- `goose`: `cargo`/`just` targets and release docs.
- `kilocode`: `pnpm install`, package scripts.
- `opencode`: `go install`, goreleaser path.
- `cliproxyapi-plusplus`: `go install` and `task` target family.
- MCP proxies (`pluggedin-mcp-proxy`): `npm/pnpm install` entries in phase-1 discovery.

## B6 Build/test/quality commands evidence
- `codex`: `just` + `rust-ci.yml`.
- `goose`: `check-everything`, `clippy`, test matrix in `just` and workflows.
- `kilocode`: `pnpm lint`, `pnpm test`, and `code-qa.yml`.
- `opencode`: archive/packaging + provider/CLI release path.
- `cliproxyapi-plusplus`: `task quality`, `go test`, optional gosec/actions lint.

## B7 Strictness profiles
- See `agent-c-governance-strictness/artifacts/strictness-equivalence-matrix.md` for cross-target policy mapping.

## B8 Delta ranking
- Added cliproxyapi++ and pluggedin-mcp-proxy to stronger priority due explicit quality command families.
- Deferred opencode from top 4 due archival status and partial evidence.

## B9 Web sweep
- Completed with web-registry scan for replacement candidates outside local scan.
- Open-signal additions: `redhat-ai-tools/mcp-registry`, `pathintegral-institute/mcpm.sh`, `kunwarVivek/mcp-github-project-manager`, `beshkenadze/openapi-mcp-generator`, `MarcusJellinghaus/mcp-code-checker`.
- Evidence: `artifacts/b9-web-registry-sweep.md`.

## B10 Phase-2 shortlist
- Phase-2 priority shortlist now includes:
  1. `goose`
  2. `kilocode`
  3. `cliproxyapi-plusplus`
  4. `pluggedin-mcp-proxy`
  5. `codex`
  6. `pathintegral-institute/mcpm.sh`
  7. `kunwarVivek/mcp-github-project-manager`

---

## Source: research/phase-2-reports/agent-b-candidate-expansion/artifacts/b9-web-registry-sweep.md

# B9 Web-Registry Sweep Evidence

Date: 2026-02-22
Scope: Open-source MCP/CLI candidates not in local scan

| candidate | source_url | summary | license | stars/forks | activity | install commands | quality gates / test surface |
|---|---|---|---|---|---|---|---|
| redhat-ai-tools/mcp-registry | https://github.com/redhat-ai-tools/mcp-registry | community-driven MCP registry service with REST API and health checks | MIT | 2 / 595 | Active commits (117 commits) | `go build ./cmd/registry` | `golangci-lint run --timeout=5m`, `gofmt -s -l .` |
| pathintegral-institute/mcpm.sh | https://github.com/pathintegral-institute/mcpm.sh | CLI MCP package manager and client profile manager with web registry integration | MIT | 892 / 94 | v2.13.0 latest Jan 15, 2026; active releases (54 total) | `brew install mcpm`, `pipx install mcpm`, `uv tool install mcpm`, `pip install mcpm` | `mcpm` CLI manager + profile/ client utilities; explicit release/package health artifacts present |
| kunwarVivek/mcp-github-project-manager | https://github.com/kunwarVivek/mcp-github-project-manager | MCP server for GitHub project management with comprehensive E2E testing | MIT | 84 / 28 | Ongoing issue/test activity and CI docs | `npm install -g mcp-github-project-manager`, `npm install`, `npm run build`, `npm test` | `npm run test`, `npm run test:e2e:tools`, `npm run test:e2e:tools:real` |
| beshkenadze/openapi-mcp-generator | https://github.com/beshkenadze/openapi-mcp-generator | OpenAPI-spec-to-MCP generator with multiple runtime modes | MIT | 0 / 0 | Recent test/build references | `bun install`, `bun run build`, `bun run test`, `bun run lint`, `bun run format` | Monorepo scripts for lint/typecheck/build/test via Bun/Task |
| MarcusJellinghaus/mcp-code-checker | https://github.com/MarcusJellinghaus/mcp-code-checker | MCP server for pylint/pytest/mypy quality operations with MCP client integration | MIT | 14 / 5 | 67 commits and active issue context | `pip install git+https://github.com/MarcusJellinghaus/mcp-code-checker.git`, `mcp-code-checker --help` | `pylint`, `pytest`, `mypy` tool surface and install CLI doc |

Cross-check:
- Each candidate is open-source and provides CLI or MCP-relevant install/test command surface.
- None were in the local `B1-B8` source-scanned list, so they extend the replacement/high-signal sweep set.
- This evidence was produced to complete B9 web sweep in-phase.

---

