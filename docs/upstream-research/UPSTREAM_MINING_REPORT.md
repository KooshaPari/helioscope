# Upstream Feature Mining Report — 2026-02-28

## Executive Summary

Comprehensive analysis of openai/codex PRs, issues, forks, and existing branches across helios-cli, cliproxyapi-plusplus, and portage. OpenAI closed codex to external contributions on 2026-02-28 — unmerged community features will never land upstream, creating a differentiation window.

---

## 1. Unmerged PRs from openai/codex (Top 10)

| Priority | PR | Feature | Lines | Difficulty |
|----------|----|---------|-------|------------|
| **P0** | #12523 | Markdown table rendering (Unicode borders) | ~5,800 | Medium |
| **P0** | #12864 | Plugin system (config.toml skill packs) | ~1,380 | Low |
| **P0** | #12703 | Model toggle Ctrl+O (ring buffer) | ~950 | Low |
| **P1** | #13100 | MCP transport recovery | ~482 | Low |
| **P1** | #11865 | Vim mode for TUI | ~795 | Medium |
| **P1** | #12951 | OSC-52 copy (tmux/SSH clipboard) | ~210 | Low |
| **P1** | #12850 | Realtime audio device picker | ~880 | Medium |
| **P2** | #12803 | Live skill refresh notifications | ~351 | Low |
| **P2** | #11549 | Permissions customization UI | ~203 | Low-Med |
| **P2** | #12687 | Realtime collaboration mode | ~800 | High |

**Key contributors:** Felipe Coury, Dylan Hurd, Michael Bolin, Adam Koszek, Skylar Graika

---

## 2. Top Issues from openai/codex (Actionable for helios-cli)

### Critical Bugs to Fix First
| Issue | Title | Comments | Effort |
|-------|-------|----------|--------|
| #12764 | 401 Unauthorized streaming failure | 26 | High |
| #13095 | Unicode confusable sandbox bypass | 0 | Low (1-2d) |
| #13103 | Cannot disable WebSocket transport | 9 | Low (1-2d) |
| #13041 | WebSocket policy closure (code 1008) | 14 | High |

### High-Value Features from Issues
| Issue | Title | Comments | Effort |
|-------|-------|----------|--------|
| #12283 | Sandbox policy config in rules | 14 | Medium |
| #11915 | Read-only approval mode restoration | 12 | Medium |
| #13024 | Persistent instructions in config.toml | 6 | Low |
| #12380 | Model metadata overrides (Azure/proxy) | 7 | Medium |

### Quick Wins (1-2 days each)
- #13103 — `--transport https` flag
- #13095 — Unicode NFC normalization
- #12840 — Auto light/dark theme detection
- #12129 — Swap ENTER/Ctrl+Enter behavior
- #12299 — Fix rate limit UX (shows 10% as limit)

---

## 3. Notable Codex Forks with Extractable Features

| Fork | Ahead | Stars | Key Features |
|------|-------|-------|--------------|
| **just-every/code** | 4,250 | 3,500 | Multi-provider orchestration (OpenAI+Claude+Gemini), browser integration, theming |
| **zapabob/codex** | 979 | 4 | Parallel subagent scaling (2x), memory management, local cache with TTL |
| **Eriz1818/xCodex** | 537 | 6 | Secret/credential redaction with reveal toggles, plan mode persistence, lazy MCP priming |
| **Haleclipse/codex (Cometix)** | 80 | 203 | Custom statusline (CxLine), full model catalog, i18n/localization |

**High-priority patterns to extract:**
- Secret/credential redaction (xCodex) — CRITICAL for enterprise
- Memory/cache management (zapabob) — performance edge
- Plan persistence with iteration (xCodex) — workflow improvement
- Multi-provider support (just-every/code) — if expanding beyond OpenAI

---

## 4. Existing Branch Status

### helios-cli (KooshaPari/helios-cli)

