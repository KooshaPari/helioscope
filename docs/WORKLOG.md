# Worklog

Active work tracking for **heliosCLI**.

---

## Current Lanes

| Lane | Branch | Worktree | Status | Notes |
| --- | --- | --- | --- | --- |
| Governance baseline review | `chore/governance-migration-hc` | `/Users/kooshapari/CodeProjects/Phenotype/repos/heliosCLI/.worktrees/chore-govern-pi` | PR in review (PR #184) | Clean governance docs/commands branch; apply all doc validation fixes here before merging. |
| Governance-pr-ready companion | `chore/governance-pr-ready` | same | Companion review (PR #182) | Shares the same clean baseline; keep this lane reserved for status-check adjustments and bin any unrelated files. |
| Canonical `main` (do not author here) | `main` | `/Users/kooshapari/CodeProjects/Phenotype/repos/heliosCLI` | Dirty | Root checkout contains extra `.worktrees/`, docs, and temporary state; do not push from this state. |
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

1. Keep all governance doc changes within `chore/governance-migration-hc` (PR #184) so the clean branch remains reviewable, and mirror any shared updates to its companion `#182` lane only when the doc is ready.
2. Avoid editing `main`; treat it as the polluted base and route any new PR work into the clean worktree above.
3. Once both governance PRs merge, decide whether to restart subsidiary lanes (`fix/ci-failures`, `refactor/decompose-key-router`) from new clean branches rather than the canonical root.

---

_Last updated: 2026-04-02_
