# Merged Fragmented Markdown

## Source: research/phase-1-reports/agent-a-codex.md

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

---

## Source: research/phase-1-reports/agent-b-opencode.md

# Lane B Discovery: opencode

## toolchain
- Language/runtime: Go (CLI written in Go, `go.mod` declares `go 1.24.0`).
- Module: `github.com/opencode-ai/opencode` with many Go deps.
- Local developer constraints in clone: build and CI use Go 1.23.2+ in GitHub Actions, so practical minimum in CI is `>=1.23.2` while project baseline is `1.24.0`.
- Release pipeline requires Go toolchain + Goreleaser for packaging.

## commands
- Install:
  - `go install github.com/opencode-ai/opencode@latest`
  - `curl .../install | bash` / `.../install | VERSION=<ver> bash`
  - Homebrew: `brew install opencode-ai/tap/opencode`
  - AUR: `yay -S opencode-ai-bin` / `paru -S opencode-ai-bin`
- Build/run:
  - Local build: `go build -o opencode`
  - Binary schema export: `go run cmd/schema/main.go > opencode-schema.json`
  - Start interactive: `opencode`
  - Non-interactive: `opencode -p "<prompt>"`, `-f json`, `-q`
- CI/release commands (from workflows/scripts):
  - `go mod download`
  - `goreleaser build --snapshot --clean`
  - `goreleaser release --clean`
  - `goreleaser build --clean --snapshot --skip validate` (in `scripts/snapshot`)
  - `./scripts/release` increments tag and triggers release artifacts

## quality gates
- GitHub Actions currently does **not** run tests/linters/formatters explicitly.
- Main enforced CI gates are artifact/build generation via Goreleaser on pushes/tags and dependency download.
- Local quality/release-hardening checks observed:
  - `scripts/check_hidden_chars.sh` scans Go files for prompt-injection-relevant hidden Unicode.
  - `go mod download` is explicitly run before builds.
- Test presence exists in repo (`_test.go` files under `internal/`), but not wired as a CI gate in current workflows.

## API / plugin model
- Tooling stack is built around a provider+tool abstraction:
  - Built-in LLM tools: `bash`, `glob`, `grep`, `ls`, `view`, `fetch`, `edit`, `write`, `patch`, `sourcegraph`, `diagnostics`, and `agent`.
  - These are assembled for the coder agent in `internal/llm/agent/tools.go`; task agent has a reduced set.
- MCP is supported via `mcpServers` config with two transport types:
  - `stdio` (command/env/args-based process launcher)
  - `sse` (URL + headers)
- MCP discovery occurs dynamically and tool names are namespaced as `<server>_<tool>` before being exposed to the model.
- Model/provider layer is pluggable; `NewProvider` selects provider implementation by mapped provider name and applies custom routing (OpenAI/Groq/Azure/etc plus Copilot).
- Config schema is generated by `cmd/schema/main.go` (authoritative for config shape and allowed values).

## runtime / auth constraints
- Config loading:
  - Search paths: `$HOME/.opencode`, `$XDG_CONFIG_HOME/opencode`, `$HOME/.config/opencode`, and local `./.opencode`.
  - Env-driven config is enabled through `OPENCODE_*` prefix via Viper `AutomaticEnv`.
  - Local config merges over global config.
- Runtime state:
  - `data.directory` default `.opencode`.
  - SQLite DB path: `<data>/opencode.db`.
  - DB migration path via Goose; PRAGMA hardening includes FK + WAL + synchronous tuning.
- Runtime permissions:
  - Permission service gates operations and supports one-time and session-level approval persistence.
  - Non-interactive mode (`-p`) calls auto-approve for the session.
  - Bash tool requires explicit approval for non-read-only commands; safe commands are not auto-gated.
