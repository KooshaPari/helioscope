<DONE>
# Lane E Discovery Report

Scope: Additional OSS CLI candidate discovery for AGENT-E from local `temp-PRODVERCEL/485` workspace.
Date: 2026-02-22 (local scan snapshot)

## Baseline (for continuity)
Wave-1 clone discovery already covers these core CLI targets:
- `heliosHarness/clones/codex`
- `heliosHarness/clones/opencode`
- `heliosHarness/clones/goose`
- `heliosHarness/clones/kilocode`

## Additional OSS CLI Candidates (Local Repository)
The following were selected as additional candidates from local `temp-PRODVERCEL/485`.

### Go-based API/proxy layer candidates
- `CLIProxyAPI` (`/Users/kooshapari/temp-PRODVERCEL/485/CLIProxyAPI`)
  - Tooling profile: Go (`go.mod`), `module github.com/kooshapari/CLIProxyAPI/v7`.
  - CLI evidence: documented command usage (`cli-proxy-api serve`, `cli-proxy-api auggie-setup`).
  - Head: `684a7c6` (`Refactor gateway integration into main /v1 routes instead of separate /v1/gateway`, 2025-11-26).
- `API/research/CLIProxyAPI` (`/Users/kooshapari/temp-PRODVERCEL/485/API/research/CLIProxyAPI`)
  - Tooling profile: Go (`go.mod`), same module lineage (`.../CLIProxyAPI/v7`).
  - CLI evidence: same documented command surface as above.
  - Head: `f80eed6` (`docs: Add CLIProxyAPI Phase 7.2 modifications audit report`, 2025-12-08).
- `cliproxyapi-plusplus` (`/Users/kooshapari/temp-PRODVERCEL/485/kush/cliproxyapi-plusplus`)
  - Tooling profile: Go (`go.mod`), `module github.com/router-for-me/CLIProxyAPI/v6`.
  - CLI evidence: explicit CLI-style server entry (`cmd/server/main.go` with flag parsing and auth/login/serve mode); multiple executable mains exist under `cmd/`.
  - Head: `0ac77bc3` (`docs: expand provider-first docs and README`, 2026-02-20).

### Node/TypeScript CLI-oriented candidates
- `claude-code-flow` (`/Users/kooshapari/temp-PRODVERCEL/485/zentest/Clow/claude-code-flow`)
  - Tooling profile: TypeScript/Node (`package.json` with `bin: { "claude-flow": "./bin/claude-flow" }`, Node >=20).
  - CLI evidence: primary command `claude-flow`.
  - Head: `d3dd13f` (`Merge pull request #270 from ruvnet/alpha.55`, 2025-07-15).
- `pluggedin-mcp-proxy` (`/Users/kooshapari/temp-PRODVERCEL/485/zentest/pluggedin-mcp-proxy`)
  - Tooling profile: TypeScript/Node (`package.json` with multiple bin entries: `mcp-cli`, `mcp-simulate`, `mcp-codegen`; Node >=18).
  - CLI evidence: explicit binary commands for MCP proxy runtime and tooling.
  - Head: `369a8fb` (`chore(ts): fix remaining type errors and config`, 2025-08-31).
- `openai-codex-mcp` (`/Users/kooshapari/temp-PRODVERCEL/485/zentest/openai-codex-mcp`)
  - Tooling profile: Python (`pyproject.toml`, `requires-python >=3.12`, `[project.scripts]` present with `codex_server = "codex_server:main"`).
  - CLI evidence: installable script entrypoint (`codex_server`), wrapper around OpenAI Codex CLI for MCP.
  - Head: `18ab4bd` (`recursive self-improvement`, 2025-04-18).

### Rust/TUI candidate
- `KLA` (`/Users/kooshapari/temp-PRODVERCEL/485/zentest/KLA`)
  - Tooling profile: Rust (`Cargo.toml`, `[[bin]]` with binary `kla`).
  - CLI evidence: explicit `kla` bin target.
  - Head: `5c24a81` (`Add comprehensive demo gallery with recursive KLA demonstrations`, 2025-07-10).

## Language / Tooling Profiles Summary
- Go: `CLIProxyAPI`, `API/research/CLIProxyAPI`, `cliproxyapi-plusplus`.
- Rust: `KLA`.
- TypeScript/Node: `claude-code-flow`, `pluggedin-mcp-proxy`, `openai-codex-mcp`.

## Initial Shortlisting Rationale
Priority 0 (strongly short-listed for lane-E evaluation):
- `claude-code-flow` — explicit CLI binary and direct AI-orchestration positioning, likely closest to an end-user coding assistant CLI.
- `openai-codex-mcp` — direct Codex integration path and executable entrypoint, useful for interoperability and Codex-specific evaluation.
- `cliproxyapi-plusplus` / `CLIProxyAPI` families — shared production-ready proxy architecture for CLI providers and explicit command-mode behavior (especially multi-provider auth/transport features).

Priority 1 (watch-list / secondary):
- `pluggedin-mcp-proxy` — strong command surface but MCP/protocol-proxy focused rather than a direct coding assistant CLI.
- `KLA` — robust Rust CLI with executable; currently terminal automation focused, only partially aligned with coding-agent workflows.

Exclusions (for now):
- Non-CLI or helper projects that were not found to expose meaningful standalone executable entrypoints were excluded from lane-E shortlisting.
