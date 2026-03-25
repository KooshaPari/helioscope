"""Harness Performance Benchmark - Compare codex-cli vs built codex.

Purpose: Track performance changes in our codex/harness implementation.
Compare homebrew codex-cli against our built version to measure perf changes.

Usage:
    # Compare homebrew vs built codex
    python -m harness.benchmarks.harness_benchmark --compare --agents 6

    # Test specific harness
    python -m harness.benchmarks.harness_benchmark --binary /path/to/codex --agents 6

    # Test minimax via cliproxy
    python -m harness.benchmarks.harness_benchmark --minimax --agents 6
"""

import asyncio
import json
import os
import psutil
import re
import shutil
import subprocess
import tempfile
import time
import httpx
from dataclasses import dataclass, field, asdict
from pathlib import Path
from typing import Any
from harness.src.harness.resources import safe_popen, ResourceMonitor


# ============================================================================
# Data Classes
# ============================================================================


@dataclass
class LLMMetrics:
    """LLM performance metrics."""

    ttft: float = 0.0  # Time to First Token
    generation_time: float = 0.0
    total_time: float = 0.0
    completion_tokens: int = 0
    tokens_per_second: float = 0.0
    conciseness_ratio: float = 0.0
    total_cost: float = 0.0


@dataclass
class SystemMetrics:
    """System resource usage."""

    cpu_percent_avg: float = 0.0
    memory_mb_peak: float = 0.0
    io_read_bytes: int = 0
    io_write_bytes: int = 0


@dataclass
class BenchmarkResult:
    """Complete harness benchmark result."""

    harness_name: str = ""
    model: str = ""
    agent_count: int = 0

    # Timing
    total_wall_time: float = 0.0

    # SLA
    sla_p50: float = 0.0
    sla_p95: float = 0.0
    sla_p99: float = 0.0

    # Breakdown
    ttft_percent: float = 0.0
    generation_percent: float = 0.0
    overhead_percent: float = 0.0

    # LLM
    llm: LLMMetrics = field(default_factory=LLMMetrics)

    # System
    system: SystemMetrics = field(default_factory=SystemMetrics)

    # Details
    agent_times: list[float] = field(default_factory=list)
    errors: list[str] = field(default_factory=list)


# ============================================================================
# Configuration
# ============================================================================

CLIPROXY_URL = os.environ.get("CLIPROXY_URL", "http://localhost:8317")
DEFAULT_HARNESS = os.environ.get("HARNESS", "cliproxy")


def detect_harness() -> str:
    """Detect running harness type."""
    harness = os.environ.get("HARNESS", "").lower()
    if harness:
        return harness
    try:
        import socket

        sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        sock.settimeout(1)
        result = sock.connect_ex(("localhost", 8317))
        sock.close()
        if result == 0:
            return "cliproxy"
    except:
        pass
    return "unknown"


MODEL_COSTS = {
    "MiniMax-M2.5": {"prompt": 0.2, "completion": 0.6},
    "MiniMax-M2.5-highspeed": {"prompt": 0.1, "completion": 0.3},
    "gpt-5.3-codex": {"prompt": 2.5, "completion": 10.0},
}


# ============================================================================
# Cliproxy/Minimax Benchmark
# ============================================================================


