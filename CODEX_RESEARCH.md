# Codex CLI Research: Performance, Parity & User Friction Analysis

**Date:** 2026-02-23  
**Purpose:** Improve Codex/Codex CLI performance and gain parity with competing CLIs

---

## 1. Executive Summary

This document provides in-depth research on OpenAI Codex CLI, analyzing user friction points, competitive landscape, and optimization opportunities. Key findings:

- **Primary Pain Points:** Shell command timeouts (90%), REPL usability issues, background process leaks
- **Competitive Gap:** Droid (Factory) outperforms Codex on Terminal-Bench by ~17 points
- **Optimization Opportunities:** TTFT reduction, streaming improvements, context management

---

## 2. Competitive Landscape

### 2.1 AI Coding CLI Comparison Matrix

| Feature | Codex CLI | Claude Code | Droid (Factory) | Gemini CLI |
|---------|-----------|-------------|-----------------|------------|
| **Default Model** | GPT-5.3-Codex | Claude Opus 4.6 | Claude Opus 4.6 | Gemini 3 Pro |
| **Terminal-Bench** | 62.9% | 58.0% | 69.9% | ~50% |
| **SWE-Bench** | 80.0% | 80.9% | N/A | ~72% |
| **Context Window** | 192K | 200K (1M beta) | 200K | 1M |
| **Max Output** | 64K | 128K | 128K | 64K |
| **Pricing** | $20/mo (ChatGPT) | $20-200/mo | Enterprise | Free + Pro |
| **Open Source** | Yes (Apache 2.0) | No | No | Yes |
| **Execution** | Sandboxed | Full system | Full system | Full system |

### 2.2 Key Differentiators

**Codex CLI Strengths:**
- Open source (Apache 2.0) - can be self-hosted/extended
- Included with ChatGPT subscription
- Strong GitHub integration
- Cloud sandbox execution (safer)
- Generous session limits

**Codex CLI Weaknesses:**
- REPL non-standard behavior (uses React-based Ink, not readline)
- Shell command timeout issues (90% failure rate reported)
- Background process leak on interrupt
- 150s hard timeout on requests
- Poor frontend framework support (React issues)

---

## 3. User Friction Analysis

### 3.1 Top User Complaints (from GitHub Issues, HN, Reddit)

#### Critical Issues (Blocking)

