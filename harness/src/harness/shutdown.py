"""Graceful shutdown handlers for clean process termination.

Provides signal handling and cleanup utilities for clean shutdown of
long-running processes.
"""

import atexit
import logging
import signal
import sys
import threading
from collections.abc import Callable
from dataclasses import dataclass, field
from enum import Enum
from typing import Any

logger = logging.getLogger(__name__)


class ShutdownStage(Enum):
    """Shutdown stages."""

    INITIATED = "initiated"
    CLEANUP = "cleanup"
    FORCE = "force"
    COMPLETE = "complete"


@dataclass
class ShutdownState:
    """Current shutdown state."""

    stage: ShutdownStage = ShutdownStage.INITIATED
    signal_received: int | None = None
    cleanups_run: int = 0
    errors: list[str] = field(default_factory=list)


class GracefulShutdown:
    """Manage graceful shutdown with cleanup handlers.

    Usage:
        shutdown = GracefulShutdown()

        @shutdown.on_shutdown
        def cleanup_connections():
            # Close connections
            pass

        @shutdown.on_shutdown
        def flush_logs():
            # Flush pending logs
            pass
    """

    _instance: GracefulShutdown | None = None

    def __init__(self):
        self._handlers: list[Callable[[], None]] = []
        self._state = ShutdownState()
        self._lock = threading.Lock()
        self._registered = False

    @classmethod
    def get_instance(cls) -> GracefulShutdown:
        """Get singleton instance."""
        if cls._instance is None:
            cls._instance = cls()
        return cls._instance

    def register(self) -> None:
        """Register signal handlers."""
        if self._registered:
            return

        # Register atexit handler
        atexit.register(self._do_shutdown)

        # Register signal handlers
        signal.signal(signal.SIGTERM, self._signal_handler)
        signal.signal(signal.SIGINT, self._signal_handler)

        # On Windows
        if hasattr(signal, "SIGBREAK"):
            signal.signal(signal.SIGBREAK, self._signal_handler)

        self._registered = True

    def _signal_handler(self, signum: int, frame: Any) -> None:
        """Handle shutdown signal."""
        logger.info(f"Received signal {signum}, initiating graceful shutdown")
        self._state.signal_received = signum
        self._do_shutdown()

    def _do_shutdown(self) -> None:
        """Execute shutdown sequence."""
        with self._lock:
            if self._state.stage == ShutdownStage.COMPLETE:
                return

            self._state.stage = ShutdownStage.CLEANUP

            # Run cleanup handlers
            for handler in self._handlers:
                try:
                    handler()
                    self._state.cleanups_run += 1
                except Exception as e:
                    error = f"Cleanup handler error: {e}"
                    logger.error(error)
                    self._state.errors.append(error)

            self._state.stage = ShutdownStage.COMPLETE

    def on_shutdown(self, func: Callable[[], None]) -> Callable[[], None]:
        """Decorator to register a cleanup handler.

        Usage:
            @shutdown.on_shutdown
            def cleanup():
                pass
        """
        self._handlers.append(func)
        return func

    def add_handler(self, handler: Callable[[], None]) -> None:
        """Add a cleanup handler."""
        self._handlers.append(handler)

    def get_state(self) -> ShutdownState:
        """Get current shutdown state."""
        return self._state

    def force_shutdown(self, code: int = 1) -> None:
        """Force immediate shutdown."""
        self._state.stage = ShutdownStage.FORCE
        sys.exit(code)


class ResourceCleanup:
    """Track and cleanup resources on shutdown."""

    def __init__(self):
        self._resources: list[tuple[str, Callable[[], None]]] = []
        self._lock = threading.Lock()

    def register(self, name: str, cleanup: Callable[[], None]) -> None:
        """Register a resource for cleanup."""
        with self._lock:
            self._resources.append((name, cleanup))

    def cleanup(self) -> None:
        """Clean up all registered resources."""
        with self._lock:
            for name, cleanup in reversed(self._resources):
                try:
                    cleanup()
                    logger.debug(f"Cleaned up: {name}")
                except Exception as e:
                    logger.error(f"Error cleaning up {name}: {e}")

            self._resources.clear()


# Global cleanup instance
_cleanup = ResourceCleanup()


def register_cleanup(name: str, cleanup: Callable[[], None]) -> None:
    """Register a resource for cleanup on shutdown."""
    _cleanup.register(name, cleanup)


def cleanup_all() -> None:
    """Clean up all registered resources."""
    _cleanup.cleanup()


# Register automatic cleanup
atexit.register(cleanup_all)


class ProcessGroup:
    """Manage a group of related processes for coordinated shutdown."""

    def __init__(self, name: str = "process_group"):
        self.name = name
        self._processes: dict[int, Any] = {}
        self._lock = threading.Lock()
        self._shutdown = GracefulShutdown()

        @self._shutdown.on_shutdown
        def kill_processes():
            self.terminate_all()

    def add(self, process: Any) -> int:
        """Add a process to the group."""
        with self._lock:
            self._processes[process.pid] = process
            return process.pid

    def remove(self, pid: int) -> None:
        """Remove a process from the group."""
        with self._lock:
            self._processes.pop(pid, None)

    def terminate_all(self, timeout: float = 10.0) -> None:
        """Terminate all processes in the group."""
        import subprocess

        with self._lock:
            for pid, proc in list(self._processes.items()):
                try:
                    if proc.poll() is None:
                        proc.terminate()
                        try:
                            proc.wait(timeout=timeout / len(self._processes))
                        except subprocess.TimeoutExpired:
                            proc.kill()
                except Exception as e:
                    logger.error(f"Error terminating {pid}: {e}")

            self._processes.clear()

    def __len__(self) -> int:
        """Get number of processes."""
        return len(self._processes)


def get_shutdown_state() -> ShutdownState:
    """Get the current shutdown state."""
    return GracefulShutdown.get_instance().get_state()
