# Upstream Dependencies Research: helios-cli Infrastructure Layer

## Overview

This research package provides a comprehensive analysis of the upstream dependencies, infrastructure technologies, and ecosystem landscape for **helios-cli** (Rust TUI AI coding agent, fork of openai/codex).

The research was conducted on **February 28, 2026** and focuses on recent developments (October 2025 – February 2026) with comprehensive coverage of 6 major technology categories, 30+ projects, and 80+ information sources.

## Files in This Package

### 1. UPSTREAM_DEPS_RESEARCH.md (Main Document)
**Size:** 810 lines | **Content:** 31 KB

The primary research document containing:

- **Executive Summary** – Key findings across all categories
- **Section 1: MCP (Model Context Protocol)** – Latest spec (2025-11-25), Rust SDK status (v0.17.0), production implementations
- **Section 2: Terminal UI Frameworks** – Ratatui 0.30.0 analysis, Crossterm updates, WezTerm and Zellij integration
- **Section 3: LLM Client Libraries** – rust-genai, Rig, Ollama, mistral.rs, direct provider clients
- **Section 4: Sandboxing & Security** – Landlock, seccomp-BPF, OWASP AI Agent Top 10, microVM recommendations
- **Section 5: Code Intelligence** – Tree-sitter, LSP integration, semantic code search (Zoekt/Sourcegraph)
- **Section 6: Streaming & Protocol** – Tokio, WebSockets, gRPC, Server-Sent Events
- **Section 7: Integration Roadmap** – Prioritized action items (immediate, medium-term, long-term)
- **Section 8: Dependency Health Dashboard** – Security status, license compliance, build optimization
- **Section 9: Strategic Recommendations** – ROI analysis and investment priorities
- **Appendix** – Key GitHub repositories organized by category

### 2. UPSTREAM_DEPS_RESEARCH_SOURCES.md (Sources Document)
**Size:** 187 lines | **Content:** 13 KB

Complete reference list of 80+ sources organized by category:

- MCP (Model Context Protocol) – 9 sources
- MCP Rust SDK & Implementations – 10 sources
- Terminal UI Frameworks – 14 sources
- LLM Client Libraries – 9 sources
- Sandboxing & Security – 18 sources
- Code Intelligence & Analysis – 11 sources
- Streaming & Protocol Communication – 15 sources
- Research Methodology documentation

### 3. README_RESEARCH.md (This File)

Navigation guide and quick reference for the research package.

## Key Findings Summary

### MCP (Model Context Protocol)
- Latest spec: **2025-11-25** (November 25, 2025)
- Tasks primitive for async, long-running operations
- Linux Foundation governance (AAIF) as of December 2025
- Rust SDK (rmcp) at **v0.17.0** (February 2026)
- Full backward compatibility with Tasks, Elicitation, OAuth 2.1

### Terminal UI
- Ratatui **0.30.0** (December 2025) – "biggest release so far"
  - No-std support for embedded systems
  - Modularized workspace for faster builds
  - 2,500+ projects using ratatui
- Crossterm 0.29.0 actively maintained
- WezTerm GPU-accelerated terminal with Lua plugin API
- Zellij WebAssembly-based multiplexer

### LLM Integration
- **Recommended:** rust-genai for unified multi-provider support
- Supports: OpenAI, Anthropic, Ollama, Groq, DeepSeek, and 8+ others
- Alternative: Rig framework with RAG support
- Local: Ollama (de facto standard) + mistral.rs (pure Rust inference)

### Security & Sandboxing (Critical for AI Agents)
- Landlock LSM for unprivileged Linux sandboxing
- Seccomp-BPF for syscall filtering
- **Production recommendation:** Firecracker microVMs or Kata Containers
- OWASP AI Agent Security Top 10 (2026) compliance needed
- Network egress control mandatory
- Pre-execution static analysis required

### Code Intelligence
- Tree-sitter 0.25.10 (774K+ monthly downloads)
- October 2025: Query auto-exposure proposal in progress
- LSP: Tower-LSP recommended for language server integration
- Code search: Zoekt (trigram-based, sub-second queries)

### Streaming & Protocols
- Tokio 1.x – production-stable async runtime
- WebSockets via tokio-tungstenite (custom fork for perf)
- gRPC via Tonic (HTTP/2, async/await native)
- SSE via eventsource-stream for LLM streaming

## Critical Gaps in helios-cli

1. **No explicit MCP server** – Should implement MCP protocol for tool exposure
2. **No semantic code understanding** – Missing LSP integration and tree-sitter queries
3. **Limited LLM abstraction** – No unified multi-provider client (should add rust-genai)
4. **Weak network sandboxing** – No egress control visible
5. **No distributed agents** – No gRPC for multi-agent coordination

## Recommended Actions (Prioritized)

### Immediate (0-3 months)
1. Audit MCP 2025-11-25 spec compliance – Low effort
2. Add rust-genai LLM client – Medium effort, High value
3. Enhance network sandboxing – Medium effort, High value
4. Upgrade to ratatui 0.30.0 – Medium effort, High value (build speed)
5. Implement code-context MCP server – High effort, High value

