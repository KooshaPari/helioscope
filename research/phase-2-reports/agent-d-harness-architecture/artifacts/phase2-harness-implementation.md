# Lane D Evidence: Harness Architecture Implementation

## D1 Command Profile Vocabulary
- Evidence: `harness/src/harness/interfaces.py`
- Canonical buckets implemented: `bootstrap`, `quality`, `test`, `build`, `api`, `runtime`, `static`.

## D2 Repository Discoverer
- Evidence file: `harness/src/harness/discoverer.py`
- Example command: `python3 harness/scripts/run-harness.py discover --root /Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/clones/codex --out artifacts/phase-2/discovery-codex.json`
- Result: `artifacts/phase-2/discovery-codex.json`

## D3 Signal Discovery
- Evidence: `harness/src/harness/discoverer.py`, `harness/src/harness/discoverer.py::_extract_signals`
- Signals collected include AGENTS, workflows, package manifests, makefiles, and script files.

## D4 Result Envelope
- Evidence: `harness/src/harness/interfaces.py`, `harness/src/harness/schema.py`
- Envelope includes: manifest, commands, runs, quality, and result_code.

## D5 Profile Runner
- Evidence: `harness/src/harness/runner.py`
- Runner executes discovered commands with per-profile config and returns stdout/stderr file references.

## D6 Normalization Rules
- Evidence: `harness/src/harness/normalizer.py`
- Buckets normalized from command text (`lint`, `fmt`, `test`, `build`) into QUALITY/STATIC/TEST/BUILD.

## D7 Strictness Escalation
- Evidence: `harness/src/harness/normalizer.py`
- Blockers created when mandatory classes have no matching execution commands.

## D8 Schema Contracts
- Evidence: `harness/schemas/harness-evidence.schema.json`, `harness/schemas/phase2-harness.schema.json`

## D9 Evidence Packager
- Evidence: `harness/src/harness/schema.py`
- Packager function: `evidence_payload()`

## D10 Integration Script
- Evidence: `harness/scripts/run-harness.py`
- Supported subcommands: discover, run, normalize, validate
