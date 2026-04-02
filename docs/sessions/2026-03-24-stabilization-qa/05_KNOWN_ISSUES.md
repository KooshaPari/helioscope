# Known Issues: heliosCLI Stabilization Session

## 1. Current Issues

| Issue | Severity | Impact | Status |
|-------|----------|--------|--------|
| Rust compilation time | Low | Time cost | Known |
| rustfmt import warnings | Low | Noise | Documented |
| Stale lane ledger entries | Medium | Misleading planning state | Open |
| Prunable worktree metadata | Medium | Queue confusion | Mitigated |

## 2. Technical Debt

| Item | Description | Priority |
|------|-------------|----------|
| CI pipeline | Not yet integrated with formatting checks | Medium |
| Coverage tracking | Need to add coverage reports | Low |
| Worktree hygiene | Reconcile or delete prunable checkout references | Medium |
| PR review hygiene | GitHub review threads and bot comments still need explicit resolution after code fixes land | Medium |

## 3. Future Work

- [ ] Integrate Prettier into CI pipeline
- [ ] Add code coverage tracking
- [ ] Performance benchmarking baseline
- [ ] Documentation for multi-stack architecture
- [x] Reconcile `chore/fix-dep-drift-python` before reusing or referencing it
- [ ] Re-evaluate the `fix/rollout-limit-expect` lane before merge
- [ ] Resolve the remaining PR #179 review threads after the harness workspace repair is pushed
- [ ] Decide whether non-member harness crates should be promoted into the root workspace or remain path-only crates
