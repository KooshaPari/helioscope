# heliosCLI Governance Summary

> Status: Active delivery repo with workflow-backed PR policy and a documented ruleset baseline.

## Repo Type

- Classification: active engineering repo
- Primary branch: `main`
- Current governance prep lane: `chore/governance-pr-ready`
- Canonical governance files:
  - [`GOVERNANCE.md`](../../GOVERNANCE.md)
  - [`.github/RULESET_BASELINE.md`](../../.github/RULESET_BASELINE.md)
  - [`.github/workflows/policy-gate.yml`](../../.github/workflows/policy-gate.yml)

## Active Enforcement Surface

- `policy-gate` blocks direct `fix/* -> main` PRs unless explicitly exempted
- `policy-gate` blocks merge commits in PR diff ranges
- `rust-ci` provides the main Rust build and drift checks
- `stage-gates` adds branch-class-based gate expectations
- `sast-quick` provides fast security and secret-scan coverage on PRs

## Current PR-Prep Reality

- root checkout is on `main`
- linked worktree `chore/governance-migration-hc` is clean
- clean PR-prep branch `chore/governance-pr-ready` is the intended governance-only lane
- root checkout still carries untracked nested surfaces, so PR prep should stay isolated to the
  dedicated worktree branch

## Immediate Next Step

Use `chore/governance-pr-ready` to ship only:

1. corrected governance docs
2. ruleset baseline documentation
3. any minimal CI-policy wiring needed to make the branch reviewable