**Has Unique Unmerged Value:**
| Branch | Ahead | Status | Content |
|--------|-------|--------|---------|
| `bug-sweep-report` | 36 | No PR | Security review UI, HTML report viewer, git blame integration, mermaid diagrams |
| `pr/network-proxy-sandbox` | 2 | Stale | Network proxy crate + sandbox integration |
| `pr/network-proxy-tui` | 3 | Stale | Network proxy + TUI approval prompts |

**Active PRs:**
- PR #276 (`codex/ascii-loader-pr`) — Animated ASCII HELIOS morph on startup
- PR #280 (`mod/cli-task-surface-v1`) — CLI task surface wrapper module
- PR #281 (`mod/policy-gate-v1`) — Reusable policy-gate composite action

**Safe to Delete (8 branches):** Already merged or superseded — `codex/tui-renderer-spec-docs-with-core`, `fix/policy-gate-pr-head-sha`, `replay/release-framework`, `wave2-lane-a-docs-unification`, `codex/tui-renderer-spec-docs`, `codex/tui-renderer-spec-docs-clean`, `wave2-lane-a-docs-unification-heliosCLI`, `chore/fresh-context-send-input`

### cliproxyapi-plusplus (KooshaPari/cliproxyapi-plusplus)

**Open PR:** #699 (`mod/proxy-auth-access-sdk-v1`) — Proxy auth access SDK scaffold, ready for review

**Closed with Unmerged Value:**
- PR #695 (`wave3/lane-c-docs-unification`) — CategorySwitcher, doc scripts (but removes executor code — selective cherry-pick needed)
- `upstream-feature-replay` branch — Enhanced VitePress docs (417 insertions)

**Already in Main:** PRs #693, #694, #696, #697, #698 (upstream replays, docs, policy gates)

### portage (KooshaPari/portage)

**Clean.** Only 1 active branch: `mod/queue-orchestrator-v1` with open PR #228 (queue orchestrator scaffold, +363 lines).

---

## 5. Implementation Roadmap

### Phase 1: Quick Wins + Security (Week 1)
- [ ] Unicode NFC sandbox bypass fix (from #13095)
- [ ] `--transport https` flag (from #13103)
- [ ] Auto theme detection (from #12840)
- [ ] Secret/credential redaction (from xCodex fork)
- [ ] Review + merge `bug-sweep-report` branch (36 commits)

### Phase 2: Core Features (Weeks 2-4)
- [ ] Markdown table rendering (PR #12523)
- [ ] Plugin system (PR #12864)
- [ ] Model toggle Ctrl+O (PR #12703)
- [ ] Sandbox policy rules config (from #12283)
- [ ] Read-only approval mode (from #11915)

### Phase 3: Power User Features (Weeks 5-7)
- [ ] Vim mode (PR #11865)
- [ ] MCP transport recovery (PR #13100)
- [ ] OSC-52 copy (PR #12951)
- [ ] Memory/cache management (from zapabob fork)
- [ ] Plan persistence (from xCodex fork)

### Phase 4: Advanced (Weeks 8+)
- [ ] Multi-provider support (from just-every/code fork)
- [ ] Realtime audio device picker (PR #12850)
- [ ] Realtime collaboration (PR #12687)
- [ ] Multi-chat display (from #13036)

---

## 6. Branch Cleanup Commands

```bash
# helios-cli: delete merged/superseded branches
git push upstream --delete codex/tui-renderer-spec-docs-with-core fix/policy-gate-pr-head-sha replay/release-framework wave2-lane-a-docs-unification codex/tui-renderer-spec-docs codex/tui-renderer-spec-docs-clean wave2-lane-a-docs-unification-heliosCLI chore/fresh-context-send-input

# portage: both branches are active, no cleanup needed
```

---

*Generated by 6 parallel research agents analyzing openai/codex PRs, issues, forks, and 3 KooshaPari repos.*
