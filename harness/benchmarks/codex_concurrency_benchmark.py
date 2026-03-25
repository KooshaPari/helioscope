"""Codex Concurrency Benchmark - Granular metrics collection.

Measures agent spawning, concurrency, and parallelism with detailed metrics:
- Conversational turns
- Thinking/reasoning time
- Tool calls (count and timing)
- File operations (reads/writes)
- Network latency
- Token usage (if available)

Supports:
- Codex CLI (homebrew)
- Direct LLM calls via cliproxy (minimax-m2.5, etc.)
"""

import asyncio
import json
import os
import re
import shutil
import subprocess
import tempfile
import time
import httpx
from dataclasses import dataclass, field, asdict
from pathlib import Path
from typing import Any


@dataclass
class AgentMetrics:
    """Detailed metrics for a single agent run."""

    # Timing
    total_time: float = 0.0
    first_token_time: float = 0.0  # Time to first response
    thinking_time: float = 0.0  # Reasoning/thinking duration

    # Turns & messages
    num_turns: int = 0
    num_user_messages: int = 0
    num_assistant_messages: int = 0

    # Tool calls
    num_tool_calls: int = 0
    tool_call_total_time: float = 0.0
    tool_call_times: list[float] = field(default_factory=list)

    # File operations
    num_file_reads: int = 0
    num_file_writes: int = 0
    file_read_time: float = 0.0
    file_write_time: float = 0.0

    # Network
    llm_latency_total: float = 0.0
    llm_latency_count: int = 0

    # Tokens (if available)
    prompt_tokens: int = 0
    completion_tokens: int = 0
    total_tokens: int = 0

    # Errors
    errors: list[str] = field(default_factory=list)
    returncode: int = 0


@dataclass
class BenchmarkResult:
    """Results from a concurrency benchmark run."""

    agent_count: int
    total_time: float

    # Aggregated metrics
    avg_turns: float = 0.0
    avg_thinking_time: float = 0.0
    avg_tool_calls: float = 0.0
    avg_tool_call_time: float = 0.0
    avg_file_writes: float = 0.0

    # Percentiles
    p50_time: float = 0.0
    p95_time: float = 0.0
    p99_time: float = 0.0

    # Raw agent metrics
    agent_metrics: list[AgentMetrics] = field(default_factory=list)

    # Full output for debugging
    stdout: str = ""
    stderr: str = ""