- Auth/provider constraints:
  - Providers are auto-populated from env (`OPENAI_API_KEY`, `ANTHROPIC_API_KEY`, `GEMINI_API_KEY`, `GROQ_API_KEY`, `OPENROUTER_API_KEY`, `XAI_API_KEY`, AWS/Azure/Vertex variables, etc.).
  - Missing API key marks provider disabled.
  - Copilot auth chain: `GITHUB_TOKEN` → `providers.copilot.apiKey` → local GitHub token files in `~/.config/github-copilot/{hosts,apps}.json`.
  - Copilot token exchange uses `api.github.com/copilot_internal/v2/token` to obtain bearer token.
- External execution constraints:
  - Bash tool keeps shared persistent shell state and executes command subprocesses with a strict command allow/deny model.
  - No network policy/egress allowlist found in repo; only command and permission gating is clear.

## readiness score
- Score: **68 / 100**
- Basis: good CLI/tool architecture, explicit provider and plugin abstractions, runtime permission model, and release/build visibility; medium risk remains from weak CI quality gates, runtime command restrictions that can be bypassed without strict allowlisting, and archived repository state.

## strictness mapping
- Shell command execution
  - Strictness: Medium
  - Controls: banned command deny-list, safe-read-only fast path, permissions for mutating commands, human approval workflow.
- Model-tool access
  - Strictness: Medium
  - Controls: plugin tools discovered and invoked with permission checks (`permission.Request`), MCP tool namespaced and user-configurable.
- Data/config safety
  - Strictness: Medium
  - Controls: config merge model, validation, provider defaults, automatic disable on missing keys, schema generation.
- Persistence/auth
  - Strictness: Medium
  - Controls: local DB with migrations + PRAGMA, token loading from env/files, Copilot exchange flow.
- Operational assurance
  - Strictness: Low
  - Gaps: no mandatory test/lint gates in CI, archived notice means upstream risk and maintenance signal.

---

## Source: research/phase-1-reports/agent-c-goose.md

# Lane C Report: goose

## Scope
- Primary source: `/Users/kooshapari/temp-PRODVERCEL/485/API/research/goose`
- Harness clone under `heliosHarness/clones/goose` is partial (only `.git`), so this report is based on the full source tree above.
- Date: `2026-02-22`

## Quality Gates
- GitHub CI workflow is defined in `.github/workflows/ci.yml` and runs on `push`, `pull_request`, `merge_group`, and `workflow_dispatch`.
- Change filtering gate: `paths-filter` determines whether code-related jobs should run (`docs-only` vs `code`).
- `rust-format` job runs `cargo fmt --check`.
- `rust-build-and-test` job installs desktop/system deps, runs `cargo test -- --skip scenario_tests::scenarios::tests`, then `cargo test --jobs 1 scenario_tests::scenarios::tests` with `CARGO_INCREMENTAL=0`.
- `rust-lint` job runs `./scripts/clippy-lint.sh` (strict `clippy --all-targets` + baseline rule checks).
- `openapi-schema-check` job runs `just check-openapi-schema` and validates generated schema drift.
- `desktop-lint` job runs `npm run lint:check` and `npm run test:run` in `ui/desktop`.
- Local developer quality tasks in `Justfile` include:
  - `just check-everything`
  - `just check-openapi-schema`
  - `just generate-openapi`
  - release/build presets for all platform targets.
- Coverage gap in automated gates: no explicit fuzz/security scan/lint policy check beyond standard clippy/tests and no explicit dependency/license drift job.

## API / MCP Surfaces
- HTTP API server command is configured in `crates/goose-server/src/commands/agent.rs` (`Commands::Agent`).
- Public routes are composed under `crates/goose-server/src/routes/mod.rs` from these route groups:
  - `status` (`/status`, `/diagnostics/{session_id}`)
  - `agent` (`/agent/start`, `/agent/resume`, `/agent/update_from_session`, `/agent/tools`, `/agent/update_provider`, `/agent/add_extension`, `/agent/remove_extension`, `/agent/stop`)
  - `reply` (`/reply`, `/confirm`)
  - `config_management` (`/config`, `/config/upsert`, `/config/read`, `/config/remove`, `/config/extensions`, `/config/providers*`, `/config/permissions`, `/config/custom-providers*`, etc.)
  - `recipe` (`/recipes/*`)
  - `session` (`/sessions*`)
  - `schedule` (`/schedule*`)
  - `setup` (`/handle_openrouter`, `/handle_tetrate`)
  - `tunnel` (`/tunnel/*`)
  - `audio` and `mcp_ui_proxy` (`/mcp-ui-proxy`)
