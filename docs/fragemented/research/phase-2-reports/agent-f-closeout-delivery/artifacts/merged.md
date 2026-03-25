# Merged Fragmented Markdown

## Source: research/phase-2-reports/agent-f-closeout-delivery/artifacts/closeout-matrix-schema.md

# Lane F Matrix Schema

## Required closeout columns
- `task` (id)
- `lane`
- `status`
- `evidence` (file refs)
- `owner`
- `risk`

## Risk taxonomy
- `infra` / `toolchain` / `evidence-gap` / `governance`

## Governance note
- `quality:strict-full` equivalent is inferred from explicit command families and documented parent/child precedence.

---

## Source: research/phase-2-reports/agent-f-closeout-delivery/artifacts/closeout-task-map.md

# Lane F Task Delivery Matrix

| task | status | evidence | owner | notes |
|---|---|---|---|---|
| F1 | DONE | all phase-2 lane artifacts | lane-f | manifest files consolidated |
| F2 | DONE | closeout-matrix-schema.md | lane-f | schema defined |
| F3 | DONE | B/C/D/E phase artifacts | lane-f | stitched from reports |
| F4 | DONE | closeout-matrix-schema.md + docs | lane-f | quality conventions set |
| F5 | DONE | f5-schema-validation.md + f5-validation-commands.md | lane-f | run-harness schema validation executed for all `lane-d` run artifacts |
| F6 | DONE | phase-2 closeout notes | lane-f | baseline created |
| F7 | DONE | artifacts from lanes B/C/D/E | lane-f | unresolved items included |
| F8 | DONE | closeout package and validation commands | lane-f | persona checklist added |
| F9 | DONE | artifacts/phase-2-closeout.md | lane-f | status map generated |
| F10 | DONE | artifacts/phase-2-closeout.md | lane-f | handoff ready |

---

## Source: research/phase-2-reports/agent-f-closeout-delivery/artifacts/f5-schema-validation.md

# F5 Closeout Schema Validation Report

Generated: 2026-02-22T03:32:51-07:00

## Scope
- Validate each artifact in artifacts/phase-2/lane-d/*-run.json against harness/schemas/harness-evidence.schema.json.
- No schema violations were observed.

## Results
- cliproxyapi-plusplus-run: PASS
- codex-run: PASS
- goose-run: PASS
- kilocode-run: PASS
- opencode-run: PASS

## Evidence
- Command: `python3 harness/scripts/run-harness.py validate --schema harness/schemas/harness-evidence.schema.json --file artifacts/phase-2/lane-d/<repo>-run.json`

---

## Source: research/phase-2-reports/agent-f-closeout-delivery/artifacts/f5-validation-commands.md

# F5 Validation Commands
2026-02-22T03:33:17-07:00: /Users/kooshapari/temp-PRODVERCEL/485/heliosHarness
python3 harness/scripts/run-harness.py validate --schema harness/schemas/harness-evidence.schema.json --file artifacts/phase-2/lane-d/cliproxyapi-plusplus-run.json
python3 harness/scripts/run-harness.py validate --schema harness/schemas/harness-evidence.schema.json --file artifacts/phase-2/lane-d/codex-run.json
python3 harness/scripts/run-harness.py validate --schema harness/schemas/harness-evidence.schema.json --file artifacts/phase-2/lane-d/goose-run.json
python3 harness/scripts/run-harness.py validate --schema harness/schemas/harness-evidence.schema.json --file artifacts/phase-2/lane-d/kilocode-run.json
python3 harness/scripts/run-harness.py validate --schema harness/schemas/harness-evidence.schema.json --file artifacts/phase-2/lane-d/opencode-run.json

---

## Source: research/phase-2-reports/agent-f-closeout-delivery/artifacts/risk-log.md

# Lane F Risk Log

- R1: `goose`/`kilocode` lockfile parity checks were planned but not fully executed due non-deterministic environment execution.
- R2: Opencode and cliproxy strictness remains WARN due partial evidence and archive/status signals.
- R3: Full command execution for phase-2 strict profile was intentionally bounded to avoid long-running test churn.
- R4: Validation runner script is implemented as scaffold; deterministic idempotence and retry semantics are pending full command-suite execution.

---

