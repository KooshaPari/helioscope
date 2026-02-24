# US-AUTO-005: Elicitation-Only User Interaction

**Epic:** EPIC-AUTO-001
**Actor:** Product Owner / Developer
**Priority:** P0-Critical
**Status:** Backlog
**Sprint:** Sprint 3
**Points:** 5

---

## User Story

**As a** product owner,
**I want** to provide strategic ideas without approval workflows,
**so that** agents execute autonomously based on direction.

---

## Acceptance Criteria

### Scenario 1: Idea Input
- [ ] **Given** user provides an idea in natural language
- [ ] **When** elicitation handler receives input
- [ ] **Then** it parses intent
- [ ] **And** generates specification
- [ ] **And** queues for autonomous execution

### Scenario 2: No Approval Required
- [ ] **Given** specification is generated
- [ ] **When** verification passes
- [ ] **Then** execution proceeds automatically
- [ ] **And** no human approval is requested

### Scenario 3: Post-Hoc Review
- [ ] **Given** execution completed
- [ ] **When** user views dashboard
- [ ] **Then** they see action summary
- [ ] **And** can provide feedback for future iterations

---

## Functional Requirements

### Implements
- FR-ELIC-001: Elicitation input handler
- FR-ELIC-002: Intent classification system
- FR-ELIC-003: Specification generation from ideas
