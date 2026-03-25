"""Multi-level caching system for heliosHarness.

Implements L1-L4 cache layers with pre-warming and coalescing."""

from __future__ import annotations

import asyncio
import hashlib
import json
import logging
import os
import sys
import threading
import time
from collections import deque
from dataclasses import dataclass
from pathlib import Path
from typing import Any, Callable

logger = logging.getLogger(__name__)

# Try to import Rust extension for 10x performance
# Add src dir to path for .so file
_rust_path = Path(__file__).parent.parent
if str(_rust_path) not in sys.path:
    sys.path.insert(0, str(_rust_path))

try:
    from helios_harness_rs import LruCache as RustLruCache
    from helios_harness_rs import CacheStats as RustCacheStats

    RUST_AVAILABLE = True
except ImportError:
    RUST_AVAILABLE = False

# Try to import cachetools and diskcache
try:
    from cachetools import TTLCache

    CACHERTOOLS_AVAILABLE = True
except ImportError:
    CACHERTOOLS_AVAILABLE = False

try:
    import diskcache

    DISKCACHE_AVAILABLE = True
except ImportError:
    DISKCACHE_AVAILABLE = False


# =============================================================================
# L1 Cache
# =============================================================================


@dataclass
class L1CacheStats:
    """L1 cache statistics."""

    hits: int = 0
    misses: int = 0

    @property
    def hit_rate(self) -> float:
        total = self.hits + self.misses
        return self.hits / total if total > 0 else 0.0


class L1Cache:
    """L1 in-memory cache with TTL - uses Rust when available for 10x speedup."""

    def __init__(self, maxsize: int = 1000, ttl: float = 60.0):
        # Use Rust implementation if available
        if RUST_AVAILABLE:
            self._rust = RustLruCache(capacity=maxsize, ttl_secs=ttl)
            self._using_rust = True
        elif CACHERTOOLS_AVAILABLE:
            self._cache = TTLCache(maxsize=maxsize, ttl=ttl)
            self._using_rust = False
        else:
            # Fallback to dict-based TTL cache
            self._cache = {}
            self._ttl = ttl
            self._timestamps = {}
            self._using_rust = False

        self._lock = threading.Lock()
        self._stats = L1CacheStats()

    def get(self, key: str) -> Any:
        """Get value from cache."""
        if self._using_rust:
            return self._rust.get(key)

        with self._lock:
            if CACHERTOOLS_AVAILABLE:
                value = self._cache.get(key)
                # Track stats for cachetools too
                if value is not None:
                    self._stats.hits += 1
                else:
                    self._stats.misses += 1
                return value
            else:
                # Manual TTL check
                if key in self._cache:
                    if time.time() - self._timestamps.get(key, 0) < self._ttl:
                        self._stats.hits += 1
                        return self._cache[key]
                    else:
                        self._stats.misses += 1
                        return None
                self._stats.misses += 1
                return None

    def set(self, key: str, value: Any):
        """Set value in cache."""
        if self._using_rust:
            self._rust.set(key, value)
            return

        with self._lock:
            if CACHERTOOLS_AVAILABLE:
                self._cache[key] = value
            else:
                self._cache[key] = value
                self._timestamps[key] = time.time()

    @property
    def stats(self) -> L1CacheStats:
        """Get cache statistics."""
        if self._using_rust:
            rust_stats = self._rust.stats()
            return L1CacheStats(hits=rust_stats.hits, misses=rust_stats.misses)
        return self._stats


# =============================================================================
# L2 Disk Cache
# =============================================================================


