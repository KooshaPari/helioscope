"""Rate limiting for API endpoints.

Provides token bucket and sliding window rate limiters.
"""

import threading
import time
from collections import deque
from collections.abc import Callable
from dataclasses import dataclass
from functools import wraps


@dataclass
class RateLimitConfig:
    """Rate limit configuration."""

    requests_per_second: float = 10.0
    burst_size: int = 20
    key: str = "default"


class TokenBucketLimiter:
    """Token bucket rate limiter."""

    def __init__(self, rate: float, burst: int):
        self.rate = rate
        self.burst = burst
        self.tokens = float(burst)
        self.last_update = time.time()
        self.lock = threading.Lock()

    def acquire(self, tokens: int = 1) -> bool:
        """Try to acquire tokens."""
        with self.lock:
            now = time.time()
            elapsed = now - self.last_update
            self.tokens = min(self.burst, self.tokens + elapsed * self.rate)
            self.last_update = now

            if self.tokens >= tokens:
                self.tokens -= tokens
                return True
            return False

    def wait_time(self, tokens: int = 1) -> float:
        """Get wait time until tokens are available."""
        with self.lock:
            if self.tokens >= tokens:
                return 0.0
            needed = tokens - self.tokens
            return needed / self.rate


class SlidingWindowLimiter:
    """Sliding window rate limiter."""

    def __init__(self, max_requests: int, window_seconds: float):
        self.max_requests = max_requests
        self.window_seconds = window_seconds
        self.requests: deque = deque()
        self.lock = threading.Lock()

    def is_allowed(self) -> bool:
        """Check if request is allowed."""
        with self.lock:
            now = time.time()
            cutoff = now - self.window_seconds

            # Remove old requests
            while self.requests and self.requests[0] < cutoff:
                self.requests.popleft()

            if len(self.requests) < self.max_requests:
                self.requests.append(now)
                return True
            return False

    def wait_time(self) -> float:
        """Get wait time until next request is allowed."""
        with self.lock:
            if not self.requests:
                return 0.0
            oldest = self.requests[0]
            now = time.time()
            elapsed = now - (oldest - self.window_seconds)
            return max(0, elapsed)


class RateLimiter:
    """Main rate limiter with multiple strategies."""

    def __init__(self):
        self._limiters: dict[str, TokenBucketLimiter] = {}
        self._sliding_limiters: dict[str, SlidingWindowLimiter] = {}
        self._lock = threading.Lock()

    def get_token_bucket(self, key: str, rate: float = 10.0, burst: int = 20) -> TokenBucketLimiter:
        """Get or create a token bucket limiter."""
        with self._lock:
            if key not in self._limiters:
                self._limiters[key] = TokenBucketLimiter(rate, burst)
            return self._limiters[key]

    def get_sliding_window(
        self, key: str, max_requests: int = 100, window_seconds: float = 60.0
    ) -> SlidingWindowLimiter:
        """Get or create a sliding window limiter."""
        with self._lock:
            if key not in self._sliding_limiters:
                self._sliding_limiters[key] = SlidingWindowLimiter(max_requests, window_seconds)
            return self._sliding_limiters[key]

    def check_rate_limit(self, key: str, strategy: str = "token_bucket", **kwargs) -> tuple[bool, float]:
        """Check if request is allowed.

        Returns:
            (allowed, wait_time_seconds)
        """
        if strategy == "token_bucket":
            limiter = self.get_token_bucket(key, kwargs.get("rate", 10.0), kwargs.get("burst", 20))
            allowed = limiter.acquire(kwargs.get("tokens", 1))
            wait_time = limiter.wait_time(kwargs.get("tokens", 1)) if not allowed else 0
            return allowed, wait_time

        elif strategy == "sliding_window":
            limiter = self.get_sliding_window(key, kwargs.get("max_requests", 100), kwargs.get("window_seconds", 60.0))
            allowed = limiter.is_allowed()
            wait_time = limiter.wait_time() if not allowed else 0
            return allowed, wait_time

        return True, 0.0


# Global rate limiter
_rate_limiter: RateLimiter | None = None


def get_rate_limiter() -> RateLimiter:
    """Get the global rate limiter."""
    global _rate_limiter
    if _rate_limiter is None:
        _rate_limiter = RateLimiter()
    return _rate_limiter


def rate_limit(key: str = "default", strategy: str = "token_bucket", **kwargs):
    """Decorator for rate limiting a function."""

    def decorator(func: Callable):
        @wraps(func)
        def wrapper(*args, **kwargs):
            limiter = get_rate_limiter()
            allowed, wait_time = limiter.check_rate_limit(key, strategy, **kwargs)

            if not allowed:
                raise RateLimitExceeded(wait_time)

            return func(*args, **kwargs)

        return wrapper

    return decorator


class RateLimitExceeded(Exception):
    """Exception raised when rate limit is exceeded."""

    def __init__(self, retry_after: float = 0):
        self.retry_after = retry_after
        super().__init__(f"Rate limit exceeded. Retry after {retry_after:.2f} seconds")


# Example usage
if __name__ == "__main__":
    limiter = get_rate_limiter()

    # Token bucket example
    for i in range(25):
        allowed, wait = limiter.check_rate_limit("api", strategy="token_bucket", rate=10, burst=20)
        print(f"Request {i + 1}: allowed={allowed}, wait={wait:.2f}s")

    print("\n--- Sliding window ---\n")

    # Sliding window example
    for i in range(5):
        for j in range(25):
            allowed, wait = limiter.check_rate_limit(
                f"api_{i}", strategy="sliding_window", max_requests=20, window_seconds=60
            )
            print(f"User {i}, Request {j + 1}: allowed={allowed}")
        time.sleep(0.1)
