# heliosCLI Governance Model

## Purpose

`heliosCLI` is an active production repository. Governance here is for protected-branch delivery,
stacked PR operation, and CI-backed merge safety, not for research-only experimentation.

## Protected Branch Baseline

- Pull requests are required for the default branch.
- Force pushes and branch deletion are disallowed on protected branches.
- At least one approval is required before merge.
- Review threads must be resolved before merge.
- CODEOWNERS review is required for governed paths.

## PR Policy

- Stacked PRs are first-class and must declare dependency order in the PR body.
- `fix/*` branches must not target `main` directly unless the PR carries `layered-pr-exception`.
- Merge commits inside PR branch history are disallowed.
- Local `--no-verify` usage does not justify bypassing server-side checks.

## CI Policy

- Non-billing PR checks are expected to pass before merge.
- Billing-only exceptions must be explicitly documented in the PR body.
- The checked-in ruleset baseline lives at `.github/rulesets/main.json`.
- The checked-in PR template lives at `.github/pull_request_template.md`.
- The required-check contract is documented in `.github/RULESET_BASELINE.md` and enforced in CI
  through `policy-gate.yml`, `pr-governance-gate.yml`, and `stage-gates.yml`.

## Canonical Surfaces

- `.github/rulesets/main.json`
- `.github/RULESET_BASELINE.md`
- `.github/CODEOWNERS`
- `.github/pull_request_template.md`
- `.github/workflows/policy-gate.yml`
- `.github/workflows/pr-governance-gate.yml`
- `.github/workflows/stage-gates.yml`
- `docs/phenodocs/docs/governance/stacked-prs/`

## Current PR-Prep Status

- Primary governance PR-prep lane is the root checkout on `refactor/decouple-harness-crates`.
- The linked worktree `chore/governance-migration-hc` is currently clean, so the active governance
  delta now lives primarily in the root checkout.
- The older `chore/fix-dep-drift-python` entry should still be treated as stale metadata until it
  is removed from the worklog and no longer referenced as a live lane.
