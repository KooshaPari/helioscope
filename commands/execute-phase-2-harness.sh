#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat <<'USAGE'
Usage: execute-phase-2-harness.sh [--help]

Run phase-2 harness discovery + execution across clones and write consolidated artifacts:
- artifacts/phase-2/discovery-all.json
- artifacts/phase-2/evidence-all.json
- artifacts/phase-2/evidence-index.ndjson
- artifacts/phase-2/lane-d/integration-matrix.md

Environment:
  HELIOS_HARNESS_ROOT            override project root (default: script parent)
  PHASE2_HARNESS_TIMEOUT         command timeout seconds (default: 20)
  PHASE2_HARNESS_MAX_PARALLEL    max parallel jobs for runner (default: 1)
  PHASE2_SKIP_REPOS              comma-separated repo names to skip in loop
USAGE
}

if [[ "${1:-}" == "--help" ]]; then
  usage
  exit 0
fi

ROOT_DIR=${HELIOS_HARNESS_ROOT:-$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)}
CLONES_DIR="$ROOT_DIR/clones"
ARTIFACT_ROOT="$ROOT_DIR/artifacts/phase-2"
LANE_D_DIR="$ARTIFACT_ROOT/lane-d"
SCRIPT_PATH="$ROOT_DIR/harness/scripts/run-harness.py"
TIMEOUT_SEC=${PHASE2_HARNESS_TIMEOUT:-20}
MAX_PARALLEL=${PHASE2_HARNESS_MAX_PARALLEL:-1}
SKIP_REPOS=${PHASE2_SKIP_REPOS:-}

mkdir -p "$LANE_D_DIR"

echo "[phase-2-harness] root: $ROOT_DIR"
echo "[phase-2-harness] clones: $CLONES_DIR"

echo "[phase-2-harness] skip list: $SKIP_REPOS"

