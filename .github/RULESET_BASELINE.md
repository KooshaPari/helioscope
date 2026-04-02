# heliosCLI Ruleset Baseline

This file defines the intended GitHub-side ruleset posture for `heliosCLI` until repo settings are
verified and synced directly.

## Branches

- protect `main`
- disallow force-push
- disallow branch deletion
- require pull requests for merge
- require at least one approval
- dismiss stale approvals on push
- require review thread resolution before merge

## Merge Policy

- no direct fix branches to `main` without explicit `layered-pr-exception`
- no merge when policy or CI checks are failing
- require branch to be up to date with base before merge
- permit stacked PRs only when each layer is independently reviewable

## Minimum Required Checks

These names should be verified against live GitHub check-run titles before server-side enforcement:

- `policy-gate`
- `Drift detection`
- `Format / etc`
- `cargo shear`
- `Stage Gates / detect-stage`
- `Semgrep Scan`
- `Secret Scanning`
- `Rust Lint`
- `License Compliance`

## Notes

- `quality.yml` is currently manual-only and should not be treated as a merge blocker until it runs
  on pull requests.
- Helper or babysit workflows should stay outside the required-check set.
- If a required job name changes, update this file and the corresponding GitHub branch protection in
  the same tranche.
