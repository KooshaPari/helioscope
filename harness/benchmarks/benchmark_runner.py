"""Standardized Codex/Harness Benchmark Runner.

Matrix: 2x models × Nx harnesses for consistent comparisons.

Models:
  - MiniMax-M2.5
  - MiniMax-M2.5-highspeed

Harnesses:
  - cliproxy    : Direct LLM call via cliproxy
  - codex-cli   : Codex CLI (requires logout from ChatGPT or API key)
  - droid       : Factory Droid (optional)
  - claude      : Anthropic Claude (optional)

Usage:
    # Full matrix (cliproxy + codex-cli)
    python -m harness.benchmarks.benchmark_runner --matrix

    # Just cliproxy vs codex-cli
    python -m harness.benchmarks.benchmark_runner --harnesses cliproxy,codex-cli

    # Add droid and claude
    python -m harness.benchmarks.benchmark_runner --matrix --with-optional

    # Single test
    python -m harness.benchmarks.benchmark_runner --harness cliproxy --model MiniMax-M2.5 --agents 6
"""

import asyncio
import json
import os
import shutil
import subprocess
import tempfile
import time
import httpx
from dataclasses import dataclass, field, asdict
from typing import Any

import psutil
from harness.src.harness.resources import safe_popen, ResourceMonitor, fd_tracker


# ============================================================================
# Configuration
# ============================================================================

CLIPROXY_URL = os.environ.get("CLIPROXY_URL", "http://localhost:8317")

MODELS = {
    "MiniMax-M2.5": {"cliproxy_name": "MiniMax-M2.5", "codex_model": "MiniMax-M2.5"},
    "MiniMax-M2.5-highspeed": {"cliproxy_name": "MiniMax-M2.5-highspeed", "codex_model": "MiniMax-M2.5-highspeed"},
}

HARNESSES = {
    "cliproxy": {"binary": None, "type": "http"},
    "codex-cli": {"binary": shutil.which("codex"), "type": "cli"},
    "droid": {"binary": shutil.which("droid"), "type": "cli"},
    "claude": {"binary": shutil.which("claude"), "type": "cli"},
}


# ============================================================================
# Data Classes
# ============================================================================


@dataclass
class TestResult:
    """Result for a single test case."""

    harness: str
    model: str
    agent_count: int

    # Timing
    total_wall_time: float = 0.0
    sla_p50: float = 0.0
    sla_p95: float = 0.0
    sla_p99: float = 0.0

    # Metrics
    ttft: float = 0.0
    generation_time: float = 0.0
    completion_tokens: int = 0
    tokens_per_second: float = 0.0

    # System
    cpu_percent: float = 0.0
    memory_mb: float = 0.0

    # Status
    success: bool = False
    error: str = ""


@dataclass
class BenchmarkMatrix:
    """Complete benchmark matrix results."""

    timestamp: str = ""
    prompt: str = ""
    agent_count: int = 0
    results: list[TestResult] = field(default_factory=list)


# ============================================================================
# Harness Runners
# ============================================================================


