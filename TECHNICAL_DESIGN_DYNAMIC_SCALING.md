# Technical Design: Dynamic Thread Limit System

**Version**: 1.0  
**Date**: 2026-02-23  
**Project**: heliosHarness

---

## Table of Contents

1. [Overview](#overview)
2. [Architecture](#architecture)
3. [Data Models](#data-models)
4. [Resource Monitoring](#resource-monitoring)
5. [Dynamic Limit Computation](#dynamic-limit-computation)
6. [Hysteresis Control](#hysteresis-control)
7. [Backpressure Mechanism](#backpressure-mechanism)
8. [Configuration](#configuration)
9. [API Reference](#api-reference)

---

## Overview

The Dynamic Thread Limit System replaces fixed concurrency limits with resource-aware adaptive limits. It:

1. Samples system resources (CPU, memory, FD, load)
2. Applies hysteresis to prevent thrashing
3. Computes dynamic limit with safety buffers
4. Provides backpressure when overloaded

---

## Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                    DynamicLimitController                         │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│  ┌──────────────────┐    ┌─────────────────────────────────┐ │
│  │  ResourceSampler │───▶│   HysteresisController          │ │
│  │  (CPU, Mem, FD, │    │   upper_threshold: 0.80         │ │
│  │   Load)          │    │   lower_threshold: 0.60         │ │
│  │  interval: 1-5s  │    │   dwell_time: 30s              │ │
│  └──────────────────┘    └─────────────────────────────────┘ │
│           │                           │                          │
│           │                           ▼                          │
│           │                  ┌─────────────────────────────────┐ │
│           └─────────────────▶│  compute_dynamic_limit          │ │
│                            │  min_buffer: 0.05               │ │
│                            │  discretionary_buffer: 0.15     │ │
│                            └─────────────────────────────────┘ │
│                                            │                   │
│                                            ▼                   │
│                            ┌─────────────────────────────────┐ │
│                            │   LimitGate (semaphore)         │ │
│                            │   effective_limit: int          │ │
│                            └─────────────────────────────────┘ │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

---

## Data Models

### ResourceSnapshot

```python
@dataclass
class ResourceSnapshot:
    """Current system resource usage."""

    # CPU
    cpu_percent: float  # 0.0 - 100.0

    # Memory
    memory_percent: float  # 0.0 - 100.0
    memory_available_mb: float

    # File Descriptors
    fd_used: int
    fd_limit: int
    fd_percent: float  # 0.0 - 100.0

    # Load Average (1min, 5min, 15min)
    load_avg_1m: float
    load_avg_5m: float
    load_avg_15m: float

    # Computed
    timestamp: float

    @property
    def load_normalized(self) -> float:
        """Normalize load to 0-1 based on CPU cores."""
        cpu_count = os.cpu_count() or 4
        return min(self.load_avg_1m / cpu_count, 1.0)
```

### LimitGateConfig

```python
@dataclass
class LimitGateConfig:
    """Configuration for dynamic limits."""

    # Buffer thresholds
    min_buffer: float = 0.05  # 5% hard limit
    discretionary_buffer: float = 0.15  # 15% soft limit

    # Resource thresholds (0.0-1.0)
    cpu_threshold: float = 0.80
    memory_threshold: float = 0.85
    fd_threshold: float = 0.80
    load_threshold: float = 0.80

    # Sampling
    sampling_interval_seconds: float = 2.0

    @classmethod
    def from_dict(cls, d: dict) -> "LimitGateConfig":
        """Build from dict (e.g. settings)."""
        return cls(
            min_buffer=d.get("min_buffer", 0.05),
            discretionary_buffer=d.get("discretionary_buffer", 0.15),
            cpu_threshold=d.get("cpu_threshold", 0.80),
            memory_threshold=d.get("memory_threshold", 0.85),
            fd_threshold=d.get("fd_threshold", 0.80),
            load_threshold=d.get("load_threshold", 0.80),
            sampling_interval_seconds=d.get("sampling_interval_seconds", 2.0),
        )
```

### HysteresisController

```python
@dataclass
class HysteresisState:
    """Current hysteresis state."""

    state: Literal["stable", "scaling_up", "scaling_down"] = "stable"
    last_change_time: float = 0.0
    consecutive_same_direction: int = 0

class HysteresisController:
    """Prevents thrashing by using upper/lower thresholds."""

    def __init__(
        self,
        upper_threshold: float = 0.80,
        lower_threshold: float = 0.60,
        dwell_time_seconds: int = 30,
    ):
        self.upper_threshold = upper_threshold
        self.lower_threshold = lower_threshold
        self.dwell_time_seconds = dwell_time_seconds
        self._state = HysteresisState()

    def get_limit(
        self,
        current_limit: int,
        running_count: int,
        target_limit: int,
    ) -> int:
        """Apply hysteresis to determine the new limit."""

        now = time.time()
        time_since_change = now - self._state.last_change_time

        # Determine direction
        if target_limit > current_limit:
            desired_direction = "up"
        elif target_limit < current_limit:
            desired_direction = "down"
        else:
            return current_limit

        # Check if we should change
        if desired_direction == "up":
            # Can always scale up after dwell time
            if time_since_change >= self.dwell_time_seconds:
                self._state = HysteresisState(
                    state="stable",
                    last_change_time=now,
                    consecutive_same_direction=0,
                )
                return target_limit
        else:
            # Scale down only if we're above target and dwell passed
            if running_count < target_limit and time_since_change >= self.dwell_time_seconds:
                self._state = HysteresisState(
                    state="stable",
                    last_change_time=now,
                    consecutive_same_direction=0,
                )
                return target_limit

        # Hold current limit
        return current_limit
```

---

## Resource Monitoring

### ResourceSampler

```python
class ResourceSampler:
    """Sample system resources cross-platform."""

    def __init__(self, use_native: bool = False):
        self.use_native = use_native

    def sample(self) -> ResourceSnapshot:
        """Sample all resources."""

        # CPU
        cpu_percent = psutil.cpu_percent(interval=0.1)

        # Memory
        mem = psutil.virtual_memory()
        memory_percent = mem.percent
        memory_available_mb = mem.available / (1024 * 1024)

        # File Descriptors (Unix)
        try:
            fd_used = len(os.listdir('/proc/self/fd'))
            fd_limit = 1024  # Default, may need adjustment
            fd_percent = fd_used / fd_limit
        except NotImplementedError:
            # macOS fallback
            try:
                import resource
                soft, hard = resource.getrlimit(resource.RLIMIT_NOFILE)
                fd_used = soft
                fd_limit = soft
                fd_percent = fd_used / fd_limit
            except:
                fd_used = 0
                fd_limit = 1024
                fd_percent = 0.0

        # Load Average
        try:
            load_avg_1m, load_avg_5m, load_avg_15m = os.getloadavg()
        except NotImplementedError:
            load_avg_1m = load_avg_5m = load_avg_15m = 0.0

        return ResourceSnapshot(
            cpu_percent=cpu_percent,
            memory_percent=memory_percent,
            memory_available_mb=memory_available_mb,
            fd_used=fd_used,
            fd_limit=fd_limit,
            fd_percent=fd_percent,
            load_avg_1m=load_avg_1m,
            load_avg_5m=load_avg_5m,
            load_avg_15m=load_avg_15m,
            timestamp=time.time(),
        )
```

---

## Dynamic Limit Computation

### compute_dynamic_limit

```python
def compute_dynamic_limit(
    snapshot: ResourceSnapshot,
    config: LimitGateConfig,
    running_count: int,
) -> tuple[int, dict]:
    """
    Compute max concurrent slots from resource gates.

    Returns (effective_limit, gate_details).
    """

    # Normalize each resource to 0-1
    cpu_util = snapshot.cpu_percent / 100.0
    mem_util = snapshot.memory_percent / 100.0
    fd_util = snapshot.fd_percent

    # Normalize load to 0-1
    load_util = snapshot.load_normalized

    # Calculate available headroom for each resource
    # Headroom = 1.0 - utilization (so 1.0 = fully available)
    cpu_headroom = max(0.0, 1.0 - cpu_util)
    mem_headroom = max(0.0, 1.0 - mem_util)
    fd_headroom = max(0.0, 1.0 - fd_util)
    load_headroom = max(0.0, 1.0 - load_util)

    # Minimum headroom across all resources
    min_headroom = min(cpu_headroom, mem_headroom, fd_headroom, load_headroom)

    # Calculate limits at different buffer levels
    # Base: assume max 100 concurrent as baseline
    max_baseline = 100

    # Hard limit: use min_buffer (5%)
    hard_limit = int(max_baseline * (1.0 - config.min_buffer))

    # Soft limit: use discretionary_buffer (15%)
    soft_limit = int(max_baseline * (1.0 - config.discretionary_buffer))

    # Dynamic limit: scale with headroom
    dynamic_limit = int(max_baseline * min_headroom)

    # Effective limit: most restrictive
    effective_limit = min(hard_limit, soft_limit, dynamic_limit)

    # Ensure minimum of 1
    effective_limit = max(1, effective_limit)

    # Gate details for logging/metrics
    gate_details = {
        "cpu_util": cpu_util,
        "mem_util": mem_util,
        "fd_util": fd_util,
        "load_util": load_util,
        "min_headroom": min_headroom,
        "hard_limit": hard_limit,
        "soft_limit": soft_limit,
        "dynamic_limit": dynamic_limit,
        "effective_limit": effective_limit,
        "running_count": running_count,
    }

    return effective_limit, gate_details
```

---

## Hysteresis Control

### Integration

```python
class DynamicLimitController:
    """Main controller for dynamic limits."""

    def __init__(
        self,
        config: LimitGateConfig | None = None,
        initial_limit: int = 10,
    ):
        self.config = config or LimitGateConfig()
        self.current_limit = initial_limit
        self.sampler = ResourceSampler()
        self.hysteresis = HysteresisController(
            upper_threshold=0.80,
            lower_threshold=0.60,
            dwell_time_seconds=30,
        )

    async def run(self):
        """Main loop - sample and adjust."""
        while True:
            # Sample resources
            snapshot = self.sampler.sample()

            # Compute dynamic limit
            target_limit, details = compute_dynamic_limit(
                snapshot,
                self.config,
                self._running_count,
            )

            # Apply hysteresis
            self.current_limit = self.hysteresis.get_limit(
                current_limit=self.current_limit,
                running_count=self._running_count,
                target_limit=target_limit,
            )

            # Log details
            logger.info(
                "dynamic_limit_update",
                current=self.current_limit,
                target=target_limit,
                running=self._running_count,
                **details,
            )

            # Wait for next sample
            await asyncio.sleep(self.config.sampling_interval_seconds)
```

---

## Backpressure Mechanism

### PriorityQueue with Backpressure

```python
class BackpressureQueue:
    """Priority queue with backpressure."""

    def __init__(
        self,
        max_size: int = 100,
        backpressure_threshold: float = 0.75,
    ):
        self.max_size = max_size
        self.backpressure_threshold = backpressure_threshold
        self._queue: asyncio.PriorityQueue = asyncio.PriorityQueue(
            maxsize=max_size
        )
        self._priority_counts = defaultdict(int)

    async def enqueue(
        self,
        item: Any,
        priority: int = 2,  # 0=CRITICAL, 1=HIGH, 2=NORMAL, 3=LOW
    ) -> bool:
        """Try to enqueue item. Returns False if rejected."""

        load_percent = self._queue.qsize() / self.max_size

        # Check backpressure based on priority
        if priority == 0:  # CRITICAL
            can_accept = self._queue.qsize() < self.max_size
        elif priority == 1:  # HIGH
            can_accept = load_percent < 0.90
        elif priority == 2:  # NORMAL
            can_accept = load_percent < 0.75
        else:  # LOW
            can_accept = load_percent < 0.50

        if not can_accept:
            logger.warning(
                "backpressure_reject",
                priority=priority,
                size=self._queue.qsize(),
                max_size=self.max_size,
            )
            return False

        await self._queue.put((priority, item))
        self._priority_counts[priority] += 1
        return True

    async def dequeue(self) -> Any:
        """Dequeue highest priority item."""
        priority, item = await self._queue.get()
        self._priority_counts[priority] -= 1
        return item
```

---

## Configuration

### YAML Configuration

```yaml
# heliosharness.yaml
scaling:
  # Buffer thresholds
  min_buffer: 0.05 # 5% - hard limit
  discretionary_buffer: 0.15 # 15% - soft limit

  # Resource thresholds
  cpu_threshold: 0.80
  memory_threshold: 0.85
  fd_threshold: 0.80
  load_threshold: 0.80

  # Sampling
  sampling_interval_seconds: 2.0

  # Hysteresis
  hysteresis:
    upper_threshold: 0.80
    lower_threshold: 0.60
    dwell_time_seconds: 30

# Queue
queue:
  max_size: 100
  backpressure_threshold: 0.75

  # Priority levels
  priority_levels:
    CRITICAL: 0
    HIGH: 1
    NORMAL: 2
    LOW: 3
```

### Environment Variables

```bash
# Scaling
HELIOS_MIN_BUFFER=0.05
HELIOS_DISCRETIONARY_BUFFER=0.15
HELIOS_SAMPLING_INTERVAL=2.0
HELIOS_HYSTERESIS_UPPER=0.80
HELIOS_HYSTERESIS_LOWER=0.60
HELIOS_HYSTERESIS_DWELL=30

# Queue
HELIOS_QUEUE_MAX_SIZE=100
HELIOS_BACKPRESSURE_THRESHOLD=0.75
```

---

## API Reference

### Python API

```python
from heliosHarness.scaling import (
    DynamicLimitController,
    LimitGateConfig,
    ResourceSampler,
    compute_dynamic_limit,
    BackpressureQueue,
)

# Create config
config = LimitGateConfig(
    min_buffer=0.05,
    discretionary_buffer=0.15,
    sampling_interval_seconds=2.0,
)

# Create controller
controller = DynamicLimitController(
    config=config,
    initial_limit=10,
)

# Get current limit
current_limit = controller.current_limit

# Create queue with backpressure
queue = BackpressureQueue(
    max_size=100,
    backpressure_threshold=0.75,
)
```

### CLI Commands

```bash
# View current scaling info
harness scaling info

# View queue status
harness queue status

# Set manual limit override
harness scaling set-limit 15

# Enable/disable dynamic scaling
harness scaling enable
harness scaling disable
```

---

**Document Version**: 1.0  
**Status**: Ready for Implementation
