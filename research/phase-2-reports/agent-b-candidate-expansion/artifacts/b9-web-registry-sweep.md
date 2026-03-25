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
