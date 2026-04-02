# heliosCLI Governance Summary

> **Status**: Active production repository.

## Merge Safety

- PRs are required for the default branch.
- Force pushes to protected branches are disallowed.
- At least one approval and resolved review threads are required before merge.
- CODEOWNERS review is required for governed paths.

## Stacked PRs

- Stacked branches are supported and expected for larger delivery waves.
- PRs should document dependency order and follow-up PRs.
- `fix/* -> main` requires an explicit exception label.

## CI Expectations

- All non-billing PR checks are expected to pass before merge.
- Billing-only exceptions must be declared in the PR body.
- `policy-gate.yml`, `pr-governance-gate.yml`, and `stage-gates.yml` remain the key CI-side
  governance workflows.
- `.github/rulesets/main.json` is the checked-in GitHub ruleset baseline.

## Canonical References

- `.github/RULESET_BASELINE.md`
- `.github/pull_request_template.md`
- `docs/phenodocs/docs/governance/stacked-prs/`
- `docs/guides/policy-gate-module.md`

## Current PR-Prep Note

- Governance changes are closest to reviewable on the root checkout branch
  `refactor/decouple-harness-crates`.
- The linked worktree `chore/governance-migration-hc` is currently clean, so the remaining cleanup
  is mainly to stop treating `chore/fix-dep-drift-python` as a live lane and to keep the root
  checkout governance files aligned.
