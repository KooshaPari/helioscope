# Research Summary: Valuable Unmerged Features from OpenAI Codex

**Research Completed:** 2026-02-28
**Repository:** openai/codex
**Analysis Scope:** 200+ open PRs + 200+ recently closed PRs (last 60 days)

## Overview

This research identifies high-value, unmerged features from the upstream OpenAI Codex repository that could provide significant competitive advantages to the helios-cli fork.

**Key Finding:** On 2026-02-28, OpenAI changed their contribution policy to "internal-only," closing the repository to external contributors. This creates a unique window to implement community-contributed features before they're lost.

## Deliverables

This analysis package includes three documents:

### 1. **CODEX_PR_ANALYSIS.md** (Comprehensive Report)
- 10 HIGH VALUE features with detailed descriptions
- 10 MEDIUM VALUE features with brief notes
- LOW VALUE features to skip
- Strategic implementation roadmap (3 phases over 7+ weeks)
- Integration notes and file structure guidance
- Full context on policy change and external contributors

**Read this for:** Complete understanding of each feature, why it matters, and how difficult it is to implement.

### 2. **CODEX_PR_QUICK_REFERENCE.txt** (One-Page Overview)
- Prioritized list of top features
- Implementation roadmap
- Quick difficulty/impact assessment
- Key contributors and their work
- Config/documentation changes needed

**Read this for:** Quick decision-making on which features to tackle first.

### 3. **CODEX_PR_LINKS.md** (PR Directory & Code Locations)
- Direct GitHub links to all PRs
- Status for each PR (OPEN, CLOSED, merged)
- Code location hints (which files/directories to review)
- Cherry-pick workflow guidance
- External contributor profiles

**Read this for:** Finding the actual code to adapt and understanding contributor expertise.

## Executive Recommendations

### Top 3 Implementation Priorities

**TIER 1 (Immediate):**

1. **Markdown Table Rendering** (PR #12523)
   - Effort: ~2-3 weeks
   - Impact: Dramatically improves perceived output quality
   - Complexity: MEDIUM (well-structured code provided)
   - Author: Felipe Coury (@fcoury)

2. **Plugin System** (PR #12864)
   - Effort: ~1 week
   - Impact: Enables extensibility; users build skill packs
   - Complexity: LOW
   - Author: xl-openai (@xl-openai)

3. **Model Toggle Ctrl+O** (PR #12703)
   - Effort: ~1 week
   - Impact: Solves real workflow friction
   - Complexity: LOW
   - Author: Felipe Coury (@fcoury)

**Expected timeline for Tier 1: 4-5 weeks**

### Tier 2 (High-Value QoL)

- MCP Transport Recovery (#13100) — fixes stability
- Vim Mode (#11865) — power user appeal
- OSC-52 Copy (#12951) — tmux/SSH support
- Realtime Audio Devices (#12850, #12849) — multi-device support

**Expected timeline for Tier 2: 3-4 weeks**

### Tier 3 (Strategic/Advanced)

- Realtime Collaboration Mode (#12687) — voice workflows
- Permissions UI (#11549) — security transparency
- Live Skill Notifications (#12803) — UX polish

**Expected timeline for Tier 3: 3+ weeks**

## Key Facts

**What Changed on 2026-02-28:**
OpenAI announced they're moving to "OpenAI-employee-only" contributions. Several high-quality external PRs were closed per policy, including some with excellent features.

**Which Features Are Lost if We Don't Act:**
- Markdown tables (massive UX improvement)
- Plugin system (extensibility)
- Model toggle (real user demand)
- MCP transport recovery (stability)
- Vim mode (power user appeal)
- And 5+ others

**Why This Matters for helios-cli:**
These are all "soft features" that improve UX and polish without changing core functionality. Upstream won't merge them (policy), so implementing them creates differentiation.

## Integration Strategy

### No Major Dependencies
- All features integrate cleanly with existing helios-cli architecture
- No new external crates required in most cases
- Build system (Bazel) already well-structured for these additions

### Test Coverage
- Most upstream PRs include unit + integration tests
- Can adapt existing test patterns
- TUI changes need snapshot test updates

### Config Schema Changes
| Feature | Config Changes |
|---------|-----------------|
| Plugin System | Add `[plugins.*]` section |
| Model Toggle | Add `model_toggle_pair` |
| Audio Devices | Add `audio_input_device`, `audio_output_device` |
| Vim Mode | Add `vim_mode` boolean (runtime toggle) |

### Documentation Updates
- README: Add sections for `/vim`, `/copy`, `/permissions custom`
- Config schema: Document new settings
- Feature discovery: Consider UI hints for new toggles

## Risk Assessment

**Low Risk:** (Safe to implement)
- OSC-52 copy (standard feature, great fallback)
- Model toggle (simple state management)
- MCP transport recovery (improves existing error handling)
- Skill notifications (additive feature)

**Medium Risk:** (Well-tested upstream, but integration required)
- Markdown tables (complex TUI changes, but excellent code)
- Plugin system (new subsystem, but follows existing patterns)
- Vim mode (keybinding integration, moderate complexity)

**Higher Risk:** (Advanced, requires careful integration)
- Realtime collaboration (concurrent state management)
- Audio device picker (platform-specific code)

## Quality of Upstream Code

**Excellent Quality:**
- Felix Coury's work (markdown tables, model toggle): Production-ready, well-tested, excellent architecture
- Michael Bolin's work (exec policy): Comprehensive, defensive, excellent test coverage
- Skylar Graika's work (MCP recovery): Minimal, focused, well-validated e2e

**Good Quality:**
- Dylan Hurd's work (vim mode, permissions): Solid TUI integration, follows patterns
- Ahmed Ibrahim's work (realtime audio): Well-structured, though realtime mode is complex

## Next Steps

1. **Review this analysis package** (you're doing that now)
2. **Review the top-priority PRs** on GitHub
3. **Assess team capacity** for implementation
4. **Plan implementation phases** (see roadmap in CODEX_PR_ANALYSIS.md)
5. **Set up upstream remote** for cherry-picking
6. **Begin with Tier 1 features**

## Questions to Answer

Before beginning implementation:

1. **Voice features:** Is realtime/audio a strategic priority? (affects prioritization)
2. **Timeline:** Do we have 4-5 weeks for Tier 1 before release? (affects scope)
3. **Power users:** Are Vim users an important segment? (affects Tier 1 vs Tier 2)
4. **Extensibility:** Is plugin system important for user adoption? (affects Tier 1 priority)

## Files in This Analysis

- `CODEX_PR_ANALYSIS.md` — Detailed analysis of all features
- `CODEX_PR_QUICK_REFERENCE.txt` — One-page summary + roadmap
- `CODEX_PR_LINKS.md` — PR directory with links and code locations
- `RESEARCH_SUMMARY.md` — This file

All files are in `/Users/kooshapari/CodeProjects/Phenotype/repos/heliosCLI/`

---

**Analysis conducted:** 2026-02-28
**Tool:** Claude Code (via Anthropic's Claude Agent SDK)
**Data source:** GitHub CLI (`gh pr list`) for openai/codex
**Status:** COMPLETE — Ready for implementation planning

