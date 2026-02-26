from __future__ import annotations

from dataclasses import dataclass
from datetime import datetime, timezone
from pathlib import Path
from subprocess import Popen, TimeoutExpired
from time import sleep
from typing import Iterable

from .interfaces import CanonicalCommand
from .id_utils import artifact_slug


class RunnerConfig:
    def __init__(
        self,
        timeout_seconds: int = 1200,
        continue_on_fail: bool = True,
        max_parallel_jobs: int = 2,
        profile: str = "strict-full",
        retries: int = 0,
        retry_delay_seconds: float = 1.0,
        budget_seconds: int | None = None,
    ):
        self.timeout_seconds = timeout_seconds
        self.continue_on_fail = continue_on_fail
        self.max_parallel_jobs = max_parallel_jobs
        self.profile = profile
        self.retries = retries
        self.retry_delay_seconds = retry_delay_seconds
        self.budget_seconds = budget_seconds


@dataclass
class RunResult:
    command: str
    bucket: str
    returncode: int
    attempts: int
    timed_out: bool
    started_at: str
    finished_at: str
    stdout_file: str
    stderr_file: str
    duration_ms: int
    artifact_dir: str
    error: str | None = None
    skipped: bool = False


class Runner:
    def __init__(self, config: RunnerConfig | None = None):
        self.config = config or RunnerConfig()

    def run_profile(self, commands: Iterable[CanonicalCommand], root: str) -> list[RunResult]:
        results: list[RunResult] = []
        budget = self.config.budget_seconds

        for command in commands:
            if budget is not None and budget <= 0:
                results.append(
                    RunResult(
                        command=command.command,
                        bucket=command.bucket.value,
                        returncode=0,
                        attempts=0,
                        timed_out=False,
                        started_at=datetime.now(tz=timezone.utc).isoformat(),
                        finished_at=datetime.now(tz=timezone.utc).isoformat(),
                        stdout_file="",
                        stderr_file="",
                        duration_ms=0,
                        artifact_dir=str(Path(root) / ".helios-harness"),
                        error="budget exhausted",
                        skipped=True,
                    )
                )
                if command.required:
                    break
                continue

            result = self.run_command(command, root)
            results.append(result)

            if budget is not None:
                budget = max(0, budget - (result.duration_ms // 1000))

            if command.required and result.returncode != 0 and not self.config.continue_on_fail:
                break

        return results

    def run_command(self, cmd: CanonicalCommand, cwd: str) -> RunResult:
        artifact_dir = Path(cwd) / ".helios-harness"
        artifact_dir.mkdir(exist_ok=True)
        stdout_path = artifact_dir / f"{self._slug(cmd.command)}.stdout.log"
        stderr_path = artifact_dir / f"{self._slug(cmd.command)}.stderr.log"

        started = datetime.now(timezone.utc)
        attempts = 0
        timed_out = False
        last_error: str | None = None
        returncode = 0

        for attempt in range(1, self.config.retries + 2):
            attempts = attempt
            try:
                with stdout_path.open("ab") as stdout, stderr_path.open("ab") as stderr:
                    proc = Popen(
                        cmd.command,
                        cwd=str(Path(cwd) / cmd.cwd),
                        shell=True,
                        executable="/bin/bash",
                        stdout=stdout,
                        stderr=stderr,
                    )
                    returncode = proc.wait(timeout=self.config.timeout_seconds)
                break
            except TimeoutExpired:
                proc.kill()
                timed_out = True
                returncode = 124
                last_error = f"timeout after {self.config.timeout_seconds}s"
            except OSError as exc:
                returncode = 127
                last_error = str(exc)
                break

            if returncode != 0 and attempt <= self.config.retries:
                if self.config.retry_delay_seconds > 0:
                    sleep(self.config.retry_delay_seconds)
            else:
                break

        finished = datetime.now(timezone.utc)
        duration_ms = int((finished - started).total_seconds() * 1000)

        return RunResult(
            command=cmd.command,
            bucket=cmd.bucket.value,
            returncode=returncode,
            attempts=attempts,
            timed_out=timed_out,
            started_at=started.isoformat(),
            finished_at=finished.isoformat(),
            stdout_file=str(stdout_path),
            stderr_file=str(stderr_path),
            duration_ms=duration_ms,
            artifact_dir=str(artifact_dir),
            error=last_error,
        )

    def _slug(self, command: str) -> str:
        return artifact_slug(command, length=12)


class QualityProfile(str):
    STRICT_FULL = "strict-full"
    STRICT = "strict"
    STRICT_LIGHT = "strict-light"
