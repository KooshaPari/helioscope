"""Request batching for optimizing network calls.

Provides utilities to batch multiple requests together to reduce network overhead.
"""

import asyncio
import time
import threading
from dataclasses import dataclass, field
from typing import Any, Callable, Generic, TypeVar, Optional
from collections import deque
import logging


logger = logging.getLogger(__name__)


T = TypeVar("T")
R = TypeVar("R")


@dataclass
class BatchRequest(Generic[T]):
    """A single request in a batch."""

    id: str
    args: tuple = field(default_factory=tuple)
    kwargs: dict = field(default_factory=dict)
    future: Optional[asyncio.Future] = None
    timestamp: float = field(default_factory=time.time)


@dataclass
class BatchResponse(Generic[R]):
    """Response for a batched request."""

    request_id: str
    result: Optional[R] = None
    error: Optional[Exception] = None
    latency_ms: float = 0.0


class RequestBatcher(Generic[T, R]):
    """Batches multiple requests for efficient processing.

    Usage:
        async def process_batch(items):
            # Process all items together
            return [process(item) for item in items]

        batcher = RequestBatcher(process_batch, batch_size=10, flush_timeout=0.1)

        # Queue requests
        result1 = await batcher.submit("req1", item1)
        result2 = await batcher.submit("req2", item2)
    """

    def __init__(
        self,
        processor: Callable[[list[T]], list[R]],
        batch_size: int = 10,
        flush_timeout: float = 0.1,
        max_queue_size: int = 1000,
    ):
        self._processor = processor
        self._batch_size = batch_size
        self._flush_timeout = flush_timeout
        self._max_queue_size = max_queue_size

        self._queue: deque[BatchRequest[T]] = deque()
        self._lock = threading.Lock()
        self._processing = False
        self._flush_event = asyncio.Event()

        # Metrics
        self._total_requests = 0
        self._total_batches = 0
        self._total_errors = 0

    async def submit(self, request_id: str, *args, **kwargs) -> R:
        """Submit a request to be batched."""
        # Check queue size
        with self._lock:
            if len(self._queue) >= self._max_queue_size:
                raise QueueFullError(f"Queue full ({self._max_queue_size})")

            future = asyncio.Future()
            request = BatchRequest(
                id=request_id,
                args=args,
                kwargs=kwargs,
                future=future,
            )
            self._queue.append(request)
            self._total_requests += 1

        # Trigger flush if batch is full
        if len(self._queue) >= self._batch_size:
            asyncio.create_task(self._flush())

        return await future

    async def _flush(self) -> None:
        """Process current batch."""
        with self._lock:
            if self._processing:
                return

            if not self._queue:
                return

            # Take batch
            batch = []
            while self._queue and len(batch) < self._batch_size:
                batch.append(self._queue.popleft())

            self._processing = True

        # Process batch
        try:
            args_list = [req.args for req in batch]
            kwargs_list = [req.kwargs for req in batch]

            # Call processor with all args
            # Simplified: assume processor takes list of first args
            first_args = [req.args[0] if req.args else None for req in batch]
            results = await self._processor(first_args)

            # Map results back
            for i, req in enumerate(batch):
                if req.future and not req.future.done():
                    if i < len(results):
                        req.future.set_result(results[i])
                    else:
                        req.future.set_exception(BatchError("No result"))

            self._total_batches += 1

        except Exception as e:
            logger.error(f"Batch processing error: {e}")
            self._total_errors += 1

            # Set exception for all
            for req in batch:
                if req.future and not req.future.done():
                    req.future.set_exception(e)

        finally:
            with self._lock:
                self._processing = False

    async def flush(self) -> None:
        """Force flush pending requests."""
        await self._flush()

    def get_stats(self) -> dict:
        """Get batching statistics."""
        return {
            "queue_size": len(self._queue),
            "total_requests": self._total_requests,
            "total_batches": self._total_batches,
            "total_errors": self._total_errors,
            "avg_batch_size": self._total_requests / max(1, self._total_batches),
        }


class QueueFullError(Exception):
    """Raised when request queue is full."""

    pass


class BatchError(Exception):
    """Raised when batch processing fails."""

    pass


# Synchronous version
class SyncRequestBatcher(Generic[T, R]):
    """Synchronous request batcher."""

    def __init__(
        self,
        processor: Callable[[list[T]], list[R]],
        batch_size: int = 10,
        flush_timeout: float = 0.1,
    ):
        self._processor = processor
        self._batch_size = batch_size
        self._flush_timeout = flush_timeout
        self._queue: list[BatchRequest[T]] = []
        self._lock = threading.Lock()
        self._last_flush = time.time()

    def submit(self, request_id: str, item: T) -> R:
        """Submit a synchronous request."""
        with self._lock:
            future = threading.Event()
            request = BatchRequest(
                id=request_id,
                args=(item,),
                future=future,  # type: ignore
            )
            self._queue.append(request)

            # Flush if needed
            if len(self._queue) >= self._batch_size:
                self._flush_locked()

        # Wait for result
        future.wait()
        return future.result

    def _flush_locked(self) -> None:
        """Flush queue (must hold lock)."""
        if not self._queue:
            return

        batch = self._queue[: self._batch_size]
        self._queue = self._queue[self._batch_size :]

        try:
            items = [req.args[0] for req in batch]
            results = self._processor(items)

            for i, req in enumerate(batch):
                if hasattr(req.future, "set_result"):
                    req.future.set_result(results[i] if i < len(results) else None)
        except Exception as e:
            for req in batch:
                if hasattr(req.future, "set_exception"):
                    req.future.set_exception(e)

    def flush(self) -> None:
        """Force flush."""
        with self._lock:
            self._flush_locked()


def batch_requests(batch_size: int = 10, timeout: float = 0.1):
    """Decorator to automatically batch function calls.

    Usage:
        @batch_requests(batch_size=5)
        def make_api_call(items):
            # Batch API call
            return [process(i) for i in items]
    """

    def decorator(func: Callable) -> Callable:
        batcher = RequestBatcher(func, batch_size=batch_size, flush_timeout=timeout)

        async def wrapper(*args, **kwargs):
            return await batcher.submit(str(id(args)), *args, **kwargs)

        return wrapper

    return decorator
