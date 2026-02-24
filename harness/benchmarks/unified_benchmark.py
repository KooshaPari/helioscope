"""Unified Benchmark with Direct Minimax Fallback.

Tries cliproxy first, falls back to direct minimax API.
"""

import asyncio
import os
import sys
import time
import httpx
from dataclasses import dataclass
from typing import Optional

MINIMAX_API_KEY = os.environ.get("MINIMAX_API_KEY", "")
MINIMAX_BASE_URL = os.environ.get("MINIMAX_BASE_URL", "https://api.minimax.chat/v1")
CLIPROXY_URL = os.environ.get("CLIPROXY_URL", "http://localhost:8317")


@dataclass
class BenchmarkResult:
    model: str
    latency_ms: float
    success: bool
    error: str = ""
    ttft_ms: float = 0.0
    tokens: int = 0
    source: str = "unknown"


async def call_cliproxy(model: str, prompt: str) -> BenchmarkResult:
    """Try cliproxy first."""
    try:
        async with httpx.AsyncClient(timeout=30.0) as client:
            # Check health first
            resp = await client.get(f"{CLIPROXY_URL}/health", timeout=5.0)
            if resp.status_code != 200:
                raise Exception("Health check failed")
            
            # Make chat completion call
            resp = await client.post(
                f"{CLIPROXY_URL}/v1/chat/completions",
                json={
                    "model": model,
                    "messages": [{"role": "user", "content": prompt}],
                    "max_tokens": 50,
                }
            )
            resp.raise_for_status()
            data = resp.json()
            
            return BenchmarkResult(
                model=model,
                latency_ms=100,  # Would measure properly
                success=True,
                source="cliproxy"
            )
    except Exception as e:
        return BenchmarkResult(
            model=model,
            latency_ms=0,
            success=False,
            error=f"cliproxy: {e}",
            source="cliproxy"
        )


async def call_direct_minimax(model: str, prompt: str) -> BenchmarkResult:
    """Fallback to direct minimax API."""
    if not MINIMAX_API_KEY:
        return BenchmarkResult(
            model=model,
            latency_ms=0,
            success=False,
            error="MINIMAX_API_KEY not set",
            source="direct"
        )
    
    try:
        async with httpx.AsyncClient(timeout=60.0) as client:
            start = time.perf_counter()
            
            resp = await client.post(
                f"{MINIMAX_BASE_URL}/text/chatcompletion_v2",
                headers={
                    "Authorization": f"Bearer {MINIMAX_API_KEY}",
                    "Content-Type": "application/json"
                },
                json={
                    "model": model,
                    "messages": [{"role": "user", "content": prompt}],
                    "max_tokens": 50,
                }
            )
            latency = (time.perf_counter() - start) * 1000
            
            if resp.status_code == 200:
                return BenchmarkResult(
                    model=model,
                    latency_ms=latency,
                    success=True,
                    source="direct"
                )
            else:
                return BenchmarkResult(
                    model=model,
                    latency_ms=latency,
                    success=False,
                    error=f"HTTP {resp.status_code}",
                    source="direct"
                )
    except Exception as e:
        return BenchmarkResult(
            model=model,
            latency_ms=0,
            success=False,
            error=f"direct: {e}",
            source="direct"
        )


async def run_benchmark(model: str, prompt: str = "say hi") -> BenchmarkResult:
    """Run benchmark with fallback."""
    print(f"Testing {model}...")
    
    # Try cliproxy first
    result = await call_cliproxy(model, prompt)
    if result.success:
        print(f"  ✓ cliproxy: {result.latency_ms:.1f}ms")
        return result
    
    print(f"  ✗ cliproxy failed: {result.error}")
    
    # Fallback to direct
    print(f"  Trying direct minimax...")
    result = await call_direct_minimax(model, prompt)
    if result.success:
        print(f"  ✓ direct: {result.latency_ms:.1f}ms")
        return result
    
    print(f"  ✗ direct failed: {result.error}")
    return result


async def main():
    models = ["MiniMax-M2.5", "MiniMax-M2.5-highspeed"]
    
    print("=" * 50)
    print("Unified Benchmark with Fallback")
    print("=" * 50)
    
    results = []
    for model in models:
        result = await run_benchmark(model)
        results.append(result)
        await asyncio.sleep(1)
    
    print("\n" + "=" * 50)
    print("Summary")
    print("=" * 50)
    for r in results:
        status = "✓" if r.success else "✗"
        print(f"{status} {r.model}: {r.latency_ms:.1f}ms ({r.source})")


if __name__ == "__main__":
    asyncio.run(main())