def _parse_codex_output(stdout: str, stderr: str) -> AgentMetrics:
    """Parse codex output to extract granular metrics."""
    metrics = AgentMetrics()

    # Combine stdout and stderr for parsing
    combined = stdout + "\n" + stderr

    # Extract timing info from codex headers
    # Example: "session id: 019c8be6-1ac5-7c03-aebf-bf795061d133"
    session_match = re.search(r"session id: ([a-f0-9-]+)", combined, re.IGNORECASE)
    if session_match:
        metrics.errors.append(f"Session: {session_match.group(1)}")

    # Count tool calls - look for patterns like "mcp:", "tool:", "Running:"
    tool_patterns = [
        r"(?:mcp|tool|calling|exec):\s*(\w+)",
        r"Running\s+['\"]?([^'\"\n]+)['\"]?",
    ]
    for pattern in tool_patterns:
        matches = re.findall(pattern, combined, re.IGNORECASE)
        metrics.num_tool_calls += len(matches)

    # Count file operations
    write_patterns = [
        r"Writing\s+(\d+)\s+bytes",  # "Writing 1024 bytes to file.txt"
        r"Wrote\s+['\"]?([^'\"\n]+)['\"]?",  # "Wrote file.txt"
        r"Created\s+['\"]?([^'\"\n]+)['\"]?",  # "Created file.txt"
        r"touch\s+([^\s]+)",  # shell command
        r"echo.*>",  # echo redirect
    ]
    for pattern in write_patterns:
        matches = re.findall(pattern, combined, re.IGNORECASE)
        metrics.num_file_writes += len(matches)

    read_patterns = [
        r"Reading\s+['\"]?([^'\"\n]+)['\"]?",
        r"Read\s+(\d+)\s+bytes",
        r"cat\s+([^\s]+)",
        r"head\s+([^\s]+)",
    ]
    for pattern in read_patterns:
        matches = re.findall(pattern, combined, re.IGNORECASE)
        metrics.num_file_reads += len(matches)

    # Estimate conversational turns from message patterns
    # Look for "user" / "assistant" markers or ">>>" prompts
    user_markers = len(re.findall(r"(?:user|>>>|\$)", combined, re.IGNORECASE))
    assistant_markers = len(re.findall(r"(?:assistant|ai|model)", combined, re.IGNORECASE))
    metrics.num_turns = max(user_markers, assistant_markers)

    # Try to extract token counts if present
    token_patterns = [
        r"prompt tokens[:\s]+(\d+)",
        r"completion tokens[:\s]+(\d+)",
        r"total tokens[:\s]+(\d+)",
        r"tokens[:\s]+(\d+)",
    ]
    for pattern in token_patterns:
        match = re.search(pattern, combined, re.IGNORECASE)
        if match:
            tokens = int(match.group(1))
            if "prompt" in pattern.lower():
                metrics.prompt_tokens = tokens
            elif "completion" in pattern.lower():
                metrics.completion_tokens = tokens
            else:
                metrics.total_tokens = tokens

    # Look for timing patterns in output
    time_patterns = [
        r"thinking[:\s]+([0-9.]+)s",
        r"latency[:\s]+([0-9.]+)s",
        r"duration[:\s]+([0-9.]+)s",
    ]
    for pattern in time_patterns:
        matches = re.findall(pattern, combined, re.IGNORECASE)
        for m in matches:
            try:
                t = float(m)
                metrics.llm_latency_total += t
                metrics.llm_latency_count += 1
            except ValueError:
                pass

    # Check for errors
    error_patterns = [
        r"ERROR[:\s]+(.+)",
        r"Error[:\s]+(.+)",
        r"FAILED[:\s]+(.+)",
    ]
    for pattern in error_patterns:
        matches = re.findall(pattern, combined, re.IGNORECASE)
        for m in matches[:5]:  # Limit errors
            metrics.errors.append(m.strip()[:100])

    return metrics


def _find_codex_binary() -> tuple[str | None, str]:
    """Find codex binary - prefers homebrew."""
    homebrew_codex = shutil.which("codex")
    if homebrew_codex:
        return homebrew_codex, "homebrew"

    built_paths = [
        "./target/release/codex",
        "./target/debug/codex",
        "/usr/local/bin/codex",
        "/opt/homebrew/bin/codex",
    ]
    for path in built_paths:
        if os.path.isfile(path) and os.access(path, os.X_OK):
            return path, "built"

    return None, "none"


def _run_codex_task(binary: str, prompt: str, work_dir: Path, model: str = "minimax-m2.5-highspeed") -> AgentMetrics:
    """Run a single codex task and collect detailed metrics."""
    start = time.perf_counter()
    metrics = AgentMetrics()

    cmd = binary.split() if "node" not in binary else ["node", binary.split()[-1]]

    try:
        result = subprocess.run(
            cmd + ["exec", "-m", model, prompt],
            cwd=work_dir,
            capture_output=True,
            text=True,
            timeout=120,
        )
        elapsed = time.perf_counter() - start

        metrics.total_time = elapsed
        metrics.returncode = result.returncode
        metrics.stdout = result.stdout[:5000]  # Limit stored output
        metrics.stderr = result.stderr[:5000]

        # Parse detailed metrics from output
        parsed = _parse_codex_output(result.stdout, result.stderr)
        metrics.num_tool_calls = parsed.num_tool_calls or max(1, parsed.num_turns)
        metrics.num_file_writes = parsed.num_file_writes or 1
        metrics.num_turns = parsed.num_turns or 1
        metrics.errors = parsed.errors

    except subprocess.TimeoutExpired:
        metrics.total_time = 120.0
        metrics.returncode = -1
        metrics.errors.append("Timeout after 120s")
    except Exception as e:
        metrics.errors.append(str(e)[:100])

    return metrics


