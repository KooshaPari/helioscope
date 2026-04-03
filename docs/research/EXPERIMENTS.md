# Experiments Log — heliosCLI

**Purpose:** Documented experiments, prototypes, and their results  
**Last Updated:** 2026-04-02  
**Status:** Active research

---

## Experiment Registry

| ID | Experiment | Date | Status | Result |
|----|-----------|------|--------|--------|
| EXP-001 | OSS CLI Strictness Comparison | 2026-Q1 | ✅ Complete | See `research/oss-cli-matrix.md` |
| EXP-002 | Phase 1 Agent Research | 2026-Q1 | ✅ Complete | 6 agent reports in `research/phase-1-reports/` |
| EXP-003 | Phase 2 Candidate Expansion | 2026-Q1 | ✅ Complete | 5 workstream reports |
| EXP-004 | Harness Architecture Design | 2026-Q1 | ✅ Complete | `research/harness-spec.md` |
| EXP-005 | TUI Integration Prototype | TBD | 📋 Planned | Research TUI compositor patterns |
| EXP-006 | MCP Server Integration | TBD | 📋 Planned | Evaluate MCP for CLI plugins |

---

## Completed Experiments

### EXP-001: OSS CLI Strictness Comparison

**Hypothesis:** Different OSS CLIs have varying strictness profiles; we can benchmark and adopt best practices.

**Methodology:**
1. Selected 8 CLI candidates (openai/codex, goose, kilocode, cliproxyapi++, pluggedin-mcp-proxy, CLIProxyAPI, openai-codex-mcp, opencode)
2. Evaluated across 6 dimensions: Install, Build, Test, Quality Gates, API/SDK, License
3. Applied strictness scale: A (strict-full-noskip), B (strict-partial), C (pragmatic)
4. Scored composite readiness: quality (40) + coverage (20) + API (20) + build (20)

**Results:**
| Candidate | Strictness | Key Strength | Gap |
|-----------|------------|--------------|-----|
| goose | A- | `check-everything` in Justfile | Mixed toolchain complexity |
| kilocode | B+ | Workspace-quality story | Heavy ecosystem packaging |
| cliproxyapi++ | A- | Explicit `task quality` | Go-only (no Rust equivalent) |
| pluggedin-mcp-proxy | A-/B | TS strictness | MCP-specific, not general CLI |

**Conclusion:** 
- **Adopt:** `just` task runner for quality targets (from goose)
- **Innovate:** Centralized `quality:strict-full` command (not distributed)
- **Gap:** No Rust CLI has explicit single-command strictness like heliosCLI targets

**Artifacts:**
- `research/oss-cli-matrix.md` — Full comparison matrix
- `research/RESEARCH_INDEX.html` — Indexed findings

---

### EXP-002: Phase 1 Agent Research

**Hypothesis:** Multi-agent research can surface more findings than single-agent analysis.

**Methodology:**
6 agents (A-F) researched in parallel:
- Agent A (codex): Core repo hardening
- Agent B (opencode): Feature expansion
- Agent C (goose): Governance strictness
- Agent D (kilo): Harness architecture
- Agent E (discovery): Validation automation
- Agent F (governance): Closeout delivery

**Results:**
- Discovered 4 additional OSS candidates in `temp-PRODVERCEL/485`
- Validated strictness equivalence across 3 governance styles
- Created consolidated evidence document

**Artifacts:**
- `research/phase-1-reports/agent-a-codex.md`
- `research/phase-1-reports/agent-b-opencode.md`
- `research/phase-1-reports/agent-c-goose.md`
- `research/phase-1-reports/agent-d-kilo.md`
- `research/phase-1-reports/agent-e-discovery.md`
- `research/phase-1-reports/agent-f-governance.md`
- `research/helios-consolidated.md`

---

### EXP-003: Phase 2 Candidate Expansion

**Hypothesis:** Deeper analysis of shortlisted candidates reveals implementation patterns.

**Methodology:**
5 workstreams (A-E) analyzed:
- WS-A: Core repo hardening artifacts
- WS-B: Candidate expansion (local scan)
- WS-C: Governance strictness matrix
- WS-D: Harness architecture implementation
- WS-E: Validation automation evidence
- WS-F: Closeout delivery

**Results:**
- Created strictness equivalence matrix
- Designed phase 2 harness implementation
- Validated F5/F10 evidence schemas

**Artifacts:**
- `research/phase-2-reports/agent-a-core-repo-harden.md`
- `research/phase-2-reports/agent-b-candidate-expansion.md`
- `research/phase-2-reports/agent-c-governance-strictness.md`
- `research/phase-2-reports/agent-d-harness-architecture.md`
- `research/phase-2-reports/agent-e-validation-automation.md`
- `research/phase-2-reports/agent-f-closeout-delivery.md`

---

### EXP-004: Harness Architecture Design

**Hypothesis:** A unified harness can orchestrate quality across multiple CLI candidates.

**Methodology:**
1. Analyzed harness patterns from goose (Justfile), cliproxyapi++ (Taskfile)
2. Designed harness spec for strictness profiles
3. Created quality profile mapping: `quality_profile=STRICT_FULL`

**Results:**
- Harness spec supports: STRICT_FULL, STRICT_PARTIAL, PRAGMATIC profiles
- Normalizes per-candidate quality commands to canonical mapping
- Commit-lock proof collection for reproducibility

**Artifacts:**
- `research/harness-spec.md` — Full harness specification

---

## Planned Experiments

### EXP-005: TUI Integration Prototype

**Research Question:** Can heliosCLI integrate with heliosApp TUI for visual deployment management?

**Hypothesis:** A TUI layer improves UX for multi-backend operations.

**Methodology (Planned):**
1. Research TUI compositor patterns (from thegent tasks)
2. Prototype `helios-cli --tui` flag
3. Test with Docker/K8s backend switching

**Dependencies:**
- thegent TUI research completion
- heliosApp lane session model

**Success Criteria:**
- [ ] TUI renders deployment status
- [ ] Backend switching via TUI
- [ ] Keyboard-driven workflow

---

### EXP-006: MCP Server Integration

**Research Question:** Can MCP (Model Context Protocol) enable CLI plugins?

**Hypothesis:** MCP provides a standard interface for extending CLI functionality.

**Methodology (Planned):**
1. Study goose MCP crate architecture
2. Prototype MCP server for heliosCLI commands
3. Test with external tool calling

**Dependencies:**
- Goose MCP research
- MCP SDK availability

**Success Criteria:**
- [ ] MCP server exposes heliosCLI commands
- [ ] External clients can invoke deployments
- [ ] Security model validated

---

## Experiment Templates

### New Experiment Checklist

```markdown
### EXP-NNN: [Title]

**Research Question:** [Question]

**Hypothesis:** [Hypothesis]

**Methodology:**
1. [Step 1]
2. [Step 2]

**Results:**
- [Finding 1]
- [Finding 2]

**Conclusion:** [Conclusion]

**Artifacts:**
- [File path]

**Next Steps:**
- [Action]
```

---

## Research Debt

| Experiment | Why Not Done | Blocker | Priority |
|------------|--------------|---------|----------|
| WASM executor backend | No use case yet | Demand | P3 |
| Nomad backend support | No Nomad deployment | Infrastructure | P3 |
| Firecracker microVMs | Complexity | Research time | P2 |

---

**Update cadence:** After each completed experiment or bi-weekly, whichever comes first.
