"""Async runtime management for efficient event loop reuse.

Provides a singleton event loop that can be reused across operations to reduce
overhead from creating new event loops.
"""

import asyncio
import threading
from typing import Optional, Any, Callable, Awaitable
from dataclasses import dataclass
from functools import wraps


@dataclass
class AsyncConfig:
    """Async runtime configuration."""
    max_workers: int = 10
    loop_timeout: float = 300.0
    debug: bool = False


class AsyncRuntime:
    """Singleton async runtime with reusable event loop.
    
    Usage:
        runtime = AsyncRuntime.get_instance()
        
        # Run async code
        result = runtime.run(async_function())
        
        # Or use the decorator
        @runtime.async_handler
        async def my_async_func():
            await do_something()
    """
    
    _instance: Optional['AsyncRuntime'] = None
    _lock = threading.Lock()
    
    def __init__(self, config: Optional[AsyncConfig] = None):
        self._config = config or AsyncConfig()
        self._loop: Optional[asyncio.AbstractEventLoop] = None
        self._thread: Optional[threading.Thread] = None
        self._running = False
        self._loop_lock = threading.Lock()
    
    @classmethod
    def get_instance(cls, config: Optional[AsyncConfig] = None) -> 'AsyncRuntime':
        """Get singleton instance."""
        if cls._instance is None:
            with cls._lock:
                if cls._instance is None:
                    cls._instance = cls(config)
        return cls._instance
    
    @classmethod
    def reset_instance(cls) -> None:
        """Reset singleton (for testing)."""
        with cls._lock:
            if cls._instance:
                cls._instance.shutdown()
            cls._instance = None
    
    def _create_loop(self) -> asyncio.AbstractEventLoop:
        """Create a new event loop with optimal settings."""
        loop = asyncio.new_event_loop()
        loop.set_debug(self._config.debug)
        
        # Set loop as current
        asyncio.set_event_loop(loop)
        
        return loop
    
    def start(self) -> None:
        """Start the async runtime (background thread)."""
        if self._running:
            return
        
        with self._loop_lock:
            if self._running:
                return
            
            self._loop = self._create_loop()
            self._running = True
            
            # Start background thread
            self._thread = threading.Thread(
                target=self._run_loop,
                daemon=True,
                name="AsyncRuntime",
            )
            self._thread.start()
    
    def _run_loop(self) -> None:
        """Run the event loop in background thread."""
        if not self._loop:
            return
        
        try:
            self._loop.run_forever()
        except Exception:
            pass
        finally:
            self._loop.close()
    
    def run(self, coro: Awaitable[Any]) -> Any:
        """Run a coroutine in the shared event loop.
        
        This avoids the overhead of creating a new event loop each time.
        """
        # Start runtime if not running
        if not self._running:
            self.start()
        
        # If we're already in the right thread, we can use the loop directly
        try:
            loop = asyncio.get_running_loop()
            # We're in an async context, just await
            return asyncio.create_task(coro)
        except RuntimeError:
            # No running loop, use our shared one
            if self._loop and self._running:
                return asyncio.run_coroutine_threadsafe(coro, self._loop).result(
                    timeout=self._config.loop_timeout
                )
            else:
                # Fallback to new loop
                return asyncio.run(coro)
    
    def run_async(self, coro: Awaitable[Any]) -> Any:
        """Alias for run()."""
        return self.run(coro)
    
    def shutdown(self) -> None:
        """Shutdown the async runtime."""
        self._running = False
        
        if self._loop:
            self._loop.call_soon_threadsafe(self._loop.stop)
        
        if self._thread:
            self._thread.join(timeout=5)
        
        with self._loop_lock:
            self._loop = None
    
    def get_stats(self) -> dict[str, Any]:
        """Get runtime statistics."""
        return {
            'running': self._running,
            'has_loop': self._loop is not None,
            'max_workers': self._config.max_workers,
        }
    
    def async_handler(self, func: Callable[..., Awaitable[Any]]) -> Callable:
        """Decorator to wrap async functions with the runtime.
        
        Usage:
            @runtime.async_handler
            async def my_func():
                await do_something()
        """
        @wraps(func)
        def wrapper(*args, **kwargs):
            coro = func(*args, **kwargs)
            return self.run(coro)
        return wrapper


# Global instance
_default_runtime: Optional[AsyncRuntime] = None


def get_async_runtime(config: Optional[AsyncConfig] = None) -> AsyncRuntime:
    """Get the default async runtime."""
    global _default_runtime
    if _default_runtime is None:
        _default_runtime = AsyncRuntime.get_instance(config)
    return _default_runtime


def run_async(coro: Awaitable[Any]) -> Any:
    """Run a coroutine using the default runtime."""
    return get_async_runtime().run(coro)


# Optimized async utilities
async def gather_with_limit(
    *tasks: Awaitable[Any],
    max_concurrent: int = 10,
) -> list[Any]:
    """Run tasks with a concurrency limit.
    
    Usage:
        results = await gather_with_limit(
            task1(), task2(), task3(),
            max_concurrent=5
        )
    """
    semaphore = asyncio.Semaphore(max_concurrent)
    
    async def bounded_task(task: Awaitable[Any]) -> Any:
        async with semaphore:
            return await task
    
    return await asyncio.gather(*(bounded_task(t) for t in tasks))


async def run_with_timeout(
    coro: Awaitable[Any],
    timeout: float,
) -> Any:
    """Run coroutine with timeout."""
    try:
        return await asyncio.wait_for(coro, timeout=timeout)
    except asyncio.TimeoutError:
        raise TimeoutError(f"Operation timed out after {timeout}s")


class AsyncBatch:
    """Batch async operations for efficiency."""
    
    def __init__(self, batch_size: int = 10, flush_timeout: float = 1.0):
        self._batch_size = batch_size
        self._flush_timeout = flush_timeout
        self._queue: asyncio.Queue = asyncio.Queue()
        self._running = False
        self._task: Optional[asyncio.Task] = None
    
    async def _processor(self) -> None:
        """Process batches."""
        while self._running:
            batch = []
            
            # Collect batch
            try:
                while len(batch) < self._batch_size:
                    try:
                        item = await asyncio.wait_for(
                            self._queue.get(),
                            timeout=self._flush_timeout,
                        )
                        batch.append(item)
                    except asyncio.TimeoutError:
                        break
            except Exception:
                pass
            
            if batch:
                # Process batch (override in subclass)
                await self._process_batch(batch)
    
    async def _process_batch(self, batch: list) -> None:
        """Override to process batch."""
        pass
    
    async def submit(self, item: Any) -> None:
        """Submit item to batch."""
        await self._queue.put(item)
    
    async def start(self) -> None:
        """Start batch processor."""
        if self._running:
            return
        self._running = True
        self._task = asyncio.create_task(self._processor())
    
    async def stop(self) -> None:
        """Stop batch processor."""
        self._running = False
        if self._task:
            self._task.cancel()
            try:
                await self._task
            except asyncio.CancelledError:
                pass
