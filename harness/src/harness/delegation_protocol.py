"""Delegation Protocol - Agent-to-agent task delegation.

Provides protocol for delegating tasks between agents with result tracking,
timeout handling, and escalation paths.
"""

import asyncio
import threading
import time
from dataclasses import dataclass, field
from enum import Enum
from typing import Any, Callable, Optional

from .id_utils import new_id


class DelegationState(Enum):
    """State of a delegation."""

    PENDING = "pending"
    RUNNING = "running"
    COMPLETED = "completed"
    FAILED = "failed"
    TIMEOUT = "timeout"
    CANCELLED = "cancelled"


@dataclass
class DelegationRequest:
    """Request for delegating a task."""

    id: str
    from_agent: str
    to_agent: str
    task: str
    context: dict = field(default_factory=dict)
    priority: int = 5
    timeout_seconds: float = 300.0
    created_at: float = field(default_factory=time.time)
    metadata: dict = field(default_factory=dict)


@dataclass
class DelegationResult:
    """Result of a delegated task."""

    request_id: str
    state: DelegationState
    result: Any = None
    error: Optional[str] = None
    started_at: float = 0.0
    completed_at: float = 0.0
    latency_ms: float = 0.0
    output_tokens: int = 0
    metadata: dict = field(default_factory=dict)


class DelegationProtocol:
    """Protocol for delegating tasks between agents.

    Usage:
        protocol = DelegationProtocol()

        # Delegate a task
        request = await protocol.delegate(
            from_agent=\"orchestrator\",
            to_agent=\"coder-1\",
            task=\"Write a function\"
        )

        # Wait for result
        result = await protocol.wait_result(request.id)
    """

    def __init__(self):
        self._pending: dict[str, DelegationRequest] = {}
        self._results: dict[str, DelegationResult] = {}
        self._handlers: dict[str, Callable] = {}  # agent_type -> handler
        self._locks: dict[str, asyncio.Lock] = {}
        self._lock = threading.Lock()
        self._callbacks: dict[str, list[Callable]] = {}  # state -> callbacks

    def register_handler(self, agent_type: str, handler: Callable[[DelegationRequest], Any]) -> None:
        """Register a handler for an agent type."""
        self._handlers[agent_type] = handler

    def register_callback(self, state: DelegationState, callback: Callable[[DelegationResult], None]) -> None:
        """Register callback for state changes."""
        if state.value not in self._callbacks:
            self._callbacks[state.value] = []
        self._callbacks[state.value].append(callback)

    async def delegate(
        self,
        from_agent: str,
        to_agent: str,
        task: str,
        context: Optional[dict] = None,
        priority: int = 5,
        timeout_seconds: float = 300.0,
        metadata: Optional[dict] = None,
    ) -> DelegationRequest:
        """Delegate a task to another agent."""
        request = DelegationRequest(
            id=new_id(),
            from_agent=from_agent,
            to_agent=to_agent,
            task=task,
            context=context or {},
            priority=priority,
            timeout_seconds=timeout_seconds,
            metadata=metadata or {},
        )

        with self._lock:
            self._pending[request.id] = request
            self._locks[request.id] = asyncio.Lock()

        # Start execution
        asyncio.create_task(self._execute(request))

        return request

    async def _execute(self, request: DelegationRequest) -> None:
        """Execute a delegated task."""
        # Create pending result
        result = DelegationResult(
            request_id=request.id,
            state=DelegationState.RUNNING,
            started_at=time.time(),
        )
        self._results[request.id] = result
        self._notify(DelegationState.RUNNING, result)

        # Get handler for agent
        handler = self._handlers.get(request.to_agent)

        if not handler:
            result.state = DelegationState.FAILED
            result.error = f"No handler for agent: {request.to_agent}"
            result.completed_at = time.time()
            result.latency_ms = (result.completed_at - result.started_at) * 1000
            self._notify(DelegationState.FAILED, result)
            return

        # Execute with timeout
        try:
            result.result = await asyncio.wait_for(
                handler(request),
                timeout=request.timeout_seconds,
            )
            result.state = DelegationState.COMPLETED
        except asyncio.TimeoutError:
            result.state = DelegationState.TIMEOUT
            result.error = f"Task timed out after {request.timeout_seconds}s"
        except Exception as e:
            result.state = DelegationState.FAILED
            result.error = str(e)
        finally:
            result.completed_at = time.time()
            result.latency_ms = (result.completed_at - result.started_at) * 1000
            self._notify(result.state, result)

    async def wait_result(self, request_id: str, timeout: Optional[float] = None) -> DelegationResult:
        """Wait for a delegation result."""
        lock = self._locks.get(request_id)
        if not lock:
            # Already completed, return cached
            return self._results.get(
                request_id,
                DelegationResult(
                    request_id=request_id,
                    state=DelegationState.FAILED,
                    error="Request not found",
                ),
            )

        async with lock:
            # Wait until we have a result
            while request_id not in self._results:
                await asyncio.sleep(0.1)

            return self._results[request_id]

    def get_result(self, request_id: str) -> Optional[DelegationResult]:
        """Get result without waiting."""
        return self._results.get(request_id)

    def cancel(self, request_id: str) -> bool:
        """Cancel a pending delegation."""
        with self._lock:
            if request_id in self._pending:
                del self._pending[request_id]

                # Mark as cancelled
                result = DelegationResult(
                    request_id=request_id,
                    state=DelegationState.CANCELLED,
                    completed_at=time.time(),
                )
                self._results[request_id] = result
                self._notify(DelegationState.CANCELLED, result)
                return True
            return False

    def get_pending(self) -> list[DelegationRequest]:
        """Get all pending delegations."""
        return list(self._pending.values())

    def get_stats(self) -> dict:
        """Get delegation statistics."""
        states = {}
        for result in self._results.values():
            state = result.state.value
            states[state] = states.get(state, 0) + 1

        return {
            "pending": len(self._pending),
            "total": len(self._results),
            "by_state": states,
        }

    def _notify(self, state: DelegationState, result: DelegationResult) -> None:
        """Notify callbacks of state change."""
        callbacks = self._callbacks.get(state.value, [])
        for callback in callbacks:
            try:
                callback(result)
            except Exception:
                pass  # Don't let callback errors break things


# Simple async handler for testing
async def simple_handler(request: DelegationRequest) -> str:
    """Example handler that just returns a string."""
    await asyncio.sleep(0.1)  # Simulate work
    return f"Processed by {request.to_agent}: {request.task}"