1. **Shell Command Timeouts** (Issue #3765)
   - 90% of shell commands hang/timeout
   - No visibility into timeout reasons
   - User frustration: "90% failure rate"

2. **Background Process Leak** (Issue #7932)
   - Ctrl+C doesn't terminate spawned processes
   - Chrome instances spawn uncontrollably
   - No job control for long-running tasks

3. **Request Timeout at ~150s** (Issue #3478)
   - Hard timeout not configurable
   - Complex multi-file edits fail
   - VS Code extension handles better (unclear why)

4. **Usage Limit Hit** (widespread)
   - "You've hit your usage limit" errors
   - Even with ChatGPT Plus
   - No clear way to check quota

#### UX Friction Issues

5. **Non-Standard REPL** (Discussion #520)
   - Ctrl+[Left/Right] instead of Option+[Left/Right]
   - Ctrl+E opens vi editor instead of end-of-line
   - Not using Node's built-in REPL/readline
   - Shift+Enter indicated but doesn't work

6. **Inconsistent Tool Execution**
   - Sometimes prompts user to run commands
   - Sometimes executes automatically
   - No clear logic for when each happens

7. **Organization Verification** (HN)
   - 90-day cooldown on org verification
   - Complicates professional/promotional use

---

## 4. Performance Optimization Opportunities

### 4.1 Timing Breakdown Analysis

Based on benchmark data from `llm_sla_benchmark.py`:

```
Component            │ Time    │ % of Total
────────────────────┼─────────┼────────────
TTFT                │ 6.76s  │ 67.1%
Network RTT          │ 0.05s  │  0.5%
Generation          │ 0.75s  │  7.5%
Overhead            │ 2.53s  │ 25.0%
────────────────────┼─────────┼────────────
TOTAL               │ 10.07s │ 100.0%
```

**Key Insight:** TTFT (Time to First Token) dominates at 67%. Reducing TTFT by 50% would improve overall latency by 33%.

### 4.2 Optimization Strategies

#### High Impact

1. **Streaming Optimization**
   - Current: Wait for full response before streaming
   - Opportunity: Stream tokens immediately on generation
   - Estimated improvement: 30-40% TTFT reduction

2. **Context Compression**
   - Current: Full file context sent to LLM
   - Opportunity: Summarize boilerplate, keep only relevant sections
   - Also reduces token costs

3. **Connection Pooling**
   - Current: New connection per request
   - Opportunity: Keepalive connections, HTTP/2 multiplexing
   - Estimated improvement: 20-30% latency reduction

#### Medium Impact

4. **Caching Strategy**
   - Cache prompt templates
   - Cache LLM responses for repeated queries
   - Local vector DB for codebase context

5. **Parallel Tool Execution**
   - Current: Sequential tool calls
   - Opportunity: Execute independent tools in parallel
   - Estimated improvement: 15-25% for multi-tool tasks

---

## 5. Feature Parity Analysis

### 5.1 Missing vs Claude Code

| Feature | Claude Code | Our Implementation |
|--------|-------------|-------------------|
| **Sub-agents/Teams** | Yes | No |
| **Rewind/Undo** | Yes | No |
| **MCP Support** | Full | Limited |
| **Hooks** | Yes | No |
| **Skills** | Yes | No |
| **Session Resume** | Yes | Yes (basic) |
| **Agent Forking** | Yes | No |

### 5.2 Missing vs Droid (Factory)

| Feature | Droid | Our Implementation |
|---------|-------|-------------------|
| **Terminal-Bench Leader** | #1 | Not benchmarked |
| **Multi-model Support** | OpenAI + Anthropic | OpenAI only |
| **Sub-agents** | Specialized | No |
| **Deep IDE Integration** | Full | CLI only |
| **Enterprise Features** | Yes | No |

---

## 6. Recommendations

### 6.1 Quick Wins (1-2 weeks)

1. **Increase Default Timeout**
   - Change from 150s to 300s (configurable)
   - Makes complex tasks possible

2. **Improve Shell Error Messages**
   - Show why commands timeout
   - Add retry suggestions

3. **Fix Background Process Cleanup**
   - Register signal handlers
   - Kill child processes on interrupt

### 6.2 Medium Term (1-2 months)

4. **Standard REPL Behavior**
   - Support Option+arrows
   - Fix Shift+Enter
   - Or document deviations clearly

5. **Streaming Improvements**
   - First token streaming
   - Progressive UI updates

6. **Multi-Agent Support**
   - Implement sub-agents for parallel tasks
   - Agent spawning/coordination

### 6.3 Long Term (3-6 months)

7. **Sub-agent Orchestration**
   - Match Droid's specialized agents
   - Task-specific agent routing

8. **Enterprise Features**
   - SSO/SAML
   - Audit logging
   - Role-based access

---

## 7. Benchmark Infrastructure

Located in `heliosHarness/harness/benchmarks/`:

- `benchmark_runner.py` - Standardized matrix (models × harnesses)
- `llm_sla_benchmark.py` - Detailed LLM metrics (TTFT, generation, cost)
- `harness_benchmark.py` - System resource tracking

**Run benchmarks:**
```bash
cd heliosHarness
PYTHONPATH=.. python3 -m harness.benchmarks.benchmark_runner \
  --matrix --agents 6
```

---

## 8. Appendix: Research Sources

### GitHub Issues (Selected)
- #3765: "Most of shell tools are timing out"
- #7932: "Background Process Leak + Missing Job Control"
- #3478: "Requests cut off at ~150s"
- #5670: "Inconsistent tool execution"
- #520: "Non-standard REPL"

### External Research
- Terminal-Bench 2.0 leaderboard (2026-02)
- SWE-bench Verified scores
- Hacker News discussions
- Reddit r/ChatGPT, r/ClaudeAI

---

*Last Updated: 2026-02-23*
