# Downstream Consumers & Integrations Research Package

## Overview

This research package documents applications, integrations, and platforms that consume or build on top of CLI AI agents like helios-cli/codex as of February 2026.

## Files Included

### 1. **DOWNSTREAM_CONSUMERS.md** (32 KB, 836 lines)
The comprehensive research document covering:
- 6 major consumer categories (IDE integrations, CI/CD, multi-agent orchestrators, developer platforms, code review bots, voice/accessibility)
- 30+ specific projects analyzed with adoption levels
- Integration patterns for each category
- Feature requirements and gaps
- Cross-cutting concerns (auth, sandboxing, observability, configuration)
- Prioritized feature roadmap with effort estimates
- References and resources

**Read this for:** Complete understanding of the ecosystem, detailed feature requirements, and implementation planning.

### 2. **DOWNSTREAM_CONSUMERS_SUMMARY.txt** (13 KB, 307 lines)
A condensed text-format summary showing:
- Ecosystem breakdown by category
- Current status of helios-cli integration (✓ working, ? unclear, ✗ missing)
- Key technical requirements
- Top 5 features for downstream adoption (Tier 1, 2, 3)
- Competitive landscape
- Integration checklist for downstream developers
- Quick reference links

**Read this for:** Quick orientation, decision-making, and sharing with teams.

## Key Findings

### Most Important Integration Targets

**Tier 1 (Immediate - 4-5 weeks)**
1. **MCP Server Mode** — Unlocks IDE plugins, multi-agent frameworks, browser integration (1M+ developers)
2. **OpenAPI Schema Export** — Enables LangGraph, AutoGen, CI/CD integration (10+ platforms)
3. **Structured Error Output** — Required for reliable automation across all systems
4. **Daemon Mode** — 10-100x performance improvement for IDE, voice, real-time workflows
5. **Config Templates** — One-click setup for specific use cases

### Largest Consumer Categories

1. **IDE Integrations** (20+ projects)
   - VS Code, Cursor, Windsurf, Neovim, JetBrains
   - Status: Partially supported; need MCP server mode for broad adoption
   - Addressable market: 1M+ developers

2. **CI/CD Agents** (5+ platforms)
   - GitHub Agentic Workflows (Feb 2026 preview), GitLab Duo, Jenkins, CircleCI
   - Status: Works via subprocess; needs OpenAPI export for framework integration
   - Addressable market: 500K+ teams

3. **Multi-Agent Orchestrators** (3 major frameworks)
   - LangGraph, CrewAI, AutoGen
   - Status: Possible via subprocess; needs structured tool export
   - Addressable market: 10K+ developers

4. **Developer Platforms** (4 major platforms)
   - Replit (20M users), GitHub Codespaces, Coder, Ona/Gitpod
   - Status: Mixed; works best with daemon mode
   - Addressable market: 50M+ users

5. **Code Review Bots** (2 market leaders)
   - Qodo (Gartner Visionary), CodeRabbit
   - Status: Don't wrap CLI agents directly; could be integration target
   - Addressable market: 5K+ organizations

6. **Voice & Accessibility** (Emerging)
   - Voice I/O, screen readers, text-to-speech
   - Status: Infrastructure ready; needs accessibility mode
   - Addressable market: Growing (2-3% of developer population today)

## Research Methodology

- **Data sources:** Web search, GitHub analysis, official documentation
- **Scope:** Global AI coding agent ecosystem as of Feb 2026
- **Coverage:** 30+ specific projects across 6 categories
- **Validation:** Cross-referenced with Gartner Magic Quadrants, GitHub star counts, adoption metrics

## Next Steps

1. **Priority Selection** — Choose which features to implement based on business impact
2. **Effort Estimation** — Use provided estimates (2-3 weeks for MCP, 1-2 weeks for OpenAPI, etc.)
3. **Engagement** — Contact IDE plugin developers, CI/CD platform teams for feedback
4. **Implementation** — Start with Tier 1 features for fastest adoption unlocking
5. **Contribution** — Contribute open-source integrations (VS Code extension, JetBrains plugin, etc.)

## Document Navigation

| Question | Answer Location |
|----------|-----------------|
| What's the big picture? | Read DOWNSTREAM_CONSUMERS_SUMMARY.txt (10 min) |
| Which projects should we target? | See "Consumer Categories" section in DOWNSTREAM_CONSUMERS.md |
| What features do we need? | See "Feature Requests & Spec Gaps" section |
| How do I integrate helios-cli? | See "Integration Checklist" in both documents |
| Which integration is lowest-effort? | GitHub Actions + OpenAPI schema export (1-2 weeks) |
| Which integration has highest adoption? | MCP server mode for IDEs (unlocks 1M+ developers) |

## Contact & Attribution

- **Research Date:** February 28, 2026
- **Researcher:** Claude Code (Anthropic Claude Agent SDK)
- **Data Sources:** Web search, local codebase analysis
- **Status:** Complete and ready for implementation planning

## Files

- `/tmp/DOWNSTREAM_CONSUMERS.md` — Main research document (836 lines)
- `/tmp/DOWNSTREAM_CONSUMERS_SUMMARY.txt` — Quick reference summary (307 lines)
- `/tmp/README_DOWNSTREAM.md` — This file

---

**Start with the summary, then dive into the full document for implementation planning.**
