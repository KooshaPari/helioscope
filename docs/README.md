# Autonomous Root Agent - Documentation Index

## Project Overview

This documentation suite covers the design and implementation of a fully autonomous AI agent system where a single root manager coordinates sub-agents with minimal human intervention.

---

## Documentation Structure

```
docs/
├── epics/
│   └── EPIC-AUTO-001_FULL_AUTONOMY.md
├── user-stories/
│   ├── US-AUTO-001_SPEC_DRIVEN_EXECUTION.md
│   ├── US-AUTO-002_CHECKPOINT_SYSTEM.md
│   ├── US-AUTO-003_AUTOMATIC_ROLLBACK.md
│   ├── US-AUTO-004_SUB_AGENT_COORDINATION.md
│   └── US-AUTO-005_ELICITATION_ONLY.md
├── functional-requirements/
│   ├── FR-SPEC-001_SPEC_PARSER.md
│   ├── FR-SPEC-002_NL_TO_SPEC.md
│   ├── FR-CHK-001_GIT_CHECKPOINT.md
│   └── FR-ROLL-001_ATOMIC_ROLLBACK.md
├── adrs/
│   ├── ADR-AUTO-001_SPEC_FORMAT.md
│   ├── ADR-AUTO-002_CHECKPOINT_STRATEGY.md
│   └── ADR-AUTO-003_AUTONOMY_LEVELS.md
├── research/
│   ├── RESEARCH_FACTORY_DROIDS.md
│   └── RESEARCH_SDD_BDD.md
└── guides/
    └── IMPL_SPEC_PARSER.md
```

---

## Quick Links

| Document | Purpose |
|----------|---------|
| EPIC-AUTO-001 | Master epic with full scope |
| US-AUTO-001 | Spec-driven execution story |
| US-AUTO-002 | Checkpoint system story |
| FR-SPEC-001 | Specification parser FR |
| ADR-AUTO-001 | Format decision (YAML) |
| ADR-AUTO-002 | Checkpoint strategy |
| RESEARCH_FACTORY_DROIDS | Factory pattern research |

---

## Key Decisions

| ADR | Decision | Status |
|-----|----------|--------|
| ADR-AUTO-001 | YAML + JSON Schema for specs | Accepted |
| ADR-AUTO-002 | Multi-layer checkpoints | Accepted |
| ADR-AUTO-003 | Level 3 autonomy (notify only) | Proposed |

---

## Progress

| Epic | Stories | Points | Progress |
|------|---------|--------|----------|
| EPIC-AUTO-001 | 8 | 76 | 27.6% |

---

## References

- Factory Droids: https://factory.ai/news/ga
- SDD Research: arXiv:2602.00180
- AWS AutoPilot: Self-driving DevOps patterns
