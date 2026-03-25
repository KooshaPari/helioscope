"""CPU profiling utilities for performance analysis.

Provides integration with perf (Linux) and Instruments (macOS) for CPU profiling.
"""

import os
import subprocess
import threading
import time
from dataclasses import dataclass, field
from typing import Optional, Any, Callable
from pathlib import Path
import json


@dataclass
class CPUProfileResult:
    """CPU profiling result."""

    success: bool = False
    duration_seconds: float = 0.0
    samples: int = 0
    hot_functions: list[dict] = field(default_factory=list)
    flamegraph_path: Optional[str] = None
    error: Optional[str] = None


class PerfProfiler:
    """Linux perf profiler wrapper.

    Usage:
        profiler = PerfProfiler()
        profiler.start()
        # ... run code ...
        result = profiler.stop()
        print(result.hot_functions)
    """

    def __init__(
        self,
        frequency: int = 99,
        duration: int = 30,
        output_dir: str = "/tmp",
    ):
        self._frequency = frequency
        self._duration = duration
        self._output_dir = Path(output_dir)
        self._running = False
        self._proc: Optional[subprocess.Popen] = None
        self._output_file: Optional[Path] = None

    def is_available(self) -> bool:
        """Check if perf is available."""
        try:
            result = subprocess.run(
                ["perf", "version"],
                capture_output=True,
                timeout=5,
            )
            return result.returncode == 0
        except FileNotFoundError, subprocess.TimeoutExpired:
            return False

    def start(self, pid: Optional[int] = None) -> bool:
        """Start profiling."""
        if self._running:
            return False

        if not self.is_available():
            return False

        try:
            self._output_file = self._output_dir / f"perf_{int(time.time())}.data"

            cmd = ["perf", "record", "-F", str(self._frequency), "-o", str(self._output_file)]
            if pid:
                cmd.extend(["-p", str(pid)])
            else:
                cmd.extend(["-a"])  # system-wide

            self._proc = subprocess.Popen(
                cmd,
                stdout=subprocess.PIPE,
                stderr=subprocess.PIPE,
            )
            self._running = True
            return True
        except Exception as e:
            return False

    def stop(self) -> CPUProfileResult:
        """Stop profiling and return results."""
        result = CPUProfileResult()

        if not self._running or not self._proc:
            result.error = "Profiler not running"
            return result

        try:
            self._proc.terminate()
            self._proc.wait(timeout=10)
            result.success = True

            # Generate report
            if self._output_file and self._output_file.exists():
                # Get symbol report
                report_cmd = [
                    "perf",
                    "report",
                    "-i",
                    str(self._output_file),
                    "--stdio",
                    "--no-children",
                    "-g",
                    "none",
                ]
                report = subprocess.run(
                    report_cmd,
                    capture_output=True,
                    text=True,
                    timeout=30,
                )

                # Parse hot functions (simplified)
                for line in report.stdout.splitlines()[:20]:
                    if line.strip() and not line.startswith("#"):
                        parts = line.split()
                        if parts:
                            result.hot_functions.append(
                                {
                                    "function": " ".join(parts[2:]) if len(parts) > 2 else parts[0],
                                    "percent": parts[1] if len(parts) > 1 else "0",
                                }
                            )

                result.samples = len(result.hot_functions)

        except Exception as e:
            result.error = str(e)

        self._running = False
        return result

    def generate_flamegraph(self) -> Optional[str]:
        """Generate flamegraph SVG."""
        if not self._output_file or not self._output_file.exists():
            return None

        try:
            # Check if flamegraph is available
            flamegraph.pl = shutil.which("flamegraph")
            if not flamegraph.pl:
                return None

            # Generate
            svg_file = self._output_file.with_suffix(".svg")

            # Run perf to flamegraph
            subprocess.run(
                f"perf script -i {self._output_file} | {flamegraph.pl} > {svg_file}",
                shell=True,
                check=True,
            )

            return str(svg_file)
        except Exception:
            return None


class CPUSampler:
    """Lightweight CPU sampling for quick hotspots."""

    def __init__(self, interval: float = 0.01):
        self._interval = interval
        self._running = False
        self._thread: Optional[threading.Thread] = None
        self._samples: list[dict] = []
        self._lock = threading.Lock()

    def start(self) -> None:
        """Start sampling."""
        if self._running:
            return
        self._running = True
        self._samples = []
        self._thread = threading.Thread(target=self._sample_loop, daemon=True)
        self._thread.start()

    def _sample_loop(self) -> None:
        """Background sampling loop."""
        import psutil

        while self._running:
            try:
                # Get current process and CPU
                proc = psutil.Process()
                cpu = proc.cpu_percent(interval=self._interval)

                # Get children
                children = proc.children(recursive=True)

                with self._lock:
                    self._samples.append(
                        {
                            "timestamp": time.time(),
                            "cpu_percent": cpu,
                            "memory_mb": proc.memory_info().rss / 1024 / 1024,
                            "num_threads": proc.num_threads(),
                            "children_count": len(children),
                        }
                    )
            except Exception:
                pass

    def stop(self) -> dict[str, Any]:
        """Stop and return statistics."""
        self._running = False

        with self._lock:
            if not self._samples:
                return {}

            cpu_values = [s["cpu_percent"] for s in self._samples]
            mem_values = [s["memory_mb"] for s in self._samples]

            return {
                "samples": len(self._samples),
                "cpu_avg": sum(cpu_values) / len(cpu_values),
                "cpu_peak": max(cpu_values),
                "memory_avg": sum(mem_values) / len(mem_values),
                "memory_peak": max(mem_values),
            }


def profile_function(func: Callable) -> Callable:
    """Decorator to profile CPU usage of a function.

    Usage:
        @profile_function
        def my_function():
            pass
    """

    def wrapper(*args, **kwargs):
        sampler = CPUSampler()
        sampler.start()

        result = func(*args, **kwargs)

        stats = sampler.stop()

        # Attach to result if possible
        if isinstance(result, dict):
            result["_cpu_profile"] = stats

        return result

    return wrapper


def get_cpu_info() -> dict[str, Any]:
    """Get CPU information."""
    import psutil

    cpu = psutil.cpu_times()
    return {
        "physical_cores": psutil.cpu_count(logical=False),
        "logical_cores": psutil.cpu_count(logical=True),
        "freq_mhz": psutil.cpu_freq().current if psutil.cpu_freq() else None,
        "arch": os.uname().machine,
    }
