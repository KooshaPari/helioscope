"""Worker pool for background task processing.

Provides thread pool and async worker management.
"""

import asyncio
import threading
from concurrent.futures import ThreadPoolExecutor, Future
from dataclasses import dataclass, field
from typing import Callable, Any, Optional, Dict, List
from queue import Queue, Empty
from enum import Enum
import time


class WorkerState(Enum):
    """Worker states."""

    IDLE = "idle"
    BUSY = "busy"
    STOPPING = "stopping"
    STOPPED = "stopped"


@dataclass
class WorkerMetrics:
    """Worker pool metrics."""

    total_tasks: int = 0
    completed_tasks: int = 0
    failed_tasks: int = 0
    avg_latency_ms: float = 0.0
    active_workers: int = 0


class WorkerPool:
    """Thread pool for background tasks.

    Usage:
        pool = WorkerPool(num_workers=4)
        pool.start()
        pool.submit(my_task, arg1, arg2)
        pool.shutdown()
    """

    def __init__(self, num_workers: int = 4, queue_size: int = 100):
        self.num_workers = num_workers
        self.queue_size = queue_size
        self._executor = ThreadPoolExecutor(max_workers=num_workers)
        self._task_queue: Queue = Queue(maxsize=queue_size)
        self._workers: List[threading.Thread] = []
        self._running = False
        self._metrics = WorkerMetrics()
        self._lock = threading.Lock()
        self._callbacks: Dict[str, Callable] = {}

    def start(self):
        """Start the worker pool."""
        self._running = True
        for i in range(self.num_workers):
            t = threading.Thread(target=self._worker_loop, daemon=True)
            t.start()
            self._workers.append(t)

    def _worker_loop(self):
        """Worker loop."""
        while self._running:
            try:
                task = self._task_queue.get(timeout=1)
                if task is None:
                    break

                func, args, kwargs, task_id = task
                start = time.time()

                try:
                    result = func(*args, **kwargs)

                    # Callback on success
                    if task_id in self._callbacks:
                        self._callbacks[task_id](result)

                    with self._lock:
                        self._metrics.completed_tasks += 1
                except Exception as e:
                    with self._lock:
                        self._metrics.failed_tasks += 1

                elapsed = (time.time() - start) * 1000
                with self._lock:
                    n = self._metrics.completed_tasks + self._metrics.failed_tasks
                    self._metrics.avg_latency_ms = (self._metrics.avg_latency_ms * (n - 1) + elapsed) / n

            except Empty:
                continue
            except Exception as e:
                pass

    def submit(self, func: Callable, *args, task_id: Optional[str] = None, **kwargs) -> Future:
        """Submit a task to the pool."""
        if not self._running:
            raise RuntimeError("Pool not started")

        with self._lock:
            self._metrics.total_tasks += 1

        return self._executor.submit(self._enqueue_task, func, args, kwargs, task_id)

    def _enqueue_task(self, func, args, kwargs, task_id):
        """Internal task wrapper."""
        self._task_queue.put((func, args, kwargs, task_id))

    def submit_callback(self, func: Callable, callback: Callable, *args, **kwargs) -> str:
        """Submit task with callback on completion."""
        task_id = str(time.time())

        def wrapped_callback(fut):
            try:
                result = fut.result()
                callback(result, None)
            except Exception as e:
                callback(None, e)

        future = self.submit(func, *args, task_id=task_id, **kwargs)
        future.add_done_callback(wrapped_callback)

        return task_id

    def shutdown(self, wait: bool = True):
        """Shutdown the pool."""
        self._running = False

        # Signal workers to stop
        for _ in range(self.num_workers):
            self._task_queue.put(None)

        self._executor.shutdown(wait=wait)
        self._workers.clear()

    def metrics(self) -> WorkerMetrics:
        """Get pool metrics."""
        with self._lock:
            return WorkerMetrics(
                total_tasks=self._metrics.total_tasks,
                completed_tasks=self._metrics.completed_tasks,
                failed_tasks=self._metrics.failed_tasks,
                avg_latency_ms=self._metrics.avg_latency_ms,
                active_workers=len([t for t in self._workers if t.is_alive()]),
            )

    def wait_completion(self, timeout: Optional[float] = None) -> bool:
        """Wait for all tasks to complete."""
        start = time.time()
        while self._task_queue.qsize() > 0:
            if timeout and (time.time() - start) > timeout:
                return False
            time.sleep(0.1)
        return True


# Global worker pool
_worker_pool: Optional[WorkerPool] = None


def get_worker_pool(num_workers: int = 4) -> WorkerPool:
    """Get or create global worker pool."""
    global _worker_pool
    if _worker_pool is None:
        _worker_pool = WorkerPool(num_workers=num_workers)
        _worker_pool.start()
    return _worker_pool


# Example
if __name__ == "__main__":
    pool = WorkerPool(num_workers=2)
    pool.start()

    def task(x):
        time.sleep(0.5)
        return x * 2

    # Submit tasks
    futures = []
    for i in range(5):
        f = pool.submit(task, i)
        futures.append(f)

    print(f"Metrics: {pool.metrics()}")

    pool.shutdown()
    print("Pool stopped")
