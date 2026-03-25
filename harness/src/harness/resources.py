"""Resource management utilities for subprocess and process handling.

This module provides safe wrappers around subprocess.Popen and other
resource-intensive operations to prevent leaks.
"""

import os
import subprocess
import threading
from contextlib import contextmanager
from typing import Any, Callable, Generator, Optional
from dataclasses import dataclass, field
import time


@dataclass
class ProcessMetrics:
    """Metrics for process execution."""

    pid: int = 0
    return_code: int = 0
    elapsed_seconds: float = 0.0
    stdout_lines: int = 0
    stderr_lines: int = 0
    fd_count_start: int = 0
    fd_count_end: int = 0


class SubprocessManager:
    """Manages subprocess lifecycle with proper resource cleanup.

    Usage:
        with SubprocessManager(cmd) as proc:
            stdout, stderr = proc.communicate()
    """

    _instance: Optional["SubprocessManager"] = None
    _lock = threading.Lock()

    def __init__(self):
        self._processes: dict[int, subprocess.Popen] = {}
        self._metrics: list[ProcessMetrics] = []

    @classmethod
    def get_instance(cls) -> "SubprocessManager":
        """Get singleton instance."""
        if cls._instance is None:
            with cls._lock:
                if cls._instance is None:
                    cls._instance = cls()
        return cls._instance

    def register(self, proc: subprocess.Popen) -> None:
        """Register a process for tracking."""
        if proc and proc.pid:
            self._processes[proc.pid] = proc

    def unregister(self, pid: int) -> Optional[subprocess.Popen]:
        """Unregister a process."""
        return self._processes.pop(pid, None)

    def get_active_count(self) -> int:
        """Get count of active processes."""
        return len(self._processes)

    def get_metrics(self) -> list[ProcessMetrics]:
        """Get collected metrics."""
        return self._metrics

    def cleanup_all(self) -> int:
        """Cleanup all tracked processes. Returns count cleaned."""
        count = 0
        for pid, proc in list(self._processes.items()):
            try:
                if proc.poll() is None:
                    proc.terminate()
                    try:
                        proc.wait(timeout=5)
                    except subprocess.TimeoutExpired:
                        proc.kill()
                proc.stdout.close()
                proc.stderr.close()
                if proc.stdin:
                    proc.stdin.close()
                count += 1
            except Exception:
                pass
        self._processes.clear()
        return count


@contextmanager
def safe_popen(*args: Any, **kwargs: Any) -> Generator[subprocess.Popen, None, None]:
    """Context manager for subprocess.Popen with automatic cleanup.

    Ensures all file handles are closed and processes are terminated
    even if an exception occurs.

    Usage:
        with safe_popen(["ls", "-la"], stdout=PIPE) as proc:
            output = proc.stdout.read()
    """
    proc = None
    try:
        proc = subprocess.Popen(*args, **kwargs)
        SubprocessManager.get_instance().register(proc)
        yield proc
    finally:
        if proc:
            try:
                SubprocessManager.get_instance().unregister(proc.pid)
                # Close pipes
                if proc.stdout:
                    proc.stdout.close()
                if proc.stderr:
                    proc.stderr.close()
                if proc.stdin:
                    proc.stdin.close()
                # Terminate if still running
                if proc.poll() is None:
                    proc.terminate()
                    try:
                        proc.wait(timeout=5)
                    except subprocess.TimeoutExpired:
                        proc.kill()
            except Exception:
                pass


def run_command(
    cmd: list[str],
    cwd: Optional[str] = None,
    env: Optional[dict] = None,
    timeout: Optional[float] = None,
    check: bool = True,
) -> tuple[int, str, str]:
    """Run a command with proper resource management.

    Returns:
        (return_code, stdout, stderr)
    """
    with safe_popen(
        cmd,
        cwd=cwd,
        env=env,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True,
    ) as proc:
        try:
            stdout, stderr = proc.communicate(timeout=timeout)
            if check and proc.returncode != 0:
                raise subprocess.CalledProcessError(proc.returncode, cmd, stdout, stderr)
            return proc.returncode, stdout, stderr
        except subprocess.TimeoutExpired:
            proc.kill()
            proc.wait()
            raise


@contextmanager
def fd_tracker() -> Generator[dict[str, int], None, None]:
    """Context manager to track file descriptor usage.

    Usage:
        with fd_tracker() as tracker:
            # do work
            print(f"FDs opened: {tracker['delta']}")
    """
    # Get current FD count
    pid = os.getpid()
    start_count = _get_fd_count(pid)

    tracker = {
        "start": start_count,
        "end": start_count,
        "delta": 0,
    }

    try:
        yield tracker
    finally:
        tracker["end"] = _get_fd_count(pid)
        tracker["delta"] = tracker["end"] - tracker["start"]


def _get_fd_count(pid: int) -> int:
    """Get file descriptor count for process."""
    try:
        return len(os.listdir(f"/proc/{pid}/fd"))
    except FileNotFoundError, PermissionError, OSError:
        # Not on Linux or no access
        return 0


class ResourceMonitor:
    """Monitor system resources during operation."""

    def __init__(self):
        self.start_time = 0.0
        self.start_fd = 0
        self.metrics: dict[str, Any] = {}

    def __enter__(self) -> "ResourceMonitor":
        self.start_time = time.perf_counter()
        try:
            self.start_fd = _get_fd_count(os.getpid())
        except Exception:
            self.start_fd = 0
        return self

    def __exit__(self, exc_type: Any, exc_val: Any, exc_tb: Any) -> None:
        self.metrics["elapsed_seconds"] = time.perf_counter() - self.start_time
        try:
            end_fd = _get_fd_count(os.getpid())
            self.metrics["fd_delta"] = end_fd - self.start_fd
            self.metrics["fd_leak"] = self.metrics["fd_delta"] > 5
        except Exception:
            pass

    def get_metrics(self) -> dict[str, Any]:
        """Get collected metrics."""
        return self.metrics
