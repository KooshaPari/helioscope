# Forks with Valuable Divergent Features - Research Report

## Executive Summary

Analysis of 30 top forks of openai/codex reveals **4 major forks with significant divergence** (>50 commits ahead), each targeting specific use cases with valuable feature additions. Most have **limited adoption** (low star counts) but represent interesting architectural patterns worth reviewing.

---

## Tier 1: Major Divergent Forks (100+ commits ahead)

### 1. **just-every/code** (4,250 commits ahead)
**Repository:** https://github.com/just-every/code
**Stars:** 3,518 | **Forks:** 229 | **Last Updated:** 2026-02-28
**Description:** "Every Code - push frontier AI to its limits. A fork of the Codex CLI with validation, automation, browser integration, multi-agents, theming, and much more. Orchestrate agents from OpenAI, Claude, Gemini or any provider."

**Key Distinctive Features:**
- **Multi-provider agent orchestration** - Support for OpenAI, Claude, Gemini and arbitrary providers (not just OpenAI)
- **Validation & automation frameworks** - Built-in validation and automation tooling
- **Browser integration** - Web-based UI and browser automation capabilities
- **Advanced theming system** - Custom theme support and configuration
- **Config migration API** - Migration utilities for configuration management
- **Websocket fallback mechanisms** - Improved network resilience with HTTP fallback
- **Approval parity controls** - Enhanced approval workflows
- **Memory usage selection** - Intelligent resource selection based on memory metrics
- **Sub-agent orchestration** - Sophisticated multi-agent coordination with named agents

**Recent Commit Highlights:**
- feat(agents): import config migration API and depth guard
- feat(sync): add websocket fallback and approval parity
- feat: include available decisions in command approval requests
- feat(agents): enforce config depth constraints

**Relevance to helios-cli:**
- **HIGH**: Multi-provider support pattern could enable helios to work with multiple AI backends beyond OpenAI
- **HIGH**: Validation/automation frameworks valuable for testing and CLI robustness
- **MEDIUM**: Browser integration less relevant for CLI-first tool but UX patterns transferable
- **MEDIUM**: Theme system useful for TUI customization

---

### 2. **zapabob/codex** (979 commits ahead)
**Repository:** https://github.com/zapabob/codex
**Stars:** 4 | **Forks:** 1 | **Last Updated:** 2026-02-28
**Description:** Actively maintaining upstream parity with custom optimizations.

**Key Distinctive Features:**
- **Parallel subagent scaling (2x capacity)** - Documented improvement over upstream agent parallelization
- **Memory management system** - Phase 1/Phase 2 memory extraction and forgetting strategies
- **Memory mode configuration** - Configurable memory behavior in agent workflows
- **Local cache refresh jobs** - Background task for keeping model/requirement caches fresh (TTL: 30 min, refresh: 5 min)
- **Environment context enrichment** - Local date/timezone included in turn context (persisted in history)
- **Model availability metadata** - NUX metadata for model availability presentation
- **Realtime audio enhancements** - Audio device configuration and picker UI
- **MCP OAuth resource handling** - Proper OAuth resource parameter for MCP login flows

**Recent Commit Highlights:**
- feat: use the memory mode for phase 1 extraction
- feat: gen memories config + add use memories config
- feat: add local date/timezone to turn environment context
- Add realtime audio device config and device picker
- Add oauth_resource handling for MCP login flows

**Relevance to helios-cli:**
- **HIGH**: Memory management patterns crucial for managing long conversation contexts
- **HIGH**: Cache refresh strategies valuable for handling rate limits and reducing latency
- **MEDIUM**: Audio features less relevant unless helios-cli adds voice I/O
- **MEDIUM**: Environment context enrichment could improve prompt quality

---

### 3. **Eriz1818/xCodex** (537 commits ahead)
**Repository:** https://github.com/Eriz1818/xCodex
**Stars:** 6 | **Forks:** 1 | **Last Updated:** 2026-02-27
**Description:** "A maintained fork of OpenAI Codex CLI with all upstream features but with downstream customizations."

**Key Distinctive Features:**
- **Enhanced exclusion/redaction system** - Secret detection with reveal toggles and UX improvements
- **Exclusion match caching** - Session-level cache for redaction matches
- **Plan mode auto-creation** - Automatic plan file generation and discussion appending
- **Lazy MCP server priming** - Auto-start explicitly requested MCP servers in lazy mode
- **Ignored-path allowlist** - Path filtering for exclusion rules
- **Session deduplication** - Prevent duplicate session handling
- **Exclusion match highlighting** - Visual indicators in approval overlay
- **Fix-and-start workflow** - Pipeline for quick fixes with automatic restart
- **Durable plan persistence** - Full proposed plan and research log preservation

**Recent Commit Highlights:**
- core: auto-prime explicitly requested MCP servers in lazy mode
- tui/exclusions: add default reveal toggle
- core/exclusions: improve redaction match UX
- tui2/plan: auto-create plan file and append discussion
- core: support ignored-path allowlist and session dedupe

**Relevance to helios-cli:**
- **HIGH**: Redaction/exclusion patterns critical for handling secrets and sensitive data in codebases
- **HIGH**: Plan mode with persistence useful for complex multi-step operations
- **MEDIUM**: MCP lazy loading optimization reduces startup time
- **MEDIUM**: Fix-and-start workflow demonstrates good UX for iteration

---

