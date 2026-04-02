# heliosCLI Ruleset Baseline

This repository already has strong policy and stage-gate workflows. The checked-in ruleset
baseline now matches the active remote protection posture for the default branch.

## Enforced Branch Protection Baseline

- require pull requests before merge
- no force push / non-fast-forward updates
- no branch deletion
- require at least 1 approval
- dismiss stale approvals on new push
- require resolved review threads before merge
- allow merge methods: `merge`, `squash`
- enable GitHub `code_quality`
- enable GitHub `copilot_code_review`

## Repo-Local Governance Gates

The repo-local workflow set remains the main policy surface for CI and staged release checks:

- `policy-gate`
- `pr-governance-gate`
- `security-guard`
- `sast-quick`
- `sast-full`
- `stage-gates`
- `rust-ci` / `quality`
- `sentry-error-tracking`
- `snyk-scan`

Required status check mapping is still pending; it should be encoded once the durable job names are
re-verified and the billing-only exception set is explicitly excluded.

## Branch Policy

- stacked PR branches such as `stack/*`, `layer/*`, `preview/*`, and release lanes are first-class
- `fix/*` must not target `main` or `master` unless the PR carries a documented exception label
  such as `layered-pr-exception`
- merge commits in PR branches are disallowed by the repo-local governance workflows
- local `--no-verify` usage is not accepted as a reason to bypass server-side workflow checks

## Exception Policy

- only documented billing or quota failures may be excluded from required checks
- billing exceptions require a `ci-billing-exception` label and an explicit note in the PR body
- review threads and blocking comments must be resolved before merge
