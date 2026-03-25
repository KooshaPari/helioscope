<DONE>
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
