<DONE>
# Phase-2 Lane B: Candidate Expansion

Scope: discovery-focused expansion for open-source CLI/MCP candidates, including:

- Local `temp-PRODVERCEL/485` scan.
- Open-source registry/web discovery shortlist.
- Evidence capture limited to installs, scripts/build hooks, licenses, and strictness profile.

## Scoring Rubric Deltas vs Phase-1 Shortlist

Compared with the Phase-1 matrix baseline (`quality 40 + quality-coverage 20 + API/SDK 20 + build 20`), add the following adjustments for Phase-2 expansion candidates:

- `Evidence completeness` (new, +20 points): +4 each for installation command evidence, +4 for explicit build commands, +4 for test/quality commands, +4 for license traceability, +4 for strictness profile evidence.
- `Open-source confidence` (new, +10 points): +5 for public repo + tag/branch strategy, +5 for permissive license file + governance docs.
- `Operational risk penalty` (new, -15 points): applies when installability is inferred from docs only, scripts are missing, or strictness evidence is absent.
- `Baseline weights retained` for API/SDK surface and build confidence but normalized for multi-command ecosystems (repo scoring now requires explicit evidence for each command family).

## Tasks

### Task B1: Freeze Phase-1 baseline and lane handoff contract
Status: `done`  
Depends on: none  
Evidence: `research/oss-cli-matrix.md`, `artifacts/phase-1-closeout.md`, `research/harness-spec.md` (snapshot of baseline candidates and scoring assumptions at task start).

### Task B2: Enumerate candidate repositories from local `/Users/kooshapari/temp-PRODVERCEL/485`
Status: `done`  
Depends on: `B1`  
Evidence: `temp-PRODVERCEL/485` directory inventory file capturing repo name, presence of CLI entrypoint signals (`README`, `Makefile`, `pyproject`, `package.json`, `Cargo.toml`, `go.mod`, `main.go`, `index.ts`), and MCP keywords (`mcp`, `tools`, `stdio`, `sse`, `json-rpc`).

### Task B3: Filter local scan to open-source CLI/MCP candidates only
Status: `done`  
Depends on: `B2`  
Evidence: evidence matrix for each candidate with: `AGENTS/README` availability, open source indicator, license availability, active entrypoint (CLI/MCP), and rationale for inclusion/exclusion.

### Task B4: Capture open-source license artifacts for local candidates
Status: `done`  
Depends on: `B3`  
Evidence: per-candidate `LICENSE*` paths, SPDX/declared license text, and exceptions/embedded third-party license notes; if missing, mark as evidence-gap.

### Task B5: Capture install execution surface for local candidates
Status: `done`  
Depends on: `B3`  
Evidence: install command evidence per candidate (`npm`, `pnpm`, `brew`, `go install`, `cargo install`, `uv/pip`, `docker`, archive/ script install), command snippets with source file and line references.

### Task B6: Capture build, test, and quality commands for local candidates
Status: `done`  
Depends on: `B3`  
Evidence: explicit command list and config files for build/test/lint/typecheck/gating (`Makefile`, CI workflow, task files, package scripts), plus whether commands are mandatory, optional, or documented.

### Task B7: Record strictness profile for local candidates
Status: `done`  
Depends on: `B4`, `B5`, `B6`  
Evidence: profile fields mapped to rubric buckets (quality gates, strictness, API/mcp exposure, build/test health, maintenance signal), with concrete evidence links and confidence level.

### Task B8: Compute local candidate ranking delta against Phase-1 shortlist
Status: `done`  
Depends on: `B7`  
Evidence: ranked delta list (`additions`, `re-rank`, `retains`, `drop`) with score changes from rubric deltas and short justification for each movement.

### Task B9: Run web/open-source registry shortlist sweep
Status: `done`  
Depends on: `B1`  
Evidence: `research/phase-2-reports/agent-b-candidate-expansion/artifacts/b9-web-registry-sweep.md`

### Task B10: Produce consolidated Phase-2 lane-B shortlist and evidence pack
Status: `done`  
Depends on: `B8`, `B9`  
Evidence: `research/phase-2-reports/agent-b-candidate-expansion/artifacts/b-local-scan.md`, `research/phase-2-reports/agent-b-candidate-expansion/artifacts/b9-web-registry-sweep.md`

## Execution status (Phase-2)
- B1-B10 completed with updated local shortlist and evidence references.
- B9-B10 web sweep and final ranked package are complete; candidate set now includes web-registry-backed replacements and public MCP/CLI signals.
