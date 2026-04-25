# Validation

Validated against the live workflow surface referenced by the docs:

- `.github/workflows/policy-gate.yml`
- `.github/actions/policy-gate/action.yml`
- `.github/workflows/rust-ci.yml`
- `.github/workflows/stage-gates.yml`
- `.github/workflows/sast-quick.yml`

The branch also remains isolated from the dirty root checkout because the edits live in the
dedicated `chore/governance-pr-ready` worktree.
