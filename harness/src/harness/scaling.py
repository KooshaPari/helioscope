"""Dynamic Scaling System - Resource-based concurrency limits with hysteresis."""

from __future__ import annotations

import asyncio
import gc
import os
import sys
import threading
import time
from collections.abc import Callable
from dataclasses import dataclass
from pathlib import Path
from typing import Any

# Lazy psutil - only load when sampling is needed
_psutil = None


def _get_psutil():
    global _psutil
    if _psutil is None:
        import psutil

        _psutil = psutil
    return _psutil


# Try to import Rust extension for 10-50x performance
# Add src dir to path for .so file
_rust_path = Path(__file__).parent.parent
if str(_rust_path) not in sys.path:
    sys.path.insert(0, str(_rust_path))

try:
    from helios_harness_rs import ResourceSampler as RustResourceSampler

    RUST_AVAILABLE = True
except ImportError:
    RUST_AVAILABLE = False


# =============================================================================
# Configuration
# =============================================================================


@dataclass
class ScalingConfig:
    """Configuration for dynamic scaling."""

    min_buffer: float = 0.05  # 5% hard limit
    discretionary_buffer: float = 0.15  # 15% soft limit
    sampling_interval_seconds: float = 2.0
    hysteresis_upper: float = 0.80
    hysteresis_lower: float = 0.60
    hysteresis_dwell: int = 30  # seconds


# =============================================================================
# Resource Monitoring
# =============================================================================


@dataclass
class ResourceSnapshot:
    """Current resource usage snapshot."""

    cpu_percent: float
    memory_percent: float
    memory_available_mb: float
    fd_count: int
    fd_limit: int
    load_avg: float
    timestamp: float


class ResourceSampler:
    """Sample system resources - uses Rust extension when available for 10-50x speedup."""

    def __init__(self):
        # Use Rust implementation if available
        self._rust = RustResourceSampler() if RUST_AVAILABLE else None

    def sample(self) -> ResourceSnapshot:
        """Sample current resources."""
        # Use Rust for 10-50x speedup
        if self._rust is not None:
            result = self._rust.sample()
            # Handle both tuple (new) and object (old) returns
            if isinstance(result, tuple):
                cpu_percent, memory_percent, memory_available_mb, fd_count, load_avg, timestamp = result
                return ResourceSnapshot(
                    cpu_percent=cpu_percent,
                    memory_percent=memory_percent,
                    memory_available_mb=memory_available_mb,
                    fd_count=fd_count,
                    fd_limit=0,
                    load_avg=load_avg,
                    timestamp=timestamp,
                )
            else:
                return ResourceSnapshot(
                    cpu_percent=result.cpu_percent,
                    memory_percent=result.memory_percent,
                    memory_available_mb=result.memory_available_mb,
                    fd_count=result.fd_count,
                    fd_limit=result.fd_limit,
                    load_avg=result.load_avg,
                    timestamp=result.timestamp,
                )

        # Fallback to Python
        # CPU
        cpu_percent = _get_psutil().cpu_percent(interval=0.1)

        # Memory
        mem = _get_psutil().virtual_memory()
        memory_percent = mem.percent
        memory_available_mb = mem.available / (1024 * 1024)

        # File descriptors - use psutil
        try:
            proc = _get_psutil().Process()
            fd_count = proc.num_fds() if hasattr(proc, "num_fds") else len(proc.open_files())
            try:
                import resource

                soft, hard = resource.getrlimit(resource.RLIMIT_NOFILE)
                fd_limit = soft
            except Exception:
                fd_limit = 1024
        except Exception:
            fd_count = 0
            fd_limit = 1024

        # Load average
        try:
            load_avg = os.getloadavg()[0]
        except NotImplementedError:
            load_avg = 0.0

        return ResourceSnapshot(
            cpu_percent=cpu_percent,
            memory_percent=memory_percent,
            memory_available_mb=memory_available_mb,
            fd_count=fd_count,
            fd_limit=fd_limit,
            load_avg=load_avg,
            timestamp=time.time(),
        )


# =============================================================================
# Dynamic Limit Controller
# =============================================================================


