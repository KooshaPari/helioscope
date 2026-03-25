#!/usr/bin/env python3
from __future__ import annotations

import argparse
import hashlib
import json
import os
import platform
from datetime import datetime, timezone
from pathlib import Path
import sys
from jsonschema import validate

ROOT = Path(__file__).resolve().parents[1] / "src"
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))


def _command_plan_hash(commands: list[dict]) -> str:
    payload = json.dumps(commands, sort_keys=True, separators=(",", ":"))
    return hashlib.sha256(payload.encode()).hexdigest()


def _canonicalize_plan(commands: list[dict]) -> list[dict]:
    return sorted(
        [
            {
                "command": command.command,
                "bucket": command.bucket.value,
                "required": command.required,
                "cwd": command.cwd,
                "source": command.source,
                "rationale": command.rationale,
            }
            for command in commands
        ],
        key=lambda entry: (
            entry["bucket"],
            entry["required"],
            entry["command"],
            entry["source"],
            entry["cwd"],
        ),
    )


def _plan_diff(before: list[dict], after: list[dict]) -> dict[str, list[str]]:
    before_set = {entry.get("command", "") for entry in before}
    after_set = {entry.get("command", "") for entry in after}
    return {
        "added": sorted(after_set - before_set),
        "removed": sorted(before_set - after_set),
    }


def _iter_commands(discovery) -> list[dict]:
    return [
        {
            "command": command.command,
            "bucket": command.bucket.value,
            "required": command.required,
            "cwd": command.cwd,
            "source": command.source,
            "rationale": command.rationale,
        }
        for bucket in discovery.buckets.values()
        for command in bucket
    ]


def _reproducibility_metadata(profile: str, args) -> dict:
    return {
        "profile": profile,
        "runner_config": {
            "max_parallel": args.max_parallel,
            "timeout": args.timeout,
            "retries": args.retries,
            "retry_delay": args.retry_delay,
            "continue_on_fail": bool(args.continue_on_fail),
            "budget_seconds": args.budget,
        },
        "environment": {
            "python": platform.python_version(),
            "platform": platform.platform(),
            "working_dir": str(Path.cwd()),
            "argv": sys.argv,
            "pid": os.getpid(),
        },
    }


def run_discovery(root: str, out: str, max_scan_depth: int) -> None:
    from harness.discoverer import Discoverer
    from harness.interfaces import DiscoverInput

    out_path = Path(out)
    discoverer = Discoverer()
    discovery = discoverer.discover(DiscoverInput(repo_root=root, max_scan_depth=max_scan_depth))
    out_path.write_text(discovery.to_json())


def run_runner(repo: str, profile: str, out: str, args) -> None:
    from harness.discoverer import Discoverer
    from harness.interfaces import DiscoverInput
    from harness.runner import Runner, RunnerConfig
    from harness.normalizer import QualityNormalizer
    from harness.schema import evidence_payload

    discoverer = Discoverer()
    discovery = discoverer.discover(DiscoverInput(repo_root=repo))
    flat_commands = [cmd for bucket in discovery.buckets.values() for cmd in bucket]

    commands = _canonicalize_plan(flat_commands)
    command_hash = _command_plan_hash(commands)

    replay_payload = None
    plan_diff = None
    if args.replay:
        prior_path = Path(args.replay)
        if prior_path.exists():
            prior_payload = json.loads(prior_path.read_text())
            prior_commands = prior_payload.get("plan")
            if not isinstance(prior_commands, list):
                prior_commands = prior_payload.get("commands", [])
            if isinstance(prior_commands, list):
                prior_plan = [
                    {
                        "command": c.get("command", ""),
                        "bucket": c.get("bucket", ""),
                        "required": c.get("required", False),
                        "cwd": c.get("cwd", "."),
                        "source": c.get("source", ""),
                        "rationale": c.get("rationale", ""),
                    }
                    for c in prior_commands
                ]
                prior_plan = sorted(
                    prior_plan,
                    key=lambda entry: (
                        entry["bucket"],
                        entry["required"],
                        entry["command"],
                        entry["source"],
                        entry["cwd"],
                    ),
                )
                prior_hash = _command_plan_hash(prior_plan)
            else:
                prior_hash = None
                prior_plan = []
            replay_payload = {
                "path": str(prior_path),
                "prior_plan_hash": prior_hash,
                "same_plan": prior_hash == command_hash,
            }
            plan_diff = _plan_diff(prior_plan, commands)

    result = {
        "repo": repo,
        "profile": profile,
        "created_at": datetime.now(tz=timezone.utc).isoformat(),
        "plan_hash": command_hash,
        "plan": commands,
        "command_count": len(commands),
        "reproducibility": _reproducibility_metadata(profile, args),
    }

    if replay_payload is not None:
        result["replay"] = {
            **replay_payload,
            "plan_diff": plan_diff,
        }

    if args.dry_run:
        result["result_code"] = "WARN" if not commands else "PASS"
        Path(out).write_text(json.dumps(result, indent=2))
        return

    runner = Runner(
        RunnerConfig(
            timeout_seconds=args.timeout,
            continue_on_fail=args.continue_on_fail,
            max_parallel_jobs=args.max_parallel,
            profile=profile,
            retries=args.retries,
            retry_delay_seconds=args.retry_delay,
            budget_seconds=args.budget,
        )
    )
    runs = runner.run_profile(flat_commands, repo)
    normalization = QualityNormalizer().normalize(runs, flat_commands)
    payload = evidence_payload(discovery, runs, normalization)
    payload["plan_hash"] = command_hash
    payload["reproducibility"] = _reproducibility_metadata(profile, args)
    payload["created_at"] = datetime.now(tz=timezone.utc).isoformat()
    payload["command_count"] = len(commands)

    if args.replay:
        replay_path = Path(args.replay)
        if replay_payload is None and replay_path.exists():
            prior = json.loads(replay_path.read_text())
            prior_commands = prior.get("plan")
            if not isinstance(prior_commands, list):
                prior_commands = prior.get("commands", [])
            prior_hash = prior.get("plan_hash")
            if prior_hash is None and isinstance(prior_commands, list):
                prior_plan = [
                    {
                        "command": c.get("command", ""),
                        "bucket": c.get("bucket", ""),
                        "required": c.get("required", False),
                        "cwd": c.get("cwd", "."),
                        "source": c.get("source", ""),
                        "rationale": c.get("rationale", ""),
                    }
                    for c in prior_commands
                ]
                prior_plan = sorted(
                    prior_plan,
                    key=lambda entry: (
                        entry["bucket"],
                        entry["required"],
                        entry["command"],
                        entry["source"],
                        entry["cwd"],
                    ),
                )
                prior_hash = _command_plan_hash(prior_plan)
            payload["replay"] = {
                "path": str(replay_path),
                "prior_plan_hash": prior_hash,
                "same_plan": prior_hash == command_hash,
                "plan_diff": _plan_diff(prior_commands, commands),
            }
        elif replay_payload is not None:
            payload["replay"] = {
                **replay_payload,
                "plan_hash": command_hash,
                "plan_diff": plan_diff,
            }

    Path(out).write_text(json.dumps(payload, indent=2))


