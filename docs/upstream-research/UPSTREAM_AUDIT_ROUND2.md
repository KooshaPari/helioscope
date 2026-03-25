# Upstream Audit: coder/agentapi, router-for-me/CLIProxyAPI(Plus), blackboardsh/colab
**Date:** 2026-02-28

---

## 1. coder/agentapi (upstream of agentapi-plusplus)

**Stats:** 1,231 stars | 108 forks | Go | 195 merged PRs (97% merge rate) | 26 open issues

### Open PRs to Cherry-Pick
| PR | Title | Lines | Age | Priority |
|----|-------|-------|-----|----------|
| **#192** | e2e asciinema test infrastructure | +604 | 8d | HIGH |
| **#181** | Fix chat rendering jank | +80/-147 | 19d | HIGH |
| **#184** | Handle partial/malformed tool calls | +48 | 16d | HIGH |
| **#177** | State persistence | +2,435 | 28d | HIGH |
| **#145** | Agent animation handling | +56 | older | MEDIUM |
| **#110** | Fix PR preview delete | +59 | older | LOW |
| **#86** | Abstract CLI Agent Handler | +519 | older | LOW |

### Rejected PRs with Salvageable Features
| PR | Title | Lines | Why Closed | Value |
|----|-------|-------|------------|-------|
| **#194** | Benchmarks module + tokenledger | 1M+ | Deprioritized | HIGH — adapt for agentapi++ |
| **#186** | Experimental ACP mode (--experimental-acp) | moderate | Experimental | HIGH — ACP framework pieces landed in #188-190 |

### All Open Issues (26)
| Issue | Title | Comments | Impact |
|-------|-------|----------|--------|
| **#191** | e2e: use asciinema recordings for testing | 0 | Enhancement |
| **#187** | Pagination for messages endpoint | 0 | **Already in our fork** |
| **#180** | Voice input (Web Speech API) | 0 | **Already in our fork** |
| **#178** | afero.Walk: open 404\index.html: file does not exist | 0 | **Already fixed in our fork** |
| **#174** | screenDiff returns wrong content for AgentTypeOpencode | 0 | **Already fixed in our fork** |
| **#171** | Support for AskUserQuestion input | 0 | **Already fixed in our fork** |
| **#156** | curl: (23) Failure writing output | 1 | Deployment issue |
| **#154** | How to set up API key for each CLI? | 2 | Documentation |
| **#146** | Better support of OpenCode | 0 | Strategic |
| **#138** | Create Python client library + CLI | 2 | External demand |
| **#126** | First line of response being trimmed | 3 | **Already fixed in our fork** |
| **#124** | Selecting/copying text is extremely hard | 0 | UX |
| **#123** | Screen stabilize timeout | 2 | CRITICAL for automated use |
| **#117** | Support for Slash Commands | 2 | **Already in our fork** |
| **#116** | Preview Builds on Tag Placement | 1 | CI/Release |
| **#114** | Add tests for initial prompt feature | 0 | Testing |
| **#60** | Improve agent detection by sniffing splash screen | 0 | Enhancement |
| **#56** | afero.Walk Windows support | 2 | **Already fixed in our fork** |
| **#53** | Claude Code tools cause scrolling UX issues | 1 | UX |
| **#52** | Sign macOS binaries | 1 | Release quality |
| **#48** | Refactor to use claude-code SDK | 5 | Architecture decision |
| **#27** | Support control mode in mobile view | 3 | Mobile UX |
| **#21** | Queue messages | 1 | Feature request |
| **#13** | go install doesn't include chat interface | 0 | Distribution |
| **#2** | Support Agent2Agent Protocol | 0 | Protocol |
| **#1** | Support MCP protocol | 1 | Protocol |

### All Branches (61)
**Core team members by branch prefix:**
- `cj/*` (28 branches) — Primary maintainer, release automation, refactors
- `hugodutka/*` (15 branches) — Chat UI, scroll, lint, storybook, release builds
- `35C4n0r/*` (5 branches) — State persistence, formatting, tool calls
- `bq/*` (3 branches) — CI, design, defaults

