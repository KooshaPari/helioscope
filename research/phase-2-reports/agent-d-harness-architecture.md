<DONE>
# Phase-2 Lane D: Harness Architecture Planning

Scope: implementation-ready plan for the harness core (`discoverer`, `runner`, `quality normalizer`, `evidence schema`) with concrete file layouts, commands, and typed interfaces.

## Baseline assumptions
- Language for initial implementation: Python 3.12 (for portable command execution and JSON schema tooling).
- Output is machine-validated and CI-consumable via JSONL and JSON schema.
- Candidate target repos are under `/Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/clones/*`.

### Canonical file targets to be introduced
- `harness/pyproject.toml`
- `harness/src/harness/discoverer.py`
- `harness/src/harness/interfaces.py`
- `harness/src/harness/runner.py`
- `harness/src/harness/normalizer.py`
- `harness/src/harness/schema.py`
- `harness/schemas/harness-evidence.schema.json`
- `harness/schemas/phase2-harness.schema.json`
- `harness/scripts/run-harness.py`
- `harness/tests/test_discoverer.py`
- `harness/tests/test_runner_contract.py`
- `harness/tests/test_normalizer.py`
- `harness/tests/test_schema_roundtrip.py`

## Tasks

### Task D1: Lock architecture contract and command profile vocabulary
Status: `done`
Dependency: none
Evidence: `research/harness-spec.md`, `research/phase-2-reports/agent-d-harness-architecture.md` (section "Canonical file targets")

Command: `python3 -m compileall harness/src/harness` after edits.

```python
# harness/src/harness/interfaces.py
from enum import Enum

class EvidenceBucket(str, Enum):
    BOOTSTRAP = "bootstrap"
    STATIC = "static"
    TEST = "test"
    BUILD = "build"
    API = "api"

@dataclass
class CanonicalCommand:
    command: str
    cwd: str
    bucket: EvidenceBucket
    required: bool
    rationale: str
    source: str
```

### Task D2: Implement repository discoverer input model and CLI discoverer interface
Status: `done`
Dependency: `D1`
Evidence: `harness/src/harness/interfaces.py`, `harness/src/harness/discoverer.py`, `artifacts/phase-2/01-discovery-model.json`

Command: `python3 harness/scripts/run-harness.py discover --root /Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/clones/codex --out artifacts/phase-2/discovery-codex.json`

```python
# harness/src/harness/interfaces.py
@dataclass
class RepoManifest:
    repo_id: str
    root: str
    remote_url: str
    default_branch: str | None
    discovered_at: str

@dataclass
class DiscoverInput:
    repo_root: str
    max_scan_depth: int = 3

@dataclass
class DiscoverOutput:
    manifest: RepoManifest
    signals: list[str]
    buckets: dict[str, list[CanonicalCommand]]
    files: list[str]
    raw_events: list[dict]
```

### Task D3: Build file-system discovery logic for strictness and quality signals
Status: `done`
Dependency: `D2`
Evidence: `harness/src/harness/discoverer.py`, `artifacts/phase-2/discovery-signal-matrix.md`

Command: `python3 -m pytest harness/tests/test_discoverer.py -q`

```python
# harness/src/harness/discoverer.py
class Discoverer:
    def discover(self, input: DiscoverInput) -> DiscoverOutput: ...
    def _extract_signals(self, root: str) -> list[str]: ...
    def _extract_scripts(self, root: str, file_patterns: list[str]) -> list[dict]: ...
```

### Task D4: Define normalized command execution result envelope and run-mode policy
Status: `done`
Dependency: `D1`
Evidence: `harness/src/harness/interfaces.py`, `artifacts/phase-2/runner-contract.md`

Command: `python3 -m pytest harness/tests/test_runner_contract.py -q`

```python
@dataclass
class RunResult:
    command: str
    bucket: EvidenceBucket
    returncode: int
    started_at: str
    finished_at: str
    stdout_file: str
    stderr_file: str
    duration_ms: int
    artifact_dir: str

@dataclass
class RunnerConfig:
    timeout_seconds: int = 1200
    max_parallel_jobs: int = 2
    continue_on_fail: bool = True
    profile: str = "strict-light"
```

### Task D5: Implement command runner with profile-driven execution
Status: `done`
Dependency: `D3`, `D4`
Evidence: `harness/src/harness/runner.py`, `artifacts/phase-2/runner-commands-codex.json`

Command: `python3 harness/scripts/run-harness.py run --repo /Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/clones/codex --profile strict-full --out artifacts/phase-2/run-codex.json`

```python
# harness/src/harness/runner.py
class Runner:
    def run_profile(self, manifest: RepoManifest, commands: list[CanonicalCommand], config: RunnerConfig) -> list[RunResult]: ...
    def run_command(self, cmd: CanonicalCommand, cwd: str) -> RunResult: ...
    def _enforce_profile(self, profile: str) -> dict: ...
```

### Task D6: Define quality normalization mapping and semantic output model
Status: `done`
Dependency: `D1`, `D5`
Evidence: `harness/src/harness/normalizer.py`, `artifacts/phase-2/quality-map.yaml`

