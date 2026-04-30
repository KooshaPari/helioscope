# Known Issues

## Current
- No known behavioral regression from the patch itself.
- The Snyk workflow now guards against missing `SNYK_TOKEN` and exits cleanly instead of
  timing out on browser auth; the guard runs before checkout so the tracked worktree
  warning is not hit on skipped runs.
- Strict output-schema requests now fail fast in `codex-api` if the schema is missing
  `additionalProperties: false` or does not list every property in `required`.
- `codex-state` now rejects invalid agent-job output schemas before insert, but the
  full `AgentJob` typed-schema redesign from `#122` is still incomplete locally.
- `codex-process-hardening` now sanitizes common Windows injection env vars, but the
  broader Windows restricted-token/job-object hardening path is still not implemented.
- The Windows hardening denylist is now shared between startup and child-env transform
  paths, but the code still does not isolate child processes with Windows job objects yet.
- `codex-windows-sandbox` now inherits the denylist in both elevated and restricted-token
  launch paths, but it still relies on the existing CreateProcessAsUserW/CreateProcessWithLogonW
  model rather than a dedicated wrapper helper.
- The sandbox job object is kill-on-close only; it does not yet configure nested job
  limits or richer Windows process mitigation policies.
- `codex-command-runner` still relies on the main CreateProcessAsUserW flow and does
  not apply extra Windows process mitigations beyond the shared kill-on-close job.
- `setup_orchestrator` now strips unsafe env vars for the non-elevated refresh helper,
  but the elevated ShellExecuteExW helper still inherits the parent shell env.
- `setup_main_win` now strips unsafe env vars for its read-ACL helper, but the parent
  elevated launch still relies on ShellExecuteExW and cannot pass a custom env block.
- Workspace-wide formatting drift still exists outside the touched files from earlier
  checks.

## Follow-up
- Re-run `cargo fmt --all --check` if you want to chase the unrelated formatting drift elsewhere in `codex-rs`.
- If repo secret access is restored later, rerun the Snyk workflow to re-enable live scan and monitor jobs.
- `cargo test -p codex-core ...` is still blocked by unrelated non-exhaustive struct
  construction errors in `mcp_connection_manager.rs` and `tools/handlers/mcp_resource.rs`.
