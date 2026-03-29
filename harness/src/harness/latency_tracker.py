"""Network latency tracking and metrics.

Provides utilities to track network latency, throughput, and performance metrics.
"""

import statistics
import threading
import time
from collections import deque
from dataclasses import dataclass
from typing import Any


@dataclass
class LatencySnapshot:
    """Single latency measurement."""

    timestamp: float
    latency_ms: float
    success: bool
    endpoint: str = ""


@dataclass
class LatencyStats:
    """Aggregated latency statistics."""

    count: int = 0
    success_count: int = 0
    failure_count: int = 0
    min_ms: float = 0.0
    max_ms: float = 0.0
    mean_ms: float = 0.0
    median_ms: float = 0.0
    p95_ms: float = 0.0
    p99_ms: float = 0.0
    stddev_ms: float = 0.0


class LatencyTracker:
    """Track network latency metrics.

    Usage:
        tracker = LatencyTracker(window_size=1000)

        # Track a request
        with tracker.track("api/users"):
            make_api_call()

        # Get stats
        stats = tracker.get_stats("api/users")
        print(f"P95: {stats.p95_ms}ms")
    """

    def __init__(self, window_size: int = 1000):
        self._window_size = window_size
        self._snapshots: dict[str, deque] = {}
        self._lock = threading.Lock()

    def track(self, endpoint: str = "default") -> _LatencyContext:
        """Context manager for tracking latency."""
        return _LatencyContext(self, endpoint)

    def record(self, endpoint: str, latency_ms: float, success: bool = True) -> None:
        """Record a latency measurement."""
        snapshot = LatencySnapshot(
            timestamp=time.time(),
            latency_ms=latency_ms,
            success=success,
            endpoint=endpoint,
        )

        with self._lock:
            if endpoint not in self._snapshots:
                self._snapshots[endpoint] = deque(maxlen=self._window_size)
            self._snapshots[endpoint].append(snapshot)

    def get_stats(self, endpoint: str = "default") -> LatencyStats:
        """Get latency statistics for an endpoint."""
        with self._lock:
            snapshots = self._snapshots.get(endpoint, [])

            if not snapshots:
                return LatencyStats()

            latencies = [s.latency_ms for s in snapshots]
            successes = [s for s in snapshots if s.success]

            if not latencies:
                return LatencyStats()

            sorted_latencies = sorted(latencies)
            n = len(sorted_latencies)

            return LatencyStats(
                count=n,
                success_count=len(successes),
                failure_count=n - len(successes),
                min_ms=min(latencies),
                max_ms=max(latencies),
                mean_ms=statistics.mean(latencies),
                median_ms=statistics.median(latencies),
                p95_ms=sorted_latencies[int(n * 0.95)],
                p99_ms=sorted_latencies[int(n * 0.99)],
                stddev_ms=statistics.stdev(latencies) if n > 1 else 0,
            )

    def get_all_stats(self) -> dict[str, LatencyStats]:
        """Get stats for all endpoints."""
        with self._lock:
            return {ep: self.get_stats(ep) for ep in self._snapshots.keys()}

    def reset(self, endpoint: str | None = None) -> None:
        """Reset latency data."""
        with self._lock:
            if endpoint:
                self._snapshots.pop(endpoint, None)
            else:
                self._snapshots.clear()


class _LatencyContext:
    """Context manager for tracking."""

    def __init__(self, tracker: LatencyTracker, endpoint: str):
        self._tracker = tracker
        self._endpoint = endpoint
        self._start_time = 0.0
        self._success = True

    def __enter__(self):
        self._start_time = time.perf_counter()
        return self

    def __exit__(self, exc_type, exc_val, exc_tb):
        latency_ms = (time.perf_counter() - self._start_time) * 1000
        self._tracker.record(self._endpoint, latency_ms, self._success)

    def mark_failed(self):
        """Mark this request as failed."""
        self._success = False


class NetworkMetrics:
    """Track overall network metrics."""

    def __init__(self):
        self._latency_tracker = LatencyTracker()
        self._bytes_sent = 0
        self._bytes_recv = 0
        self._requests_total = 0
        self._requests_failed = 0
        self._lock = threading.Lock()

    def record_request(
        self,
        endpoint: str,
        latency_ms: float,
        success: bool,
        bytes_sent: int = 0,
        bytes_recv: int = 0,
    ) -> None:
        """Record a network request."""
        with self._lock:
            self._latency_tracker.record(endpoint, latency_ms, success)
            self._bytes_sent += bytes_sent
            self._bytes_recv += bytes_recv
            self._requests_total += 1
            if not success:
                self._requests_failed += 1

    def get_summary(self) -> dict[str, Any]:
        """Get network metrics summary."""
        with self._lock:
            all_stats = self._latency_tracker.get_all_stats()

            # Aggregate latency
            all_latencies = []
            for stats in all_stats.values():
                all_latencies.extend([stats.mean_ms] * stats.count)

            return {
                "total_requests": self._requests_total,
                "failed_requests": self._requests_failed,
                "success_rate": (self._requests_total - self._requests_failed) / max(1, self._requests_total),
                "bytes_sent_mb": self._bytes_sent / 1024 / 1024,
                "bytes_recv_mb": self._bytes_recv / 1024 / 1024,
                "avg_latency_ms": statistics.mean(all_latencies) if all_latencies else 0,
                "p95_latency_ms": sorted(all_latencies)[int(len(all_latencies) * 0.95)] if all_latencies else 0,
                "endpoints": list(all_stats.keys()),
            }

    @property
    def latency_tracker(self) -> LatencyTracker:
        """Get latency tracker."""
        return self._latency_tracker


# Global metrics instance
_global_metrics: NetworkMetrics | None = None
_metrics_lock = threading.Lock()


def get_network_metrics() -> NetworkMetrics:
    """Get global network metrics instance."""
    global _global_metrics
    with _metrics_lock:
        if _global_metrics is None:
            _global_metrics = NetworkMetrics()
        return _global_metrics


# Decorator for automatic latency tracking
def track_latency(endpoint: str = "default"):
    """Decorator to automatically track function latency.

    Usage:
        @track_latency("api/users")
        def get_users():
            return make_api_call()
    """

    def decorator(func):
        def wrapper(*args, **kwargs):
            metrics = get_network_metrics()
            start = time.perf_counter()
            success = True
            try:
                return func(*args, **kwargs)
            except Exception:
                success = False
                raise
            finally:
                latency_ms = (time.perf_counter() - start) * 1000
                metrics.record_request(endpoint, latency_ms, success)

        return wrapper

    return decorator
