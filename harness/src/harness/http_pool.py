"""HTTP connection pooling for efficient network requests.

Provides a shared httpx.Client with connection pooling that can be reused
across all HTTP requests to reduce latency and connection overhead.
"""

import threading
from dataclasses import dataclass
from typing import Any

import httpx


@dataclass
class PoolConfig:
    """Configuration for HTTP connection pool."""

    max_connections: int = 100
    max_keepalive_connections: int = 20
    keepalive_expiry: float = 30.0
    timeout: float = 120.0
    connect_timeout: float = 10.0
    read_timeout: float = 60.0
    pool_timeout: float = 5.0


class HTTPConnectionPool:
    """Singleton HTTP connection pool manager.

    Usage:
        pool = HTTPConnectionPool.get_instance()
        client = pool.get_client()

        # Or use the context manager
        with pool.get_client() as client:
            response = client.get(url)
    """

    _instance: HTTPConnectionPool | None = None
    _lock = threading.Lock()

    def __init__(self, config: PoolConfig | None = None):
        self._config = config or PoolConfig()
        self._client: httpx.Client | None = None
        self._async_client: httpx.AsyncClient | None = None
        self._client_lock = threading.Lock()

    @classmethod
    def get_instance(cls, config: PoolConfig | None = None) -> HTTPConnectionPool:
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
                cls._instance.close()
            cls._instance = None

    def get_client(self) -> httpx.Client:
        """Get a synchronous HTTP client with connection pooling."""
        if self._client is None:
            with self._client_lock:
                if self._client is None:
                    limits = httpx.Limits(
                        max_connections=self._config.max_connections,
                        max_keepalive_connections=self._config.max_keepalive_connections,
                        keepalive_expiry=self._config.keepalive_expiry,
                    )
                    self._client = httpx.Client(
                        limits=limits,
                        timeout=httpx.Timeout(
                            connect=self._config.connect_timeout,
                            read=self._config.read_timeout,
                            pool=self._config.pool_timeout,
                        ),
                        http2=True,  # Enable HTTP/2 for better multiplexing
                    )
        return self._client

    def get_async_client(self) -> httpx.AsyncClient:
        """Get an async HTTP client with connection pooling."""
        if self._async_client is None:
            with self._client_lock:
                if self._async_client is None:
                    limits = httpx.Limits(
                        max_connections=self._config.max_connections,
                        max_keepalive_connections=self._config.max_keepalive_connections,
                        keepalive_expiry=self._config.keepalive_expiry,
                    )
                    self._async_client = httpx.AsyncClient(
                        limits=limits,
                        timeout=httpx.Timeout(
                            connect=self._config.connect_timeout,
                            read=self._config.read_timeout,
                            pool=self._config.pool_timeout,
                        ),
                        http2=True,
                    )
        return self._async_client

    @property
    def config(self) -> PoolConfig:
        """Get pool configuration."""
        return self._config

    def get_stats(self) -> dict[str, Any]:
        """Get pool statistics."""
        stats = {
            "max_connections": self._config.max_connections,
            "max_keepalive": self._config.max_keepalive_connections,
            "keepalive_expiry": self._config.keepalive_expiry,
        }
        if self._client:
            stats["client_active"] = True
        if self._async_client:
            stats["async_client_active"] = True
        return stats

    def close(self) -> None:
        """Close the connection pool."""
        with self._client_lock:
            if self._client:
                self._client.close()
                self._client = None
            if self._async_client:
                # For async, we need to run the close in an event loop
                # This is a simplified version
                try:
                    import asyncio

                    asyncio.get_event_loop().run_until_complete(self._async_client.aclose())
                except Exception:
                    pass
                self._async_client = None


def get_http_client(config: PoolConfig | None = None) -> httpx.Client:
    """Get a shared HTTP client with connection pooling.

    Usage:
        client = get_http_client()
        response = client.get("https://api.example.com")
    """
    return HTTPConnectionPool.get_instance(config).get_client()


def get_async_http_client(config: PoolConfig | None = None) -> httpx.AsyncClient:
    """Get a shared async HTTP client with connection pooling.

    Usage:
        client = await get_async_http_client()
        response = await client.get("https://api.example.com")
    """
    return HTTPConnectionPool.get_instance(config).get_async_client()


# Decorator for easy client usage
def with_client(func):
    """Decorator to automatically inject HTTP client.

    Usage:
        @with_client
        def make_request(client, url):
            return client.get(url)
    """

    def wrapper(*args, **kwargs):
        client = get_http_client()
        return func(client, *args, **kwargs)

    return wrapper
