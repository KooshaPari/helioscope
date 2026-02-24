"""Memory profiling utilities for benchmark analysis.

Provides memory tracking and profiling capabilities for detecting leaks
and analyzing memory usage patterns.
"""

import gc
import os
import sys
import threading
import time
import tracemalloc
from dataclasses import dataclass, field
from typing import Any, Optional, Callable
from functools import wraps


@dataclass
class MemorySnapshot:
    """Memory usage snapshot at a point in time."""
    timestamp: float
    rss_mb: float = 0.0
    vms_mb: float = 0.0
    python_allocated_mb: float = 0.0
    python_peak_mb: float = 0.0
    gc_counts: tuple = field(default_factory=lambda: (0, 0, 0))
    object_count: int = 0


@dataclass
class MemoryReport:
    """Complete memory analysis report."""
    snapshots: list[MemorySnapshot] = field(default_factory=list)
    leaks_detected: list[str] = field(default_factory=list)
    peak_rss_mb: float = 0.0
    avg_rss_mb: float = 0.0
    rss_growth_mb: float = 0.0
    object_growth: int = 0
    allocation_spikes: list[dict[str, Any]] = field(default_factory=list)


class MemoryProfiler:
    """Memory profiler for tracking usage over time.
    
    Usage:
        profiler = MemoryProfiler()
        profiler.start()
        # ... run code ...
        report = profiler.stop()
        print(report.rss_growth_mb)
    """
    
    def __init__(self, interval: float = 0.1, track_allocations: bool = True):
        self._interval = interval
        self._track_allocations = track_allocations
        self._running = False
        self._thread: Optional[threading.Thread] = None
        self._snapshots: list[MemorySnapshot] = []
        self._start_time = 0.0
        self._start_rss = 0.0
        self._start_objects = 0
    
    def start(self) -> None:
        """Start memory profiling."""
        if self._running:
            return
        
        self._running = True
        self._snapshots = []
        self._start_time = time.perf_counter()
        
        # Start tracemalloc if available
        if self._track_allocations:
            tracemalloc.start()
        
        # Record initial state
        self._take_snapshot()
        self._start_rss = self._snapshots[0].rss_mb if self._snapshots else 0
        self._start_objects = self._snapshots[0].object_count if self._snapshots else 0
        
        # Start background thread
        self._thread = threading.Thread(target=self._collect_loop, daemon=True)
        self._thread.start()
    
    def _take_snapshot(self) -> MemorySnapshot:
        """Take a single memory snapshot."""
        import psutil
        process = psutil.Process(os.getpid())
        
        mem_info = process.memory_info()
        rss_mb = mem_info.rss / 1024 / 1024
        vms_mb = mem_info.vms / 1024 / 1024
        
        # Python-specific
        python_allocated = 0.0
        python_peak = 0.0
        if self._track_allocations and tracemalloc.is_tracing():
            current, peak = tracemalloc.get_traced_memory()
            python_allocated = current / 1024 / 1024
            python_peak = peak / 1024 / 1024
        
        # GC stats
        gc_counts = gc.get_count()
        
        # Object count
        object_count = sum(len(gc.get_objects(i)) for i in range(gc.get_count()[0]))
        
        snapshot = MemorySnapshot(
            timestamp=time.perf_counter(),
            rss_mb=rss_mb,
            vms_mb=vms_mb,
            python_allocated_mb=python_allocated,
            python_peak_mb=python_peak,
            gc_counts=gc_counts,
            object_count=object_count,
        )
        
        self._snapshots.append(snapshot)
        return snapshot
    
    def _collect_loop(self) -> None:
        """Background collection loop."""
        while self._running:
            self._take_snapshot()
            time.sleep(self._interval)
    
    def stop(self) -> MemoryReport:
        """Stop profiling and return report."""
        self._running = False
        
        # Take final snapshot
        self._take_snapshot()
        
        # Stop tracemalloc
        if self._track_allocations and tracemalloc.is_tracing():
            tracemalloc.stop()
        
        # Generate report
        return self._generate_report()
    
    def _generate_report(self) -> MemoryReport:
        """Generate memory analysis report."""
        if not self._snapshots:
            return MemoryReport()
        
        report = MemoryReport(snapshots=self._snapshots)
        
        # Calculate RSS stats
        rss_values = [s.rss_mb for s in self._snapshots]
        report.peak_rss_mb = max(rss_values)
        report.avg_rss_mb = sum(rss_values) / len(rss_values)
        
        # Calculate growth
        if len(self._snapshots) >= 2:
            report.rss_growth_mb = self._snapshots[-1].rss_mb - self._snapshots[0].rss_mb
        
        # Object growth
        if self._snapshots:
            report.object_growth = self._snapshots[-1].object_count - self._start_objects
        
        # Detect leaks (growth > threshold)
        if report.rss_growth_mb > 50:  # 50MB threshold
            report.leaks_detected.append(f"RSS grew by {report.rss_growth_mb:.1f}MB")
        
        if report.object_growth > 1000:
            report.leaks_detected.append(f"Objects grew by {report.object_growth}")
        
        # Find allocation spikes
        for i in range(1, len(self._snapshots)):
            delta = self._snapshots[i].rss_mb - self._snapshots[i-1].rss_mb
            if delta > 20:  # 20MB spike
                report.allocation_spikes.append({
                    'time': self._snapshots[i].timestamp - self._start_time,
                    'delta_mb': delta,
                    'rss_mb': self._snapshots[i].rss_mb,
                })
        
        return report
    
    def get_current_snapshot(self) -> Optional[MemorySnapshot]:
        """Get current memory snapshot."""
        if self._running:
            return self._take_snapshot()
        return self._snapshots[-1] if self._snapshots else None