async def run_cliproxy(
    model: str,
    prompt: str,
    agent_count: int,
) -> TestResult:
    """Run via cliproxy direct HTTP."""
    result = TestResult(harness="cliproxy", model=model, agent_count=agent_count)

    start = time.perf_counter()

    async def call_llm():
        async with httpx.AsyncClient(timeout=120.0) as client:
            first_token = None
            content_parts = []

            async with client.stream(
                "POST",
                f"{CLIPROXY_URL}/v1/chat/completions",
                json={
                    "model": model,
                    "messages": [{"role": "user", "content": prompt}],
                    "stream": True,
                },
            ) as resp:
                async for line in resp.aiter_lines():
                    if line.strip() and line.startswith("data:"):
                        data = line[5:].strip()
                        if data == "[DONE]":
                            break
                        try:
                            chunk = json.loads(data)
                            delta = chunk.get("choices", [{}])[0].get("delta", {})
                            if delta.get("content"):
                                if first_token is None:
                                    first_token = time.perf_counter() - start
                                content_parts.append(delta["content"])
                        except:
                            pass

            return first_token, "".join(content_parts)

    try:
        # Run concurrent calls
        tasks = [call_llm() for _ in range(agent_count)]
        outcomes = await asyncio.gather(*tasks, return_exceptions=True)

        total_wall = time.perf_counter() - start
        times = []
        ttfts = []
        tokens_list = []

        for o in outcomes:
            if isinstance(o, tuple):
                ft, content = o
                ttfts.append(ft or 0)
                times.append(total_wall / agent_count)  # Approximate
                tokens_list.append(len(content) // 4)

        times.sort()
        n = len(times)

        result.total_wall_time = total_wall
        result.sla_p50 = times[n // 2] if n > 0 else 0
        result.sla_p95 = times[int(n * 0.95)] if n > 0 else 0
        result.sla_p99 = times[int(n * 0.99)] if n > 0 else 0

        if ttfts:
            result.ttft = sum(ttfts) / len(ttfts)
        if tokens_list:
            result.completion_tokens = int(sum(tokens_list) / len(tokens_list))
            if result.ttft > 0:
                result.tokens_per_second = result.completion_tokens / (result.total_wall_time - result.ttft)

        result.success = True

    except Exception as e:
        result.error = str(e)[:100]
        result.success = False

    return result


def run_codex_cli(
    model: str,
    prompt: str,
    agent_count: int,
) -> TestResult:
    """Run via codex-cli with minimax profile."""
    result = TestResult(harness="codex-cli", model=model, agent_count=agent_count)

    codex_path = shutil.which("codex")
    if not codex_path:
        result.error = "codex not found"
        return result

    # Set env for minimax (via config, not base_url)
    env = os.environ.copy()
    env["MINIMAX_API_KEY"] = "dummy"

    start = time.perf_counter()

    with tempfile.TemporaryDirectory() as tmpdir:
        # Init git repo
        subprocess.run(["git", "init"], cwd=tmpdir, capture_output=True)
        subprocess.run(["git", "config", "user.email", "test@test.com"], cwd=tmpdir, capture_output=True)
        subprocess.run(["git", "config", "user.name", "Test"], cwd=tmpdir, capture_output=True)

        procs = []
        for i in range(agent_count):
            agent_dir = os.path.join(tmpdir, f"agent_{i}")
            os.makedirs(agent_dir)
            subprocess.run(["git", "init"], cwd=agent_dir, capture_output=True)

            # Use minimax provider via config - with safe_popen for proper cleanup
            with safe_popen(
                [codex_path, "-c", "model_provider=minimax", "-c", f"model={model}", "exec", prompt],
                cwd=agent_dir,
                stdout=subprocess.PIPE,
                stderr=subprocess.PIPE,
                text=True,
                env=env,
            ) as p:
                procs.append(p)

        # Wait for all
        times = []
        for p in procs:
            t_start = time.perf_counter()
            p.wait()
            times.append(time.perf_counter() - t_start)

    total_wall = time.perf_counter() - start
    times.sort()
    n = len(times)

    result.total_wall_time = total_wall
    result.sla_p50 = times[n // 2] if n > 0 else 0
    result.sla_p95 = times[int(n * 0.95)] if n > 0 else 0
    result.sla_p99 = times[int(n * 0.99)] if n > 0 else 0

    # Check for errors in output (usage limit etc)
    result.success = all(p.returncode == 0 for p in procs)
    if not result.success:
        result.error = "codex returned non-zero"

    return result


def run_droid(
    model: str,
    prompt: str,
    agent_count: int,
) -> TestResult:
    """Run via droid (Factory)."""
    result = TestResult(harness="droid", model=model, agent_count=agent_count)

    droid_path = shutil.which("droid")
    if not droid_path:
        result.error = "droid not found"
        return result

    # Droid doesn't support model selection via CLI typically
    # Run single prompt for baseline
    start = time.perf_counter()

    with ResourceMonitor() as monitor:
        try:
            with safe_popen(
                [droid_path, "exec", prompt],
                stdout=subprocess.PIPE,
                stderr=subprocess.PIPE,
                text=True,
            ) as proc:
                proc.wait()
            elapsed = time.perf_counter() - start

            result.total_wall_time = elapsed
            result.sla_p50 = elapsed
            result.sla_p95 = elapsed
            result.sla_p99 = elapsed
            result.success = proc.returncode == 0
        except Exception as e:
            result.error = str(e)[:100]
            result.success = False

    return result


def run_claude(
    model: str,
    prompt: str,
    agent_count: int,
) -> TestResult:
    """Run via Claude CLI."""
    result = TestResult(harness="claude", model=model, agent_count=agent_count)

    claude_path = shutil.which("claude")
    if not claude_path:
        result.error = "claude not found"
        return result

    start = time.perf_counter()

    with ResourceMonitor() as monitor:
        try:
            with safe_popen(
                [claude_path, "-p", prompt],
                stdout=subprocess.PIPE,
                stderr=subprocess.PIPE,
                text=True,
            ) as proc:
                proc.wait()
            elapsed = time.perf_counter() - start

            result.total_wall_time = elapsed
            result.sla_p50 = elapsed
            result.sla_p95 = elapsed
            result.sla_p99 = elapsed
            result.success = proc.returncode == 0
        except Exception as e:
            result.error = str(e)[:100]
            result.success = False

    return result


# ============================================================================
# Matrix Runner
# ============================================================================


async def run_benchmark_matrix(
    harnesses: list[str],
    models: list[str],
    agent_count: int = 6,
    prompt: str = "Say hello and be brief",
) -> BenchmarkMatrix:
    """Run full benchmark matrix."""

    matrix = BenchmarkMatrix(
        timestamp=time.strftime("%Y-%m-%d %H:%M:%S"),
        prompt=prompt,
        agent_count=agent_count,
    )

    print(f"\n{'=' * 70}")
    print(f"BENCHMARK MATRIX: {len(models)} models × {len(harnesses)} harnesses")
    print(f"Agents: {agent_count} | Prompt: {prompt[:30]}...")
    print(f"{'=' * 70}\n")

    for model in models:
        model_info = MODELS.get(model, {})
        cliproxy_name = model_info.get("cliproxy_name", model)

        for harness in harnesses:
            print(f"Testing: {harness} + {model}...", end=" ", flush=True)

            if harness == "cliproxy":
                r = await run_cliproxy(cliproxy_name, prompt, agent_count)
            elif harness == "codex-cli":
                codex_model = model_info.get("codex_model", model)
                r = run_codex_cli(codex_model, prompt, agent_count)
            elif harness == "droid":
                r = run_droid(model, prompt, agent_count)
            elif harness == "claude":
                r = run_claude(model, prompt, agent_count)
            else:
                r = TestResult(harness=harness, model=model, agent_count=agent_count)
                r.error = "unknown harness"

            matrix.results.append(r)

            status = "✓" if r.success else "✗"
            print(f"{status} P50={r.sla_p50:.2f}s")

    return matrix


def print_matrix_report(matrix: BenchmarkMatrix) -> None:
    """Print formatted matrix report."""

    print(f"\n{'=' * 80}")
    print(f"BENCHMARK RESULTS: {matrix.timestamp}")
    print(f"{'=' * 80}")

    # Group by model
    for model in MODELS.keys():
        print(f"\n{'─' * 80}")
        print(f"MODEL: {model}")
        print(f"{'─' * 80}")
        print(f"{'Harness':<15} {'P50':>8} {'P95':>8} {'P99':>8} {'TTFT':>8} {'Tokens':>8} {'tps':>8} {'Status':>8}")
        print(f"{'─' * 80}")

        for r in matrix.results:
            if r.model == model or r.model == MODELS.get(model, {}).get("cliproxy_name"):
                status = "OK" if r.success else "FAIL"
                print(
                    f"{r.harness:<15} {r.sla_p50:>7.2f}s {r.sla_p95:>7.2f}s {r.sla_p99:>7.2f}s {r.ttft:>7.2f}s {r.completion_tokens:>8} {r.tokens_per_second:>7.1f} {status:>8}"
                )

    # Summary comparison
    print(f"\n{'=' * 80}")
    print("SUMMARY (P50 latency)")
    print(f"{'=' * 80}")

    # Get cliproxy baseline
    baseline = None
    for r in matrix.results:
        if r.harness == "cliproxy" and r.success:
            baseline = r
            break

    if baseline:
        print(f"\nDelta from cliproxy baseline:")
        for r in matrix.results:
            if r.success and r.harness != "cliproxy":
                delta = r.sla_p50 - baseline.sla_p50
                pct = (delta / baseline.sla_p50) * 100 if baseline.sla_p50 > 0 else 0
                print(f"  {r.harness} + {r.model}: {pct:+.1f}% ({delta:+.2f}s)")


# ============================================================================
# Main
# ============================================================================

if __name__ == "__main__":
    import argparse
    import shutil

    parser = argparse.ArgumentParser(description="Standardized Benchmark Runner")
    parser.add_argument(
        "--harness",
        type=str,
        default="cliproxy",
        help="Harness to test: cliproxy, codex, claude, droid, or 'all' for all",
    )
    parser.add_argument("--agents", "-n", type=int, default=6)
    parser.add_argument("--prompt", "-p", type=str, default="Say hello and be brief")
    parser.add_argument("--models", type=str, help="Comma-separated: MiniMax-M2.5,MiniMax-M2.5-highspeed")
    parser.add_argument("--json", "-j", action="store_true", help="JSON output")
    parser.add_argument("--json", "-j", action="store_true", help="JSON output")

    args = parser.parse_args()

    # Determine harness (new --harness primary param with "all" support)
    harness_input = args.harness.lower() if args.harness else "cliproxy"
    if harness_input == "all":
        harness_list = ["cliproxy", "codex", "claude", "droid"]
    else:
        harness_list = [harness_input]

    # Determine models
    if args.models:
        model_list = [m.strip() for m in args.models.split(",")]
    else:
        model_list = list(MODELS.keys())

    # Run matrix
    matrix = asyncio.run(
        run_benchmark_matrix(
            harness_list,
            model_list,
            args.agents,
            args.prompt,
        )
    )

    if args.json:
        print(json.dumps(asdict(matrix), indent=2, default=str))
    else:
        print_matrix_report(matrix)