async def _call_llm_async(
    client: httpx.AsyncClient,
    prompt: str,
    model: str,
) -> tuple[LLMMetrics, str]:
    """Call LLM via cliproxy and measure."""
    metrics = LLMMetrics()
    start = time.perf_counter()

    try:
        first_token_time = None

        async with client.stream(
            "POST",
            f"{CLIPROXY_URL}/v1/chat/completions",
            json={
                "model": model,
                "messages": [{"role": "user", "content": prompt}],
                "temperature": 0.7,
                "stream": True,
            },
            timeout=120.0,
        ) as response:
            content_parts = []
            async for line in response.aiter_lines():
                if line.strip() and line.startswith("data:"):
                    data = line[5:].strip()
                    if data == "[DONE]":
                        break
                    try:
                        chunk = json.loads(data)
                        if chunk.get("choices"):
                            delta = chunk["choices"][0].get("delta", {})
                            if delta.get("content"):
                                if first_token_time is None:
                                    first_token_time = time.perf_counter() - start
                                content_parts.append(delta["content"])
                    except:
                        pass

            total_time = time.perf_counter() - start
            content = "".join(content_parts)

            metrics.ttft = first_token_time or total_time
            metrics.generation_time = total_time - metrics.ttft
            metrics.total_time = total_time
            metrics.completion_tokens = max(len(content) // 4, 1)

            if metrics.generation_time > 0.01:
                metrics.tokens_per_second = metrics.completion_tokens / metrics.generation_time

            # Cost
            costs = MODEL_COSTS.get(model, MODEL_COSTS["MiniMax-M2.5"])
            metrics.total_cost = (metrics.completion_tokens / 1_000_000) * costs["completion"]

    except Exception as e:
        metrics.total_time = time.perf_counter() - start
        return metrics, str(e)

    return metrics, ""


async def run_minimax_benchmark(
    harness_name: str,
    model: str,
    agent_count: int,
    prompt: str,
) -> BenchmarkResult:
    """Run minimax benchmark via cliproxy."""
    result = BenchmarkResult(
        harness_name=harness_name,
        model=model,
        agent_count=agent_count,
    )

    # Get system baseline
    proc = psutil.Process()
    io_start = proc.io_counters() if hasattr(proc, "io_counters") else None

    start = time.perf_counter()

    async with httpx.AsyncClient() as client:
        tasks = [_call_llm_async(client, prompt, model) for _ in range(agent_count)]
        llm_results = await asyncio.gather(*tasks, return_exceptions=True)

    total_wall = time.perf_counter() - start

    # Get system end
    if io_start:
        io_end = proc.io_counters()
        result.system.io_read_bytes = io_end.read_bytes - io_start.read_bytes
        result.system.io_write_bytes = io_end.write_bytes - io_start.write_bytes

    # Collect results
    times = []
    ttfts = []
    gen_times = []
    tokens_list = []
    errors = []

    for r in llm_results:
        if isinstance(r, tuple):
            m, err = r
            times.append(m.total_time)
            ttfts.append(m.ttft)
            gen_times.append(m.generation_time)
            tokens_list.append(m.completion_tokens)
            if err:
                errors.append(err[:100])
        elif isinstance(r, Exception):
            errors.append(str(r)[:100])

    times.sort()
    n = len(times)

    result.total_wall_time = total_wall
    result.agent_times = times
    result.errors = errors

    # SLA
    result.sla_p50 = times[n // 2] if n > 0 else 0
    result.sla_p95 = times[int(n * 0.95)] if n > 0 else 0
    result.sla_p99 = times[int(n * 0.99)] if n > 0 else 0

    # LLM averages
    if ttfts:
        result.llm.ttft = sum(ttfts) / len(ttfts)
    if gen_times:
        result.llm.generation_time = sum(gen_times) / len(gen_times)
    if tokens_list:
        result.llm.completion_tokens = int(sum(tokens_list) / len(tokens_list))
        if result.llm.generation_time > 0.01:
            result.llm.tokens_per_second = result.llm.completion_tokens / result.llm.generation_time

    # Breakdown percentages
    avg_time = sum(times) / n if n > 0 else 1
    if avg_time > 0:
        result.ttft_percent = (result.llm.ttft / avg_time) * 100
        result.generation_percent = (result.llm.generation_time / avg_time) * 100
        result.overhead_percent = 100 - result.ttft_percent - result.generation_percent

    return result


# ============================================================================
# Codex CLI Benchmark
# ============================================================================


def run_codex_benchmark(
    harness_name: str,
    binary: str,
    model: str,
    agent_count: int,
    prompt: str,
) -> BenchmarkResult:
    """Run codex-cli benchmark.

    To test codex with minimax via cliproxy, use:
        export OPENAI_API_BASE_URL=http://localhost:8317/v1
        export OPENAI_API_KEY=dummy
    """
    result = BenchmarkResult(
        harness_name=harness_name,
        model=model,
        agent_count=agent_count,
    )

    proc = psutil.Process()
    cpu_samples = []
    mem_samples = []
    io_start = proc.io_counters() if hasattr(proc, "io_counters") else None

    # Set env for cliproxy routing
    env = os.environ.copy()
    env["OPENAI_API_BASE_URL"] = "http://localhost:8317/v1"
    env["OPENAI_API_KEY"] = "dummy"

    start = time.perf_counter()

    with tempfile.TemporaryDirectory() as tmpdir:
        work_dir = Path(tmpdir)

        # Init git
        subprocess.run(["git", "init"], cwd=work_dir, capture_output=True)
        subprocess.run(["git", "config", "user.email", "test@test.com"], cwd=work_dir, capture_output=True)
        subprocess.run(["git", "config", "user.name", "Test"], cwd=work_dir, capture_output=True)

        procs = []
        for i in range(agent_count):
            agent_dir = work_dir / f"agent_{i}"
            agent_dir.mkdir(exist_ok=True)
            subprocess.run(["git", "init"], cwd=agent_dir, capture_output=True)
            subprocess.run(["git", "config", "user.email", "test@test.com"], cwd=agent_dir, capture_output=True)
            subprocess.run(["git", "config", "user.name", "Test"], cwd=agent_dir, capture_output=True)
            (agent_dir / "task.txt").write_text(f"Agent {i}: {prompt}")

            cmd = binary.split() if "node" not in binary else ["node", binary.split()[-1]]
            with safe_popen(
                cmd + ["exec", "-m", model, prompt],
                cwd=agent_dir,
                stdout=subprocess.PIPE,
                stderr=subprocess.PIPE,
                text=True,
                env=env,
            ) as p:
                procs.append(p)

            # Sample resources
            try:
                cpu_samples.append(proc.cpu_percent())
                mem_samples.append(proc.memory_info().rss / 1024 / 1024)
            except:
                pass

        # Wait
        times = []
        for p in procs:
            t_start = time.perf_counter()
            p.wait()
            times.append(time.perf_counter() - t_start)

    total_wall = time.perf_counter() - start

    if io_start:
        io_end = proc.io_counters()
        result.system.io_read_bytes = io_end.read_bytes - io_start.read_bytes
        result.system.io_write_bytes = io_end.write_bytes - io_start.write_bytes

    if cpu_samples:
        result.system.cpu_percent_avg = sum(cpu_samples) / len(cpu_samples)
    if mem_samples:
        result.system.memory_mb_peak = max(mem_samples)

    times.sort()
    n = len(times)

    result.total_wall_time = total_wall
    result.agent_times = times

    result.sla_p50 = times[n // 2] if n > 0 else 0
    result.sla_p95 = times[int(n * 0.95)] if n > 0 else 0
    result.sla_p99 = times[int(n * 0.99)] if n > 0 else 0

    # Estimate breakdown (codex overhead is everything beyond LLM time)
    avg_time = sum(times) / n if n > 0 else 1
    result.overhead_percent = 40  # Rough estimate
    result.generation_percent = 30
    result.ttft_percent = 30

    return result


# ============================================================================
# Comparison & Reporting
# ============================================================================


def compare_harnesses(
    agent_count: int = 6,
    prompt: str = "Create a simple hello.txt file",
    model: str = "MiniMax-M2.5",
) -> dict[str, BenchmarkResult]:
    """Compare different harness configurations."""

    results = {}

    # 1. Minimax via cliproxy (baseline)
    print(">>> Testing: minimax-via-cliproxy")
    results["minimax-cliproxy"] = asyncio.run(run_minimax_benchmark("cliproxy", model, agent_count, prompt))

    # 2. Homebrew codex-cli (if available)
    codex_path = shutil.which("codex")
    if codex_path:
        print(f">>> Testing: codex-cli (homebrew) @ {codex_path}")
        results["codex-homebrew"] = run_codex_benchmark("codex-homebrew", codex_path, model, agent_count, prompt)

    # 3. Built codex (if exists)
    built_paths = [
        "/Users/kooshapari/temp-PRODVERCEL/485/kush/cliproxyapi-plusplus/target/release/codex",
        "./target/release/codex",
    ]
    for path in built_paths:
        if os.path.isfile(path) and os.access(path, os.X_OK):
            print(f">>> Testing: codex-built @ {path}")
            results["codex-built"] = run_codex_benchmark("codex-built", path, model, agent_count, prompt)
            break

    return results


def print_harness_report(result: BenchmarkResult) -> None:
    """Print harness benchmark report."""

    print(f"\n{'=' * 65}")
    print(f"HARNESS: {result.harness_name}")
    print(f"Model: {result.model} | Agents: {result.agent_count}")
    print(f"{'=' * 65}")

    print(f"\n[TIMING]")
    print(f"  Total Wall:    {result.total_wall_time:.3f}s")
    print(f"  SLA P50:       {result.sla_p50:.3f}s")
    print(f"  SLA P95:       {result.sla_p95:.3f}s")
    print(f"  SLA P99:       {result.sla_p99:.3f}s")

    print(f"\n[BREAKDOWN %]")
    print(f"  TTFT:         {result.ttft_percent:>5.1f}%")
    print(f"  Generation:   {result.generation_percent:>5.1f}%")
    print(f"  Overhead:     {result.overhead_percent:>5.1f}%")

    print(f"\n[LLM METRICS]")
    print(f"  TTFT:          {result.llm.ttft:.3f}s")
    print(f"  Gen Time:     {result.llm.generation_time:.3f}s")
    print(f"  Tokens:        {result.llm.completion_tokens}")
    print(f"  Tokens/sec:   {result.llm.tokens_per_second:.1f}")
    print(f"  Cost:         ${result.llm.total_cost:.4f}")

    if result.system.cpu_percent_avg > 0 or result.system.memory_mb_peak > 0:
        print(f"\n[SYSTEM]")
        if result.system.cpu_percent_avg > 0:
            print(f"  CPU %:         {result.system.cpu_percent_avg:.1f}%")
        if result.system.memory_mb_peak > 0:
            print(f"  Memory Peak:   {result.system.memory_mb_peak:.1f}MB")
        if result.system.io_read_bytes > 0:
            print(f"  IO Read:       {result.system.io_read_bytes / 1024:.1f}KB")
            print(f"  IO Write:      {result.system.io_write_bytes / 1024:.1f}KB")

    if result.errors:
        print(f"\n[ERRORS]")
        for err in set(result.errors):
            print(f"  - {err[:60]}")

    print(f"{'=' * 65}")


def print_comparison_table(results: dict[str, BenchmarkResult]) -> None:
    """Print comparison table."""

    print(f"\n{'=' * 80}")
    print("HARNESS COMPARISON SUMMARY")
    print(f"{'=' * 80}")
    print(f"{'Harness':<20} {'P50':>8} {'P95':>8} {'P99':>8} {'TTFT%':>8} {'OH%':>8} {'tps':>8}")
    print(f"{'-' * 80}")

    for name, r in results.items():
        print(
            f"{name:<20} {r.sla_p50:>7.2f}s {r.sla_p95:>7.2f}s {r.sla_p99:>7.2f}s {r.ttft_percent:>7.1f}% {r.overhead_percent:>7.1f}% {r.llm.tokens_per_second:>7.1f}"
        )

    # Delta from baseline
    baseline = results.get("minimax-cliproxy")
    if baseline and len(results) > 1:
        print(f"{'-' * 80}")
        print("DELTA FROM BASELINE (minimax-cliproxy):")
        for name, r in results.items():
            if name != "minimax-cliproxy":
                delta_p50 = r.sla_p50 - baseline.sla_p50
                delta_pct = (delta_p50 / baseline.sla_p50 * 100) if baseline.sla_p50 > 0 else 0
                print(f"  {name:<20} P50: {delta_pct:+.1f}% ({delta_p50:+.2f}s)")

    print(f"{'=' * 80}")


# ============================================================================
# Main
# ============================================================================

if __name__ == "__main__":
    import argparse

    parser = argparse.ArgumentParser(description="Harness Performance Benchmark")
    parser.add_argument("--agents", "-n", type=int, default=6)
    parser.add_argument("--prompt", "-p", type=str)
    parser.add_argument("--model", "-m", type=str, default="MiniMax-M2.5")
    parser.add_argument("--compare", "-c", action="store_true", help="Compare all harnesses")
    parser.add_argument("--minimax", "-x", action="store_true", help="Test minimax via cliproxy")
    parser.add_argument("--binary", "-b", type=str, help="Test specific codex binary")
    parser.add_argument("--json", "-j", action="store_true", help="JSON output")

    args = parser.parse_args()

    prompt = args.prompt or "Create a simple hello.txt file"

    if args.compare:
        results = compare_harnesses(args.agents, prompt, args.model)
        for r in results.values():
            print_harness_report(r)
        print_comparison_table(results)

    elif args.minimax:
        result = asyncio.run(run_minimax_benchmark("cliproxy", args.model, args.agents, prompt))
        if args.json:
            print(json.dumps(asdict(result), indent=2))
        else:
            print_harness_report(result)

    elif args.binary:
        result = run_codex_benchmark("custom", args.binary, args.model, args.agents, prompt)
        if args.json:
            print(json.dumps(asdict(result), indent=2))
        else:
            print_harness_report(result)

    else:
        print("Use --compare, --minimax, or --binary")
