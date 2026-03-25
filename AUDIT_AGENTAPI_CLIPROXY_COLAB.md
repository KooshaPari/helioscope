# Audit Report: agentapi, CLIProxyAPI, colab — 2026-02-28

---

## 1. agentapi-plusplus (fork of coder/agentapi)

**Status:** Very active — 20 merged PRs in last 3 days

### Branches (5)
| Branch | Ahead | Behind | Action |
|--------|-------|--------|--------|
| `mod/openapi-agent-client-v1` | 2 | 2 | **KEEP** — Open PR #288 (SDK scaffold) |
| `stack/httpapi-fix-v2-13` | 13 | 11 | DELETE — features already merged via #269-279 |
| `garden/pre-commit-config` | 1 | 5 | DELETE — merged via #283 |
| `codex/coderabbit-rate-limit-bridge` | 4 | 11 | DELETE — merged via #268 |

### Open PRs (1)
- **#288** — OpenAPI agent client module scaffold (+169/-4, today)

### Closed Unmerged PRs (19) — Mostly stacked/decomposed work
- **PR #243** (voice input, 1M+ LOC) — abandoned; re-implemented via merged PR #279
- PRs #246-267 — individual fixlets consolidated into merged stack #269-287. No unique value lost.

### Already Ahead of Upstream On
Our fork has **already fixed 7+ open upstream issues:**
| Our PR | Upstream Issue | Fix |
|--------|---------------|-----|
| #270 | #178 | afero.Walk on embedded FS |
| #272 | #126 | First line of response trimmed |
| #254 | #174 | OpenCode screenDiff wrong content |
| #248 | #171 | AskUserQuestion input support |
| #278 | #117 | Slash command support |
| #279 | #180 | Voice input (Web Speech API) |
| #277 | #187 | Pagination for messages endpoint |

**Action:** Contribute these fixes back to upstream coder/agentapi.

### Upstream Cherry-Pick Candidates
| PR | Feature | Lines | Priority |
|----|---------|-------|----------|
| #177 | State persistence | 2,435 | HIGH |
| #192 | Asciinema e2e testing | 604 | MEDIUM |
| #184 | Partial/malformed tool call handling | 48 | MEDIUM |
| #86 | Abstract CLI Agent Handler | 519 | LOW |

### Upstream Issues (100+)
Top bugs already fixed in our fork. Remaining interesting:
- #123 — Screen stabilize timeout (CRITICAL)
- #48 — Refactor to use claude-code SDK
- #138 — Python client library + CLI
- #52 — Sign macOS binaries

### Forks
30 forks, none with significant adoption or divergent features. Our fork is the most active.

---

## 2. CLIProxyAPI (original, DEPRECATED)

**Status:** Archived/deprecated → successor is cliproxyapi-plusplus

### Branches (13)
| Branch | Ahead | Behind | Action |
|--------|-------|--------|--------|
| `blacklist` | 4 | 21 | REVIEW then DELETE |
| `dev` | 0 | 21 | DELETE — dead |
| `legacy` | 0 | 574 | DELETE — ancient |
| 9x dependabot/* | 1 | 3 each | Merge or close all |

### Open PRs (9) — All Dependabot
- Alpine 3.23.3, gin 1.11.0, pgx 5.8.0, logrus 1.9.4, brotli 1.2.0, codecov-action 5, goreleaser-action 7, golang-org updates, actions group updates
- All stalled 4 days without review

### Closed PRs: None (no rejected work)

### Issues (100 open, 0 closed)
- 30 high-priority, 58 medium, 12 low
- 70 feature requests, 14 refactors, 7 security
- Key: #115 (TLS/mTLS support), #103 (provider outage handling), #102 (token expiry)
- **None addressed** — all planning, zero execution

### Recommendation
- Mass-close/migrate issues to cliproxyapi-plusplus
- Merge or close 9 dependabot PRs
- Delete stale branches (dev, legacy, blacklist)
- Consider full archive (read-only)

---

## 3. colab (fork of blackboardsh/colab)

**Status:** Active — 4 helios integration phases merged in 2 days

### Fork Sync: 4 ahead, 0 behind upstream (clean divergence)

### Branches (5)
| Branch | Ahead | Behind | Action |
|--------|-------|--------|--------|
| `helios-integration` | 2 | 4 | Merged — safe to delete |
| `helios-phase3` | 3 | 2 | Merged — safe to delete |
| `helios-phase4` | 2 | 1 | Merged — safe to delete |
| `toolchain-modernization` | 2 | 0 | **KEEP** — Open PR #6 |

### PRs
| PR | Title | Status | Lines |
|----|-------|--------|-------|
| #1 | Phase 1: helios runtime integration | MERGED | +2370/-2 |
| #2 | Phase 2: persistence, boundary dispatcher | MERGED | +429/-35 |
| #4 | Phase 3: adapter wiring, xterm.js, RuntimeMetrics | MERGED | +442/-24 |
| #5 | Phase 4: renderer surfaces + engine switch | MERGED | +2841/-7 |
| **#6** | **Toolchain modernization** | **OPEN** | **+1887/-278** |
| #3 | Phase 3 (superseded by #4) | CLOSED | — |

### Upstream Opportunities
- **PR #2** (theme switching, +1491/-352) — cherry-pick candidate for UI work
- **Issue #3** — macOS M1/Sequoia launch failure (UNRESOLVED) — verify if helios stack resolves this

### Forks
10 total forks of upstream, none with significant divergent features. Our fork is the most feature-rich.

---

## 4. agentapi (original, planning-only)

**Status:** Empty planning repo — 28 open issues, 0 code, 0 PRs

- 7 epics covering: Foundation/CI, Docs, Quality, API Gateway, Multi-Agent Orchestration, Provider Integration, Memory, Core Framework, Tool Integration
- All implementation is in agentapi-plusplus
- **Action:** Keep as roadmap reference or consolidate into agentapi-plusplus issues

---

## Cleanup Commands

```bash
# agentapi-plusplus: delete merged branches
gh api -X DELETE repos/KooshaPari/agentapi-plusplus/git/refs/heads/garden/pre-commit-config
gh api -X DELETE repos/KooshaPari/agentapi-plusplus/git/refs/heads/codex/coderabbit-rate-limit-bridge
gh api -X DELETE repos/KooshaPari/agentapi-plusplus/git/refs/heads/stack/httpapi-fix-v2-13

# CLIProxyAPI: delete stale branches
gh api -X DELETE repos/KooshaPari/CLIProxyAPI/git/refs/heads/dev
gh api -X DELETE repos/KooshaPari/CLIProxyAPI/git/refs/heads/legacy

# colab: delete merged phase branches
gh api -X DELETE repos/KooshaPari/colab/git/refs/heads/helios-integration
gh api -X DELETE repos/KooshaPari/colab/git/refs/heads/helios-phase3
gh api -X DELETE repos/KooshaPari/colab/git/refs/heads/helios-phase4
```

---

## Cross-Repo Summary

| Repo | Active Branches | Open PRs | Unmerged Value | Upstream Ops |
|------|----------------|----------|----------------|---------------|
| agentapi-plusplus | 1 (SDK scaffold) | 1 (#288) | Low (stacked work merged) | HIGH (7 fixes to contribute back + 3 cherry-picks) |
| CLIProxyAPI | 0 (deprecated) | 9 (dependabot) | None | N/A (deprecated) |
| colab | 1 (toolchain mod) | 1 (#6) | PR #6 (+1887 lines) | LOW (theme PR #2) |
| agentapi | 0 (planning only) | 0 | N/A | N/A |

*Generated by 4 parallel research agents.*
