# ADR-AUTO-003: Autonomy Level Strategy

**Status:** Proposed
**Date:** 2026-02-24
**Deciders:** heliosHarness Core Team
**Supersedes:** N/A

---

## Context

We need to define how much autonomy the root manager has at each stage of execution. Too much autonomy is risky; too little defeats the purpose.

## Decision

| Stage | Autonomy Level | Human Touchpoint |
|-------|----------------|-------------------|
| Spec creation | L3 | Elicitation only |
| Implementation | L3 | Notification only |
| Verification | L2 | Auto-pass with alert |
| Deployment | L2 | Staged rollout |
| Production | L3 | Alert only |

### Autonomy Levels

| Level | Name | Description |
|-------|------|-------------|
| L0 | Manual | Full approval required |
| L1 | Suggested | Propose, wait for approve |
| L2 | Conditional | Execute, alert on issues |
| L3 | Autonomous | Execute, notify after |
| L4 | Self-healing | Full automation |

## Rationale

- L3 for spec/execution achieves our goal
- L2 for verification adds safety gate
- Staged rollout for deployment reduces blast radius
- Alerts keep humans informed without approvals

## Consequences

### Positive
- Speed: No approval bottlenecks
- Safety: Verification gates
- Transparency: Notifications always

### Negative
- Trust building required
- Risk of autonomous errors

---

## Implementation

### Decision Matrix

| Decision | Autonomy | Escalation |
|----------|----------|------------|
| Code change | Full | None |
| Config change | Full | None |
| Deploy to staging | Full | None |
| Deploy to prod | Conditional | Alert |
| Schema change | Conditional | Log |
| Rollback | Full | Notification |
| Emergency stop | Full | Immediate alert |