for repo_path in "$CLONES_DIR"/*; do
  [[ -d "$repo_path" ]] || continue
  repo_name=$(basename "$repo_path")

  if [[ ",${SKIP_REPOS}," == *",${repo_name},"* ]]; then
    echo "[phase-2-harness] skip $repo_name"
    continue
  fi

  discovery_out="$ARTIFACT_ROOT/discovery-${repo_name}.json"
  run_out="$LANE_D_DIR/${repo_name}-run.json"

  echo "[phase-2-harness] discover $repo_name"
  if ! python3 "$SCRIPT_PATH" discover --root "$repo_path" --out "$discovery_out"; then
    echo "[phase-2-harness] WARN: discover failed for $repo_name"
  fi

  echo "[phase-2-harness] run $repo_name"
  if ! python3 "$SCRIPT_PATH" run \
    --repo "$repo_path" \
    --out "$run_out" \
    --profile strict-full \
    --timeout "$TIMEOUT_SEC" \
    --max-parallel "$MAX_PARALLEL" \
    --continue-on-fail; then
    echo "[phase-2-harness] WARN: run returned non-zero for $repo_name"
  fi

done

ROOT_DIR="$ROOT_DIR" \
python3 - <<PY
from datetime import datetime, timezone
from pathlib import Path
import json
import os

ROOT_DIR = Path(os.environ["ROOT_DIR"])
CLONES_DIR = ROOT_DIR / "clones"
ARTIFACT_ROOT = ROOT_DIR / "artifacts/phase-2"
LANE_D_DIR = ARTIFACT_ROOT / "lane-d"

discoveries = []
evidence = {
    "generated_at": datetime.now(tz=timezone.utc).isoformat(),
    "artifact_root": str(ARTIFACT_ROOT),
    "targets": [],
}

for repo_path in sorted(p for p in CLONES_DIR.iterdir() if p.is_dir()):
    repo_name = repo_path.name

    if not (ARTIFACT_ROOT / f"discovery-{repo_name}.json").exists():
        discovery_payload = {"repo_name": repo_name, "status": "missing", "path": str(ARTIFACT_ROOT / f"discovery-{repo_name}.json")}
    else:
        try:
            discovery_payload = json.loads((ARTIFACT_ROOT / f"discovery-{repo_name}.json").read_text())
            discovery_payload["status"] = "ok"
        except Exception as e:  # noqa: BLE001
            discovery_payload = {
                "repo_name": repo_name,
                "status": "error",
                "path": str(ARTIFACT_ROOT / f"discovery-{repo_name}.json"),
                "error": str(e),
            }

    run_path = LANE_D_DIR / f"{repo_name}-run.json"
    if not run_path.exists():
        run_entry = {
            "repo_name": repo_name,
            "result_code": "MISSING",
            "path": str(run_path),
        }
    else:
        try:
            run_payload = json.loads(run_path.read_text())
            quality = run_payload.get("quality", {})
            run_entry = {
                "repo_name": repo_name,
                "result_code": run_payload.get("result_code", "WARN"),
                "repo_id": run_payload.get("manifest", {}).get("repo_id") or run_payload.get("repo_id") or repo_name,
                "path": str(run_path),
                "created_at": run_payload.get("created_at"),
                "commands": len(run_payload.get("commands", [])),
                "runs": len(run_payload.get("runs", [])),
                "blockers": len(quality.get("blockers", [])),
                "warnings": len(quality.get("warnings", [])),
                "quality": quality,
            }
        except Exception as e:  # noqa: BLE001
            run_entry = {
                "repo_name": repo_name,
                "result_code": "ERROR",
                "path": str(run_path),
                "error": str(e),
            }

    evidence["targets"].append({"repo_name": repo_name, "discovery": discovery_payload, "run": run_entry})
    discoveries.append(discovery_payload)

(ARTIFACT_ROOT / "discovery-all.json").write_text(json.dumps(discoveries, indent=2))
(ARTIFACT_ROOT / "evidence-all.json").write_text(json.dumps(evidence, indent=2))

index_lines = []
for target in evidence["targets"]:
    run_info = target["run"]
    index_lines.append(
        json.dumps(
            {
                "repo": run_info["repo_name"],
                "result_code": run_info.get("result_code", "MISSING"),
                "commands": run_info.get("commands", 0),
                "runs": run_info.get("runs", 0),
                "blockers": run_info.get("blockers", 0),
                "warnings": run_info.get("warnings", 0),
            }
        )
    )
(ARTIFACT_ROOT / "evidence-index.ndjson").write_text("\n".join(index_lines) + ("\n" if index_lines else ""))

status_counts = {}
for target in evidence["targets"]:
    status = target["run"].get("result_code", "MISSING")
    status_counts[status] = status_counts.get(status, 0) + 1

matrix_lines = [
    "# Integration Matrix",
    "",
    f"Generated: {evidence['generated_at']}",
    "",
    "| Repo | Discovery | Result | Commands | Runs | Blockers | Warnings |",
    "| --- | --- | --- | --- | --- | --- | --- |",
]

for target in evidence["targets"]:
    run_info = target["run"]
    matrix_lines.append(
        f"| {target['repo_name']} | {target['discovery'].get('status')} | "
        f"{run_info.get('result_code', 'MISSING')} | {run_info.get('commands', 0)} | "
        f"{run_info.get('runs', 0)} | {run_info.get('blockers', 0)} | {run_info.get('warnings', 0)} |"
    )

matrix_lines.append("")
matrix_lines.append("## Summary")
for key in sorted(status_counts):
    matrix_lines.append(f"- {key}: {status_counts[key]}")
(LANE_D_DIR / "integration-matrix.md").write_text("\n".join(matrix_lines) + "\n")
PY

echo "[phase-2-harness] emitted artifacts:"
echo "  - $ARTIFACT_ROOT/discovery-all.json"
echo "  - $ARTIFACT_ROOT/evidence-all.json"
echo "  - $ARTIFACT_ROOT/evidence-index.ndjson"
echo "  - $ARTIFACT_ROOT/lane-d/integration-matrix.md"
