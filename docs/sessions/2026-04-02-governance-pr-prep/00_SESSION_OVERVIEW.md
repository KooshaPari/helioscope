# heliosCLI Governance PR Prep

Date: 2026-04-02
Branch: `refactor/decouple-harness-crates`
Worktree: `/Users/kooshapari/CodeProjects/Phenotype/repos/heliosCLI`

## Goal

Prepare the current governance delta for a reviewable PR without widening the lane into unrelated
refactor or parked-worktree cleanup.

## Current Scope

- `.github/CODEOWNERS`
- `.github/RULESET_BASELINE.md`
- `.github/rulesets/main.json`
- `.github/pull_request_template.md`
- `.github/workflows/policy-gate.yml`
- `.github/workflows/pr-governance-gate.yml`
- `GOVERNANCE.md`
- `docs/governance/GOVERNANCE_SUMMARY.md`
- `docs/WORKLOG.md`

## Constraints

- `chore/governance-migration-hc` linked worktree is clean and should be treated as support state,
  not the primary lane.
- `chore/fix-dep-drift-python` is stale metadata and should stop being treated as an active lane.
- Do not pull parked work from `wip/codex-rs-core`, `fix/ci-failures`, or
  `refactor/decompose-key-router` into this PR.

## Success Criteria

- governance surfaces agree on required PR behavior
- JSON and workflow YAML validate locally
- stale lane references are reduced
- the remaining delta is small enough to split into one governance PR or a small stack