Command: `python3 -m pytest harness/tests/test_normalizer.py -q`

```python
# harness/src/harness/normalizer.py
class QualityProfile(str, Enum):
    STRICT_FULL = "strict-full"
    STRICT = "strict"
    STRICT_LIGHT = "strict-light"

@dataclass
class QualityNormalization:
    discovered_rules: list[str]
    inferred_profile: QualityProfile
    mapped_buckets: dict[str, list[str]]
    blockers: list[dict]
    warnings: list[dict]

class QualityNormalizer:
    def normalize(self, run_results: list[RunResult], discovered: list[CanonicalCommand]) -> QualityNormalization: ...
```

### Task D7: Implement quality command normalization rules and strictness escalation logic
Status: `done`
Dependency: `D6`
Evidence: `harness/src/harness/normalizer.py`, `harness/test-fixtures/codex_quality_rules.md`, `artifacts/phase-2/quality-normalization-codex.json`

Command: `python3 harness/scripts/run-harness.py normalize --in artifacts/phase-2/run-codex.json --out artifacts/phase-2/normalized-codex.json`

Normalization rules example:
- `pnpm lint` or `npm run lint` without skips -> `STATIC` hard-required
- `pnpm test` + `coverage` markers -> `TEST` required under `strict` and `strict-full`
- `cargo clippy`/`ruff` + explicit `--fix` pattern with warning only -> warning escalation under `strict-light`, blocker under `strict-full`

### Task D8: Define evidence schema + strict JSON validation contracts
Status: `done`
Dependency: `D3`, `D6`
Evidence: `harness/schemas/harness-evidence.schema.json`, `harness/schemas/phase2-harness.schema.json`, `harness/tests/test_schema_roundtrip.py`

Command: `python3 harness/scripts/run-harness.py validate --schema harness/schemas/phase2-harness.schema.json --file artifacts/phase-2/evidence-codex.json`

```json
{
  "$id": "https://example.com/schemas/harness-evidence",
  "type": "object",
  "required": ["repo_id", "manifest", "commands", "runs", "quality", "result_code", "evidence"],
  "properties": {
    "repo_id": { "type": "string" },
    "result_code": { "enum": ["PASS", "WARN", "FAIL"] },
    "commands": { "type": "array" },
    "quality": { "type": "object" },
    "evidence": { "type": "array" }
  }
}
```

### Task D9: Build end-to-end evidence packager and artifact emission format
Status: `done`
Dependency: `D7`, `D8`
Evidence: `harness/src/harness/schema.py`, `harness/scripts/run-harness.py`, `artifacts/phase-2/evidence-codex.json`, `artifacts/phase-2/evidence-index.ndjson`

Command: `python3 harness/scripts/run-harness.py emit --repo /Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/clones/codex --strictness strict --evidence-root artifacts/phase-2`

```python
# harness/src/harness/schema.py
@dataclass
class HarnessEvidence:
    repo_id: str
    manifest: RepoManifest
    command_inventory: list[CanonicalCommand]
    run_results: list[RunResult]
    quality: QualityNormalization
    result_code: str
    evidence: list[dict]

class EvidenceSerializer:
    def to_json(self, ev: HarnessEvidence) -> dict: ...
    def write_json(self, ev: HarnessEvidence, path: str) -> None: ...
    def write_ndjson(self, batch: list[HarnessEvidence], path: str) -> None: ...
```

### Task D10: Add CLI integration tests and executable integration matrix run
Status: `done`
Dependency: `D9`
Evidence: `harness/tests/test_cli_integration.py`, `commands/execute-phase-2-harness.sh`, `artifacts/phase-2/integration-matrix.md`

Commands:
- `bash commands/execute-phase-2-harness.sh`
- `python3 harness/scripts/run-harness.py discover --root /Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/clones --out artifacts/phase-2/discovery-all.json`
- `python3 harness/scripts/run-harness.py run --repo <target-repo> --out artifacts/phase-2/<repo>-run.json --profile strict-full`

`commands/execute-phase-2-harness.sh` should generate:
1) `/Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/artifacts/phase-2/discovery-all.json`
2) `/Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/artifacts/phase-2/evidence-all.json`
3) `/Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/artifacts/phase-2/evidence-index.ndjson`
4) `/Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/artifacts/phase-2/lane-d/integration-matrix.md`

Execution evidence:
- `harness/tests/test_cli_integration.py` added for smoke validation of the full CLI script path.
- `commands/execute-phase-2-harness.sh` executed across local repos; consolidated outputs now include `discovery-all.json`, `evidence-all.json`, `evidence-index.ndjson`, and `artifacts/phase-2/lane-d/integration-matrix.md`.

## Execution status (Phase-2)
- D1-D10 implemented as scaffolding artifacts + end-to-end script execution evidence.
- Remaining: resolve full kilocode command coverage in a later pass; current strict-full matrix now captures a constrained strict command sample for kilocode (5/34 commands executed with 12s timeouts) as `FAIL`, which is better than the prior `MISSING` state and leaves full parity coverage as a follow-up item.
