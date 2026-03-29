"""Request queue with priority support.

Provides async queue with priority levels and backpressure.
"""

import asyncio
import time
from collections import deque
from collections.abc import Callable
from dataclasses import dataclass, field
from enum import Enum
from typing import Any

from .id_utils import new_id


class Priority(Enum):
    """Priority levels."""

    CRITICAL = 0
    HIGH = 1
    NORMAL = 2
    LOW = 3
    BACKGROUND = 4


@dataclass(order=True)
class QueuedRequest:
    """Queued request with priority."""

    priority: int
    timestamp: float = field(compare=True)
    request_id: str = field(compare=False, default_factory=lambda: new_id())
    payload: Any = field(compare=False, default=None)
    callback: Callable | None = field(compare=False, default=None)


class RequestQueue:
    """Async request queue with priority and backpressure.

    Usage:
        queue = RequestQueue(max_size=1000)

        await queue.enqueue(payload, Priority.HIGH)
        item = await queue.dequeue()
    """

    def __init__(self, max_size: int = 1000, max_wait: float = 30.0):
        self.max_size = max_size
        self.max_wait = max_wait
        self._queue = asyncio.PriorityQueue(maxsize=max_size)
        self._waiting = deque()
        self._metrics = {"enqueued": 0, "dequeued": 0, "rejected": 0, "expired": 0}

    async def enqueue(
        self, payload: Any, priority: Priority = Priority.NORMAL, callback: Callable | None = None
    ) -> str:
        """Enqueue a request."""
        request = QueuedRequest(priority=priority.value, timestamp=time.time(), payload=payload, callback=callback)

        try:
            self._queue.put_nowait(request)
            self._metrics["enqueued"] += 1
            return request.request_id
        except asyncio.QueueFull:
            self._metrics["rejected"] += 1
            raise QueueFull(f"Queue at capacity ({self.max_size})")

    async def dequeue(self, timeout: float | None = None) -> QueuedRequest | None:
        """Dequeue a request."""
        timeout = timeout or self.max_wait

        try:
            request = await asyncio.wait_for(self._queue.get(), timeout=timeout)
            self._metrics["dequeued"] += 1
            return request
        except TimeoutError:
            return None

    async def size(self) -> int:
        """Get current queue size."""
        return self._queue.qsize()

    def is_full(self) -> bool:
        """Check if queue is full."""
        return self._queue.full()

    def is_empty(self) -> bool:
        """Check if queue is empty."""
        return self._queue.empty()

    def metrics(self) -> dict[str, int]:
        """Get queue metrics."""
        return self._metrics.copy()


class QueueFull(Exception):
    """Exception raised when queue is full."""

    pass


# Global queue
_queue: RequestQueue | None = None


def get_queue(max_size: int = 1000) -> RequestQueue:
    """Get or create the global queue."""
    global _queue
    if _queue is None:
        _queue = RequestQueue(max_size=max_size)
    return _queue
