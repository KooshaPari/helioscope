"""Bounded cache with max size limits and eviction policies.

Provides a cache implementation that enforces maximum size limits
to prevent unbounded memory growth.
"""

import threading
import time
from collections import OrderedDict
from collections.abc import Callable
from dataclasses import dataclass, field
from enum import Enum
from typing import Any, Generic, TypeVar

T = TypeVar("T")


class EvictionPolicy(Enum):
    """Cache eviction policy."""

    LRU = "lru"  # Least Recently Used
    LFU = "lfu"  # Least Frequently Used
    FIFO = "fifo"  # First In First Out
    TTL = "ttl"  # Time To Live


@dataclass
class CacheEntry(Generic[T]):
    """Single cache entry."""

    key: str
    value: T
    created_at: float = field(default_factory=time.time)
    last_accessed: float = field(default_factory=time.time)
    access_count: int = 0
    expires_at: float | None = None

    def touch(self) -> None:
        """Update access time and count."""
        self.last_accessed = time.time()
        self.access_count += 1

    def is_expired(self) -> bool:
        """Check if entry has expired."""
        if self.expires_at is None:
            return False
        return time.time() > self.expires_at


class BoundedCache(Generic[T]):
    """Cache with bounded size and configurable eviction.

    Usage:
        cache = BoundedCache[str](max_size=1000, ttl=300)
        cache.set("key", "value")
        value = cache.get("key")

        # With eviction callback
        def on_evict(key, value):
            print(f"Evicted: {key}")

        cache = BoundedCache(on_evict=on_evict)
    """

    def __init__(
        self,
        max_size: int = 1000,
        ttl: float | None = None,
        policy: EvictionPolicy = EvictionPolicy.LRU,
        on_evict: Callable[[str, T], None] | None = None,
    ):
        self._max_size = max_size
        self._ttl = ttl
        self._policy = policy
        self._on_evict = on_evict
        self._lock = threading.RLock()
        self._cache: OrderedDict[str, CacheEntry[T]] = OrderedDict()
        self._hits = 0
        self._misses = 0

    @property
    def max_size(self) -> int:
        """Get max cache size."""
        return self._max_size

    @max_size.setter
    def max_size(self, value: int) -> None:
        """Set max cache size (triggers eviction if needed)."""
        with self._lock:
            self._max_size = value
            self._evict_if_needed()

    def set(self, key: str, value: T, ttl: float | None = None) -> None:
        """Set a cache entry."""
        with self._lock:
            # Check if we need to evict
            if key not in self._cache and len(self._cache) >= self._max_size:
                self._evict_one()

            # Calculate expiry
            effective_ttl = ttl if ttl is not None else self._ttl
            expires_at = time.time() + effective_ttl if effective_ttl else None

            entry = CacheEntry(
                key=key,
                value=value,
                expires_at=expires_at,
            )
            self._cache[key] = entry

    def get(self, key: str, default: T | None = None) -> T | None:
        """Get a cache entry."""
        with self._lock:
            entry = self._cache.get(key)

            if entry is None:
                self._misses += 1
                return default

            # Check expiry
            if entry.is_expired():
                self._remove_entry(key)
                self._misses += 1
                return default

            # Update access for LRU/LFU
            entry.touch()

            # Move to end for LRU
            if self._policy == EvictionPolicy.LRU:
                self._cache.move_to_end(key)

            self._hits += 1
            return entry.value

    def delete(self, key: str) -> bool:
        """Delete a cache entry."""
        with self._lock:
            if key in self._cache:
                self._remove_entry(key)
                return True
            return False

    def clear(self) -> int:
        """Clear all cache entries."""
        with self._lock:
            count = len(self._cache)
            self._cache.clear()
            self._hits = 0
            self._misses = 0
            return count

    def _remove_entry(self, key: str) -> None:
        """Remove entry and call eviction callback."""
        entry = self._cache.pop(key, None)
        if entry and self._on_evict:
            self._on_evict(key, entry.value)

    def _evict_if_needed(self) -> None:
        """Evict entries if cache is over capacity."""
        while len(self._cache) >= self._max_size:
            self._evict_one()

    def _evict_one(self) -> None:
        """Evict a single entry based on policy."""
        if not self._cache:
            return

        if self._policy == EvictionPolicy.LRU:
            # Remove oldest (first) item
            key = next(iter(self._cache))
        elif self._policy == EvictionPolicy.FIFO:
            # Remove oldest (first) item - same as LRU for OrderedDict
            key = next(iter(self._cache))
        elif self._policy == EvictionPolicy.LFU:
            # Remove least frequently used
            key = min(self._cache.keys(), key=lambda k: self._cache[k].access_count)
        elif self._policy == EvictionPolicy.TTL:
            # Remove expired first, then oldest
            key = None
            for k, v in self._cache.items():
                if v.is_expired():
                    key = k
                    break
            if key is None:
                key = next(iter(self._cache))
        else:
            key = next(iter(self._cache))

        self._remove_entry(key)

    def __contains__(self, key: str) -> bool:
        """Check if key exists and is not expired."""
        with self._lock:
            if key not in self._cache:
                return False
            entry = self._cache[key]
            if entry.is_expired():
                self._remove_entry(key)
                return False
            return True

    def __len__(self) -> int:
        """Get number of entries."""
        return len(self._cache)

    def get_stats(self) -> dict[str, Any]:
        """Get cache statistics."""
        with self._lock:
            total = self._hits + self._misses
            hit_rate = self._hits / total if total > 0 else 0
            return {
                "size": len(self._cache),
                "max_size": self._max_size,
                "hits": self._hits,
                "misses": self._misses,
                "hit_rate": hit_rate,
                "policy": self._policy.value,
                "ttl": self._ttl,
            }

    def cleanup_expired(self) -> int:
        """Remove all expired entries."""
        with self._lock:
            expired_keys = [k for k, v in self._cache.items() if v.is_expired()]
            for key in expired_keys:
                self._remove_entry(key)
            return len(expired_keys)


def bounded_cache(
    max_size: int = 1000,
    ttl: float | None = 300,
    policy: EvictionPolicy = EvictionPolicy.LRU,
):
    """Decorator for caching function results.

    Usage:
        @bounded_cache(max_size=100, ttl=60)
        def expensive_function(arg):
            return compute(arg)
    """
    cache = BoundedCache(max_size=max_size, ttl=ttl, policy=policy)

    def decorator(func: Callable):
        def wrapper(*args, **kwargs):
            # Create key from args
            key = f"{func.__name__}:{args}:{kwargs}"
            result = cache.get(key)
            if result is None:
                result = func(*args, **kwargs)
                cache.set(key, result)
            return result

        wrapper.cache = cache
        return wrapper

    return decorator