**Key active branches:**
- `cj/e2e-asciicast` — e2e testing (matches PR #192)
- `cj/exp/acp`, `cj/exp/acp-c`, `cj/exp-acp` — ACP protocol experiments
- `35C4n0r/agentapi-state-persistence` — matches PR #177
- `35C4n0r/handle-partial-tool-call` — matches PR #184
- `gh-pages` — documentation site
- `embedded-chat`, `feat-embed` — embedded chat experiments

### Forks (108 total, top 30 by stars)
| Fork | Stars | Last Push | Notes |
|------|-------|-----------|-------|
| ssushant0011/agentapi | 1 | 2026-01-28 | |
| milhy545/agentapi | 1 | 2025-12-03 | |
| blackbutterfly313/agentapi | 1 | 2025-10-21 | |
| Zeeeepa/agentapi | 1 | 2026-02-13 | Multiple branches |
| SystemExecuteLLC/agentapi | 1 | 2025-08-11 | |
| BunsDev/agentapi | 1 | 2025-09-11 | |
| MahoneyContextProtocol/agentapi | 1 | 2025-10-17 | |
| allenlorusso/agentapi | 1 | 2025-08-04 | |
| **KooshaPari/agentapi-plusplus** | 0 | **2026-02-28** | **Most active — 74 commits ahead** |
| biodoia/happy-openclaw | 0 | 2026-02-25 | Recent activity |
| Kaairofelipe/agentapi | 0 | 2026-02-26 | Recent activity |
| heretolya/agentapi | 0 | 2026-02-13 | |
| kartikmandar/agentapi | 0 | 2026-01-27 | |
| mberg/agentapi | 0 | 2026-01-14 | |
| *(14 more with 0 stars, older pushes)* | | | |

**No forks have significant divergent features.** Our fork is the ecosystem leader.

### Our Fork Advantage — 7 Fixes to Contribute Back
| Our PR | Upstream Issue | Fix |
|--------|---------------|-----|
| #270 | #178 | afero.Walk on embedded FS |
| #272 | #126 | First line of response trimmed |
| #254 | #174 | OpenCode screenDiff wrong content |
| #248 | #171 | AskUserQuestion input support |
| #278 | #117 | Slash command support |
| #279 | #180 | Voice input (Web Speech API) |
| #277 | #187 | Pagination for messages endpoint |

---

## 2. router-for-me/CLIProxyAPI + CLIProxyAPIPlus (upstream of cliproxyapi-plusplus)

### Source: CLIProxyAPI
**Stats:** 12,252 stars | 1,956 forks | 131 open issues | 18+ active PRs

### Parent: CLIProxyAPIPlus
**Stats:** 1,021 stars | 218 forks | 63 open issues | 1 stalled PR

### Fork Sync
- **786 commits ahead** of CLIProxyAPIPlus, 35 behind
- **1,326 commits ahead** of CLIProxyAPI source, only 22 behind
- Parent (CLIProxyAPIPlus) is in **maintenance mode** — sync from source directly

### CLIProxyAPI Source Branches
| Branch | Purpose |
|--------|---------|
| `main` | Primary release branch |
| `dev` | Development/integration (active) |
| `feature/ampcode-alias` | AMP model mapping (in progress) |
| `amp` | AMP-specific integration |
| `legacy` | Deprecated code preservation |

### CLIProxyAPIPlus Branches
| Branch | Purpose |
|--------|---------|
| `main` | Primary |
| `pr-59-resolve-conflicts` | Stalled conflict resolution |

### Priority 1: Must Cherry-Pick (Bug Fixes from Source)
| PR | Title | Impact |
|----|-------|--------|
| **#1764** | fix(kimi): drop orphan empty assistant messages | Kimi auth failures |
| **#1760** | fix(auth): unified refresh retry logic | Prevents cascade failures |
| **#1745** | fix(openai): sanitize tool/function names | Schema validation errors |

### Priority 2: Should Cherry-Pick (Features from Source)
| PR | Title | Comments | Impact |
|----|-------|----------|--------|
| **#1738** | feat(translator): image content in tool results | 3 | OpenAI↔Claude translation |
| **#1673** | feat: sticky-round-robin routing | 6 | Session-aware load balancing |
| **#1684** | fix: input_audio preservation for Antigravity | 6 | Audio completeness |
| **#1672** | fix: prevent JSON corruption from control chars | 6 | Output integrity |

### Priority 3: Consider
| PR | Title |
|----|-------|
| **#1744** | feat(Antigravity): User-Agent configuration |
| **#1735** | feat(antigravity): v1.19.5 + proxy emulation |
| **#1733** | feat: Wails desktop app + auto-update |

### CLIProxyAPI Open Issues (Top 50 — 131 total)
| Issue | Title | Comments | Severity |
|-------|-------|----------|----------|
| **#1765** | antigravity: Claude-compat payload leaks output_config | 0 | HIGH |
| **#1763** | Gzip-compressed error responses not decompressed — false 400 | 0 | CRITICAL |
| **#1759** | claude-sonnet-4-6 lacks thinking in Antigravity | 0 | HIGH |
| **#1757** | Codex OAuth登录报错 | 0 | HIGH |
| **#1756** | GeminiCLI auth not rotating — RATE_LIMIT_EXCEEDED | 0 | HIGH |
| **#1741** | Claude Code can't execute local tools via OpenAI-compat | 1 | CRITICAL |
| **#1730** | Kimi chat/completions fails with OAuth | 1 | HIGH |
| **#1729** | Multiple accounts stuck on 403 VALIDATION_REQUIRED | 1 | HIGH |
| **#1725** | Codex 400 "Unknown parameter: tools.defer_loading" | 3 | HIGH |
| **#1721** | Model loading OK but chat fails | 2 | MEDIUM |
| **#1715** | Stuck in restart loop | 1 | HIGH |
| **#1706** | auth unavailable on /v1/messages with Anthropic format | 1 | MEDIUM |
| **#1696** | Auto-disable credential when quota exceeded | 0 | FEATURE |
| **#1691** | Management API: bcrypt secret-key validation broken | 1 | HIGH |
| **#1671** | Cannot use Claude Models in Codex CLI | 2 | HIGH |
| **#1670** | Image content in tool results (OpenAI↔Claude) | 0 | FEATURE |
| **#1667** | Codex translator: Responses compaction fields | 0 | MEDIUM |
| **#1665** | Gemini Web support removal concerns | 0 | MEDIUM |
| **#1658** | Qwen OAuth fails | 2 | MEDIUM |
| **#1655** | All credentials cooling down for claude-sonnet-4-6 | 3 | HIGH |
| **#1651** | Claude Sonnet 4.5 deprecated — remove from panel | 0 | LOW |
| **#1649** | Gemini API: incorrect renaming parameters→parametersJsonSchema | 0 | MEDIUM |
| **#1641** | Docker Image Error | 0 | MEDIUM |
| **#1637** | Google blocked 3 email IDs at once | **27** | CRITICAL |
| **#1630** | Claude Sonnet 4.5→4.6 migration | **8** | HIGH |
| **#1620** | Claude Code streaming not working (全部一次输出) | 0 | MEDIUM |
| **#1617** | Session-Aware Hybrid Routing Strategy | 0 | FEATURE |
| **#1615** | JetBrains IDE support? | 1 | FEATURE |
| **#1609** | response.function_call_arguments.done in codex→claude | 0 | MEDIUM |
| **#1584** | Thinking block signature error switching Gemini→Claude | 1 | HIGH |
| **#1564** | 能否再难用一点?! (Usability complaint) | **27** | UX |
| **#1547** | ENABLE_TOOL_SEARCH - MCP not in available tools 400 | 0 | MEDIUM |
| **#1540** | Support Claude output_config.effort parameter (Opus 4.6) | 1 | FEATURE |
| **#1538** | Gemini-3-pro-high corrupted thought signature | 3 | HIGH |
| **#1535** | 400 "Invalid Argument" with claude-opus-4-6-thinking | 1 | HIGH |
| **#1533** | Persistent 400 with claude-opus-4-6-thinking | **9** | CRITICAL |
| **#1531** | Unknown name "deprecated" in JSON payload | 3 | MEDIUM |
| **#1530** | proxy_ prefix on tool_choice.name but not tools[].name | 0 | HIGH |
| **#1525** | Support openai image generations API | 0 | FEATURE |
| **#1521** | 503/429 despite available credit | 2 | HIGH |
| **#1514** | Token refresh 500 "server busy" from iflow | 1 | HIGH |
| **#1513** | Nullable type arrays cause 400 on Antigravity | 0 | HIGH |
| **#1509** | Force thinking on claude-opus-4-6-thinking | 1 | FEATURE |
| **#1508** | Per-OAuth-Account outbound proxy for Google/OpenAI | 0 | FEATURE |

### CLIProxyAPIPlus Open Issues (50 — 63 total)
| Issue | Title | Comments | Severity |
|-------|-------|----------|----------|
| **#398** | Codex models return Responses API SSE format on chat/completions | 0 | HIGH |
| **#397** | OAuth model alias问题 | 2 | MEDIUM |
| **#396** | v6.8.32-0 后Kiro秒封 | 1 | CRITICAL |
| **#391** | **prompt_tokens reports non-cached only (BREAKS OpenAI standard)** | 0 | **CRITICAL** |
| **#296** | Kiro login overlap | 0 | MEDIUM |
| **#294** | Add Warp auth as Kiro | 0 | FEATURE |
| **#292** | [New Provider] OpenCode Go Support | 0 | FEATURE |
| **#284** | Codex models not on ChatGPT cookies (gpt-5.3-codex-spark) | 0 | MEDIUM |
| **#282** | Antigravity quota not correctly fetched | 1 | MEDIUM |
| **#258** | Support `variant` as fallback for `reasoning_effort` | **7** | HIGH |
| **#253** | Codex support | 4 | HIGH |
| **#251** | Bug thinking | 5 | HIGH |
| **#232** | Add AMP auth as Kiro | 5 | FEATURE |
| **#221** | Kiro账号被封 | 1 | MEDIUM |
| **#219** | Opus 4.6 | 1 | MEDIUM |
| **#213** | Add kilocode CLI support | 1 | FEATURE |
| **#210** | Kiro vs Ampcode Bash tool incompatibility | 2 | HIGH |
| **#206** | Nullable type arrays cause 400 on Antigravity | 0 | HIGH |
| **#201** | read-only file system config save | 0 | MEDIUM |
| **#198** | Cursor CLI Auth Support | 2 | FEATURE |
| **#183** | Why no Kiro in dashboard | 1 | UX |
| **#178** | Claude thought_signature forwarded to Gemini → Base64 error | 0 | HIGH |
| **#177** | Kiro Token import failure: Refresh token required | 0 | MEDIUM |
| **#169** | Kimi Code support | 1 | FEATURE |
| **#163** | Kiro: empty content → Bad Request | 0 | HIGH |
| **#158** | Custom upstream URL for all OAuth channels | 0 | FEATURE |
| **#160** | Kiro反代重复输出 | 0 | MEDIUM |
| **#149** | Kiro IDC refresh token failure | 0 | MEDIUM |
| **#147** | Docker ARM architecture support | 1 | FEATURE |
| **#146** | Kiro quota display feature | 0 | FEATURE |
| **#145** | OpenAI compat mode: improve Claude protocol translation | 0 | HIGH |
| **#136** | Kiro IDC login needs manual refresh | 2 | MEDIUM |
| **#133** | fill-first routing not working | 0 | HIGH |
| **#129** | ClawCloud deployment support | 2 | FEATURE |
| **#125** | Error 403 | 0 | MEDIUM |
| **#115** | Kiro AWS login → 封号 | 1 | MEDIUM |
| **#111** | Antigravity auth failed | 0 | MEDIUM |
| **#102** | Incognito parameter无效 | 0 | MEDIUM |
| **#101** | OpenAI-compat hardcodes /v1/models (breaks Z.ai v4) | 0 | HIGH |
| **#97** | ADD TRAE IDE support | **13** | FEATURE |
| **#99** | GitHub Copilot Model Call Failure | 2 | HIGH |
| **#94** | Veo Video Generation Support | 0 | FEATURE |

### CLIProxyAPI Forks (1,956 total, top 30)
| Fork | Stars | Last Push | Notes |
|------|-------|-----------|-------|
| **router-for-me/CLIProxyAPIPlus** | 1,021 | 2026-02-28 | Our parent |
| **ben-vargas/ai-cli-proxy-api** | 67 | 2026-02-28 | **7 branches, active — has amp-cli snapshots + custom fixes** |
| **HALDRO/CLIProxyAPI-Extended** | 22 | 2026-02-13 | **32MB — enhanced KiroExecutorV2 with fingerprint mgr, retry logic, web_search routing** |
| cursoredu2/CLIProxyAPI | 7 | 2025-12-23 | |
| ngochip/CLIProxyAPI | 5 | 2026-02-27 | 2 branches |
| DSLZL/CLIProxyAPI | 5 | 2026-02-28 | 2 branches, active |
| liupeiyu114514-cmd/CLIProxyAPI | 4 | 2026-02-28 | Active |
| AoaoMH/CLIProxyAPI-Aoao | 4 | 2026-02-28 | Active |
| jeffnash/CLIProxyAPI | 4 | 2026-02-25 | |
| can1357/CLIProxyAPI | 3 | 2026-01-03 | |
| caidaoli/CLIProxyAPI | 3 | 2026-02-28 | Active |
| Zhong-Ze-Wei/CLIProxyAPI-zz | 3 | 2026-01-23 | |
| em4go/CLIProxyAPI | 3 | 2025-12-07 | |
| *(17 more with 1-2 stars)* | | | |

### CLIProxyAPIPlus Forks (218 total, top 30)
| Fork | Stars | Last Push | Notes |
|------|-------|-----------|-------|
| **KooshaPari/cliproxyapi-plusplus** | 4 | 2026-02-28 | **Us — 786 ahead, most active** |
| lemon07r/CLIProxyAPIPlus | 3 | 2026-02-28 | Active mirror |
| jc01rho/CLIProxyAPIPlus | 1 | 2026-02-28 | Active |
| thebtf/CLIProxyAPIPlus | 1 | 2026-02-28 | Active |
| tetrabit/CLIProxyAPIPlus | 0 | 2026-02-28 | Active |
| fivesheeplive/CLIProxyAPIPlus | 0 | 2026-02-27 | Active |
| AdamBear/CLIProxyAPIPlus | 0 | 2026-02-27 | |
| *(23 more, mostly mirrors)* | | | |

### Notable Competing Forks
**ben-vargas/ai-cli-proxy-api** (67 stars, 7 branches):
- Has `amp-cli-snapshot-*` branches (5 snapshots from Nov 2025)
- Recent fix: thinking chain display for Amp client, configurable max-retry-credentials
- Active development on `plus-main` branch

**HALDRO/CLIProxyAPI-Extended** (22 stars, 32MB):
- Enhanced KiroExecutorV2 with dynamic User-Agent fingerprinting
- Pooled HTTP client for connection reuse
- Retry logic for 500/502/503/504
- web_search→remote_web_search tool renaming
- **Worth monitoring for Kiro-specific enhancements**

---

## 3. blackboardsh/colab (upstream of colab)

**Stats:** 89 stars | 10 forks | TypeScript | 4 open issues | Single-branch repo (`main` only)

### Fork Sync
- **4 commits ahead**, 0 behind (clean divergence)

### Open PRs
| PR | Title | Lines | Author | Priority |
|----|-------|-------|--------|----------|
| **#2** | Theme switching + CSS variables | +1,491/-352 | @wzzc-dev | HIGH — stalled 1 month |

No other PRs exist (open or closed).

### All Open Issues (4)
| Issue | Title | Comments | Priority |
|-------|-------|----------|----------|
| **#5** | GIT GUI (submodule state bug) | 0 | MEDIUM |
| **#4** | Git GUI — add shallow cloning | 0 | MEDIUM (quick win) |
| **#3** | macOS 15.5 Sequoia M1 Max — launches but no window (GPU/CEF crash) | 0 | **CRITICAL** |
| **#1** | Make file formatter an extension | 0 | LOW (architecture blocker) |

### Branches
Only `main` — single-branch development model.

### All Forks (10)
| Fork | Stars | Last Push | Notes |
|------|-------|-----------|-------|
| dreemrworld/blackboardsh-colab | 1 | 2026-02-13 | |
| Acumen-Desktop/colab | 0 | 2026-01-21 | |
| wzzc-dev/colab | 0 | 2026-01-29 | **PR #2 author — has theme work** |
| oc-agent-clode/colab | 0 | 2026-01-30 | |
| ghkweon/colab | 0 | 2026-02-16 | |
| siathalysedI/colab | 0 | 2026-02-16 | |
| errahmounispace/colab | 0 | 2026-02-16 | |
| Shahfarzane/colab | 0 | 2026-02-22 | |
| wtyler2505/colab | 0 | 2026-02-24 | |
| **KooshaPari/colab** | 0 | **2026-02-28** | **Us — most recently active** |

No forks have divergent features. wzzc-dev/colab has v0.13.0-canary theme builds.

### Upstream Development
- Active release cadence: v0.14.6 → v0.14.10 in ~2 weeks
- Focus: Electrobun native wrapper, performance ("make it much snappier"), CEF renderer
- Single maintainer (@YoavCodes)

### Recommendations
1. Cherry-pick PR #2 (theme switching) — high-value, low-risk
2. Investigate macOS M1 crash (issue #3) — GPU/CEF renderer issue
3. Implement shallow clone (issue #4) — quick win
4. Monitor upstream weekly — active release cycle

---

## Cross-Upstream Summary

| Upstream | Stars | Forks | Our Ahead | Open Issues | Open PRs | Cherry-Picks | Fork Competition |
|----------|-------|-------|-----------|-------------|----------|-------------|-----------------|
| coder/agentapi | 1,231 | 108 | ~74 | 26 | 7 | 4 high-priority | None — we're #1 |
| CLIProxyAPI (source) | 12,252 | 1,956 | 1,326 | 131 | 18+ | 7 high-priority | ben-vargas (67★), HALDRO (22★) |
| CLIProxyAPIPlus (parent) | 1,021 | 218 | 786 | 63 | 1 (stalled) | 0 | None — parent in maintenance |
| blackboardsh/colab | 89 | 10 | 4 | 4 | 1 | 1 (theme) | None |

### Strategic Takeaways

1. **We are the most active fork** across all three ecosystems
2. **CLIProxyAPI source** has the most value — 131 issues, 18+ PRs, 12K stars. Sync from source, not Plus parent.
3. **Critical bug:** prompt_tokens billing (#391) — verify if we inherited this
4. **coder/agentapi** — contribute our 7 fixes back upstream for goodwill + visibility
5. **blackboardsh/colab** — small project, easy to stay ahead; grab theme PR
6. **Notable competing forks:** ben-vargas/ai-cli-proxy-api (amp snapshots), HALDRO/CLIProxyAPI-Extended (Kiro enhancements with fingerprinting + retry logic)

*Generated by 3 parallel upstream audit agents + manual data collection.*
