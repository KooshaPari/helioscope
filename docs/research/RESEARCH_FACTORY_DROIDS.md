# Research: Factory Droids Architecture Deep Dive

**Date:** 2026-02-24
**Status:** Completed

---

## Executive Summary

Factory.ai's Droids represent the state-of-the-art in autonomous software engineering agents. Their architecture provides key insights for heliosHarness development.

---

## Core Architecture

### Memory System

| Feature | Implementation | Notes |
|---------|----------------|-------|
| Persistent Context | SQLite + Redis | Cross-session memory |
| Organizational Context | GitHub/Jira/Slack | Integration-first |
| Session Memory | In-memory | Current task context |

### Agent Types

| Droid | Function |
|-------|----------|
| Root Manager | Orchestrator/coordinator |
| Coder | Feature implementation |
| Reviewer | Code review |
| Researcher | Investigation |
| Tester | Verification |

### Autonomy Patterns

1. **Spec-first**: Clear specs before action
2. **Checkpoint-based**: State preservation
3. **Verification-first**: Tests pass before merge
4. **Rollback-ready**: Always reversible
5. **Audit trail**: Full traceability

---

## Key Learnings

### What Works
- Memory persistence across sessions
- Integration-first approach (GitHub, Jira, Slack)
- Hierarchical agent structure
- Safety controls for autonomous operation

### What Doesn't Work
- Pure autonomous without verification
- Lack of checkpoint/restore
- No audit trail

---

## Implications for heliosHarness

| Factory Pattern | heliosHarness Implementation |
|----------------|------------------------------|
| Memory system | Checkpoint + audit logging |
| Spec-driven | SDD specification engine |
| Root manager | Orchestrator agent |
| Sub-agents | Task workers |
| Verification | Pre-execution gates |

---

## References

- Factory Droid GA: https://factory.ai/news/ga
- Factory Series B: https://siliconangle.com/2025/09/25/factory-unleashes-droids