def profile_memory(func: Callable) -> Callable:
    """Decorator to profile memory usage of a function.
    
    Usage:
        @profile_memory
        def my_function():
            # ... code ...
            pass
        
        result = my_function()
        print(result['report'].rss_growth_mb)
    """
    @wraps(func)
    def wrapper(*args, **kwargs):
        profiler = MemoryProfiler()
        profiler.start()
        
        try:
            result = func(*args, **kwargs)
            return result
        finally:
            report = profiler.stop()
            # Attach report to result if possible
            if isinstance(result, dict):
                result['_memory_report'] = report
    
    # Also attach profiler for manual control
    wrapper.profiler = MemoryProfiler()
    return wrapper


class LeakDetector:
    """Detects memory leaks in long-running processes.
    
    Usage:
        detector = LeakDetector(threshold_mb=100)
        detector.start()
        
        # Run for a while...
        
        if detector.check():
            print("LEAK DETECTED!")
    """
    
    def __init__(self, threshold_mb: float = 100, window_size: int = 10):
        self._threshold_mb = threshold_mb
        self._window_size = window_size
        self._profiler = MemoryProfiler()
        self._baseline_rss = 0.0
    
    def start(self) -> None:
        """Start leak detection."""
        self._profiler.start()
        # Wait a bit for baseline
        import time
        time.sleep(0.5)
        snapshot = self._profiler.get_current_snapshot()
        if snapshot:
            self._baseline_rss = snapshot.rss_mb
    
    def check(self) -> bool:
        """Check for memory leak."""
        report = self._profiler.stop()
        
        # Check if RSS grew beyond threshold
        if report.rss_growth_mb > self._threshold_mb:
            return True
        
        # Check for consistent growth pattern
        if len(report.snapshots) >= self._window_size:
            recent = report.snapshots[-self._window_size:]
            growth_rates = []
            for i in range(1, len(recent)):
                rate = (recent[i].rss_mb - recent[i-1].rss_mb)
                growth_rates.append(rate)
            
            # If consistently growing
            if all(g > 0 for g in growth_rates) and sum(growth_rates) > self._threshold_mb:
                return True
        
        return False
    
    def get_report(self) -> MemoryReport:
        """Get detailed memory report."""
        return self._profiler.stop()


def get_memory_usage() -> dict[str, float]:
    """Get current memory usage statistics.
    
    Returns:
        Dictionary with RSS, VMS, Python allocated in MB
    """
    import psutil
    process = psutil.Process(os.getpid())
    mem_info = process.memory_info()
    
    result = {
        'rss_mb': mem_info.rss / 1024 / 1024,
        'vms_mb': mem_info.vms / 1024 / 1024,
    }
    
    if tracemalloc.is_tracing():
        current, peak = tracemalloc.get_traced_memory()
        result['python_current_mb'] = current / 1024 / 1024
        result['python_peak_mb'] = peak / 1024 / 1024
    
    return result
