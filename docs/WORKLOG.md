# Worklog

Active work tracking for **heliosCLI**.

---

## Current Lanes

| Lane | Branch | Worktree | Status | Notes |
| --- | --- | --- | --- | --- |
| Root checkout stabilization | `refactor/decouple-harness-crates` | `/Users/kooshapari/CodeProjects/Phenotype/repos/heliosCLI` | Active | Current main checkout; use this for decouple/refactor follow-up and validation |
| Rollout limit safety fix | `fix/rollout-limit-expect` | `/Users/kooshapari/CodeProjects/Phenotype/repos/heliosCLI` | Draft PR open | PR [#130](https://github.com/KooshaPari/heliosCLI/pull/130), base `main`; merge only after limit behavior is re-verified |
| Governance migration lane | `chore/governance-migration-hc` | `/Users/kooshapari/CodeProjects/Phenotype/repos/heliosCLI/.worktrees/chore-govern-pi` | Clean support lane | Linked worktree is clean; active governance delta is now primarily in the root checkout |
| Codex core parked work | `wip/codex-rs-core` | `/Users/kooshapari/CodeProjects/Phenotype/repos/heliosCLI-wtrees/codex-rs-core` | Active | Parked WIP lane, not merged |
| CI failures lane | `fix/ci-failures` | `/Users/kooshapari/CodeProjects/Phenotype/repos/heliosCLI-wtrees/fix-ci-failures` | Active | Side lane in progress |
| Key router decomposition | `refactor/decompose-key-router` | `/Users/kooshapari/CodeProjects/Phenotype/repos/heliosCLI/heliosCLI-wtrees/decompose-key-router` | Active | Refactor lane in progress |
| Drifted worktree metadata | `chore/fix-dep-drift-python` | `/Users/kooshapari/CodeProjects/Phenotype/repos/heliosCLI/worktrees/chore/fix-dep-drift-python` | Stale | Path is no longer active locally; remove or stop referencing it as a live lane |

---

## Merged Baseline on `main`

Recent merged commits already on `origin/main`:

1. `#126` Deprecated criterion cleanup
2. `#128` kitty-specs to docs/specs migration
3. `#125` rust-ci/codespell/cargo-deny fixes
4. `#127` Additional deprecated criterion cleanup
5. `#124` KeyEventRouter extraction refactor

---

## Remaining Work

1. Re-verify the rollout limit expectation lane before merging `#130`; the branch is live but the safety behavior still needs a fresh pass.
2. Decide disposition of `wip/codex-rs-core`: split into reviewable PRs or keep it explicitly parked.
3. Finish or close `fix/ci-failures` and `refactor/decompose-key-router` lanes, then prune any stale worktree metadata.
4. Remove or archive the stale `chore/fix-dep-drift-python` lane reference so the ledger matches the actual checkout set.

## PR-Prep Focus

The governance lane is the first reviewable candidate once worktree drift is reconciled. The
checked-in surfaces that must stay aligned are:

- `.github/rulesets/main.json`
- `.github/RULESET_BASELINE.md`
- `.github/pull_request_template.md`
- `.github/workflows/policy-gate.yml`
- `.github/workflows/pr-governance-gate.yml`
- `docs/sessions/2026-04-02-governance-pr-prep/`

---

_Last updated: 2026-04-01_