def normalize_run(input_file: str, out: str) -> None:
    payload = json.loads(Path(input_file).read_text())
    from harness.normalizer import QualityNormalizer
    from harness.interfaces import RunResult

    raw_runs = payload.get("runs", [])
    if not raw_runs:
        raise SystemExit("input file missing runs")

    runs = [
        RunResult(
            command=run.get("command"),
            bucket=run.get("bucket", ""),
            returncode=run.get("returncode", 0),
            started_at=run.get("started_at", ""),
            finished_at=run.get("finished_at", ""),
            stdout_file=run.get("stdout_file", ""),
            stderr_file=run.get("stderr_file", ""),
            duration_ms=run.get("duration_ms", 0),
            artifact_dir=run.get("artifact_dir", ""),
            attempts=run.get("attempts", 1),
            timed_out=run.get("timed_out", False),
            error=run.get("error"),
            skipped=run.get("skipped", False),
        )
        for run in raw_runs
    ]

    discovered_commands = payload.get("commands", [])
    result = QualityNormalizer().normalize(runs, discovered_commands)
    Path(out).write_text(json.dumps({"quality": result.__dict__, "source": str(input_file)}, indent=2))


def validate_artifacts(schema: str, file: str) -> None:
    payload = json.loads(Path(file).read_text())
    schema_json = json.loads(Path(schema).read_text())
    validate(instance=payload, schema=schema_json)
    print("VALID")


# =============================================================================
# Teammates CLI
# =============================================================================


def cmd_teammates_list(agents_dir: str) -> None:
    from harness import TeammateRegistry

    registry = TeammateRegistry(agents_dir=Path(agents_dir))
    teammates = registry.discover()

    if not teammates:
        print("No teammates found")
        return

    print(f"Found {len(teammates)} teammates:\n")
    for t in teammates.values():
        print(f"  {t.id}: {t.name} ({t.role})")
        print(f"    {t.description[:60]}...")


def cmd_teammates_delegate(teammate_id: str, task: str, timeout: int, profile: str) -> None:
    import asyncio
    from harness import TeammateRegistry, DelegationRequest, DelegationProtocol, CodexExecutor, Priority

    async def run():
        registry = TeammateRegistry()
        registry.discover()

        teammate = registry.get(teammate_id)
        if not teammate:
            print(f"Teammate not found: {teammate_id}")
            return

        protocol = DelegationProtocol()
        executor = CodexExecutor(profile=profile)

        request = DelegationRequest(
            teammate_id=teammate_id, task_description=task, priority=Priority.NORMAL, timeout_seconds=timeout
        )

        result = await protocol.delegate(request, executor)

        print(f"Delegation: {result.delegation_id}")
        print(f"Status: {result.status}")
        print(f"Duration: {result.duration_ms}ms")
        if result.result:
            print(f"Result: {result.result[:200]}...")
        if result.error:
            print(f"Error: {result.error}")

    asyncio.run(run())


