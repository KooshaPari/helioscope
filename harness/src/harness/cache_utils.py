"""Caching utilities for heliosHarness.

Provides in-memory and distributed caching with TTL support.
"""

import hashlib
import threading
import time
from collections import OrderedDict
from collections.abc import Callable
from dataclasses import dataclass
from functools import wraps
from typing import Any


@dataclass
class CacheConfig:
    """Cache configuration."""

    max_size: int = 1000
    default_ttl: float = 300.0  # seconds
    eviction_policy: str = "lru"  # lru, lfu, fifo


class LRUCache:
    """Thread-safe LRU cache with TTL."""

    def __init__(self, max_size: int = 1000, default_ttl: float = 300.0):
        self.max_size = max_size
        self.default_ttl = default_ttl
        self._cache: OrderedDict = OrderedDict()
        self._expiry: dict[str, float] = {}
        self._lock = threading.RLock()
        self._hits = 0
        self._misses = 0

    def _make_key(self, key: str) -> str:
        """Create cache key hash if needed."""
        if len(key) > 256:
            return hashlib.sha256(key.encode()).hexdigest()
        return key

    def get(self, key: str) -> Any | None:
        """Get value from cache."""
        key = self._make_key(key)
        with self._lock:
            if key not in self._cache:
                self._misses += 1
                return None

            # Check expiry
            if key in self._expiry and time.time() > self._expiry[key]:
                del self._cache[key]
                del self._expiry[key]
                self._misses += 1
                return None

            # Move to end (most recently used)
            self._cache.move_to_end(key)
            self._hits += 1
            return self._cache[key]

    def set(self, key: str, value: Any, ttl: float | None = None):
        """Set value in cache."""
        key = self._make_key(key)
        ttl = ttl or self.default_ttl

        with self._lock:
            if key in self._cache:
                self._cache.move_to_end(key)
            else:
                if len(self._cache) >= self.max_size:
                    # Evict oldest
                    oldest_key = next(iter(self._cache))
                    del self._cache[oldest_key]
                    self._expiry.pop(oldest_key, None)

                self._cache[key] = value

            if ttl > 0:
                self._expiry[key] = time.time() + ttl

    def delete(self, key: str):
        """Delete value from cache."""
        key = self._make_key(key)
        with self._lock:
            self._cache.pop(key, None)
            self._expiry.pop(key, None)

    def clear(self):
        """Clear all cache."""
        with self._lock:
            self._cache.clear()
            self._expiry.clear()

    def stats(self) -> dict[str, Any]:
        """Get cache statistics."""
        with self._lock:
            total = self._hits + self._misses
            hit_rate = self._hits / total if total > 0 else 0
            return {
                "size": len(self._cache),
                "max_size": self.max_size,
                "hits": self._hits,
                "misses": self._misses,
                "hit_rate": hit_rate,
            }

    def cleanup_expired(self):
        """Remove expired entries."""
        now = time.time()
        with self._lock:
            expired = [k for k, exp in self._expiry.items() if now > exp]
            for k in expired:
                self._cache.pop(k, None)
                self._expiry.pop(k, None)


# Global cache instance
_cache: LRUCache | None = None
_cache_config: CacheConfig | None = None


def get_cache(config: CacheConfig | None = None) -> LRUCache:
    """Get or create the global cache."""
    global _cache, _cache_config
    if _cache is None or config != _cache_config:
        _cache_config = config
        _cache = LRUCache(
            max_size=config.max_size if config else 1000, default_ttl=config.default_ttl if config else 300.0
        )
    return _cache


def cached(ttl: float | None = None, key_func: Callable | None = None):
    """Decorator to cache function results.

    Usage:
        @cached(ttl=60)
        def expensive_func(arg1, arg2):
            ...
    """

    def decorator(func: Callable) -> Callable:
        @wraps(func)
        def wrapper(*args, **kwargs):
            cache = get_cache()

            # Generate key
            if key_func:
                cache_key = key_func(*args, **kwargs)
            else:
                key_parts = [func.__module__, func.__name__]
                key_parts.extend(str(a) for a in args)
                key_parts.extend(f"{k}={v}" for k, v in sorted(kwargs.items()))
                cache_key = ":".join(key_parts)

            # Try cache first
            result = cache.get(cache_key)
            if result is not None:
                return result

            # Compute and cache
            result = func(*args, **kwargs)
            cache.set(cache_key, result, ttl)
            return result

        wrapper.cache = get_cache()
        return wrapper

    return decorator


# Example usage
if __name__ == "__main__":
    cache = LRUCache(max_size=100, default_ttl=1.0)

    # Test
    cache.set("key1", {"data": "value1"})
    print(f"Get key1: {cache.get('key1')}")
    print(f"Stats: {cache.stats()}")

    # Wait for expiry
    time.sleep(1.5)
    print(f"Get key1 after TTL: {cache.get('key1')}")
