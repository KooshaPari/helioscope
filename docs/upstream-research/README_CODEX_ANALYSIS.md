# OpenAI Codex Analysis - Start Here

**Analysis Date:** 2026-02-28
**Status:** Complete and ready for implementation planning

## Quick Start

If you only have 5 minutes, read this file.
If you have 15 minutes, read `RESEARCH_SUMMARY.md`.
If you want complete details, read `CODEX_PR_ANALYSIS.md`.

## What This Is

A comprehensive analysis of valuable, unmerged features from the upstream OpenAI Codex repository. The analysis identifies features that could provide significant competitive advantages to helios-cli if implemented.

## Key Finding

On 2026-02-28, OpenAI changed their contribution policy to "internal-only," stopping external contributor PRs. This creates a **unique opportunity window** to implement community-developed features that upstream will never merge.

## The Top 3 Features You Should Implement

### 1. Markdown Table Rendering (PR #12523)
**What:** Render Markdown tables with Unicode box-drawing borders in TUI output
**Why:** LLMs output tables constantly; current rendering wastes space and hurts readability
**Effort:** 2-3 weeks
**Impact:** Massive (users will immediately notice improvement)
**Author:** Felipe Coury (@fcoury) — excellent code quality

### 2. Plugin System (PR #12864)
**What:** First-class plugin architecture via config.toml
**Why:** Enables extensibility; users can create and share custom skill packs
**Effort:** 1 week
**Impact:** High (enables differentiation from upstream)
**Author:** xl-openai (@xl-openai)

### 3. Model Toggle Ctrl+O (PR #12703)
**What:** Hotkey to toggle between two most-recently-used models
**Why:** Users constantly switch models (fast vs. capable); this friction point solved
**Effort:** 1 week
**Impact:** High (solves real user pain point)
**Author:** Felipe Coury (@fcoury)

**Timeline for all 3: 4-5 weeks**

## Other Valuable Features

**Quick wins (1-2 weeks each):**
- OSC-52 copy (#12951) — Works in tmux/SSH
- MCP transport recovery (#13100) — Fixes browser MCP stability
- Vim mode (#11865) — Modal editing for power users
- Live skill notifications (#12803) — UX polish

**If voice is strategic:**
- Realtime audio device picker (#12850, #12849)
- Realtime collaboration mode (#12687)

**If security is critical:**
- Permissions UI (#11549)

## Document Guide

| Document | Size | Reading Time | Purpose |
|----------|------|--------------|---------|
| `README_CODEX_ANALYSIS.md` (this file) | 3 KB | 5 min | Quick orientation |
| `RESEARCH_SUMMARY.md` | 7 KB | 15 min | Executive summary + next steps |
| `CODEX_PR_QUICK_REFERENCE.txt` | 4 KB | 10 min | Prioritized feature list + roadmap |
| `CODEX_PR_ANALYSIS.md` | 15 KB | 30-40 min | Comprehensive analysis of all features |
| `CODEX_PR_LINKS.md` | 8 KB | 15 min | PR directory with GitHub links |

## What You Need to Do

### Step 1: Get Oriented (Today)
- Read this file (5 min)
- Read `RESEARCH_SUMMARY.md` (15 min)

### Step 2: Review Top Features (This Week)
- Open PR #12523 (Markdown tables) on GitHub
- Open PR #12864 (Plugin system) on GitHub
- Open PR #12703 (Model toggle) on GitHub
- Read the PR descriptions and code changes

### Step 3: Make Priority Decisions (Next Meeting)
Answer these questions:
1. Do we want to implement these 3 features? (Recommended: YES)
2. What's our timeline? (4-5 weeks recommended for Tier 1)
3. Are voice features strategic? (Affects PR #12687, #12850 priority)
4. Is Vim important for target users? (Affects PR #11865 priority)

### Step 4: Plan Implementation (Next Sprint)
Use the roadmap in `CODEX_PR_QUICK_REFERENCE.txt` to sequence work.
Start with Markdown tables (highest visual impact).

### Step 5: Begin Implementation
Use `CODEX_PR_LINKS.md` to find:
- Direct GitHub links to each PR
- Which files/directories to review
- Cherry-pick workflow guidance

## Key Stats

- **Total PRs analyzed:** 400+ (200 open + 200 recently closed)
- **High-value features identified:** 10
- **Medium-value features identified:** 10
- **Total implementation effort:** 4-5 weeks for Tier 1 (top 3 + 4-5 quick wins)
- **Code quality:** High (reviewed upstream author experience)
- **Risk level:** Low to Medium (well-tested, clear integration patterns)

## Why This Matters

These features represent:
- **UX Improvements:** Markdown tables, model toggle, Vim mode
- **Stability Fixes:** MCP transport recovery
- **Extensibility:** Plugin system
- **Power User Appeal:** Vim, OSC-52, permissions UI
- **Feature Parity:** Realtime audio, collaboration features

Implementing them would create **clear differentiation** from upstream while upstream cannot merge them (internal-only policy).

## The Big Picture

OpenAI Codex is excellent but under internal-only contribution policy. helios-cli can now:
1. Inherit the best community-contributed features
2. Move faster than upstream on UX/polish
3. Build features upstream won't
4. Appeal to users who want extensibility (plugins) and power-user features (Vim)

This is a **rare strategic opportunity** to gain real differentiation at moderate cost.

## Questions?

Refer to the detailed documents:
- **"How does this work?"** → CODEX_PR_ANALYSIS.md
- **"What's the roadmap?"** → CODEX_PR_QUICK_REFERENCE.txt
- **"Where's the code?"** → CODEX_PR_LINKS.md
- **"What should I do next?"** → RESEARCH_SUMMARY.md

---

**Analysis conducted by:** Claude Code (Anthropic Claude Agent SDK)
**Data source:** GitHub CLI (`gh pr list openai/codex`)
**Status:** COMPLETE — Ready for implementation planning

Start with `RESEARCH_SUMMARY.md` for the next level of detail.
