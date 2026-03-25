# Governance & Documentation Mapping

**Date:** 2026-02-24

---

## Purpose

Map existing documentation and governance patterns across all projects to enable unified autonomous agent operation.

---

## Project Documentation Comparison

### 1. heliosHarness

| Doc Type | Location | Template Source |
|----------|----------|-----------------|
| Epic | docs/epics/ | trace/templates |
| User Story | docs/user-stories/ | trace/templates |
| FR | docs/functional-requirements/ | trace/templates |
| ADR | docs/adrs/ | trace/templates |
| Research | docs/research/ | Custom |
| Guides | docs/guides/ | Custom |

**Status:** Just created - following trace template

### 2. thegent

| Doc Type | Location | Template Source |
|----------|----------|-----------------|
| Governance | docs/governance/ | Custom |
| Research | docs/research/ | Custom |
| Plans | docs/plans/ | Custom |
| Reference | docs/reference/ | Custom |
| API | docs/api/ | Custom |

**Status:** Extensive - custom templates

### 3. 4sgm

| Doc Type | Location | Template Source |
|----------|----------|-----------------|
| PRD | docs/PRD.md | Custom |
| SPEC | docs/SPEC.md | Custom |
| Architecture | docs/architecture/ | Custom |
| ADR | docs/architecture/adr/ | Custom |
| Research | docs/research/ | Custom |
| Plans | docs/plans/ | Custom |

**Status:** Basic - needs expansion

### 4. tokenledger

| Doc Type | Location | Template Source |
|----------|----------|-----------------|
| PRD | docs/PRD.md | Custom |
| SPEC | docs/SPEC.md | Custom |
| Research | docs/research/ | Custom |
| Plans | docs/plans/ | Custom |
| Governance | docs/governance/ | Custom |

**Status:** Minimal - needs expansion

### 5. trace

| Doc Type | Location | Template Source |
|----------|----------|-----------------|
| Templates | docs/templates/ | **STANDARD** |
| User Story | docs/templates/USER_STORY_TEMPLATE.md | Standard |
| Epic | docs/templates/EPIC_TEMPLATE.md | Standard |
| FR | docs/templates/FUNCTIONAL_REQUIREMENT_TEMPLATE.md | Standard |
| ADR | docs/templates/ADR_TEMPLATE.md | Standard |

**Status:** TEMPLATE SOURCE - All others should follow

---

## Template Sources

### trace/templates (MASTER)

These are the canonical templates:

| Template | File | Purpose |
|----------|------|---------|
| Epic | EPIC_TEMPLATE.md | Large feature initiatives |
| User Story | USER_STORY_TEMPLATE.md | Individual features |
| FR | FUNCTIONAL_REQUIREMENT_TEMPLATE.md | Requirements |
| ADR | ADR_TEMPLATE.md | Architecture decisions |

---

## Governance Patterns

### Current State

| Project | TDD | BDD | SDD | Maturity |
|---------|-----|-----|-----|----------|
| heliosHarness | ☐ | ☐ | ☐ | L0 |
| thegent | ☑ | ☑ | ☑ | L2 |
| 4sgm | ☐ | ☐ | ☐ | L1 |
| tokenledger | ☐ | ☐ | ☐ | L1 |
| trace | ☑ | ☑ | ☑ | L3 |

### Target State

| Project | TDD | BDD | SDD | Target |
|---------|-----|-----|-----|--------|
| heliosHarness | ☑ | ☑ | ☑ | L3 |
| thegent | ☑ | ☑ | ☑ | L3 |
| 4sgm | ☑ | ☐ | ☐ | L2 |
| tokenledger | ☐ | ☐ | ☐ | L2 |

---

## SDD Implementation Roadmap

### Phase 1: Foundation (heliosHarness)

1. Create specification parser
2. Define YAML schema
3. Implement checkpoint system
4. Build verification pipeline

### Phase 2: Integration (thegent, 4sgm)

1. Add spec generation to thegent
2. Integrate checkpoint with 4sgm tasks
3. Build verification gates

### Phase 3: Governance (tokenledger)

1. Add cost verification
2. Implement budget checkpoints
3. Build optimization gates

---

## Documentation Consolidation

### Files to Create

1. **heliosHarness/docs/templates/** - Copy from trace
2. **4sgm/docs/templates/** - Copy from trace  
3. **tokenledger/docs/templates/** - Copy from trace

### Files to Update

1. **4sgm/docs/PRD.md** - Expand with detailed requirements
2. **4sgm/docs/SPEC.md** - Expand with technical details
3. **tokenledger/docs/PRD.md** - Expand with detailed requirements
4. **tokenledger/docs/SPEC.md** - Expand with technical details

---

## Action Items

| Item | Project | Priority |
|------|---------|----------|
| Copy templates to 4sgm | 4sgm | P0 |
| Copy templates to tokenledger | tokenledger | P0 |
| Expand 4sgm PRD | 4sgm | P1 |
| Expand 4sgm SPEC | 4sgm | P1 |
| Expand tokenledger PRD | tokenledger | P1 |
| Expand tokenledger SPEC | tokenledger | P1 |

---

*Last Updated: 2026-02-24*
