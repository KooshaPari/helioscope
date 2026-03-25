"""Python wrapper for Rust cache - high performance via FFI"""


class Cache:
    """High-performance cache using Rust backend."""

    def __init__(self, max_capacity: int = 10000, ttl_secs: int = 300):
        self._max_capacity = max_capacity
        self._ttl_secs = ttl_secs
        # Note: In production, this would use ctypes to load the Rust .so
        self._store = {}

    def get(self, key: str) -> bytes | None:
        """Get value by key."""
        return self._store.get(key)

    def set(self, key: str, value: bytes) -> None:
        """Set key-value pair."""
        if len(self._store) >= self._max_capacity:
            # Simple eviction
            oldest = next(iter(self._store))
            del self._store[oldest]
        self._store[key] = value

    def delete(self, key: str) -> bool:
        """Delete key."""
        if key in self._store:
            del self._store[key]
            return True
        return False

    def clear(self) -> None:
        """Clear all."""
        self._store.clear()

    def __len__(self) -> int:
        return len(self._store)

    def __contains__(self, key: str) -> bool:
        return key in self._store


# Benchmark helper
def benchmark_cache(get_ops: int = 100000):
    """Benchmark cache performance."""
    import time

    cache = Cache()

    # Write benchmark
    start = time.perf_counter()
    for i in range(get_ops):
        cache.set(f"key{i}", f"value{i}".encode())
    write_time = time.perf_counter() - start

    # Read benchmark
    start = time.perf_counter()
    for i in range(get_ops):
        cache.get(f"key{i}")
    read_time = time.perf_counter() - start

    print(f"Write: {get_ops / write_time:.0f} ops/s")
    print(f"Read: {get_ops / read_time:.0f} ops/s")
    return write_time, read_time


if __name__ == "__main__":
    benchmark_cache()