class DynamicLimitController:
    """Computes dynamic limits based on resources."""

    def __init__(self, config: ScalingConfig | None = None):
        self.config = config or ScalingConfig()
        self.current_limit = 10
        self.sampler = ResourceSampler()
        self._last_change_time = 0.0
        self._state = "stable"

    def compute_limit(self, running_count: int) -> int:
        """Compute limit based on resources."""
        snapshot = self.sampler.sample()

        # Normalize resources to 0-1
        cpu_util = snapshot.cpu_percent / 100.0
        mem_util = snapshot.memory_percent / 100.0
        fd_util = snapshot.fd_count / snapshot.fd_limit

        # Load normalized by CPU cores
        cpu_count = os.cpu_count() or 4
        load_util = min(snapshot.load_avg / cpu_count, 1.0)

        # Headroom = 1.0 - utilization
        headroom = 1.0 - max(cpu_util, mem_util, fd_util, load_util)

        # Base baseline
        baseline = 100

        # Compute limits
        hard_limit = int(baseline * (1.0 - self.config.min_buffer))
        discretionary_limit = int(baseline * (1.0 - self.config.discretionary_buffer))
        dynamic_limit = int(baseline * headroom)

        # Effective limit
        effective_limit = min(hard_limit, discretionary_limit, dynamic_limit)
        effective_limit = max(1, effective_limit)

        return effective_limit

    def get_limit(self, running_count: int) -> int:
        """Get limit with hysteresis."""
        now = time.time()
        target = self.compute_limit(running_count)

        # Apply hysteresis
        if target > self.current_limit:
            # Scale up after dwell time
            if now - self._last_change_time >= self.config.hysteresis_dwell:
                self.current_limit = target
                self._last_change_time = now
                self._state = "scaling_up"
        elif target < self.current_limit:
            # Scale down
            if running_count < target and now - self._last_change_time >= self.config.hysteresis_dwell:
                self.current_limit = target
                self._last_change_time = now
                self._state = "scaling_down"
        else:
            self._state = "stable"

        return self.current_limit


# =============================================================================
# Memory Pressure Handler
# =============================================================================


class MemoryPressureHandler:
    """Handle memory pressure gracefully."""

    WARNING = 0.75
    CRITICAL = 0.90

    def __init__(self, callback: Callable[[str, float], None] | None = None):
        self.callback = callback or self._default_handler()
        self.enabled = True

    def _default_handler(self):
        def handler(level: str, percent: float):
            if level == "critical":
                gc.collect()

        return handler

    async def check(self):
        """Check memory pressure."""
        mem = _get_psutil().virtual_memory()
        percent = mem.percent / 100.0

        if percent >= self.CRITICAL:
            self.callback("critical", percent)
            gc.collect()
        elif percent >= self.WARNING:
            self.callback("warning", percent)

        return percent


# =============================================================================
# FD Manager
# =============================================================================


class FDManager:
    """Manage file descriptors with limits."""

    SOFT_LIMIT = 512
    HARD_LIMIT = 1024

    def __init__(self):
        self._open_fds: dict[int, dict] = {}
        self._lock = asyncio.Lock()

    async def acquire(self, path: str, flags: int) -> int:
        """Acquire FD."""
        async with self._lock:
            if len(self._open_fds) >= self.HARD_LIMIT:
                raise RuntimeError(f"FD limit {self.HARD_LIMIT} reached")

            fd = os.open(path, flags)
            self._open_fds[fd] = {"path": path, "opened": time.time()}

            if len(self._open_fds) >= self.SOFT_LIMIT:
                import logging

                logging.warning(f"FD warning: {len(self._open_fds)}/{self.HARD_LIMIT}")

            return fd

    async def release(self, fd: int):
        """Release FD."""
        async with self._lock:
            if fd in self._open_fds:
                os.close(fd)
                del self._open_fds[fd]


# =============================================================================
# Circuit Breaker
# =============================================================================


class CircuitBreaker:
    """Circuit breaker for fault tolerance."""

    def __init__(
        self,
        failure_threshold: int = 5,
        timeout_seconds: int = 60,
        success_threshold: int = 3,
    ):
        self.failure_threshold = failure_threshold
        self.timeout_seconds = timeout_seconds
        self.success_threshold = success_threshold
        self._failures = 0
        self._state = "closed"
        self._last_failure_time = 0.0
        self._last_state_change = time.time()

    def record_success(self):
        """Record successful call."""
        if self._state == "half_open":
            self._failures = 0
            self._state = "closed"

    def record_failure(self):
        """Record failure."""
        self._failures += 1
        self._last_failure_time = time.time()

        if self._failures >= self.failure_threshold:
            self._state = "open"

    def can_execute(self) -> bool:
        """Check if execution allowed."""
        now = time.time()

        if self._state == "open":
            if now - self._last_failure_time >= self.timeout_seconds:
                self._state = "half_open"
                return True
            return False

        return True
