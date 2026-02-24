"""Bulkhead Pattern - Resource isolation for agent pools.

Provides resource isolation between different agent pools to prevent
resource contention and ensure fair scheduling.
"""

import asyncio
import threading
import time
from dataclasses import dataclass, field
from typing import Any, Callable, Optional
from enum import Enum


class BulkheadState(Enum):
    """Bulkhead availability state."""
    AVAILABLE = "available"
    FULL = "full"
    ISOLATED = "isolated"
    DEGRADED = "degraded"


@dataclass
class BulkheadConfig:
    """Configuration for a bulkhead."""
    name: str
    max_concurrent: int = 10
    max_queue_size: int = 100
    timeout_seconds: float = 60.0
    isolation_enabled: bool = True


@dataclass
class BulkheadMetrics:
    """Metrics for a bulkhead."""
    name: str
    state: BulkheadState = BulkheadState.AVAILABLE
    current_usage: int = 0
    queue_size: int = 0
    total_requests: int = 0
    successful_requests: int = 0
    rejected_requests: int = 0
    timed_out_requests: int = 0
    avg_wait_time_ms: float = 0.0


class Bulkhead:
    """Bulkhead pattern implementation for resource isolation.
    
    Usage:
        bulkhead = Bulkhead(
            name=\"coder-pool\",
            max_concurrent=5,
            max_queue_size=10
        )
        
        async with bulkhead.acquire():
            # Execute task
            pass
    """
    
    def __init__(self, config: BulkheadConfig):
        self._config = config
        self._semaphore = asyncio.Semaphore(config.max_concurrent)
        self._queue: asyncio.Queue = asyncio.Queue(maxsize=config.max_queue_size)
        self._running = 0
        self._lock = threading.Lock()  # Use threading lock for sync access
        
        # Metrics
        self._total_requests = 0
        self._successful = 0
        self._rejected = 0
        self._timeouts = 0
        self._wait_times: list[float] = []
    
    @property
    def name(self) -> str:
        return self._config.name
    
    async def acquire(self, timeout: Optional[float] = None) -> '_BulkheadContext':
        """Acquire a slot in the bulkhead."""
        start = time.perf_counter()
        
        # Try to acquire semaphore
        try:
            await asyncio.wait_for(
                self._semaphore.acquire(),
                timeout=timeout or self._config.timeout_seconds
            )
        except asyncio.TimeoutError:
            self._rejected += 1
            self._timeouts += 1
            raise BulkheadFullError(f"Bulkhead {self._config.name} is full")
        
        wait_time = (time.perf_counter() - start) * 1000
        self._wait_times.append(wait_time)
        
        with self._lock:
            self._running += 1
            self._total_requests += 1
        
        return _BulkheadContext(self)
    
    def release(self) -> None:
        """Release a slot in the bulkhead."""
        self._semaphore.release()
        
        with self._lock:
            self._running -= 1
            self._successful += 1
    
    def get_metrics(self) -> BulkheadMetrics:
        """Get bulkhead metrics."""
        avg_wait = sum(self._wait_times) / len(self._wait_times) if self._wait_times else 0
        
        state = BulkheadState.AVAILABLE
        if self._running >= self._config.max_concurrent:
            state = BulkheadState.FULL
        elif self._rejected > 0:
            state = BulkheadState.DEGRADED
        
        return BulkheadMetrics(
            name=self._config.name,
            state=state,
            current_usage=self._running,
            queue_size=self._queue.qsize(),
            total_requests=self._total_requests,
            successful_requests=self._successful,
            rejected_requests=self._rejected,
            timed_out_requests=self._timeouts,
            avg_wait_time_ms=avg_wait,
        )


class _BulkheadContext:
    """Context manager for bulkhead acquisition."""
    
    def __init__(self, bulkhead: Bulkhead):
        self._bulkhead = bulkhead
    
    async def __aenter__(self):
        return self
    
    async def __aexit__(self, exc_type, exc_val, exc_tb):
        self._bulkhead.release()


class BulkheadFullError(Exception):
    """Raised when bulkhead is at capacity."""
    pass


class BulkheadPool:
    """Pool of bulkheads for different agent types.
    
    Usage:
        pool = BulkheadPool()
        
        # Register bulkheads
        pool.register(BulkheadConfig(name=\"coders\", max_concurrent=5))
        pool.register(BulkheadConfig(name=\"reviewers\", max_concurrent=3))
        
        # Get bulkhead
        bulkhead = pool.get(\"coders\")
        async with bulkhead.acquire():
            pass
    """
    
    _instance: Optional['BulkheadPool'] = None
    
    def __init__(self):
        self._bulkheads: dict[str, Bulkhead] = {}
        self._lock = threading.Lock()
    
    @classmethod
    def get_instance(cls) -> 'BulkheadPool':
        if cls._instance is None:
            cls._instance = cls()
        return cls._instance
    
    def register(self, config: BulkheadConfig) -> Bulkhead:
        """Register a new bulkhead."""
        with self._lock:
            bulkhead = Bulkhead(config)
            self._bulkheads[config.name] = bulkhead
            return bulkhead
    
    def get(self, name: str) -> Optional[Bulkhead]:
        """Get bulkhead by name."""
        return self._bulkheads.get(name)
    
    def get_all(self) -> list[Bulkhead]:
        """Get all bulkheads."""
        return list(self._bulkheads.values())
    
    def get_all_metrics(self) -> dict[str, BulkheadMetrics]:
        """Get metrics for all bulkheads."""
        return {
            name: bulkhead.get_metrics()
            for name, bulkhead in self._bulkheads.items()
        }


# Decorator for easy bulkhead usage
def bulkhead(bulkhead_name: str, timeout: float = 60.0):
    """Decorator to wrap function with bulkhead.
    
    Usage:
        @bulkhead(\"coders\")
        async def process_code(task):
            pass
    """
    def decorator(func: Callable):
        async def wrapper(*args, **kwargs):
            pool = BulkheadPool.get_instance()
            bulkhead = pool.get(bulkhead_name)
            
            if not bulkhead:
                raise ValueError(f"Bulkhead not found: {bulkhead_name}")
            
            async with bulkhead.acquire(timeout=timeout):
                return await func(*args, **kwargs)
        
        return wrapper
    return decorator
