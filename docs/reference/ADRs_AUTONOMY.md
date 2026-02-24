# Architecture Decision Records

## ADR-001: Specification Format

### Status: Proposed

### Context

We need a machine-readable specification format for agent instructions.

### Decision

Use YAML with JSON schema validation.

```yaml
spec: feature_auth
version: "1.0"
owner: auth-team
verification:
  - test: auth_flow_test
rollback:
  strategy: git_revert
success_criteria:
  - metric: login_latency_ms < 100
```

### Consequences

| Positive | Negative |
|----------|----------|
| Schema validation | Learning curve |
| Version control | Template maintenance |
| Diff reviewable | Tooling required |

---

## ADR-002: Autonomy Level Strategy

### Status: Proposed

### Context

How much autonomy at each stage?

### Decision

| Stage | Level | Human Touch |
|-------|-------|--------------|
| Spec creation | L3 | Elicitation |
| Implementation | L3 | Notification |
| Verification | L2 | Auto-pass |
| Deployment | L2 | Staged rollout |
| Production | L3 | Alert only |

### Consequences

| Positive | Negative |
|----------|----------|
| Speed | Risk of errors |
| Efficiency | Trust building |

---

## ADR-003: Checkpoint Strategy

### Status: Proposed

### Context

How to ensure rollback capability?

### Decision

Multi-layer checkpoints:

| Layer | Trigger | Retention |
|--------|----------|----------|
| Git state | Pre-action | Forever |
| Config | Pre-deploy | 30 days |
| DB snapshot | Migration | 7 days |
| Metrics | Hourly | 90 days |

---

## ADR-004: Sub-agent Communication

### Status: Proposed

### Context

How do agents coordinate?

### Decision

Use message queue with priority channels:

```python
# Priority channels
HIGH = "agent:priority:high"    # Security
NORMAL = "agent:priority:normal"
LOW = "agent:priority:low"
```

---

## ADR-005: Verification Gate

### Status: Proposed

### Context

When to allow deployment?

### Decision

Verification gates:

```yaml
gates:
  - name: unit_tests
    threshold: 80%
  - name: security_scan
    blockers: [critical,high]
  - name: performance_baseline
    regression_threshold: 10%
```

---

## ADR-006: Rollback Strategy

### Status: Proposed

### Context

How to ensure safe rollback?

### Decision

Always-able rollback:

1. Pre-action checkpoint
2. Canary deployment (1% → 10% → 100%)
3. Metrics monitoring
4. Auto-rollback on thresholds

---

## ADR-007: User Interaction Model

### Status: Proposed

### Context

How does user interact?

### Decision

| Interaction Type | Channel | Frequency |
|-----------------|---------|-------------|
| Elicitation | CLI/API | Per feature |
| Review | Dashboard | On-demand |
| Alerting | Slack/Email | Exception only |
| Feedback | Retro | Weekly |