def cmd_teammates_status(delegation_id: str) -> None:
    from harness import DelegationProtocol

    protocol = DelegationProtocol()
    result = protocol.get_status(delegation_id)

    if result:
        print(f"Delegation: {result.delegation_id}")
        print(f"Status: {result.status}")
        print(f"Duration: {result.duration_ms}ms")
    else:
        print(f"Delegation not found: {delegation_id}")


# =============================================================================
# Scaling CLI
# =============================================================================


def cmd_scaling_status() -> None:
    from harness import ScalingConfig, DynamicLimitController, ResourceSampler

    sampler = ResourceSampler()
    controller = DynamicLimitController()

    snapshot = sampler.sample()

    print("Resource Status:")
    print(f"  CPU: {snapshot.cpu_percent:.1f}%")
    print(f"  Memory: {snapshot.memory_percent:.1f}% ({snapshot.memory_available_mb:.0f}MB available)")
    print(f"  FDs: {snapshot.fd_count}/{snapshot.fd_limit}")
    print(f"  Load: {snapshot.load_avg:.2f}")
    print(f"\nDynamic Limit: {controller.current_limit}")
    print(f"State: {controller._state}")


# =============================================================================
# Cache CLI
# =============================================================================


def cmd_cache_stats() -> None:
    from harness import L1Cache

    cache = L1Cache()
    stats = cache.stats

    print("L1 Cache Stats:")
    print(f"  Hits: {stats.hits}")
    print(f"  Misses: {stats.misses}")
    print(f"  Hit Rate: {stats.hit_rate:.1%}")


def cmd_cache_clear() -> None:
    from harness import L1Cache, L2Cache

    l1 = L1Cache()
    l2 = L2Cache()

    # Clear L1 (recreate)
    l1._cache.clear()
    print("L1 cache cleared")

    # Clear L2
    l2.clear()
    print("L2 cache cleared")


def main() -> None:
    p = argparse.ArgumentParser()
    sp = p.add_subparsers(dest="cmd", required=True)

    d = sp.add_parser("discover")
    d.add_argument("--root", required=True)
    d.add_argument("--out", required=True)
    d.add_argument("--max-scan-depth", type=int, default=3)

    r = sp.add_parser("run")
    r.add_argument("--repo", required=True)
    r.add_argument("--profile", default="strict-full")
    r.add_argument("--out", required=True)
    r.add_argument("--max-parallel", type=int, default=2)
    r.add_argument("--timeout", type=int, default=1200)
    r.add_argument("--retries", type=int, default=0)
    r.add_argument("--retry-delay", type=float, default=1.0)
    r.add_argument("--budget", type=int)
    r.add_argument("--continue-on-fail", action="store_true")
    r.add_argument("--dry-run", action="store_true")
    r.add_argument("--replay")

    n = sp.add_parser("normalize")
    n.add_argument("--in", dest="input_file", required=True)
    n.add_argument("--out", required=True)

    v = sp.add_parser("validate")
    v.add_argument("--schema", required=True)
    v.add_argument("--file", required=True)

    # Teammates commands
    t = sp.add_parser("teammates")
    t_sp = t.add_subparsers(dest="teammates_cmd")

    t_list = t_sp.add_parser("list")
    t_list.add_argument("--agents-dir", default="agents")

    t_delegate = t_sp.add_parser("delegate")
    t_delegate.add_argument("--teammate", required=True)
    t_delegate.add_argument("--task", required=True)
    t_delegate.add_argument("--timeout", type=int, default=300)
    t_delegate.add_argument("--profile", default="default")

    t_status = t_sp.add_parser("status")
    t_status.add_argument("--delegation-id", required=True)

    # Scaling commands
    s = sp.add_parser("scaling")
    s_sp = s.add_subparsers(dest="scaling_cmd")

    s_status = s_sp.add_parser("status")

    # Cache commands
    c = sp.add_parser("cache")
    c_sp = c.add_subparsers(dest="cache_cmd")

    c_stats = c_sp.add_parser("stats")
    c_clear = c_sp.add_parser("clear")

    args = p.parse_args()

    if args.cmd == "discover":
        run_discovery(args.root, args.out, args.max_scan_depth)
    elif args.cmd == "run":
        run_runner(args.repo, args.profile, args.out, args)
    elif args.cmd == "normalize":
        normalize_run(args.input_file, args.out)
    elif args.cmd == "validate":
        validate_artifacts(args.schema, args.file)
    elif args.cmd == "teammates":
        if args.teammates_cmd == "list":
            cmd_teammates_list(args.agents_dir)
        elif args.teammates_cmd == "delegate":
            cmd_teammates_delegate(args.teammate, args.task, args.timeout, args.profile)
        elif args.teammates_cmd == "status":
            cmd_teammates_status(args.delegation_id)
    elif args.cmd == "scaling":
        if args.scaling_cmd == "status":
            cmd_scaling_status()
    elif args.cmd == "cache":
        if args.cache_cmd == "stats":
            cmd_cache_stats()
        elif args.cache_cmd == "clear":
            cmd_cache_clear()


if __name__ == "__main__":
    main()
