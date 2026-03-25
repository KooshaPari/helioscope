"""Tests for schema module."""

import pytest
from datetime import datetime
import sys

sys.path.insert(0, "src")

from harness.schema import evidence_payload, _result_code
from harness.interfaces import DiscoverOutput, RepoManifest, RunResult, QualityNormalization


class TestResultCode:
    """Tests for _result_code function."""

    def test_empty_runs_returns_warn(self):
        """Empty runs list should return WARN."""
        assert _result_code([]) == "WARN"

    def test_all_pass_returns_pass(self):
        """All successful runs should return PASS."""
        runs = [
            RunResult(
                command="test",
                bucket="test",
                returncode=0,
                attempts=1,
                timed_out=False,
                started_at=datetime.now(),
                finished_at=datetime.now(),
                stdout_file="",
                stderr_file="",
                duration_ms=100,
                artifact_dir="",
            )
        ]
        assert _result_code(runs) == "PASS"

    def test_failure_returns_fail(self):
        """Any failed run should return FAIL."""
        runs = [
            RunResult(
                command="test",
                bucket="test",
                returncode=1,  # Non-zero return code
                attempts=1,
                timed_out=False,
                started_at=datetime.now(),
                finished_at=datetime.now(),
                stdout_file="",
                stderr_file="",
                duration_ms=100,
                artifact_dir="",
            )
        ]
        assert _result_code(runs) == "FAIL"

    def test_none_returncode_passes(self):
        """None returncode should be treated as success."""
        runs = [
            RunResult(
                command="test",
                bucket="test",
                returncode=None,
                attempts=1,
                timed_out=False,
                started_at=datetime.now(),
                finished_at=datetime.now(),
                stdout_file="",
                stderr_file="",
                duration_ms=100,
                artifact_dir="",
            )
        ]
        assert _result_code(runs) == "PASS"


class TestEvidencePayload:
    """Tests for evidence_payload function."""

    def test_evidence_payload_with_empty_data(self):
        """Evidence payload with minimal data."""
        manifest = DiscoverOutput(
            manifest=RepoManifest(
                repo_id="test-repo",
                root="/tmp/test",
                remote_url="",
                default_branch="main",
                discovered_at="2024-01-01",
            ),
            files=[],
            buckets={},
        )

        runs: list[RunResult] = []

        quality = QualityNormalization(
            blockers=[],
            warnings=[],
        )

        result = evidence_payload(manifest, runs, quality)

        assert result["repo_id"] == "test-repo"
        assert result["result_code"] == "WARN"