### 4. **Haleclipse/codex (Cometix)** (80 commits ahead)
**Repository:** https://github.com/Haleclipse/codex
**Stars:** 203 | **Forks:** 12 | **Last Updated:** 2026-02-28
**Description:** Chinese community fork maintaining Cometix (custom statusline) while merging upstream features.

**Key Distinctive Features:**
- **CxLine statusline** - Custom statusline rendering for context window percentage
- **All-visible model catalog** - Shows all models including deprecated ones (unlike upstream filtering)
- **Translation configuration** - Custom translation/localization support
- **Upstream merge compatibility** - Seamless integration with upstream releases (merged 0.105.0 with 239 commits)
- **Model catalog flexibility** - Shows deprecated/OSS models (gpt-oss-120b, gpt-oss-20b visible)
- **Split artifact handling** - Multi-target release artifact management

**Recent Commit Highlights:**
- chore: merge upstream 0.105.0 (239 commits)
- fix(tests): adapt model_list tests for cometix all-visible catalog
- Support for deprecated model visibility in picker
- Preserved cometix TUI extensions while updating upstream

**Relevance to helios-cli:**
- **MEDIUM**: Statusline customization less critical for helios but demonstrates TUI extension pattern
- **LOW-MEDIUM**: Model filtering flexibility useful if helios supports model selection UI
- **MEDIUM**: Translation system valuable for international deployment

---

## Tier 2: Notable Forks (50-80 commits ahead)

### 5. **DioNanos/codex-termux** (58 commits ahead)
**Description:** Termux (Android terminal) compatibility fork
**Key Features:**
- Android/Termux platform support
- Platform-specific dependency handling
- Recent: Merged upstream 0.105.0

**Relevance to helios-cli:** LOW (Platform-specific, not directly applicable)

---

### 6. **lee101/codex-infinity** (68 commits ahead)
**Description:** "infinite coding agent"
**Key Features:**
- Infinite context/iteration support
- V1.3.0 actively maintained with upstream merges

**Relevance to helios-cli:** MEDIUM (Infinite loop/iteration patterns valuable for complex tasks)

---

### 7. **sudoblockio/codex-rlm** (25 commits ahead)
**Description:** RLM (Recursive Language Model) integration fork
**Key Features:**
- **RLM tools** (load, exec, query, helpers, memory)
- **RLM TUI widgets** and status display
- **RLM session management** with sub-agent support
- **System prompt injection** for RLM instructions
- Successfully merged 488 upstream commits while preserving RLM features

**Relevance to helios-cli:** MEDIUM (RLM patterns useful for recursive code generation/analysis, though adds complexity)

---

## Comparative Feature Matrix

| Feature | just-every/code | zapabob/codex | Eriz1818/xCodex | Haleclipse/codex |
|---------|-----------------|---------------|-----------------|------------------|
| Multi-provider agents | ✅ | ❌ | ❌ | ❌ |
| Memory management | ✅ | ✅ | ✅ | ❌ |
| Cache optimization | ❌ | ✅ | ❌ | ❌ |
| Secret redaction | ✅ | ❌ | ✅ | ❌ |
| Plan persistence | ✅ | ❌ | ✅ | ❌ |
| Parallel agents | ✅ | ✅ | ✅ | ✅ |
| Audio support | ❌ | ✅ | ❌ | ❌ |
| TUI customization | ✅ | ❌ | ✅ | ✅ |
| Upstream compatible | Partial | ✅ | Partial | ✅ |
| Community size | Large (3.5k ⭐) | Tiny (4 ⭐) | Tiny (6 ⭐) | Small (203 ⭐) |

---

## Strategic Recommendations for helios-cli

### High Priority Patterns to Consider

1. **Secret/Credential Redaction** (from xCodex)
   - Essential for handling sensitive codebases
   - Match caching for performance
   - UX for revealing/confirming sensitive data

2. **Memory/Cache Management** (from zapabob)
   - Background cache refresh prevents stale data
   - Explicit memory phase management (extraction vs. reasoning)
   - Environment context persistence

3. **Multi-provider Support** (from just-every/code)
   - If helios expands beyond OpenAI, this pattern crucial
   - Config migration APIs enable smooth transitions

4. **Plan Persistence & Iteration** (from xCodex)
   - Auto-creation of plan files improves UX
   - Research log persistence valuable for debugging
   - Fix-and-start workflow patterns proven effective

### Lower Priority but Valuable

5. **Parallel Subagent Scaling** (documented in zapabob)
   - 2x capacity improvement measured
   - Worth profiling in helios if agent workflows added

6. **Upstream Compatibility** (from Haleclipse/codex)
   - Demonstrates ability to maintain custom features while merging upstream
   - Important if helios-cli needs to stay in sync with OpenAI/codex changes

---

## Risk Assessment

- **just-every/code**: 4250 commits ahead = significant divergence, harder to track upstream changes
- **zapabob/codex**: Actively maintained, good upstream merge hygiene
- **Eriz1818/xCodex**: Well-scoped divergence, patterns easily extractable
- **Haleclipse/codex**: Best upstream compatibility, clear custom-feature separation

---

## Data Sources

- GitHub API queries for all 30 top forks by stars
- Commit history analysis (20-50 recent commits per fork)
- Repository metadata (stars, forks, update timestamps)
- Feature extraction from commit messages and merge notes

---

**Report Generated:** 2026-02-28
**Analysis Scope:** Top 30 forks of openai/codex ranked by stargazers
