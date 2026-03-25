# heliosCLI Migration Checklist (2026-03-03)

- [x] Canonical target confirmed as `heliosCLI`.
- [x] Captured both repo HEAD SHAs and active branches.
- [x] Generated tracked-file delta manifests excluding artifact/build noise (`dist/`, `build/`, `target/`, `.cache/`).
- [x] Normalized dirty worktrees onto dedicated safety branches with non-destructive commits.
- [x] Preserved untracked artifact noise (`helios-cli/repos/`) without destructive operations.
- [x] Wrote local archive snapshot package under `.archive/absorb-readiness-2026-03-03/`.
- [ ] Open PR from `chore/normalize-dirty-20260303-heliosCLI` after review.
- [ ] Open PR from `chore/normalize-dirty-20260303-helios-cli` after review.
- [ ] Execute absorb plan: merge required deltas into canonical `heliosCLI` in stacked PR order.
- [ ] Archive or quarantine lowercase mirror paths only after post-merge verification.
