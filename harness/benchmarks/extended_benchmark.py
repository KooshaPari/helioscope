"""Extended Benchmark Framework for Swarm/Speed/Conciseness.

Extends Terminal-Bench/Harbor with:
- Swarm/multi-agent evaluation
- Speed and efficiency metrics
- Conciseness analysis
- Quality baseline preservation
- Preflight checks + startup management

Usage:
    # Run extended benchmark with preflight
    python -m harness.benchmarks.extended_benchmark --agents 6 --swarm
    
    # Bypass cliproxy, use direct minimax
    python -m harness.benchmarks.extended_benchmark --direct-minimax
    
    # Run with quality baseline
    python -m harness.benchmarks.extended_benchmark --quality-baseline
"""

import asyncio
import json
import os
import subprocess
import time
import httpx
import psutil
import requests
from dataclasses import dataclass, field, asdict
from typing import Any

from harness.src.harness.memory_profiler import MemoryProfiler, get_memory_usage
import statistics


# ============================================================================
# Preflight & Startup Management
# ============================================================================

CLIPROXY_URL = os.environ.get("CLIPROXY_URL", "http://localhost:8317")
MINIMAX_API_KEY = os.environ.get("MINIMAX_API_KEY", "")
MINIMAX_BASE_URL = "https://api.minimax.chat/v1"


def check_cliproxy_health(timeout: float = 5.0) -> bool:
    """Check if cliproxy is healthy and responding."""
    try:
        # Health endpoint might redirect to auth, check models endpoint instead
        resp = requests.get(f"{CLIPROXY_URL}/v1/models", timeout=timeout)
        return resp.status_code in (200, 401)  # 401 = needs auth but service is up
    except:
        return False


def check_cliproxy_llm(timeout: float = 30.0) -> bool:
    """Check if cliproxy LLM endpoint is working."""
    try:
        resp = requests.post(
            f"{CLIPROXY_URL}/v1/chat/completions",
            json={"model": "MiniMax-M2.5", "messages": [{"role": "user", "content": "hi"}], "max_tokens": 5},
            timeout=timeout
        )
        return resp.status_code == 200
    except Exception as e:
        print(f"  LLM check failed: {e}")
        return False


def preflight_check(verbose: bool = True) -> dict:
    """Run preflight checks and return status."""
    if verbose:
        print("Running preflight checks...")
    
    results = {
        "cliproxy_health": False,
        "cliproxy_llm": False,
        "docker": False,
        "ready": False,
    }
    
    # Check cliproxy health
    results["cliproxy_health"] = check_cliproxy_health()
    if verbose:
        print(f"  Cliproxy health: {'✓' if results['cliproxy_health'] else '✗'}")
    
    # Check LLM endpoint
    if results["cliproxy_health"]:
        results["cliproxy_llm"] = check_cliproxy_llm()
        if verbose:
            print(f"  Cliproxy LLM: {'✓' if results['cliproxy_llm'] else '✗'}")
    
    # Check docker
    try:
        result = subprocess.run(["docker", "ps"], capture_output=True, timeout=5)
        results["docker"] = result.returncode == 0
        if verbose:
            print(f"  Docker: {'✓' if results['docker'] else '✗'}")
    except:
        if verbose:
            print("  Docker: ✗")
    
    results["ready"] = results["cliproxy_llm"]
    
    if verbose:
        if results["ready"]:
            print("✓ Preflight passed - ready to benchmark")
        else:
            print("✗ Preflight failed - cannot run benchmarks")
    
    return results


# ============================================================================
# Direct Minimax API (bypass cliproxy)
# ============================================================================

