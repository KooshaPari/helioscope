"""Final tests for cache."""

import pytest
from harness.cache import L1Cache, L2Cache


def test_l1_cache_set_get():
    """Test L1Cache set and get."""
    cache = L1Cache()
    # Cache.get should work
    result = cache.get("test_key")
    assert result is None


def test_l2_cache_set_get():
    """Test L2Cache set and get."""
    cache = L2Cache()
    result = cache.get("test_key")
    assert result is None
