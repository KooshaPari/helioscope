# Validation

## Commands
- `cargo check -p codex-rmcp-client` in `codex-rs/`
- `cargo test -p codex-rmcp-client` in `codex-rs/`
- `cargo test -p codex-api` in `codex-rs/`
- `cargo test -p codex-core client_common::tests::rejects_non_strict_output_schema -- --nocapture` in `codex-rs/`
- `cargo test -p codex-process-hardening` in `codex-rs/`
- `cargo fmt --all --check` in `codex-rs/`

## Outcome
- `cargo check -p codex-rmcp-client` passed.
- `cargo test -p codex-rmcp-client --test resources` passed when run with
  `CARGO_TARGET_DIR=/tmp/helioscli-rmcp-target`.
- `cargo test -p codex-rmcp-client` passed in the repo target dir.
- The streamable HTTP resources test now short-circuits cleanly when
  `CODEX_SANDBOX_NETWORK_DISABLED` is set.
- `cargo test -p codex-api` passed after adding strict output-schema validation.
- `cargo test -p codex-state` passed after adding strict output-schema validation to
  `StateRuntime::create_agent_job`.
- `cargo test -p codex-core client_common::tests::rejects_non_strict_output_schema -- --nocapture`
  still fails in unrelated `mcp_connection_manager.rs` and `tools/handlers/mcp_resource.rs`
  non-exhaustive struct construction sites.
- `cargo test -p codex-process-hardening` passed after extracting the Windows
  injection-env denylist into a tested helper.
- `codex-core` now reuses the shared Windows injection-env denylist when transforming
  Windows restricted-token exec requests, and the child-env sanitize helper is unit-tested.
- `codex-windows-sandbox` now strips the same Windows injection env vars before both
  restricted-token and elevated launches.
- `codex-windows-sandbox` now assigns both Windows sandbox launch paths to a
  kill-on-close job object after process creation.
- `codex-command-runner` now uses the shared kill-on-close job helper instead of a
  local copy of the same Windows job-object setup.
- `setup_orchestrator` now removes common Windows injection env vars before the
  non-elevated setup refresh helper is spawned.
- `setup_main_win` now strips the same Windows injection env vars before spawning its
  internal read-ACL helper.
- `rustfmt --check` on the touched `rmcp-client` Rust files passed.
- `cargo fmt --all --check` still reports pre-existing formatting drift in unrelated `codex-rs/core` and `codex-rs/tui` files.
- `python3 - <<'PY' ... yaml.safe_load(...)` passed for `.github/workflows/snyk-scan.yml`
  after adding the missing-token preflight guard.
- The Snyk workflow now skips cleanly before checkout when `SNYK_TOKEN` is absent instead
  of entering interactive browser auth.
- `cargo test -p codex-rmcp-client --test resources` now covers both stdio and streamable HTTP
  resource transports.
- `create_text_param_for_request` now rejects non-strict JSON Schemas before request
  serialization, including missing `required` members or `additionalProperties: false`.
- `codex-state` now rejects invalid job output schemas before persisting an agent job.
- `codex-process-hardening` now strips common Windows injection env vars in its
  pre-main hardening hook, and the denylist is pinned by a unit test.