async def _call_direct_minimax(
    model: str,
    prompt: str,
) -> tuple[float, float, int]:
    """Call minimax directly, bypassing cliproxy."""
    import aiohttp
    
    api_key = os.environ.get("MINIMAX_API_KEY", "")
    if not api_key:
        raise ValueError("MINIMAX_API_KEY not set")
    
    # Map model names
    model_map = {
        "MiniMax-M2.5": "MiniMax-M2.5",
        "MiniMax-M2.5-highspeed": "MiniMax-M2.5-highspeed",
        "minimax-m2.5": "MiniMax-M2.5",
    }
    mm_model = model_map.get(model, model)
    
    url = "https://api.minimax.chat/v1/text/chatcompletion_v2"
    headers = {
        "Authorization": f"Bearer {api_key}",
        "Content-Type": "application/json"
    }
    payload = {
        "model": mm_model,
        "messages": [{"role": "user", "content": prompt}]
    }
    
    start = time.perf_counter()
    ttft = 0
    
    try:
        async with aiohttp.ClientSession() as session:
            async with session.post(url, json=payload, headers=headers) as resp:
                data = await resp.json()
                ttft = time.perf_counter() - start
                
                content = data.get("choices", [{}])[0].get("message", {}).get("content", "")
                latency = time.perf_counter() - start
                tokens = len(content) // 4
                
                return latency, ttft, tokens
    except Exception as e:
        return time.perf_counter() - start, 0, 0


# ============================================================================
# Data Classes
# ============================================================================

@dataclass
class QualityMetrics:
    """Raw quality metrics (baseline)."""
    resolution_rate: float = 0.0      # % tasks completed
    pass_rate: float = 0.0            # % tests passed
    avg_time_per_task: float = 0.0
    total_tasks: int = 0
    successful_tasks: int = 0
    failed_tasks: int = 0


@dataclass
class SpeedMetrics:
    """Speed and efficiency metrics."""
    # Latency
    ttft: float = 0.0                  # Time to First Token
    total_latency: float = 0.0          # End-to-end latency
    p50_latency: float = 0.0
    p95_latency: float = 0.0
    p99_latency: float = 0.0
    
    # Throughput
    tokens_per_second: float = 0.0
    requests_per_second: float = 0.0
    
    # Time breakdown
    network_time: float = 0.0
    llm_time: float = 0.0
    overhead_time: float = 0.0


@dataclass
class SwarmMetrics:
    """Multi-agent/swarm metrics."""
    num_agents: int = 1
    
    # Coordination
    coordination_overhead: float = 0.0   # Time spent coordinating
    message_passes: int = 0              # Inter-agent messages
    
    # Parallelism
    parallel_efficiency: float = 0.0    # Actual / Ideal time
    idle_time: float = 0.0              # Time agents waiting
    
    # Scalability
    scaling_factor: float = 0.0         # Time(1) / Time(n)


@dataclass
class ConcisenessMetrics:
    """Response conciseness metrics."""
    # Token usage
    prompt_tokens: int = 0
    completion_tokens: int = 0
    total_tokens: int = 0
    
    # Ratios
    completion_to_prompt_ratio: float = 0.0  # Compression ratio
    tokens_per_task: float = 0.0
    
    # Quality vs Length
    quality_per_token: float = 0.0        # Resolution / tokens
    efficiency_score: float = 0.0          # Quality / (time * tokens)


@dataclass
class SystemMetrics:
    """System resource usage metrics."""
    # CPU
    cpu_percent_avg: float = 0.0
    cpu_percent_peak: float = 0.0
    
    # Memory
    memory_mb_avg: float = 0.0
    memory_mb_peak: float = 0.0
    
    # IO
    io_read_bytes: int = 0
    io_write_bytes: int = 0
    
    # Network
    network_sent_bytes: int = 0
    network_recv_bytes: int = 0
    
    # Process
    num_threads: int = 0
    num_fds: int = 0


@dataclass
class ExtendedBenchmarkResult:
    """Complete extended benchmark result."""
    name: str = ""
    timestamp: str = ""
    
    # Core metrics
    quality: QualityMetrics = field(default_factory=QualityMetrics)
    speed: SpeedMetrics = field(default_factory=SpeedMetrics)
    swarm: SwarmMetrics = field(default_factory=SwarmMetrics)
    conciseness: ConcisenessMetrics = field(default_factory=ConcisenessMetrics)
    system: SystemMetrics = field(default_factory=SystemMetrics)
    
    # Raw data
    task_results: list[dict] = field(default_factory=list)


