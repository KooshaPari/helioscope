"""Bulkhead pattern for resource isolation.

Provides thread pool and semaphore-based bulkheads to prevent cascade failures.
"""

import time
import threading
from concurrent.futures import ThreadPoolExecutor, Future
from dataclasses import dataclass, field
from typing import Callable, Any, Optional, Dict
from functools import wraps
from enum import Enum
import queue


class BulkheadState(Enum):
    """Bulkhead states."""
    HEALTHY = "healthy"
    LOADED = "loaded"
    REJECTED = "rejected"


@dataclass
class BulkheadMetrics:
    """Bulkhead metrics."""
    total_calls: int = 0
    successful_calls: int = 0
    rejected_calls: int = 0
    queued_calls: int = 0
    avg_wait_time: float = 0.0
    state: BulkheadState = BulkheadState.HEALTHY


class ThreadPoolBulkhead:
    """Thread pool-based bulkhead for resource isolation."""
    
    def __init__(self, max_workers: int = 10, max_queue_size: int = 100):
        self.max_workers = max_workers
        self.max_queue_size = max_queue_size
        self._executor = ThreadPoolExecutor(max_workers=max_workers)
        self._metrics = BulkheadMetrics()
        self._lock = threading.Lock()
    
    def submit(self, fn: Callable, *args, **kwargs) -> Future:
        """Submit a task to the bulkhead."""
        with self._lock:
            self._metrics.total_calls += 1
            
            # Check queue capacity
            qsize = self._executor._work_queue.qsize()
            self._metrics.queued_calls = qsize
            
            if qsize >= self.max_queue_size:
                self._metrics.rejected_calls += 1
                self._metrics.state = BulkheadState.REJECTED
                raise BulkheadRejected("Bulkhead at capacity")
            
            if qsize > self.max_workers * 2:
                self._metrics.state = BulkheadState.LOADED
            else:
                self._metrics.state = BulkheadState.HEALTHY
        
        start_time = time.time()
        future = self._executor.submit(fn, *args, **kwargs)
        
        def callback(f: Future):
            with self._lock:
                self._metrics.successful_calls += 1
                elapsed = time.time() - start_time
                self._metrics.avg_wait_time = (
                    (self._metrics.avg_wait_time * (self._metrics.successful_calls - 1) + elapsed) 
                    / self._metrics.successful_calls
                )
        
        future.add_done_callback(callback)
        return future
    
    def shutdown(self, wait: bool = True):
        """Shutdown the bulkhead."""
        self._executor.shutdown(wait=wait)
    
    def metrics(self) -> BulkheadMetrics:
        """Get bulkhead metrics."""
        with self._lock:
            return BulkheadMetrics(
                total_calls=self._metrics.total_calls,
                successful_calls=self._metrics.successful_calls,
                rejected_calls=self._metrics.rejected_calls,
                queued_calls=self._executor._work_queue.qsize(),
                avg_wait_time=self._metrics.avg_wait_time,
                state=self._metrics.state
            )


class BulkheadRejected(Exception):
    """Exception raised when bulkhead rejects a call."""
    pass


class SemaphoreBulkhead:
    """Semaphore-based bulkhead for limiting concurrent access."""
    
    def __init__(self, max_concurrent: int = 10):
        self.max_concurrent = max_concurrent
        self._semaphore = threading.Semaphore(max_concurrent)
        self._metrics = BulkheadMetrics()
        self._lock = threading.Lock()
    
    def acquire(self, timeout: Optional[float] = None) -> bool:
        """Acquire the semaphore."""
        acquired = self._semaphore.acquire(timeout=timeout)
        
        with self._lock:
            self._metrics.total_calls += 1
            if not acquired:
                self._metrics.rejected_calls += 1
                self._metrics.state = BulkheadState.REJECTED
            else:
                self._metrics.successful_calls += 1
                self._metrics.state = BulkheadState.HEALTHY
        
        return acquired
    
    def release(self):
        """Release the semaphore."""
        self._semaphore.release()
    
    def __enter__(self):
        """Context manager entry."""
        if not self.acquire(timeout=0):
            raise BulkheadRejected("Bulkhead at capacity")
        return self
    
    def __exit__(self, *args):
        """Context manager exit."""
        self.release()
    
    def metrics(self) -> BulkheadMetrics:
        """Get bulkhead metrics."""
        with self._lock:
            return BulkheadMetrics(
                total_calls=self._metrics.total_calls,
                successful_calls=self._metrics.successful_calls,
                rejected_calls=self._metrics.rejected_calls,
                queued_calls=self.max_concurrent - self._semaphore._value,
                state=self._metrics.state
            )


def bulkhead(max_workers: int = 10, max_queue: int = 100):
    """Decorator to apply bulkhead pattern to a function.
    
    Usage:
        @bulkhead(max_workers=5, max_queue=10)
        def my_function():
            ...
    """
    _bulkhead = ThreadPoolBulkhead(max_workers, max_queue)
    
    def decorator(fn):
        @wraps(fn)
        def wrapper(*args, **kwargs):
            return _bulkhead.submit(fn, *args, **kwargs)
        
        wrapper.bulkhead = _bulkhead
        return wrapper
    
    return decorator


# Global bulkhead instances
_bulkheads: Dict[str, ThreadPoolBulkhead] = {}


def get_bulkhead(name: str, max_workers: int = 10, max_queue: int = 100) -> ThreadPoolBulkhead:
    """Get or create a named bulkhead."""
    if name not in _bulkheads:
        _bulkheads[name] = ThreadPoolBulkhead(max_workers, max_queue)
    return _bulkheads[name]


# Example
if __name__ == "__main__":
    import random
    
    bh = ThreadPoolBulkhead(max_workers=2, max_queue=5)
    
    def slow_task(i):
        time.sleep(random.uniform(0.1, 0.5))
        return f"Task {i} done"
    
    # Submit tasks
    futures = []
    for i in range(10):
        try:
            f = bh.submit(slow_task, i)
            futures.append(f)
            print(f"Submitted task {i}")
        except BulkheadRejected as e:
            print(f"Rejected task {i}: {e}")
    
    # Wait for completion
    for f in futures:
        try:
            print(f"Result: {f.result(timeout=2)")
        except Exception as e:
            print(f"Error: {e}")
    
    print(f"Metrics: {bh.metrics()}")
    bh.shutdown()
