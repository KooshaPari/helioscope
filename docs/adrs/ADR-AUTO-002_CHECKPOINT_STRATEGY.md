# ADR-AUTO-002: Checkpoint Strategy - Multi-Layer Approach

**Status:** Accepted
**Date:** 2026-02-24
**Deciders:** heliosHarness Core Team
**Supersedes:** N/A

---

## Context

To ensure reliable rollback capability, we need a checkpoint strategy that captures all relevant system state. Single-layer checkpoints (just git) are insufficient for modern deployments that include database, configuration, and runtime state.

## Decision

We will use a multi-layer checkpoint strategy:

| Layer | Trigger | Retention | Storage |
|-------|----------|----------|----------|
| Git state | Pre-action | Forever | Git |
| Configuration | Pre-deploy | 30 days | SQLite |
| Database snapshot | Migration | 7 days | PostgreSQL |
| Metrics baseline | Hourly | 90 days | Time-series |

```
Checkpoint
├── git_sha: "abc123..."
├── config_snapshot: { ... }
├── db_snapshot_id: "snap_xyz"
└── metrics_baseline: { ... }
```

## Rationale

- Different state types need different preservation methods
- Git handles code/state perfectly
- Config files need serialization
- DB snapshots need point-in-time recovery
- Metrics needed for rollback verification

## Alternatives Rejected

### Single Git Checkpoint
- Doesn't capture DB state
- Doesn't capture config changes

### Full VM Snapshot
- Too expensive
- Too slow
- Overkill for most cases

## Consequences

### Positive
- Complete state capture
- Flexible restore options
- Verification capability

### Negative
- More complex implementation
- Storage costs
- Checkpoint time

---

## Implementation

### Components
- CheckpointManager: Orchestrates layers
- GitCheckpoint: Code state
- ConfigSnapshot: Configuration
- DbSnapshot: Database state
- MetricsBaseline: Performance metrics

### Checkpoint Flow
1. Trigger checkpoint (pre-execution)
2. Git commit with stash
3. Serialize config files
4. Create DB snapshot (if needed)
5. Record metrics baseline
6. Store checkpoint metadata
