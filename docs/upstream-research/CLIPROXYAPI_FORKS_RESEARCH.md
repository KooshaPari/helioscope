# Research Report: CLIProxyAPI Forks Ahead of Upstream

## Overview
Analyzed 294 recently-active forks (pushed since 2026-01-01) of `router-for-me/CLIProxyAPI` to identify which are ahead of upstream `main`.

**Total forks scanned:** 294 (from 1,956 total forks)  
**Forks ahead of main:** 43  
**Research strategy:** Focused on recently-pushed forks (pages 1-3, sorted by newest) as inactive forks are unlikely to be ahead.

---

## High-Impact Forks (Most Commits Ahead)

### 1. jiaqiwang969/CLIProxyAPI
- **Ahead:** 25 commits | Behind: 78 commits
- **Branches:** `entire/checkpoints/v1`, `main`
- **Key Work:** WebMCP console integration
  - WebMCPAdapter for tool and workflow definitions
  - WebMCPHandler with REST API endpoints

### 2. niudakok-kok/CLIProxyAPI
- **Ahead:** 22 commits | Behind: 8 commits (most current with upstream)
- **Branches:** `amp`, `dev`, `feature/ampcode-alias`, `legacy`, `main`
- **Key Work:** Auth hook integration (OnResult handler)
  - Recently merged upstream changes
  - Multiple feature branches for amp/alias support

### 3. pedrospota/CLIProxyAPI
- **Ahead:** 15 commits | Behind: 22 commits
- **Branches:** `main` only
- **Key Work:** Build system fixes
  - Usage statistics enablement
  - Railway deployment (printf/heredoc fix)

### 4. xiaojiou176/CLIProxyAPI
- **Ahead:** 15 commits | Behind: 78 commits
- **Branches:** 4 branches including coverage-gate and env-hardening
- **Key Work:** Test infrastructure and thinking model updates
  - Matrix expectations sync with provider effort mapping
  - Auth suite alignment with runtime behavior

### 5. sofs2005/CLIProxyAPI
- **Ahead:** 14 commits | Behind: 8 commits (most current)
- **Branches:** `main` only
- **Key Work:** Auth hook integration + merged upstream
  - Recently integrated pull #1734 changes
  - Actively tracking upstream

---

## Notable Feature Additions

### Desktop Application Support
- **alanbulan/CLIProxyAPI** (7 commits ahead)
  - Desktop app updater fixes
  - Desktop icon packaging and release features
  - Branches: `feat/desktop-app`, `feat/desktop-icon-packaging-and-release`

### Vertex AI / Claude Partner Routing
- **Pawpaw-Technology/CLIProxyAPI** (5 commits ahead)
  - Claude partner endpoint routing
  - Model exposure for OAuth routing
  - Specific focus on Vertex integration

### Sticky Round-Robin Load Balancing
- **kknd0/CLIProxyAPI** (3 commits ahead)
  - Sticky round-robin with X-Session-Key header
  - Fall-back to fill-first behavior
  - Multiple feature branches for amp/legacy support

### Cloudflare Tunnel Integration
- **MohamedKamran/CLIProxyAPI** (3 commits ahead)
  - Automatic Cloudflare tunnel support
  - Configuration management
  - PR-based feature work

### OpenAI Compatibility Mode
- **mvmik/CLIProxyAPI** (3 commits ahead)
  - default_reasoning_effort parameter support
  - Assistant content merge with tool calls
  - Translator improvements

---

## Regional/Language Customizations

- **yanhaoluo0/CLIProxyAPI** (3 commits ahead)
  - Chinese logging for debugging
  - claude-cli user-agent specific handling
  - Config examples for allowed-user-agent-prefix

- **yzsnstotz/CLIProxyAPI** (1 commit ahead)
  - Model ID caching and validation
  - Chinese documentation and error messages
  - Skill integration fixes (502 error handling)

- **15515151/CLIProxyAPI** (1 commit ahead)
  - Minimal divergence, mostly merged upstream

---

## Model Support Enhancements

- **anonym-uz/CLIProxyAPI** (1 commit ahead)
  - Claude Sonnet 4.6 model definition
  - Qwen 3.5 Plus support
  - Multiple feature branches for different model variants

- **benjaminkitt/CLIProxyAPI** (1 commit ahead)
  - Claude 4.6 Sonnet with thinking support
  - Antigravity model config
  - Qwen 3.5 support

- **xkonjin/CLIProxyAPI** (1 commit ahead)
  - Multiple documentation branches
  - Wildcard API key auth feature
  - Qwen 3.5 Plus with updated user agent headers

---

## CI/CD and Tooling

- **CassiopeiaCode/CLIProxyAPI** (4 commits ahead)
  - GHCR image name casing fixes
  - Codex output alignment improvements
  - pprof server documentation branch

- **q2d5Z3By/CPA-TTL** (3 commits ahead)
  - Docker image workflow updates
  - Build system improvements

- **vincent-kk/CLIProxyAPI** (2 commits ahead)
  - Fetch-panel command for management control
  - Compile-time asset embedding

---

## Summary by Divergence Pattern

### Most Current (Behind < 10)
- niudakok-kok (behind 8)
- learner-wqm (behind 8)
- suminhthanh (behind 8)
- xoxgladxox-max (behind 8)
- 136906a (behind 8)
- sofs2005 (behind 8)
- matchch (behind 8)
- kslr (behind 9)
- jt-ai-tools (behind 9)

These forks are tracking upstream closely while maintaining their own work.

### Significantly Diverged (Behind > 100)
- Pawpaw-Technology (behind 126)
- mvmik (behind 114)
- yanhaoluo0 (behind 119)
- anonym-uz (behind 114)
- benjaminkitt (behind 114)
- xkonjin (behind 117)
- thoitiettxl-cyber (behind 126)
- MohamedKamran (behind 126)
- qingwo1991-debug (behind 126)
- lisa111120 (behind 126)
- lisa111121 (behind 126)
- Catfish872 (behind 126)

These forks have substantial divergence and would require careful merging.

---

## Recommendations

1. **High-Value Integration Candidates:**
   - niudakok-kok: Well-maintained with recent auth improvements
   - sofs2005: Actively tracking upstream with auth work
   - matchch: Feature work on cache and user ID

2. **Feature Work to Review:**
   - alanbulan: Desktop application support (7 commits)
   - jiaqiwang969: WebMCP integration (25 commits - most divergent)
   - Pawpaw-Technology: Vertex/Claude partner routing

3. **Model Registry Updates:**
   - Multiple forks have Claude 4.6 Sonnet and Qwen 3.5 definitions
   - Consider consolidating model definitions across forks

4. **Build System Improvements:**
   - pedrospota: Railway deployment fixes worth reviewing

---

## Complete Fork List (Sorted by Commits Ahead)

See summary table above for all 43 forks and their ahead/behind counts.
