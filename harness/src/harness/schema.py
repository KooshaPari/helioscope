from __future__ import annotations

from dataclasses import asdict
from typing import Any

from .interfaces import DiscoverOutput, RunResult, QualityNormalization


def evidence_payload(manifest: DiscoverOutput, runs: list[RunResult], quality: QualityNormalization) -> dict[str, Any]:
    return {
        "repo_id": manifest.manifest.repo_id,
        "manifest": asdict(manifest.manifest),
        "commands": [asdict(cmd) for group in manifest.buckets.values() for cmd in group],
        "runs": [
            {
                "command": r.command,
                "bucket": r.bucket,
                "returncode": r.returncode,
                "attempts": r.attempts,
                "timed_out": r.timed_out,
                "started_at": r.started_at,
                "finished_at": r.finished_at,
                "stdout_file": r.stdout_file,
                "stderr_file": r.stderr_file,
                "duration_ms": r.duration_ms,
                "error": getattr(r, "error", None),
                "skipped": getattr(r, "skipped", False),
                "artifact_dir": r.artifact_dir,
            }
            for r in runs
        ],
        "quality": asdict(quality),
        "result_code": _result_code(runs),
        "evidence": [
            {
                "path": f,
                "artifact_type": "signal",
            }
            for f in manifest.files
        ],
    }


def _result_code(runs: list[RunResult]) -> str:
    if not runs:
        return "WARN"
    bad = any(r.returncode not in (0, None) for r in runs)
    return "FAIL" if bad else "PASS"
