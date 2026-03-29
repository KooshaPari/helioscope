"""Circuit breaker pattern for fault tolerance.

Provides circuit breaker implementation to prevent cascade failures.
"""

import threading
import time
from collections.abc import Callable
from dataclasses import dataclass
from enum import Enum
from functools import wraps
from typing import Any


class CircuitState(Enum):
    """Circuit breaker states."""

    CLOSED = "closed"
    OPEN = "open"
    HALF_OPEN = "half_open"


@dataclass
class CircuitBreakerConfig:
    """Circuit breaker configuration."""

    failure_threshold: int = 5
    success_threshold: int = 3
    timeout: float = 30.0
    half_open_max_calls: int = 3


class CircuitBreakerError(Exception):
    """Exception raised when circuit is open."""

    def __init__(self, message: str = "Circuit breaker is open"):
        self.message = message
        super().__init__(self.message)


class CircuitBreaker:
    """Circuit breaker implementation.

    Usage:
        breaker = CircuitBreaker(failure_threshold=5, timeout=30)

        try:
            result = breaker.call(risky_function)
        except CircuitBreakerError:
            # Handle fallback
    """

    def __init__(self, config: CircuitBreakerConfig | None = None):
        self.config = config or CircuitBreakerConfig()
        self._state = CircuitState.CLOSED
        self._failure_count = 0
        self._success_count = 0
        self._last_failure_time = 0.0
        self._lock = threading.Lock()

    @property
    def state(self) -> CircuitState:
        """Get current state."""
        with self._lock:
            if self._state == CircuitState.OPEN:
                # Check if timeout has passed
                if time.time() - self._last_failure_time > self.config.timeout:
                    self._state = CircuitState.HALF_OPEN
            return self._state

    def call(self, fn: Callable, *args, **kwargs) -> Any:
        """Execute function with circuit breaker protection."""
        if self.state == CircuitState.OPEN:
            raise CircuitBreakerError(f"Circuit is OPEN for {fn.__name__}")

        try:
            result = fn(*args, **kwargs)
            self._on_success()
            return result
        except Exception:
            self._on_failure()
            raise

    def _on_success(self):
        """Handle successful call."""
        with self._lock:
            self._failure_count = 0
            if self._state == CircuitState.HALF_OPEN:
                self._success_count += 1
                if self._success_count >= self.config.success_threshold:
                    self._state = CircuitState.CLOSED
                    self._success_count = 0

    def _on_failure(self):
        """Handle failed call."""
        with self._lock:
            self._failure_count += 1
            self._last_failure_time = time.time()
            if self._failure_count >= self.config.failure_threshold:
                self._state = CircuitState.OPEN

    def reset(self):
        """Reset circuit breaker to closed state."""
        with self._lock:
            self._state = CircuitState.CLOSED
            self._failure_count = 0
            self._success_count = 0

    def get_state(self) -> dict:
        """Get circuit breaker state for monitoring."""
        with self._lock:
            return {
                "state": self._state.value,
                "failures": self._failure_count,
                "successes": self._success_count,
                "last_failure": self._last_failure_time,
                "timeout": self.config.timeout,
            }


def circuit_breaker(failure_threshold: int = 5, timeout: float = 30.0):
    """Decorator to apply circuit breaker to a function."""
    _breaker = CircuitBreaker(CircuitBreakerConfig(failure_threshold=failure_threshold, timeout=timeout))

    def decorator(fn):
        @wraps(fn)
        def wrapper(*args, **kwargs):
            return _breaker.call(fn, *args, **kwargs)

        wrapper.circuit_breaker = _breaker
        return wrapper

    return decorator


# Global circuit breakers
_breakers = {}


def get_circuit_breaker(name: str, **kwargs) -> CircuitBreaker:
    """Get or create a named circuit breaker."""
    if name not in _breakers:
        _breakers[name] = CircuitBreaker(CircuitBreakerConfig(**kwargs))
    return _breakers[name]
