# Worklog

Active work tracking for `heliosCLI`.

Updated: 2026-04-02

## Current Lanes

| Lane | Branch | Worktree | Status | Notes |
| --- | --- | --- | --- | --- |
| Governance PR prep | `chore/governance-pr-ready` | `/Users/kooshapari/CodeProjects/Phenotype/repos/heliosCLI/heliosCLI/.worktrees/governance-pr-ready` | Active | Clean prep lane for repo-local governance docs and ruleset baseline |
| Governance migration side lane | `chore/governance-migration-hc` | `/Users/kooshapari/CodeProjects/Phenotype/repos/heliosCLI/.worktrees/chore-govern-pi` | Clean / parked | Existing side worktree, not currently carrying new diff |
| Canonical checkout | `main` | `/Users/kooshapari/CodeProjects/Phenotype/repos/heliosCLI` | Mixed local state | Untracked nested `.worktrees/`, `docs/phenodocs/`, and accidental nested `heliosCLI/` path |

## PR-Prep Assessment

- The earlier story that `heliosCLI` had no active PR lane was wrong.
- The repo does have a clean governance-only lane now, but it lives in the dedicated
  `chore/governance-pr-ready` worktree rather than the root checkout.
- The root checkout should not be used as the review branch because its local state is polluted by
  nested and untracked surfaces.

## Merge Policy Baseline

The intended merge blockers for active governance work are:

- `policy-gate`
- stable Rust CI jobs from `rust-ci`
- `Stage Gates / detect-stage`
- fast security checks from `sast-quick`

See:

- [../../GOVERNANCE.md](../../GOVERNANCE.md)
- [../../.github/RULESET_BASELINE.md](../../.github/RULESET_BASELINE.md)
- [governance/GOVERNANCE_SUMMARY.md](./governance/GOVERNANCE_SUMMARY.md)

## Follow-Up

1. Keep governance-only edits in `chore/governance-pr-ready`.
2. Leave unrelated root-checkout untracked surfaces out of the PR.
3. Reconcile or remove the accidental nested worktree path only after the governance PR branch is
   safely captured elsewhere.
