# ZAP DAST workflow fix

## Goal

Fix the `ZAP DAST failing on main` alert by repairing the GitHub Actions workflow in
`.github/workflows/zap-dast.yml`.

## Scope

- Stop pinning a dead ZAP release asset URL.
- Reduce checkout cleanup risk with archived submodule/worktree baggage.

## Success criteria

- Workflow can download the scanner bundle again.
- The alert-sync issue can be re-evaluated after the patch is merged.
