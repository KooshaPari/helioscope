# Research

## Live repo state

- root checkout branch: `main`
- linked worktree: `chore/governance-migration-hc`
- clean PR-prep worktree: `chore/governance-pr-ready`
- root checkout is not PR-clean because it shows untracked nested surfaces

## Governance evidence used

- `policy-gate.yml` exists and runs on pull requests
- composite `policy-gate` action exists under `.github/actions/policy-gate/action.yml`
- `rust-ci.yml` provides the main Rust drift/build jobs
- `stage-gates.yml` runs on pull requests and adds branch-class policy
- `sast-quick.yml` provides fast Semgrep, secret scan, lint, and license checks

## Key correction

The tracked governance docs were stale and still described `heliosHarness` as a research repo.
That did not match the actual project identity or the workflow surface present in this checkout.
