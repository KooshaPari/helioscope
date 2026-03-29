"""Health, readiness, and metrics endpoints for heliosHarness.

This module provides infrastructure endpoints for monitoring and observability:
- /health - Basic health check
- /ready - Readiness check (includes dependencies)
- /metrics - Prometheus-style metrics
- /live - Liveness probe
"""

import os
import platform
import time
from dataclasses import dataclass, field
from enum import Enum
from typing import Any


class HealthStatus(Enum):
    """Health status levels."""

    HEALTHY = "healthy"
    DEGRADED = "degraded"
    UNHEALTHY = "unhealthy"


@dataclass
class ComponentHealth:
    """Health of a single component."""

    name: str
    status: HealthStatus
    message: str = ""
    latency_ms: float = 0.0
    details: dict[str, Any] = field(default_factory=dict)


@dataclass
class HealthResponse:
    """Health check response."""

    status: HealthStatus
    timestamp: float
    uptime_seconds: float
    version: str
    components: list[ComponentHealth] = field(default_factory=list)

    def to_dict(self) -> dict[str, Any]:
        return {
            "status": self.status.value,
            "timestamp": self.timestamp,
            "uptime_seconds": self.uptime_seconds,
            "version": self.version,
            "components": [
                {
                    "name": c.name,
                    "status": c.status.value,
                    "message": c.message,
                    "latency_ms": c.latency_ms,
                    "details": c.details,
                }
                for c in self.components
            ],
        }


class HealthChecker:
    """Main health checker for heliosHarness."""

    def __init__(self):
        self._start_time = time.time()
        self._version = self._get_version()

    def _get_version(self) -> str:
        """Get version from environment or default."""
        return os.environ.get("HELIOS_VERSION", "0.0.0")

    def check_health(self) -> HealthResponse:
        """Perform full health check."""
        timestamp = time.time()
        components = []

        # Check Python/runtime
        components.append(
            ComponentHealth(
                name="python",
                status=HealthStatus.HEALTHY,
                message=f"Python {platform.python_version()}",
                details={"version": platform.python_version()},
            )
        )

        # Check system resources
        try:
            import psutil

            memory = psutil.virtual_memory()
            cpu_percent = psutil.cpu_percent(interval=0.1)

            memory_status = HealthStatus.HEALTHY
            if memory.percent > 90:
                memory_status = HealthStatus.UNHEALTHY
            elif memory.percent > 75:
                memory_status = HealthStatus.DEGRADED

            components.append(
                ComponentHealth(
                    name="memory",
                    status=memory_status,
                    message=f"Memory at {memory.percent}%",
                    details={
                        "total_mb": memory.total // (1024 * 1024),
                        "available_mb": memory.available // (1024 * 1024),
                        "percent": memory.percent,
                    },
                )
            )

            components.append(
                ComponentHealth(
                    name="cpu",
                    status=HealthStatus.HEALTHY,
                    message=f"CPU at {cpu_percent}%",
                    details={"percent": cpu_percent},
                )
            )
        except ImportError:
            components.append(
                ComponentHealth(
                    name="system",
                    status=HealthStatus.DEGRADED,
                    message="psutil not available",
                    details={"note": "optional dependency"},
                )
            )

        # Determine overall status
        statuses = [c.status for c in components]
        if any(s == HealthStatus.UNHEALTHY for s in statuses):
            overall = HealthStatus.UNHEALTHY
        elif any(s == HealthStatus.DEGRADED for s in statuses):
            overall = HealthStatus.DEGRADED
        else:
            overall = HealthStatus.HEALTHY

        return HealthResponse(
            status=overall,
            timestamp=timestamp,
            uptime_seconds=timestamp - self._start_time,
            version=self._version,
            components=components,
        )

    def check_readiness(self) -> HealthResponse:
        """Check readiness (includes dependency checks)."""
        health = self.check_health()

        # Add readiness-specific checks
        # In a real implementation, you'd check:
        # - Database connectivity
        # - Cache availability
        # - External API availability

        # For now, readiness = health
        return health

    def check_liveness(self) -> dict[str, str]:
        """Simple liveness probe."""
        return {"status": "alive"}


# Global health checker instance
_health_checker: HealthChecker | None = None


def get_health_checker() -> HealthChecker:
    """Get or create the global health checker."""
    global _health_checker
    if _health_checker is None:
        _health_checker = HealthChecker()
    return _health_checker


# Prometheus-style metrics
class MetricsCollector:
    """Collects and exposes Prometheus-style metrics."""

    def __init__(self):
        self._counters: dict[str, float] = {}
        self._gauges: dict[str, float] = {}
        self._histograms: dict[str, list[float]] = {}

    def inc_counter(self, name: str, value: float = 1.0):
        """Increment a counter."""
        self._counters[name] = self._counters.get(name, 0) + value

    def set_gauge(self, name: str, value: float):
        """Set a gauge value."""
        self._gauges[name] = value

    def observe_histogram(self, name: str, value: float):
        """Observe a histogram value."""
        if name not in self._histograms:
            self._histograms[name] = []
        self._histograms[name].append(value)

    def get_metrics(self) -> str:
        """Get metrics in Prometheus format."""
        lines = []

        # Counters
        for name, value in sorted(self._counters.items()):
            lines.append(f"# TYPE {name} counter")
            lines.append(f"{name} {value}")

        # Gauges
        for name, value in sorted(self._gauges.items()):
            lines.append(f"# TYPE {name} gauge")
            lines.append(f"{name} {value}")

        # Histograms (simplified)
        for name, values in sorted(self._histograms.items()):
            if values:
                avg = sum(values) / len(values)
                lines.append(f"# TYPE {name} histogram")
                lines.append(f"{name}_sum {sum(values)}")
                lines.append(f"{name}_count {len(values)}")
                lines.append(f"{name}_avg {avg}")

        return "\n".join(lines)


# Global metrics collector
_metrics = MetricsCollector()


def get_metrics_collector() -> MetricsCollector:
    """Get the global metrics collector."""
    return _metrics


# FastAPI/Flask route handlers (for integration)
def health_handler() -> dict[str, Any]:
    """Health check endpoint handler."""
    checker = get_health_checker()
    return checker.check_health().to_dict()


def ready_handler() -> dict[str, Any]:
    """Readiness check endpoint handler."""
    checker = get_health_checker()
    return checker.check_readiness().to_dict()


def live_handler() -> dict[str, str]:
    """Liveness probe handler."""
    return get_health_checker().check_liveness()


def metrics_handler() -> str:
    """Metrics endpoint handler."""
    return get_metrics_collector().get_metrics()
