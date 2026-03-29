from harness.interfaces import RunResult
from harness.normalizer import QualityNormalizer


def test_quality_infer_schema():
    q = QualityNormalizer().normalize([], [])
    assert q.inferred_profile == "strict-full"


def test_quality_normalizer_detects_blockers_from_executed_markers():
    q = QualityNormalizer().normalize(
        [
            RunResult(
                command="pnpm lint",
                bucket="STATIC",
                returncode=0,
                attempts=1,
                timed_out=False,
                started_at="2026-01-01T00:00:00Z",
                finished_at="2026-01-01T00:00:00Z",
                stdout_file="",
                stderr_file="",
                duration_ms=1,
                artifact_dir=".",
            ),
            RunResult(
                command="npm test",
                bucket="TEST",
                returncode=0,
                attempts=1,
                timed_out=False,
                started_at="2026-01-01T00:00:00Z",
                finished_at="2026-01-01T00:00:00Z",
                stdout_file="",
                stderr_file="",
                duration_ms=1,
                artifact_dir=".",
            ),
            RunResult(
                command="go build ./...",
                bucket="BUILD",
                returncode=0,
                attempts=1,
                timed_out=False,
                started_at="2026-01-01T00:00:00Z",
                finished_at="2026-01-01T00:00:00Z",
                stdout_file="",
                stderr_file="",
                duration_ms=1,
                artifact_dir=".",
            ),
        ],
        [],
    )
    assert q.blockers == []
