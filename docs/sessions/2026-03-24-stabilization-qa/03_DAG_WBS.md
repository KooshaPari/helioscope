# DAG & WBS: heliosCLI Stabilization Session

## 1. Dependency Graph (DAG)

```
[Check Formatting] → [Fix Issues] → [Run Tests] → [Verify Dependencies] → [Static Analysis] → [Commit]
       ↓                    ↓              ↓               ↓                    ↓            ↓
   Prettier            Manual fix      pytest          cargo build         clippy      git push
   rustfmt               ↕             ↕                  ↕                ↕
                    [Auto-fix]       [Debug]          [Resolve deps]      [Review]
```

## 2. Work Breakdown Structure (WBS)

### Phase 1: Formatting
| Task | ID | Effort | Status |
|------|----|--------|--------|
| Check Prettier on JS/TS | 1.1 | 2 min | ✅ Done |
| Check rustfmt on Rust | 1.2 | 5 min | ✅ Done |
| Fix any formatting issues | 1.3 | Varies | ✅ Done |

### Phase 2: Testing
| Task | ID | Effort | Status |
|------|----|--------|--------|
| Run TypeScript tests | 2.1 | 3 min | ✅ Done |
| Verify all pass | 2.2 | 1 min | ✅ Done |

### Phase 3: Dependencies
| Task | ID | Effort | Status |
|------|----|--------|--------|
| Install pnpm deps | 3.1 | 5 min | ✅ Done |
| Build Rust workspaces | 3.2 | 10 min | ✅ Done |
| Verify SDK builds | 3.3 | 3 min | ✅ Done |

### Phase 4: Analysis
| Task | ID | Effort | Status |
|------|----|--------|--------|
| Run clippy | 4.1 | 5 min | ✅ Done |
| Review warnings | 4.2 | 2 min | ✅ Done |
| Verify UTF-8 encoding | 4.3 | 2 min | ✅ Done |

## 3. Total Estimated Time
- **Actual**: ~35 minutes
- **Formatting**: 10 min
- **Testing**: 4 min
- **Dependencies**: 18 min
- **Analysis**: 9 min

## 4. Current Follow-Up DAG

```
[Verify rollout limit behavior] → [Reconcile parked worktrees] → [Prune stale metadata]
                ↓                         ↓                         ↓
           fix/rollout-limit-expect   wip/codex-rs-core       chore/fix-dep-drift-python
```

### 4.1 Current WBS
| Task | ID | Effort | Status |
|------|----|--------|--------|
| Re-verify `fix/rollout-limit-expect` | 4.1 | 15 min | In progress |
| Decide `wip/codex-rs-core` disposition | 4.2 | 10 min | Open |
| Close or prune `fix/ci-failures` and `refactor/decompose-key-router` | 4.3 | 20 min | Open |
| Reconcile prunable worktree metadata | 4.4 | 10 min | Open |
