# Phase-1 Closeout: CLI/API/SDK Harness Research

## Status snapshot
- Phase-1 tasks executed: 70 / 70 planned.
- Current status counts:
  - `done`: 70
  - `todo`: 0
- Lane reports created:
  - `research/phase-1-reports/agent-a-codex.md`
  - `research/phase-1-reports/agent-b-opencode.md`
  - `research/phase-1-reports/agent-c-goose.md`
  - `research/phase-1-reports/agent-d-kilo.md`
  - `research/phase-1-reports/agent-e-discovery.md`
  - `research/phase-1-reports/agent-f-governance.md`
- Synthesis artifacts generated:
  - `research/oss-cli-matrix.md`
  - `research/harness-spec.md`
  - `commands/clone-playbook.md`

## What changed
1. Completed all Phase-1 tasks (`A1–G10`), with evidence artifacts created/updated under `wbs/phase-1-reports`.
2. Advanced closeout to reflect partial progress and next-open work.
3. Added and preserved baseline strictness mapping and parent/child fallback model for quality equivalence.
4. Drafted Phase-1 closeout schema with ranked shortlist.

## Confirmed technical observations
- `goose` clone in harness currently had broken ref state; API/research copy was stable enough to complete evidence gathering.
- `opencode` is archived and explicitly moved to Crush; quality profile is lighter than other primaries.
- `codex` shows strong multi-workflow quality surface but no single named `quality:strict-full` command; strictness must be normalized by workflow mapping.
- `kilocode` has explicit strict command surface via monorepo CI and command scripts.
- `cliproxyapi-plusplus` includes a practical `task quality` alias with lint/test/typecheck checks and is a strong proxy-type onboarding candidate.
- `pluggedin-mcp-proxy` and local proxy-family repos provide useful API/MCP candidate coverage for future harness extensibility.

## Risks / open items
- Incomplete upstream provenance capture for some local clone-only candidates (non-core).
- Some local repo states are noisy/dirty (large diffs in working tree outside this phase scope).
- `cli proxy` family candidate quality depth remains asymmetric; rank kept as Priority-1 unless hardening evidence is collected.

## Phase-2 handoff checklist
- [ ] Normalize and verify strictness aliases per repository (`STRICT_FULL` mapping file).
- [ ] Capture machine-readable execution logs for each candidate.
- [ ] Freeze shortlist by governance criteria (max-strictness threshold + API maturity).
- [ ] Add CLI/API/SDK harness schema and CI execution driver.