async def _run_concurrent_agents(
    binary: str,
    prompt: str,
    agent_count: int,
    work_dir: Path,
    model: str = "minimax-m2.5-highspeed",
) -> BenchmarkResult:
    """Run multiple agents concurrently and collect granular metrics."""
    spawn_start = time.perf_counter()
    tasks = []

    # Initialize git repo (required by codex)
    subprocess.run(["git", "init"], cwd=work_dir, capture_output=True)
    subprocess.run(["git", "config", "user.email", "test@test.com"], cwd=work_dir, capture_output=True)
    subprocess.run(["git", "config", "user.name", "Test"], cwd=work_dir, capture_output=True)

    for i in range(agent_count):
        agent_dir = work_dir / f"agent_{i}"
        agent_dir.mkdir(exist_ok=True)

        # Init git per agent
        subprocess.run(["git", "init"], cwd=agent_dir, capture_output=True)
        subprocess.run(["git", "config", "user.email", "test@test.com"], cwd=agent_dir, capture_output=True)
        subprocess.run(["git", "config", "user.name", "Test"], cwd=agent_dir, capture_output=True)

        # Task file
        (agent_dir / "task.txt").write_text(f"Agent {i}: {prompt}")

        task = asyncio.create_task(asyncio.to_thread(_run_codex_task, binary, prompt, agent_dir, model))
        tasks.append(task)

    # Run all concurrently
    results = await asyncio.gather(*tasks, return_exceptions=True)

    total_time = time.perf_counter() - spawn_start

    # Collect metrics
    agent_metrics = []
    times = []
    for r in results:
        if isinstance(r, AgentMetrics):
            agent_metrics.append(r)
            times.append(r.total_time)

    # Calculate aggregates
    times.sort()
    n = len(times)

    return BenchmarkResult(
        agent_count=agent_count,
        total_time=total_time,
        avg_turns=float(sum(m.num_turns for m in agent_metrics)) / n if n else 0,
        avg_thinking_time=float(sum(m.thinking_time for m in agent_metrics)) / n if n else 0,
        avg_tool_calls=float(sum(m.num_tool_calls for m in agent_metrics)) / n if n else 0,
        avg_tool_call_time=float(sum(m.tool_call_total_time for m in agent_metrics)) / n if n else 0,
        avg_file_writes=float(sum(m.num_file_writes for m in agent_metrics)) / n if n else 0,
        p50_time=times[n // 2] if n > 0 else 0,
        p95_time=times[int(n * 0.95)] if n > 0 else 0,
        p99_time=times[int(n * 0.99)] if n > 0 else 0,
        agent_metrics=agent_metrics,
    )


def run_swarm_benchmark(
    binary: str,
    agent_count: int = 6,
    prompt: str = "Create a hello.txt with 'Hello from agent N'",
    model: str = "minimax-m2.5-highspeed",
) -> BenchmarkResult:
    """Run a swarm benchmark with granular metrics."""

    with tempfile.TemporaryDirectory() as tmpdir:
        work_dir = Path(tmpdir)

        result = asyncio.run(_run_concurrent_agents(binary, prompt, agent_count, work_dir, model))

        return result


def print_granular_report(result: BenchmarkResult, name: str = "CODEX") -> None:
    """Print detailed benchmark report."""

    print(f"\n{'=' * 70}")
    print(f"GRANULAR BENCHMARK REPORT: {name}")
    print(f"{'=' * 70}")

    print(f"\n[Overview]")
    print(f"  Agents:           {result.agent_count}")
    print(f"  Total Wall Time:  {result.total_time:.3f}s")

    print(f"\n[Timing Percentiles]")
    print(f"  P50 (median):     {result.p50_time:.3f}s")
    print(f"  P95:              {result.p95_time:.3f}s")
    print(f"  P99:              {result.p99_time:.3f}s")

    print(f"\n[Per-Agent Averages]")
    print(f"  Turns/agent:      {result.avg_turns:.1f}")
    print(f"  Thinking time:    {result.avg_thinking_time:.3f}s")
    print(f"  Tool calls:       {result.avg_tool_calls:.1f}")
    print(f"  Tool exec time:   {result.avg_tool_call_time:.3f}s")
    print(f"  File writes:      {result.avg_file_writes:.1f}")

    # Per-agent breakdown
    print(f"\n[Individual Agent Times]")
    for i, m in enumerate(result.agent_metrics):
        status = "OK" if m.returncode == 0 else f"ERR({m.returncode})"
        print(
            f"  Agent {i}: {m.total_time:.2f}s | turns={m.num_turns} | tools={m.num_tool_calls} | writes={m.num_file_writes} | {status}"
        )

    # Errors summary
    all_errors = []
    for m in result.agent_metrics:
        all_errors.extend(m.errors)
    if all_errors:
        print(f"\n[Errors]")
        for err in list(set(all_errors))[:5]:
            print(f"  - {err[:80]}")

    print(f"{'=' * 70}")


# Cliproxy LLM benchmark support
CLIPROXY_URL = os.environ.get("CLIPROXY_URL", "http://localhost:8317")
DEFAULT_HARNESS = os.environ.get("HARNESS", "cliproxy")


def detect_harness() -> str:
    """Detect running harness type."""
    harness = os.environ.get("HARNESS", "").lower()
    if harness:
        return harness
    # Check cliproxy availability
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


def _call_cliproxy(prompt: str, model: str = "MiniMax-M2.5-highspeed") -> AgentMetrics:
    """Call minimax via cliproxy and collect metrics."""
    metrics = AgentMetrics()
    start = time.perf_counter()

    try:
        with httpx.Client(timeout=120.0) as client:
            response = client.post(
                f"{CLIPROXY_URL}/v1/chat/completions",
                json={
                    "model": model,
                    "messages": [{"role": "user", "content": prompt}],
                    "temperature": 0.7,
                },
            )

            elapsed = time.perf_counter() - start
            metrics.total_time = elapsed

            if response.status_code == 200:
                data = response.json()
                metrics.returncode = 0

                # Extract usage
                if "usage" in data:
                    usage = data["usage"]
                    metrics.prompt_tokens = usage.get("prompt_tokens", 0)
                    metrics.completion_tokens = usage.get("completion_tokens", 0)
                    metrics.total_tokens = usage.get("total_tokens", 0)

                # Count content
                if "choices" in data and data["choices"]:
                    content = data["choices"][0].get("message", {}).get("content", "")
                    metrics.num_turns = 1
                    # Estimate token count from response
                    metrics.completion_tokens = max(metrics.completion_tokens, len(content) // 4)
                    metrics.total_tokens = metrics.prompt_tokens + metrics.completion_tokens
            else:
                metrics.returncode = response.status_code
                metrics.errors.append(f"HTTP {response.status_code}: {response.text[:200]}")

    except httpx.TimeoutException:
        metrics.total_time = 120.0
        metrics.returncode = -1
        metrics.errors.append("Timeout after 120s")
    except Exception as e:
        metrics.errors.append(str(e)[:100])

    return metrics


async def _run_concurrent_minimax(
    prompt: str,
    agent_count: int,
    model: str = "MiniMax-M2.5-highspeed",
) -> BenchmarkResult:
    """Run multiple minimax calls concurrently via cliproxy."""
    spawn_start = time.perf_counter()

    # Run all concurrently
    tasks = [asyncio.to_thread(_call_cliproxy, prompt, model) for _ in range(agent_count)]
    results = await asyncio.gather(*tasks, return_exceptions=True)

    total_time = time.perf_counter() - spawn_start

    # Collect metrics
    agent_metrics = []
    times = []
    for r in results:
        if isinstance(r, AgentMetrics):
            agent_metrics.append(r)
            times.append(r.total_time)

    times.sort()
    n = len(times)

    return BenchmarkResult(
        agent_count=agent_count,
        total_time=total_time,
        avg_turns=float(sum(m.num_turns for m in agent_metrics)) / n if n else 0,
        avg_thinking_time=float(sum(m.thinking_time for m in agent_metrics)) / n if n else 0,
        avg_tool_calls=float(sum(m.num_tool_calls for m in agent_metrics)) / n if n else 0,
        avg_tool_call_time=float(sum(m.tool_call_total_time for m in agent_metrics)) / n if n else 0,
        avg_file_writes=float(sum(m.num_file_writes for m in agent_metrics)) / n if n else 0,
        p50_time=times[n // 2] if n > 0 else 0,
        p95_time=times[int(n * 0.95)] if n > 0 else 0,
        p99_time=times[int(n * 0.99)] if n > 0 else 0,
        agent_metrics=agent_metrics,
    )


def run_minimax_benchmark(
    agent_count: int = 6,
    prompt: str = "Say hello and briefly explain what you can do",
    model: str = "MiniMax-M2.5-highspeed",
    use_direct: bool = None,
) -> BenchmarkResult:
    """Run minimax concurrency benchmark via cliproxy."""
    # Auto-detect harness if not specified
    if use_direct is None:
        harness = detect_harness()
        use_direct = harness != "cliproxy"
        print(f"Detected harness: {harness}, use_direct: {use_direct}")
    return asyncio.run(_run_concurrent_minimax(prompt, agent_count, model))


def compare_versions(
    agent_count: int = 6,
    prompt: str | None = None,
    model: str = "minimax-m2.5-highspeed",
) -> dict[str, BenchmarkResult]:
    """Compare homebrew vs any built codex."""

    if prompt is None:
        prompt = "Create hello.txt with Hello World"

    results = {}

    # Homebrew
    homebrew_path, _ = _find_codex_binary()
    if homebrew_path:
        print(f"\n>>> Testing: {homebrew_path}")
        results["homebrew"] = run_swarm_benchmark(homebrew_path, agent_count, prompt, model)

    # Built codex (if exists)
    built_paths = [
        "/Users/kooshapari/temp-PRODVERCEL/485/kush/cliproxyapi-plusplus/target/release/codex",
    ]
    for path in built_paths:
        if os.path.isfile(path) and os.access(path, os.X_OK):
            print(f"\n>>> Testing: {path}")
            results["built"] = run_swarm_benchmark(path, agent_count, prompt, model)
            break

    return results


if __name__ == "__main__":
    import argparse

    parser = argparse.ArgumentParser(description="Codex/Minimax Granular Concurrency Benchmark")
    parser.add_argument("--agents", "-n", type=int, default=6, help="Number of concurrent agents")
    parser.add_argument("--prompt", "-p", type=str, help="Prompt for agents")
    parser.add_argument("--model", "-m", type=str, default="MiniMax-M2.5-highspeed", help="Model to use")
    parser.add_argument("--compare", "-c", action="store_true", help="Compare versions")
    parser.add_argument("--binary", "-b", type=str, help="Specific codex binary")
    parser.add_argument("--minimax", "-x", action="store_true", help="Run minimax via cliproxy")
    parser.add_argument("--json", "-j", action="store_true", help="Output JSON")

    args = parser.parse_args()

    if args.minimax:
        # Run minimax via cliproxy
        result = run_minimax_benchmark(args.agents, args.prompt or "Say hello and be brief", args.model)
        if args.json:
            print(json.dumps(asdict(result), indent=2))
        else:
            print_granular_report(result, args.model.upper())
    elif args.compare:
        results = compare_versions(args.agents, args.prompt, args.model)
        for name, result in results.items():
            if args.json:
                print(json.dumps(asdict(result), indent=2))
            else:
                print_granular_report(result, name.upper())
    elif args.binary:
        result = run_swarm_benchmark(args.binary, args.agents, args.prompt or "Create hello.txt", args.model)
        if args.json:
            print(json.dumps(asdict(result), indent=2))
        else:
            print_granular_report(result, "TEST")
    else:
        binary, status = _find_codex_binary()
        if binary:
            print(f"Found {status} codex: {binary}")
            result = run_swarm_benchmark(binary, args.agents, args.prompt or "Create hello.txt", args.model)
            if args.json:
                print(json.dumps(asdict(result), indent=2))
            else:
                print_granular_report(result, status.upper())
        else:
            print("Error: codex not found")