- MCP/agent extension model:
  - MCP CLI mode in `crates/goose-server/src/main.rs`: `goose mcp {autovisualiser|computercontroller|developer|memory|tutorial}`.
  - Extension config supports typed variants in `crates/goose/src/agents/extension.rs`:
    - `sse`, `streamable_http`, `stdio`, `builtin`, `platform`, `frontend`, `inline_python`
    - default/builder helpers for common variants.
  - Extension manager runtime handling in `crates/goose/src/agents/extension_manager.rs`:
    - `Sse` connects via `SseClientTransport`
    - `StreamableHttp` uses HTTP client transport and optional auth retry (`oauth_flow`)
    - `Stdio` spawns command processes through `child_process_client`
    - `Builtin`/`Platform` resolve internal extension clients (`PLATFORM_EXTENSIONS` includes `developer`, `chatrecall`, `extensionmanager`, `skills`)
    - `InlinePython` writes temp script and launches `uvx` with optional package dependencies
    - `Frontend` explicitly rejected for direct server add path.

## Build / Test Evidence
- Core build/test commands:
  - Workspace root: `cargo fmt`, `cargo test`, `cargo clippy`
  - CI canonical: `cargo test` with scenario split and `./scripts/clippy-lint.sh`
  - OpenAPI generation via `cargo run -p goose-server --bin generate_schema` and `npx @hey-api/openapi-ts` (`crates/goose-server/src/main.rs` + `ui/desktop`).
  - Linux desktop docs in `BUILDING_LINUX.md` specify `cargo build --release -p goose-server`, `npm install`, and Electron Forge `npm run make` flows.
- `Justfile` provides runnable pipelines:
  - `just release-binary` (`cargo build --release` + copy binaries)
  - `just generate-openapi`
  - `just check-openapi-schema`
  - `just run-ui`, `just run-ui-windows`, `just run-ui-playwright`
- No local `cargo test`/`just` validation was executed in this report pass (only source-based evidence collected).

## Plugin / Config / Security Notes
- Config system (`crates/goose/src/config/base.rs`):
  - Non-secret keys: env override (`KEY` uppercase env var) → YAML config file.
  - Secret keys: env override → keyring/file fallback.
  - Default config file is `~/.config/goose/config.yaml` (`CONFIG_YAML_NAME`).
  - Secret storage uses keyring (`service: goose`, username `secrets`) unless `GOOSE_DISABLE_KEYRING` is set.
  - Paths are `goose`-scoped (`Paths::config_dir()`, `Paths::data_dir()`, `Paths::state_dir()`).
- Config surface includes `/config/extensions` CRUD and per-extension runtime toggles through config + `/agent/add_extension` / `/agent/remove_extension`.
- Permission model in `config/permission.rs` supports `always_allow`, `ask_before`, `never_allow` levels with YAML persistence to `permission.yaml`.
- MCP extension security controls:
  - `Envs::new` blocks writes to critical/unsafe env keys (PATH, LD_PRELOAD, PYTHONPATH, NODE_OPTIONS, etc.) and warns on attempts.
  - `extension_malware_check` performs optional OSV advisory lookup for `npx`/`uvx` package launches; failures are non-blocking (fail-open behavior with logged errors).
  - `child_process_client` routes process stderr capture and applies `configure_command_no_window` plus controlled PATH override from `SearchPaths`.
  - `normalize` sanitizes extension names to ASCII alphanumerics and prevents odd chars.
