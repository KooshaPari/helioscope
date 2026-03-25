from __future__ import annotations

import json
from dataclasses import dataclass, field, asdict
from datetime import datetime, timezone
from enum import Enum
from pathlib import Path
from typing import Any


class EvidenceBucket(str, Enum):
    BOOTSTRAP = "bootstrap"
    STATIC = "static"
    QUALITY = "quality"
    TEST = "test"
    BUILD = "build"
    API = "api"
    RUNTIME = "runtime"


class TaskStatus(str, Enum):
    PASS = "PASS"
    WARN = "WARN"
    FAIL = "FAIL"
    SKIP = "SKIP"


class QualityProfile(str, Enum):
    STRICT_FULL = "strict-full"
    STRICT = "strict"
    STRICT_LIGHT = "strict-light"


@dataclass
class CanonicalCommand:
    command: str
    bucket: EvidenceBucket
    cwd: str
    required: bool = True
    rationale: str = ""
    source: str = ""


@dataclass
class RepoManifest:
    repo_id: str
    root: str
    remote_url: str
    default_branch: str | None
    discovered_at: str
    commit: str | None = None
    branch: str | None = None

    @classmethod
    def from_repo(cls, repo_root: Path, repo_id: str) -> "RepoManifest":
        from subprocess import CalledProcessError, check_output

        root = str(repo_root.resolve())
        try:
            remote = check_output(["git", "-C", root, "remote", "get-url", "origin"], text=True).strip()
        except CalledProcessError:
            remote = "(no-remote)"
        try:
            branch = check_output(["git", "-C", root, "branch", "--show-current"], text=True).strip()
            commit = check_output(["git", "-C", root, "rev-parse", "--short", "HEAD"], text=True).strip()
        except CalledProcessError:
            branch = "(no-git)"
            commit = ""
        default_branch = branch or "main"
        return cls(
            repo_id=repo_id,
            root=root,
            remote_url=remote,
            default_branch=default_branch,
            discovered_at=datetime.now(tz=timezone.utc).isoformat(),
            commit=commit,
            branch=branch,
        )


@dataclass
class DiscoverInput:
    repo_root: str
    max_scan_depth: int = 3


@dataclass
class DiscoverOutput:
    manifest: RepoManifest
    signals: list[str] = field(default_factory=list)
    buckets: dict[EvidenceBucket, list[CanonicalCommand]] = field(default_factory=dict)
    files: list[str] = field(default_factory=list)
    raw_events: list[dict[str, Any]] = field(default_factory=list)

    def to_json(self) -> str:
        return json.dumps(
            {
                "manifest": asdict(self.manifest),
                "signals": self.signals,
                "buckets": {k.value: [asdict(v) for v in vals] for k, vals in self.buckets.items()},
                "files": self.files,
                "raw_events": self.raw_events,
            },
            indent=2,
        )


@dataclass
class RunResult:
    command: str
    bucket: str
    returncode: int
    started_at: str
    finished_at: str
    stdout_file: str
    stderr_file: str
    duration_ms: int
    artifact_dir: str
    skipped: bool = False
    attempts: int = 1
    timed_out: bool = False
    error: str | None = None


@dataclass
class RunnerConfig:
    timeout_seconds: int = 1200
    continue_on_fail: bool = True
    profile: str = "strict-full"
    max_parallel_jobs: int = 2


@dataclass
class QualityNormalization:
    discovered_rules: list[str] = field(default_factory=list)
    inferred_profile: str = "strict-light"
    mapped_buckets: dict[str, list[str]] = field(default_factory=dict)
    blockers: list[dict[str, Any]] = field(default_factory=list)
    warnings: list[dict[str, Any]] = field(default_factory=list)

    def score(self) -> dict[str, int]:
        pass_count = 0
        warn_count = 0
        block_count = 0
        for blocker in self.blockers:
            if blocker.get("severity", "warn") == "block":
                block_count += 1
            else:
                warn_count += 1
        for warning in self.warnings:
            if warning.get("severity", "warn") == "warn":
                warn_count += 1
        if block_count == 0 and warn_count == 0:
            pass_count = 1
        return {
            "pass": pass_count,
            "warn": warn_count,
            "block": block_count,
        }
