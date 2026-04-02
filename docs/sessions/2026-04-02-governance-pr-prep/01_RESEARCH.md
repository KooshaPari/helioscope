# Research

## Local State

- Root checkout branch: `refactor/decouple-harness-crates`
- Linked governance worktree: `heliosCLI/.worktrees/chore-govern-pi`
- Stale metadata lane: `heliosCLI/worktrees/chore/fix-dep-drift-python`
- One unrelated stash exists:
  - `stash@{0}: WIP on ci/trigger-workflows: 31ee576 ci: trigger security workflows`

## Validation Results

- `.github/rulesets/main.json` parses successfully with `jq`
- `.github/workflows/policy-gate.yml` parses successfully
- `.github/workflows/pr-governance-gate.yml` parses successfully

## Key Finding

The governance lane is coherent enough for PR preparation. The remaining blocker is not broken
policy files; it is operational cleanup:

- keep the governance scope isolated
- stop referencing stale worktree metadata as if it were live
- decide whether the current delta should ship as one governance PR or a small stacked set