@dataclass
class L2Cache:
    """L2 persistent disk cache."""

    def __init__(
        self,
        directory: str = "~/.cache/heliosharness",
        ttl: int = 3600,
        size_limit: int = 10**9,  # 1GB
    ):
        self.directory = Path(directory).expanduser()
        self.directory.mkdir(parents=True, exist_ok=True)
        self.ttl = ttl
        self.size_limit = size_limit

        if DISKCACHE_AVAILABLE:
            self._cache = diskcache.Cache(str(self.directory))
        else:
            self._cache = None
            logger.warning("diskcache not available, L2 disabled")

    def get(self, key: str) -> Any:
        """Get value from L2 cache."""
        if not self._cache:
            return None

        try:
            value = self._cache.get(key, expire_time=self.ttl)
            return json.loads(value) if value else None
        except Exception:
            return None

    def set(self, key: str, value: Any):
        """Set value in L2 cache."""
        if not self._cache:
            return

        try:
            self._cache.set(key, json.dumps(value), expire=self.ttl)
        except Exception as e:
            logger.error(f"L2 cache set failed: {e}")

    def clear(self):
        """Clear L2 cache."""
        if self._cache:
            self._cache.clear()


# =============================================================================
# Request Coalescer
# =============================================================================


class RequestCoalescer:
    """Coalesce concurrent requests for same key."""

    def __init__(self, timeout: float = 5.0):
        self.timeout = timeout
        self._inflight: dict[str, asyncio.Future] = {}
        self._lock = asyncio.Lock()

    async def get_or_fetch(self, key: str, fetch_fn: Callable) -> Any:
        """Get cached or fetch new value."""
        async with self._lock:
            if key in self._inflight:
                return await self._inflight[key]

            future = asyncio.Future()
            self._inflight[key] = future

        try:
            result = await asyncio.wait_for(fetch_fn(), timeout=self.timeout)
            future.set_result(result)
        except Exception as e:
            future.set_exception(e)
        finally:
            async with self._lock:
                self._inflight.pop(key, None)

        return result


# =============================================================================
# Predictive Pre-warmer
# =============================================================================


@dataclass
class WarmingStrategy:
    """Warming strategy configuration."""

    name: str
    predict_fn: Callable[[], list[str]]
    load_fn: Callable[[str], Any]
    schedule_seconds: int = 300


class CachePreWarmer:
    """Predictive cache pre-warmer."""

    def __init__(self, l1_cache: L1Cache, l2_cache: L2Cache | None = None):
        self.l1 = l1_cache
        self.l2 = l2_cache
        self._running = False

    def register_strategy(self, strategy: WarmingStrategy):
        """Register warming strategy."""
        self.strategy = strategy

    async def warm_all(self):
        """One-shot warming run."""
        if not hasattr(self, "strategy"):
            return

        try:
            keys = self.strategy.predict_fn()
            for key in keys:
                if not self.l1.get(key):
                    try:
                        value = await self.strategy.load_fn(key)
                        self.l1.set(key, value)
                        if self.l2:
                            self.l2.set(key, value)
                    except Exception:
                        pass
        except Exception:
            pass

    async def start_background(self):
        """Start background warming daemon."""
        self._running = True
        while self._running:
            await self.warm_all()
            await asyncio.sleep(self.strategy.schedule_seconds)

    def stop(self):
        """Stop background warming."""
        self._running = False


# =============================================================================
# Speculative Executor
# =============================================================================


class SpeculativeExecutor:
    """Speculative execution with provider racing."""

    async def race_first(self, providers: list[Callable], task: Any) -> Any:
        """Execute providers, return first success."""
        tasks = [asyncio.create_task(p(task)) for p in providers]
        done, pending = await asyncio.wait(tasks, timeout=30.0, return_when=asyncio.FIRST_COMPLETED)

        # Cancel pending
        for t in pending:
            t.cancel()

        # Return first success
        for t in done:
            try:
                return t.result()
            except Exception:
                pass

        raise RuntimeError("All providers failed")

    async def race_best(
        self,
        providers: list[Callable],
        task: Any,
        quality_fn: Callable[[Any], float] | None = None,
    ) -> Any:
        """Race providers, return best quality."""
        results = await asyncio.gather(*[p(task) for p in providers], return_exceptions=True)

        valid = [r for r in results if not isinstance(r, Exception)]
        if not valid:
            raise RuntimeError("All providers failed")

        if quality_fn:
            return max(valid, key=quality_fn)
        return valid[0]