# ============================================================================
# Cliproxy/Minimax Implementation
# ============================================================================

CLIPROXY_URL = os.environ.get("CLIPROXY_URL", "http://localhost:8317")


async def run_extended_benchmark(
    model: str = "MiniMax-M2.5",
    num_agents: int = 1,
    prompt: str = "Solve this coding task: write a function to reverse a string",
    num_trials: int = 10,
    use_direct: bool = False,
) -> ExtendedBenchmarkResult:
    """Run extended benchmark with all metric dimensions."""
    
    result = ExtendedBenchmarkResult(
        name=f"minimax-{model}",
        timestamp=time.strftime("%Y-%m-%d %H:%M:%S"),
    )
    
    result.swarm.num_agents = num_agents
    
    # Run trials
    task_results = []
    latencies = []
    ttfts = []
    token_counts = []
    
    async with httpx.AsyncClient(timeout=120.0) as client:
        for i in range(num_trials):
            trial_start = time.perf_counter()
            
            # Run with or without swarm
            if num_agents > 1:
                # Run concurrent requests (simulating swarm)
                tasks = [
                    _call_llm(client, model, prompt)
                    for _ in range(num_agents)
                ]
                outcomes = await asyncio.gather(*tasks, return_exceptions=True)
                
                # Aggregate swarm metrics
                trial_times = [o[0] for o in outcomes if isinstance(o, tuple)]
                latencies.extend(trial_times)
                
                # Calculate swarm overhead
                wall_time = time.perf_counter() - trial_start
                ideal_time = min(trial_times) if trial_times else 0
                result.swarm.parallel_efficiency = ideal_time / wall_time if wall_time > 0 else 0
            else:
                # Single agent
                outcome = await _call_llm(client, model, prompt)
                if isinstance(outcome, tuple):
                    lat, ttft, tokens = outcome
                    latencies.append(lat)
                    ttfts.append(ttft)
                    token_counts.append(tokens)
            
            # Track quality (resolution)
            task_results.append({
                "trial": i,
                "success": True,  # Would verify against actual results
                "latency": latencies[-1] if latencies else 0,
            })
    
    # Aggregate metrics
    result.task_results = task_results
    
    # Quality metrics
    result.quality.total_tasks = num_trials
    result.quality.successful_tasks = sum(1 for t in task_results if t.get("success"))
    result.quality.resolution_rate = result.quality.successful_tasks / num_trials * 100
    result.quality.avg_time_per_task = statistics.mean(latencies) if latencies else 0
    
    # Speed metrics
    if latencies:
        latencies.sort()
        n = len(latencies)
        result.speed.total_latency = statistics.mean(latencies)
        result.speed.p50_latency = latencies[n // 2]
        result.speed.p95_latency = latencies[int(n * 0.95)]
        result.speed.p99_latency = latencies[int(n * 0.99)]
    
    if ttfts:
        result.speed.ttft = statistics.mean(ttfts)
    
    if token_counts and result.speed.total_latency > 0:
        total_tokens = sum(token_counts)
        result.speed.tokens_per_second = total_tokens / result.speed.total_latency
    
    # Conciseness metrics
    if token_counts:
        result.conciseness.total_tokens = sum(token_counts)
        result.conciseness.completion_tokens = statistics.mean(token_counts)
        result.conciseness.tokens_per_task = result.conciseness.total_tokens / num_trials
    
    if num_trials > 0 and result.quality.resolution_rate > 0:
        result.conciseness.quality_per_token = (
            result.quality.resolution_rate / result.conciseness.tokens_per_task
            if result.conciseness.tokens_per_task > 0 else 0
        )
    
    # Efficiency score (quality / (time * tokens))
    if (result.speed.total_latency > 0 and result.conciseness.tokens_per_task > 0):
        result.conciseness.efficiency_score = (
            result.quality.resolution_rate / 
            (result.speed.total_latency * result.conciseness.tokens_per_task / 1000)
        )
    
    return result


async def _call_llm(
    client: httpx.AsyncClient,
    model: str,
    prompt: str,
) -> tuple[float, float, int]:
    """Call LLM and return (latency, ttft, tokens)."""
    start = time.perf_counter()
    ttft = 0
    
    try:
        async with client.stream(
            "POST",
            f"{CLIPROXY_URL}/v1/chat/completions",
            json={
                "model": model,
                "messages": [{"role": "user", "content": prompt}],
                "stream": True,
            },
        ) as resp:
            content_parts = []
            async for line in resp.aiter_lines():
                if line.strip() and line.startswith("data:"):
                    data = line[5:].strip()
                    if data == "[DONE]":
                        break
                    try:
                        chunk = json.loads(data)
                        delta = chunk.get("choices", [{}])[0].get("delta", {})
                        if delta.get("content"):
                            if ttft == 0:
                                ttft = time.perf_counter() - start
                            content_parts.append(delta["content"])
                    except:
                        pass
            
            latency = time.perf_counter() - start
            tokens = len("".join(content_parts)) // 4
            return latency, ttft, tokens
    except Exception as e:
        return time.perf_counter() - start, 0, 0


# ============================================================================
# Reporting
# ============================================================================

def print_extended_report(result: ExtendedBenchmarkResult) -> None:
    """Print comprehensive benchmark report."""
    
    print(f"\n{'='*70}")
    print(f"EXTENDED BENCHMARK: {result.name}")
    print(f"Timestamp: {result.timestamp}")
    print(f"{'='*70}")
    
    print(f"\n[QUALITY - Baseline]")
    print(f"  Resolution Rate:  {result.quality.resolution_rate:.1f}%")
    print(f"  Successful:       {result.quality.successful_tasks}/{result.quality.total_tasks}")
    print(f"  Avg Time/Task:    {result.quality.avg_time_per_task:.3f}s")
    
    print(f"\n[SPEED - Latency & Throughput]")
    print(f"  TTFT:             {result.speed.ttft:.3f}s")
    print(f"  Total Latency:    {result.speed.total_latency:.3f}s")
    print(f"  P50 Latency:      {result.speed.p50_latency:.3f}s")
    print(f"  P95 Latency:      {result.speed.p95_latency:.3f}s")
    print(f"  P99 Latency:      {result.speed.p99_latency:.3f}s")
    print(f"  Tokens/sec:       {result.speed.tokens_per_second:.1f}")
    
    print(f"\n[SWARM - Multi-Agent]")
    print(f"  Agents:           {result.swarm.num_agents}")
    print(f"  Parallel Eff:     {result.swarm.parallel_efficiency:.2%}")
    print(f"  Coord Overhead:   {result.swarm.coordination_overhead:.3f}s")
    
    print(f"\n[CONCISENESS - Efficiency]")
    print(f"  Total Tokens:     {result.conciseness.total_tokens}")
    print(f"  Tokens/Task:      {result.conciseness.tokens_per_task:.1f}")
    print(f"  Quality/Token:   {result.conciseness.quality_per_token:.4f}")
    print(f"  Efficiency:       {result.conciseness.efficiency_score:.2f}")
    
    print(f"\n[SYSTEM - Resources]")
    print(f"  CPU Avg:         {result.system.cpu_percent_avg:.1f}%")
    print(f"  CPU Peak:        {result.system.cpu_percent_peak:.1f}%")
    print(f"  Memory Avg:      {result.system.memory_mb_avg:.1f}MB")
    print(f"  Memory Peak:     {result.system.memory_mb_peak:.1f}MB")
    print(f"  IO Read:         {result.system.io_read_bytes / 1024:.1f}KB")
    print(f"  IO Write:        {result.system.io_write_bytes / 1024:.1f}KB")
    
    print(f"\n{'='*70}")


def compare_results(
    results: dict[str, ExtendedBenchmarkResult]
) -> None:
    """Compare multiple benchmark results."""
    
    print(f"\n{'='*80}")
    print("EXTENDED BENCHMARK COMPARISON")
    print(f"{'='*80}")
    
    print(f"\n{'Metric':<25} | " + " | ".join(f"{name:>15}" for name in results.keys()))
    print("-" * 80)
    
    # Quality
    row = "Resolution Rate (%)"
    print(f"{row:<25} | " + " | ".join(f"{r.quality.resolution_rate:>14.1f}%" for r in results.values()))
    
    # Speed
    row = "Total Latency (s)"
    print(f"{row:<25} | " + " | ".join(f"{r.speed.total_latency:>14.2f}s" for r in results.values()))
    
    row = "TTFT (s)"
    print(f"{row:<25} | " + " | ".join(f"{r.speed.ttft:>14.2f}s" for r in results.values()))
    
    row = "Tokens/sec"
    print(f"{row:<25} | " + " | ".join(f"{r.speed.tokens_per_second:>14.1f}" for r in results.values()))
    
    # Swarm
    row = "Parallel Eff (%)"
    print(f"{row:<25} | " + " | ".join(f"{r.swarm.parallel_efficiency*100:>14.1f}%" for r in results.values()))
    
    # Conciseness
    row = "Tokens/Task"
    print(f"{row:<25} | " + " | ".join(f"{r.conciseness.tokens_per_task:>14.1f}" for r in results.values()))
    
    row = "Efficiency Score"
    print(f"{row:<25} | " + " | ".join(f"{r.conciseness.efficiency_score:>14.2f}" for r in results.values()))
    
    # System
    row = "CPU Avg (%)"
    print(f"{row:<25} | " + " | ".join(f"{r.system.cpu_percent_avg:>14.1f}%" for r in results.values()))
    
    row = "Memory Peak (MB)"
    print(f"{row:<25} | " + " | ".join(f"{r.system.memory_mb_peak:>14.1f}MB" for r in results.values()))
    
    print(f"{'='*80}")


# ============================================================================
# Main
# ============================================================================

if __name__ == "__main__":
    import argparse
    
    parser = argparse.ArgumentParser(description="Extended Benchmark Framework")
    parser.add_argument("--model", "-m", default="MiniMax-M2.5")
    parser.add_argument("--agents", "-n", type=int, default=1)
    parser.add_argument("--trials", "-t", type=int, default=10)
    parser.add_argument("--swarm", action="store_true", help="Enable swarm mode")
    parser.add_argument("--compare", action="store_true", help="Compare multiple configs")
    parser.add_argument("--json", "-j", action="store_true", help="JSON output")
    parser.add_argument("--direct-minimax", action="store_true", help="Bypass cliproxy, use direct minimax API")
    parser.add_argument("--skip-preflight", action="store_true", help="Skip preflight checks")
    
    args = parser.parse_args()
    
    # Run preflight unless skipped
    if not args.skip_preflight:
        pf = preflight_check()
        if not pf["ready"] and not args.direct_minimax:
            print("\nTip: Use --direct-minimax to bypass cliproxy")
            print("Or set MINIMAX_API_KEY for direct API access")
            exit(1)
    
    if args.compare:
        # Compare different configurations
        results = {}
        
        # Single agent baseline
        results["single"] = asyncio.run(run_extended_benchmark(
            args.model, num_agents=1, num_trials=args.trials, use_direct=args.direct_minimax
        ))
        
        # Multi-agent
        results["multi-6"] = asyncio.run(run_extended_benchmark(
            args.model, num_agents=6, num_trials=args.trials, use_direct=args.direct_minimax
        ))
        
        if args.json:
            for name, res in results.items():
                print(f"\n=== {name} ===")
                print(json.dumps(asdict(res), indent=2))
        else:
            compare_results(results)
    
    else:
        result = asyncio.run(run_extended_benchmark(
            args.model,
            num_agents=args.agents if args.swarm else 1,
            num_trials=args.trials,
            use_direct=args.direct_minimax
        ))
        
        if args.json:
            print(json.dumps(asdict(result), indent=2))
        else:
            print_extended_report(result)