- Transport/auth security:
  - Server middleware requires `X-Secret-Key` header for all endpoints except `/status` and `/mcp-ui-proxy` (`crates/goose-server/src/auth.rs`).
  - Secret is sourced from `GOOSE_SERVER__SECRET_KEY` and defaults to `"test"` in `commands/agent.rs`.
  - `CORS` policy is currently open (`CorsLayer::allow_origin(Any)`, any methods/headers).

## Readiness Scoring
- Overall score: `73 / 100`
- Breakdown:
  - API/MCP coverage and design: `18 / 25`
  - Plugin/config extensibility: `16 / 20`
  - Build/test + release surface: `16 / 20`
  - Security and secret handling: `11 / 20`
  - Operational quality/resilience: `12 / 15`
- Readiness decision: `Proceed with caveats` (usable, but security hardening around secrets and default secret/key handling needs explicit validation before production-like deployment).

---

## Source: research/phase-1-reports/agent-d-kilo.md

# Lane D Report: kilocode

Scope: Local clone assessment of `kilo` (`/Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/clones/kilocode`).
Date: 2026-02-22

## 1) Build / lint / test command surface

- Install/bootstrap: `pnpm install`
  - Mirrors `DEVELOPMENT.md` and root script hooks (`preinstall`/`install` run bootstrap).
- Build: `pnpm build`
  - Root delegates to `pnpm vsix`.
- Test: `pnpm test`
  - Delegates to `turbo test --log-order grouped --output-logs new-only`.
- Lint: `pnpm lint`
  - Delegates to `turbo lint --log-order grouped --output-logs new-only`.
- Type checking: `pnpm check-types`
  - Delegates to `turbo check-types --log-order grouped --output-logs new-only`.
- Release/packaging variants: `pnpm bundle`, `pnpm vsix`, `pnpm vsix:production`, `pnpm vsix:nightly`.
- CLI operations (from `cli/`):
  - build: `pnpm --filter @kilocode/cli run build`
  - run: `pnpm --filter @kilocode/cli run start`
  - dev: `pnpm --filter @kilocode/cli run dev`

## 2) API architecture

- Monorepo package graph in `pnpm-workspace.yaml` includes:
  - `src`, `webview-ui`, `cli`, `apps/*`, `packages/*`, `jetbrains/host`, `jetbrains/plugin`.
- Provider dispatch lives in `src/api/index.ts`.
  - `buildApiHandler(configuration)` switches on `configuration.apiProvider` to instantiate a provider handler.
  - Includes built-ins plus Kilo-specific providers (`kilocode`, `gemini-cli`, `virtual-quota-fallback`, `nano-gpt`, `synthetic`, `inception`, etc.).
- Provider abstraction:
  - `src/api/providers/base-provider.ts` defines common provider interface and shared behavior (`convertToolsForOpenAI`, schema conversion, token counting).
  - `base-openai-compatible-provider.ts` centralizes OpenAI-style stream setup, param normalization, usage reporting, and error handling.
- API provider examples:
  - Roo provider uses cloud token from `CloudService`, builds `/v1` base URL from env, and dynamically loads model metadata via `getRooModels` + cache refresh.
  - Fetcher (`src/api/providers/fetchers/roo.ts`) validates API payload with `RooModelsResponseSchema` and sends `Authorization: Bearer <apiKey>` when present.
- Runtime/API boundary:
  - Extension exposes IPC API through `@roo-code/ipc` in `src/extension/api.ts` when `KILO_IPC_SOCKET_PATH` / `ROO_CODE_IPC_SOCKET_PATH` is set.
  - API handles task start/send/cancel lifecycle and broadcasts task events back to connected IPC clients.

## 3) Extensions

