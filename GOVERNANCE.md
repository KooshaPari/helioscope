# heliosCLI Governance Model

## Purpose

`heliosCLI` is an active Rust CLI repo. Governance here is delivery-focused, not research-only:

- changes land through reviewable pull requests
- protected branches do not accept force-pushes or direct fixes
- CI and policy gates must be green before merge
- stacked PRs are allowed only when each layer is independently reviewable

This file is the repo-local control surface for branch and merge discipline. It complements
[`AGENTS.md`](./AGENTS.md), the workflow files under [`.github/workflows`](./.github/workflows), and
the repo-local ruleset baseline at [`.github/RULESET_BASELINE.md`](./.github/RULESET_BASELINE.md).

## Protected Branch Posture

- Treat `main` as protected.
- Do not force-push shared branches.
- Do not merge with unresolved review threads or failing checks.
- Do not use `--no-verify` as normal workflow on active lanes.
- Use personal feature branches or worktree branches for intermediate cleanup.

## Active Governance Gates

The repo currently has these live governance and CI surfaces:

- [`.github/workflows/policy-gate.yml`](./.github/workflows/policy-gate.yml)
- [`.github/actions/policy-gate/action.yml`](./.github/actions/policy-gate/action.yml)
- [`.github/workflows/rust-ci.yml`](./.github/workflows/rust-ci.yml)
- [`.github/workflows/stage-gates.yml`](./.github/workflows/stage-gates.yml)
- [`.github/workflows/sast-quick.yml`](./.github/workflows/sast-quick.yml)

These are the real local enforcement surfaces. This branch does not currently expose a tracked
`.github/rulesets/` directory, so repo settings must be synchronized against the documented
baseline rather than inferred from missing local JSON.

## Pull Request Rules

- `fix/*` branches must not target `main` directly unless the PR carries the explicit
  `layered-pr-exception` label.
- Merge commits inside a PR branch diff are blocked by `policy-gate`.
- Prefer a short stack over one oversized mixed PR.
- Every PR should map to one logical lane: governance, CI/bootstrap, docs, runtime, or product.
- Helper automation workflows are not themselves merge policy; the merge blockers should stay tied
  to stable CI and policy jobs.

## Local Worktree Discipline

Current local work shows three relevant surfaces:

- root checkout on `main`
- linked worktree on `chore/governance-migration-hc`
- clean PR-prep worktree on `chore/governance-pr-ready`

The nested worktree path for `chore/governance-pr-ready` is awkward, but the branch itself is
valid and currently serves as the clean isolation surface for governance-only prep.

## Merge Exception Policy

The only acceptable merge exception is GitHub Actions billing or quota exhaustion where jobs never
start. That exception still requires:

- equivalent local verification
- explicit worklog or session evidence
- no red failing policy or test jobs

Failing checks, unresolved comments, and convenience bypasses are not valid exception paths.
