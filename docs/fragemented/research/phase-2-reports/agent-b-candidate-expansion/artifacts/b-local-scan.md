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