### Medium-term (3-6 months)
1. LSP client integration – High effort, High value
2. Firecracker integration – High effort, High value
3. gRPC bidirectional streaming – High effort, High value
4. Zellij plugin – Medium effort
5. WezTerm integration – Medium effort

### Long-term (6-12 months)
1. Sourcegraph/Zoekt integration – High effort
2. Tree-sitter custom grammars – High effort
3. Multi-agent orchestration – Very high effort
4. OpenTelemetry observability – Medium effort

## Technology Landscape Assessment

### Maturity Levels

| Technology | Status | Adoption | Recommendation |
|-----------|--------|----------|-----------------|
| MCP | ✅ Stable | Industry standard | Implement core |
| Ratatui | ✅ Mature | 2,500+ projects | Upgrade to 0.30.0 |
| Crossterm | ✅ Stable | 73M downloads | Keep current |
| Tokio | ✅ Production | De facto standard | Current version fine |
| Tree-sitter | ✅ Mature | 774K/month | Monitor for upgrades |
| LSP | ✅ Stable | Industry standard | Add integration |
| Landlock | ✅ Established | Growing | Enhance usage |
| Zoekt | ⚠️ Niche | Sourcegraph/GitHub | Evaluate later |
| WezTerm | ✅ Active | Growing | Plugin support |
| Zellij | ✅ Active | Growing | Plugin support |

### Risk Assessment

**High Risk:**
- Network sandboxing gaps (potential data exfiltration)
- Insufficient code execution isolation (kernel-level attacks possible)
- No semantic code understanding (context quality issues)

**Medium Risk:**
- Custom fork maintenance (nornagon patches)
- LLM provider coupling (no abstraction layer)
- MCP compliance ambiguity (spec updated Nov 2025)

**Low Risk:**
- Core async runtime (Tokio mature and stable)
- Terminal UI (Ratatui ecosystem healthy)
- Dependency availability (all projects actively maintained)

## How to Use This Research

### For Architecture Decisions
Reference **Section 7: Integration Roadmap** and **Section 9: Strategic Recommendations** for investment prioritization.

### For Implementation Details
Consult specific technology sections (1-6) for:
- Latest API documentation
- Recent improvements
- Known issues
- Relevant GitHub repositories
- Example code patterns

### For Compliance & Security
Review **Section 4: Sandboxing & Security** for OWASP AI Agent Top 10 requirements and isolation strategies.

### For Dependency Management
Check **Section 8: Dependency Health Dashboard** for:
- Current versions vs. upstream
- Security status
- License compliance
- Build optimization opportunities

### For Quick Reference
Use **Appendix: Key GitHub Repositories** for direct links to all 30+ projects analyzed.

## Research Scope & Limitations

### Included
- Upstream Rust crate ecosystem
- Terminal UI frameworks
- LLM client libraries
- Security/sandboxing technologies
- Code analysis tools
- Streaming protocols

### Excluded
- TypeScript/JavaScript equivalents (unless cross-cutting)
- Specific helios-cli implementation details
- Closed-source or proprietary tools
- Non-networking distributed systems

### Methodology Notes
- Prioritized official repositories and documentation
- Cross-referenced release dates with version numbers
- Validated adoption metrics from crates.io and GitHub
- Focused on 2025-2026 developments with historical context

## Document Information

**Created:** February 28, 2026  
**Version:** 1.0  
**Total Sources:** 80+ unique URLs  
**Categories:** 6 major technology domains  
**Projects Analyzed:** 30+ repositories and tools  
**Total Lines:** ~1,000 lines across 3 documents  

---

## Quick Navigation

| Topic | Document | Section |
|-------|----------|---------|
| MCP Specification & SDKs | UPSTREAM_DEPS_RESEARCH.md | Section 1 |
| Terminal UI & Frameworks | UPSTREAM_DEPS_RESEARCH.md | Section 2 |
| LLM Integration | UPSTREAM_DEPS_RESEARCH.md | Section 3 |
| Security & Sandboxing | UPSTREAM_DEPS_RESEARCH.md | Section 4 |
| Code Intelligence | UPSTREAM_DEPS_RESEARCH.md | Section 5 |
| Streaming & Protocols | UPSTREAM_DEPS_RESEARCH.md | Section 6 |
| Implementation Roadmap | UPSTREAM_DEPS_RESEARCH.md | Section 7 |
| Dependency Health | UPSTREAM_DEPS_RESEARCH.md | Section 8 |
| Strategic Priorities | UPSTREAM_DEPS_RESEARCH.md | Section 9 |
| All Sources & URLs | UPSTREAM_DEPS_RESEARCH_SOURCES.md | All sections |

---

For detailed analysis, start with **UPSTREAM_DEPS_RESEARCH.md**.  
For source verification, consult **UPSTREAM_DEPS_RESEARCH_SOURCES.md**.
