# Phase-2 Lane E Evidence (E3-E10)

## E3 Build command discovery and normalization runner
- Evidence command: `python3 -m pytest harness/tests/test_discoverer.py harness/tests/test_normalizer.py -q`
- Result: PASS (`10 passed` after adding deterministic plan normalization and replay behavior)

## E5 Dry-run mode + deterministic plan diff
- Evidence command: `python3 -m pytest harness/tests/test_run_harness.py -q`
- Result: PASS (both dry-run and replay checks validate deterministic plan hash behavior)
- Dry-run output now includes:
  - `plan_hash`
  - `plan` in canonical order
  - `command_count`
  - `reproducibility` environment block
  - replay block when `--replay` is provided

## E7 Runner execution with retry + budget
- Evidence command: `python3 -m pytest harness/tests/test_runner_contract.py -q`
- Result: PASS (`runner` tests cover retries and timeout/budget behavior, unchanged semantics)

## E8 Evidence bundles with reproducibility metadata
- Evidence files now include `reproducibility` and `command_count` in both dry-run and full run payloads (`run` command output).
- Evidence command: `python3 -m pytest harness/tests/test_run_harness.py -q`

## E9 Replay/idempotency and hash parity
- Evidence command: `python3 -m pytest harness/tests/test_run_harness.py -q`
- Result: PASS (replay path validates same-plan detection, prior plan hash and diff reporting)

## E10 Lane-E runbook and escalation drill
- Evidence command: `python3 -m pytest harness/tests -q`
- Result: PASS (all harness tests green) and runbook file exists at:
  - `research/phase-2-reports/agent-e-validation-automation/artifacts/e5-e10-evidence.md`
