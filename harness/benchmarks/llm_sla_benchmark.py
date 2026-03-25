"""Comprehensive LLM & Codex Benchmark Suite.

Measures:
- System resources: CPU, Memory, Network, FD
- LLM metrics: TTFT, generation time, tokens/sec, cost
- Codex overhead breakdown
- Conciseness analysis
- SLA compliance

Component     │ Time      │ % of Total
--------------│-----------│-----------
TTFT          │ 0.50s    │ 12%
Network RTT   │ 0.10s    │ 2%
Generation    │ 0.50s    │ 12%
Codex OH     │ 3.10s    │ 74%
TOTAL         │ 4.20s    │ 100%
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
from harness.src.harness.resources import safe_popen, ResourceMonitor
from typing import Any


# ============================================================================
# Data Classes
# ============================================================================


@dataclass
class SystemMetrics:
    """System resource metrics during benchmark."""

    # CPU
    cpu_percent_start: float = 0.0
    cpu_percent_end: float = 0.0
    cpu_percent_avg: float = 0.0

    # Memory
    memory_mb_start: float = 0.0
    memory_mb_end: float = 0.0
    memory_mb_peak: float = 0.0

    # Network
    bytes_sent: int = 0
    bytes_recv: int = 0

    # File Descriptors
    fd_count_start: int = 0
    fd_count_end: int = 0

    # Process
    num_threads: int = 0


@dataclass
class LLMMetrics:
    """LLM-specific performance metrics."""

    # Timing breakdown
    ttft: float = 0.0  # Time to First Token
    generation_time: float = 0.0  # Time for token generation
    total_time: float = 0.0

    # Token metrics
    prompt_tokens: int = 0
    completion_tokens: int = 0
    total_tokens: int = 0

    # Speed
    tokens_per_second: float = 0.0

    # Cost (estimated)
    prompt_cost: float = 0.0
    completion_cost: float = 0.0
    total_cost: float = 0.0

    # Conciseness
    conciseness_ratio: float = 0.0  # completion/prompt ratio


@dataclass
class BenchmarkResult:
    """Complete benchmark result."""

    name: str = ""
    agent_count: int = 0
    total_wall_time: float = 0.0

    # System metrics
    system: SystemMetrics = field(default_factory=SystemMetrics)

    # LLM metrics
    llm: LLMMetrics = field(default_factory=LLMMetrics)

    # Breakdown
    network_rtt_estimate: float = 0.0
    codex_overhead: float = 0.0

    # SLA
    sla_p50: float = 0.0
    sla_p95: float = 0.0
    sla_p99: float = 0.0

    # Per-agent details
    agent_times: list[float] = field(default_factory=list)
    errors: list[str] = field(default_factory=list)


# ============================================================================
# Cost Constants (per 1M tokens)
# ============================================================================

MODEL_COSTS = {
    "MiniMax-M2.5": {"prompt": 0.2, "completion": 0.6},
    "MiniMax-M2.5-highspeed": {"prompt": 0.1, "completion": 0.3},
    "gpt-5.3-codex": {"prompt": 2.5, "completion": 10.0},
    "default": {"prompt": 1.0, "completion": 3.0},
}


def _get_model_costs(model: str) -> dict:
    """Get cost per 1M tokens for model."""
    for key in MODEL_COSTS:
        if key.lower() in model.lower():
            return MODEL_COSTS[key]
    return MODEL_COSTS["default"]


# ============================================================================
# Cliproxy LLM Benchmark
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


async def _monitor_system_resources(pid: int, interval: float = 0.1) -> SystemMetrics:
    """Monitor system resources in background."""
    metrics = SystemMetrics()

    try:
        proc = psutil.Process(pid)
        metrics.cpu_percent_start = proc.cpu_percent()
        metrics.memory_mb_start = proc.memory_info().rss / 1024 / 1024
        metrics.fd_count_start = proc.num_fds() if hasattr(proc, "num_fds") else 0

        # Sample during execution
        samples = []
        mem_samples = []
        start_time = time.time()

        while time.time() - start_time < 30:  # Max 30s
            try:
                samples.append(proc.cpu_percent())
                mem_samples.append(proc.memory_info().rss / 1024 / 1024)
                await asyncio.sleep(interval)
            except psutil.NoSuchProcess, psutil.AccessDenied:
                break

        if samples:
            metrics.cpu_percent_avg = sum(samples) / len(samples)
            metrics.cpu_percent_end = samples[-1] if samples else 0

        if mem_samples:
            metrics.memory_mb_peak = max(mem_samples)
            metrics.memory_mb_end = mem_samples[-1]

    except Exception:
        pass

    return metrics


async def _call_llm_async(
    client: httpx.AsyncClient,
    prompt: str,
    model: str,
) -> tuple[LLMMetrics, str]:
    """Call LLM and measure detailed metrics."""
    metrics = LLMMetrics()
    start = time.perf_counter()

    # Estimate network RTT (ping to cliproxy)
    rtt_start = time.perf_counter()
    try:
        await client.get(f"{CLIPROXY_URL}/health", timeout=5.0)
        rtt_estimate = (time.perf_counter() - rtt_start) * 1000  # ms
    except:
        rtt_estimate = 0

    try:
        # Stream request to measure TTFT
        first_token_time = None
        tokens_received = 0

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
            token_count = 0
            async for line in response.aiter_lines():
                if line.strip() and line.startswith("data:"):
                    data = line[6:].strip()
                    if data == "[DONE]":
                        break
                    try:
                        chunk = json.loads(data)
                        if "choices" in chunk and chunk["choices"]:
                            delta = chunk["choices"][0].get("delta", {})
                            if delta.get("content"):
                                if first_token_time is None:
                                    first_token_time = time.perf_counter() - start
                                content_parts.append(delta["content"])
                                token_count += 1
                    except:
                        pass

            total_time = time.perf_counter() - start

            # Calculate metrics
            content = "".join(content_parts)

            metrics.ttft = first_token_time or total_time
            metrics.generation_time = total_time - metrics.ttft
            metrics.total_time = total_time

            # Use actual token count - estimate ~4 chars per token
            content = "".join(content_parts)
            metrics.completion_tokens = max(len(content) // 4, 1)
            metrics.total_tokens = metrics.completion_tokens

            # Speed (tokens per second)
            if metrics.generation_time > 0.01:  # Avoid division by zero
                metrics.tokens_per_second = metrics.completion_tokens / metrics.generation_time

            # Cost
            costs = _get_model_costs(model)
            metrics.prompt_cost = (metrics.prompt_tokens / 1_000_000) * costs["prompt"]
            metrics.completion_cost = (metrics.completion_tokens / 1_000_000) * costs["completion"]
            metrics.total_cost = metrics.prompt_cost + metrics.completion_cost

            # Conciseness
            if metrics.prompt_tokens > 0:
                metrics.conciseness_ratio = metrics.completion_tokens / metrics.prompt_tokens
            else:
                # Estimate from character count
                prompt_tokens_est = len(prompt) // 4
                metrics.conciseness_ratio = metrics.completion_tokens / max(prompt_tokens_est, 1)

    except Exception as e:
        metrics.total_time = time.perf_counter() - start
        return metrics, str(e)

    return metrics, ""


async def _run_concurrent_llm_benchmark(
    prompt: str,
    agent_count: int,
    model: str,
) -> BenchmarkResult:
    """Run concurrent LLM calls with full metrics."""
    result = BenchmarkResult(name=model, agent_count=agent_count)
    start = time.perf_counter()

    async with httpx.AsyncClient() as client:
        # Run all requests concurrently
        tasks = [_call_llm_async(client, prompt, model) for _ in range(agent_count)]

        llm_results = await asyncio.gather(*tasks, return_exceptions=True)

    total_wall = time.perf_counter() - start

    # Collect results
    times = []
    ttfts = []
    gen_times = []
    tokens_list = []
    errors = []

    for r in llm_results:
        if isinstance(r, tuple):
            metrics, err = r
            times.append(metrics.total_time)
            ttfts.append(metrics.ttft)
            gen_times.append(metrics.generation_time)
            tokens_list.append(metrics.completion_tokens)
            if err:
                errors.append(err[:100])
        elif isinstance(r, Exception):
            errors.append(str(r)[:100])

    # Calculate aggregates
    times.sort()
    n = len(times)

    result.total_wall_time = total_wall
    result.agent_times = times
    result.errors = errors

    # SLA percentiles
    result.sla_p50 = times[n // 2] if n > 0 else 0
    result.sla_p95 = times[int(n * 0.95)] if n > 0 else 0
    result.sla_p99 = times[int(n * 0.99)] if n > 0 else 0

    # Average LLM metrics
    if ttfts:
        result.llm.ttft = sum(ttfts) / len(ttfts)
    if gen_times:
        result.llm.generation_time = sum(gen_times) / len(gen_times)
    if tokens_list:
        result.llm.completion_tokens = int(sum(tokens_list) / len(tokens_list))

    # Calculate tokens per second
    if result.llm.generation_time > 0.01:
        result.llm.tokens_per_second = result.llm.completion_tokens / result.llm.generation_time

    # Calculate overhead breakdown
    avg_time = sum(times) / n if n > 0 else 1
    result.codex_overhead = avg_time - result.llm.ttft - result.llm.generation_time
    result.network_rtt_estimate = 0.05  # Approximate

    return result


def run_llm_benchmark(
    agent_count: int = 6,
    prompt: str = "Explain quantum computing in one sentence",
    model: str = "MiniMax-M2.5",
) -> BenchmarkResult:
    """Run LLM concurrency benchmark."""
    return asyncio.run(_run_concurrent_llm_benchmark(prompt, agent_count, model))


# ============================================================================
# Codex CLI Benchmark (when available)
# ============================================================================


def _run_codex_benchmark(
    binary: str,
    agent_count: int,
    prompt: str,
    model: str = "minimax-m2.5-highspeed",
) -> BenchmarkResult:
    """Run Codex CLI benchmark with system metrics."""
    result = BenchmarkResult(name="codex", agent_count=agent_count)

    # Get initial system state
    proc_self = psutil.Process()
    result.system.cpu_percent_start = proc_self.cpu_percent()
    result.system.memory_mb_start = proc_self.memory_info().rss / 1024 / 1024

    start = time.perf_counter()
    tasks = []

    with tempfile.TemporaryDirectory() as tmpdir:
        work_dir = Path(tmpdir)

        # Init git repos
        subprocess.run(["git", "init"], cwd=work_dir, capture_output=True)
        subprocess.run(["git", "config", "user.email", "test@test.com"], cwd=work_dir, capture_output=True)
        subprocess.run(["git", "config", "user.name", "Test"], cwd=work_dir, capture_output=True)

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
            ) as proc:
                tasks.append(proc)

        # Wait for all
        times = []
        for t in tasks:
            t_start = time.perf_counter()
            t.wait()
            times.append(time.perf_counter() - t_start)

    total_wall = time.perf_counter() - start

    # Get final system state
    result.system.cpu_percent_end = proc_self.cpu_percent()
    result.system.memory_mb_end = proc_self.memory_info().rss / 1024 / 1024

    times.sort()
    n = len(times)

    result.total_wall_time = total_wall
    result.agent_times = times
    result.sla_p50 = times[n // 2] if n > 0 else 0
    result.sla_p95 = times[int(n * 0.95)] if n > 0 else 0
    result.sla_p99 = times[int(n * 0.99)] if n > 0 else 0

    # Estimate overhead (total - LLM time)
    avg_time = sum(times) / n if n > 0 else 1
    result.codex_overhead = avg_time * 0.7  # Rough estimate

    return result


# ============================================================================
# Reporting
# ============================================================================


def print_sla_report(result: BenchmarkResult) -> None:
    """Print detailed SLA benchmark report."""

    print(f"\n{'=' * 70}")
    print(f"COMPREHENSIVE BENCHMARK: {result.name}")
    print(f"{'=' * 70}")

    # Timing breakdown
    print(f"\n[TIMING BREAKDOWN]")
    print(f"  {'Component':<20} │ {'Time':>10} │ {'% of Total':>10}")
    print(f"  {'-' * 20}─┼{'-' * 12}─┼{'-' * 12}")

    total = result.total_wall_time
    if total <= 0:
        total = 1

    print(f"  {'TTFT':<20} │ {result.llm.ttft:>9.3f}s │ {result.llm.ttft / total * 100:>9.1f}%")
    print(
        f"  {'Network RTT (est)':<20} │ {result.network_rtt_estimate:>9.3f}s │ {result.network_rtt_estimate / total * 100:>9.1f}%"
    )
    print(
        f"  {'Generation':<20} │ {result.llm.generation_time:>9.3f}s │ {result.llm.generation_time / total * 100:>9.1f}%"
    )
    print(f"  {'Codex/Overhead':<20} │ {result.codex_overhead:>9.3f}s │ {result.codex_overhead / total * 100:>9.1f}%")
    print(f"  {'-' * 20}─┼{'-' * 12}─┼{'-' * 12}")
    print(f"  {'TOTAL':<20} │ {total:>9.3f}s │ {'100.0%':>10}")

    # SLA metrics
    print(f"\n[SLA METRICS]")
    print(f"  P50 (median):  {result.sla_p50:.3f}s")
    print(f"  P95:           {result.sla_p95:.3f}s")
    print(f"  P99:           {result.sla_p99:.3f}s")
    print(f"  Total Wall:    {result.total_wall_time:.3f}s")

    # Token metrics
    print(f"\n[TOKEN METRICS]")
    print(f"  Completion tokens: {result.llm.completion_tokens}")
    print(f"  Tokens/sec:       {result.llm.tokens_per_second:.1f}")
    print(f"  Conciseness:      {result.llm.conciseness_ratio:.2f}x")

    # Cost
    print(f"\n[COST ESTIMATE]")
    print(f"  Total cost:       ${result.llm.total_cost:.4f}")
    print(f"  Prompt cost:      ${result.llm.prompt_cost:.4f}")
    print(f"  Completion cost:   ${result.llm.completion_cost:.4f}")

    # Per-agent
    print(f"\n[PER-AGENT TIMES]")
    for i, t in enumerate(result.agent_times):
        print(f"  Agent {i}: {t:.2f}s")

    # Errors
    if result.errors:
        print(f"\n[ERRORS]")
        for err in set(result.errors):
            print(f"  - {err[:80]}")

    print(f"{'=' * 70}")


def compare_models(
    agent_count: int = 6,
    prompt: str = "Explain quantum computing in one sentence",
    models: list[str] | None = None,
) -> dict[str, BenchmarkResult]:
    """Compare multiple models."""

    if models is None:
        models = ["MiniMax-M2.5", "MiniMax-M2.5-highspeed"]

    results = {}
    for model in models:
        print(f"\n>>> Benchmarking {model}...")
        results[model] = run_llm_benchmark(agent_count, prompt, model)

    return results


# ============================================================================
# Main
# ============================================================================

if __name__ == "__main__":
    import argparse

    parser = argparse.ArgumentParser(description="Comprehensive LLM & Codex Benchmark")
    parser.add_argument("--agents", "-n", type=int, default=6)
    parser.add_argument("--prompt", "-p", type=str)
    parser.add_argument("--model", "-m", type=str, default="MiniMax-M2.5")
    parser.add_argument("--compare", "-c", action="store_true")
    parser.add_argument("--json", "-j", action="store_true")

    args = parser.parse_args()

    prompt = args.prompt or "Explain quantum computing in one sentence"

    if args.compare:
        results = compare_models(args.agents, prompt)
        for name, result in results.items():
            print_sla_report(result)

        # Comparison summary
        print(f"\n{'=' * 70}")
        print("COMPARISON SUMMARY")
        print(f"{'=' * 70}")
        for name, result in results.items():
            print(
                f"  {name:<30} P50={result.sla_p50:.2f}s  TTFT={result.llm.ttft:.2f}s  tps={result.llm.tokens_per_second:.1f}"
            )
    else:
        result = run_llm_benchmark(args.agents, prompt, args.model)
        if args.json:
            print(json.dumps(asdict(result), indent=2))
        else:
            print_sla_report(result)