- Recommended IDE extensions (from `.vscode/extensions.json`):
  - `dbaeumer.vscode-eslint`
  - `esbenp.prettier-vscode`
  - `csstools.postcss`
  - `bradlc.vscode-tailwindcss`
  - `connor4312.esbuild-problem-matchers`
  - `yoavbls.pretty-ts-errors`
- Product extension surface (`src/package.json`):
  - activation events: `onLanguage`, `onStartupFinished`.
  - contributes: view container (`kilo-code-ActivityBar`), webview panel, and many commands (plus settings, context actions, MCP controls, ask/ask-mode entry points).

## 4) Runtime / auth model

- Extension runtime (VS Code):
  - Activation entrypoint `activate` in `src/extension.ts` initializes telemetry, `ContextProxy`, command/terminal/file-index managers, and `ClineProvider`.
  - `CloudService.createInstance` is used to create singleton runtime services and register auth/settings/telemetry listeners.
- Cloud auth model (`packages/cloud/`):
  - If `ROO_CODE_CLOUD_TOKEN` exists, runtime uses `StaticTokenAuthService` with a static token.
  - Otherwise, runtime uses `WebAuthService` for browser/device-style auth.
  - `WebAuthService` stores credentials in VS Code secret storage, validates/states CSRF-safe callback flow, and refreshes tokens on timer.
  - `CloudService` exposes session token and user/organization info for API calls and event-driven updates.
  - API services attach bearer tokens for auth when available (`Authorization: Bearer ${sessionToken}`).
- CLI runtime:
  - Binary name: `kilocode` / `kilo` (`cli/package.json`).
  - `kilocode` root command requires config unless env mode is provided; otherwise launches `auth` wizard.
  - CLI auth provider set includes:
    - `Kilo Gateway` device flow (POST `/api/device-auth/codes`, then polling `/api/device-auth/codes/:code`).
    - `Kilo Gateway (Manual)` token flow with token validation.
    - Generic provider prompts for provider fields.
  - Environment-driven auth/provider overrides supported:
    - `KILO_*` for generic providers,
    - `KILOCODE_*` for Kilo provider fields.
  - Config is schema-validated (AJV) plus business-rule validation for selected provider and required fields.

## 5) Strictness coverage

- TypeScript strict mode is broadly enabled:
  - `strict: true` in `src`, `webview-ui`, `cli`, `apps/storybook`, `apps/playwright-e2e`, `apps/vscode-e2e`, `jetbrains/host`, `src/test-llm-autocompletion`.
  - Shared base config (`packages/config-typescript/base.json`) also enables `strict: true` and `noUncheckedIndexedAccess: true`.
- Strong controls present in major packages:
  - `src`: `noImplicitReturns`, `noFallthroughCasesInSwitch`, but `noUnusedLocals: false`, `useUnknownInCatchVariables: false`.
  - `cli`: `strict`, `noImplicitReturns`, `noFallthroughCasesInSwitch`, `noUnusedLocals: false`, `noUnusedParameters: false`, `noUncheckedIndexedAccess: true`.
  - `apps/storybook`: `strict`, `noUnusedLocals: false`, `noUnusedParameters: false`.
- Runtime validation:
  - JSON config validation uses AJV in `cli/src/config/validation.ts` with explicit schema + business validation.
  - Error handling and auth flows include multiple guard checks for expired/invalid sessions and missing tokens.

## 6) Readiness score

- Score: **78 / 100**
- Basis:
  - + clear monorepo architecture with explicit API/provider abstraction and shared IPC/task lifecycle.
  - + enforced command surface and auth modes for both extension and CLI.
  - + broad provider coverage and real validation/refresh paths.
  - - uneven lint/type strictness posture in places (`noUnusedLocals/Parameters` disabled in key tsconfigs)
  - - mixed runtime guarantees (runtime config validation uses permissive AJV strictness) and no strict CLI/e2e policy evidence in one place.

---

## Source: research/phase-1-reports/agent-e-discovery.md

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

---

## Source: research/phase-1-reports/agent-f-governance.md

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

---

