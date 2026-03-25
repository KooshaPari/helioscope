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
