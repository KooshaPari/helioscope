# Phase 3: Usage Guide - phenotype-py-kit & Hexagonal Ports

Quick reference for using Phase 3 deliverables in Phenotype Python services.

## Part 1: Using phenotype-py-kit

### Installation

Add to your project's `pyproject.toml`:

```toml
[project]
dependencies = [
    "phenotype-py-kit @ git+https://github.com/Phenotype-org/phenotype-commons#phenotype-py-kit",
]
```

Or install locally for development:

```bash
cd /Users/kooshapari/CodeProjects/Phenotype/repos/template-commons/phenotype-py-kit
pip install -e .
```

### Configuration Pattern

Create your service's settings:

```python
# src/myservice/config.py
from typing import Optional
from phenotype_kit import BaseConfig, get_settings

class MyServiceSettings(BaseConfig):
    """Settings for my service."""

    # Service-specific settings
    api_port: int = 8000
    database_url: str
    redis_url: Optional[str] = None

    # Optional: add validation
    def validate_setup(self) -> None:
        """Custom validation on startup."""
        super().validate_setup()

        if self.environment == "production" and not self.redis_url:
            raise ValueError("Redis URL required in production")

# Use in your application
settings = get_settings(MyServiceSettings)
```

### Logging Setup

In your application entry point:

```python
# src/myservice/main.py
from phenotype_kit import configure_logging, get_logger

# Configure at startup
configure_logging(
    level="DEBUG" if settings.debug else "INFO",
    service_name=settings.service_name or "my-service",
    json_output=settings.environment != "development",
)

logger = get_logger(__name__)
logger.info("service_starting", version="1.0.0")
```

Logs in production are JSON:
```json
{
  "timestamp": "2024-03-01T12:34:56.789Z",
  "level": "INFO",
  "logger": "myservice.main",
  "service": "my-service",
  "message": "service_starting",
  "version": "1.0.0"
}
```

### FastAPI Integration

Create your API with standard configuration:

```python
# src/myservice/api.py
from fastapi import FastAPI, Depends
from phenotype_kit import create_app, get_logger

logger = get_logger(__name__)

# Create pre-configured app
app = create_app(
    title="My Service API",
    version="1.0.0",
    cors_origins=["http://localhost:3000"],
)

@app.get("/api/example")
async def get_example():
    """Example endpoint."""
    logger.info("example_called")
    return {"message": "Hello"}

# Health check included automatically at /health
```

Request IDs are automatically injected:
```
curl http://localhost:8000/health -H "x-request-id: my-request-123"
# Response includes: x-request-id: my-request-123
```

### Testing with Fixtures

In your tests:

```python
# tests/test_example.py
import pytest
from phenotype_kit.testing import tmp_dir, mock_config, async_helper

def test_file_handling(tmp_dir, mock_config):
    """Test that uses temp directory and mock config."""
    # tmp_dir is a pathlib.Path to temporary directory
    test_file = tmp_dir / "test.txt"
    test_file.write_text("Hello")

    # mock_config is BaseConfig(environment="test", debug=True)
    assert mock_config.environment == "test"

@pytest.mark.asyncio
async def test_async_operation(async_helper):
    """Test async operations with helper."""
    ready = False

    async def condition():
        nonlocal ready
        return ready

    # Wait for condition with timeout
    ready = True
    result = await async_helper.wait_for_condition(condition)
    assert result is True
```

### Configuration via Environment

Your settings automatically load from environment:

```bash
# .env file
MYSERVICE_DATABASE_URL=postgresql://localhost/mydb
MYSERVICE_REDIS_URL=redis://localhost:6379
MYSERVICE_DEBUG=true
MYSERVICE_ENVIRONMENT=development

# Or as environment variables
export MYSERVICE_DATABASE_URL=postgresql://localhost/mydb
export MYSERVICE_DEBUG=true
```

Then in code:
```python
settings = get_settings(MyServiceSettings)
print(settings.database_url)  # postgresql://localhost/mydb
print(settings.debug)          # True
```

---

## Part 2: Using Hexagonal Ports in Harbor

### Understanding the Ports

The four ports define architectural boundaries:

```
                      ┌─ Benchmark
                      │
    Ports (Protocols) ├─ API
                      │
                      └─ Test Mocks
                          ↓
    Harbor Core (Job, Trial, Metrics)
```

### Creating a Benchmark Adapter

Implement the ports for benchmark execution:

```python
# src/helios_bench/adapters/trial_executor.py
from typing import Protocol
from harbor.models.trial.config import TrialConfig
from harbor.models.trial.result import TrialResult
from harbor.ports import TrialExecutor

class BenchmarkTrialExecutor:
    """Benchmark implementation of TrialExecutor port."""

    def __init__(self, resource_monitor=None):
        self.resource_monitor = resource_monitor

    def __call__(self, config: TrialConfig) -> TrialResult:
        """Execute trial with resource monitoring."""
        # Start monitoring
        if self.resource_monitor:
            self.resource_monitor.start()

        try:
            # Execute trial
            result = self._run_trial(config)
            return result
        finally:
            # Stop monitoring
            if self.resource_monitor:
                self.resource_monitor.stop()
```

### Creating a Metrics Adapter

Implement metrics collection:

```python
# src/helios_bench/adapters/metrics.py
from harbor.ports import MetricsCollector
from harbor.models.trial.result import TrialResult

class BenchmarkMetricsCollector:
    """Benchmark resource metrics collector."""

    def collect(self, results: list[TrialResult]) -> dict[str, float]:
        """Collect resource metrics from trial results."""
        return {
            "mean_runtime_seconds": self._mean_runtime(results),
            "max_memory_mb": self._max_memory(results),
            "mean_cpu_percent": self._mean_cpu(results),
        }

    def name(self) -> str:
        return "benchmark_metrics"
```

### Creating an Orchestrator Adapter

Implement job coordination:

```python
# src/helios_bench/adapters/orchestrator.py
from harbor.ports import JobOrchestrator, TrialExecutor
from harbor.models.trial.config import TrialConfig
from harbor.models.trial.result import TrialResult

class BenchmarkOrchestrator:
    """Benchmark orchestrator - sequential execution."""

    async def run(
        self,
        trial_configs: list[TrialConfig],
        executor: TrialExecutor,
    ) -> list[TrialResult]:
        """Run trials sequentially (simpler than parallel)."""
        results = []
        for config in trial_configs:
            result = executor(config)
            results.append(result)
        return results

    def cancel(self) -> None:
        """Cancel (no-op for sequential)."""
        pass
```

### Creating a Reporter Adapter

Implement result reporting:

```python
# src/helios_bench/adapters/reporter.py
from harbor.ports import JobReporter
from harbor.models.trial.result import TrialResult

class BenchmarkJobReporter:
    """Benchmark reporter - console output."""

    def report_job_start(self, metadata: dict) -> None:
        print(f"Starting benchmark: {metadata['name']}")

    def report_trial(self, result: TrialResult) -> None:
        print(f"  Trial {result.trial_id}: {result.elapsed_seconds}s")

    def report_job_complete(self, job_summary: dict) -> None:
        print(f"Benchmark complete. Mean: {job_summary['mean_time']}s")
```

### Using Adapters with Harbor

```python
# In your benchmark main function
from harbor import Job
from helios_bench.adapters import (
    BenchmarkTrialExecutor,
    BenchmarkOrchestrator,
    BenchmarkJobReporter,
)

# Create job with standard Harbor API
job = Job(config=job_config)

# Inject benchmark implementations
job._executor = BenchmarkTrialExecutor()
job._orchestrator = BenchmarkOrchestrator()
job._reporter = BenchmarkJobReporter()

# Run - uses benchmark adapters transparently
results = await job.run()
```

### Benefits of This Pattern

1. **Isolation**: Benchmark code doesn't depend on Harbor internals
2. **Testing**: Can mock ports with simple implementations
3. **Flexibility**: Swap implementations without changing core code
4. **Clarity**: Port names document architectural intent

---

## Common Patterns

### Pattern 1: Dependency Injection

```python
# Define port dependency
def create_job_service(
    orchestrator: JobOrchestrator,
    reporter: JobReporter,
) -> JobService:
    """Create job service with injected adapters."""
    return JobService(orchestrator, reporter)

# Inject in main
benchmark_orchestrator = BenchmarkOrchestrator()
benchmark_reporter = BenchmarkJobReporter()
service = create_job_service(benchmark_orchestrator, benchmark_reporter)
```

### Pattern 2: Adapter Registry

```python
# Registry of adapter implementations
adapters = {
    "benchmark": {
        "executor": BenchmarkTrialExecutor,
        "orchestrator": BenchmarkOrchestrator,
    },
    "api": {
        "executor": APITrialExecutor,
        "orchestrator": APIOrchestrator,
    },
}

# Select adapter set based on environment
adapter_set = adapters[environment]
executor = adapter_set["executor"]()
```

### Pattern 3: Decorator for Cross-Cutting Concerns

```python
# Logging decorator for any TrialExecutor
def logging_executor(executor: TrialExecutor) -> TrialExecutor:
    def wrapper(config: TrialConfig) -> TrialResult:
        logger.info("executing_trial", trial_id=config.trial_id)
        result = executor(config)
        logger.info("trial_complete", trial_id=config.trial_id)
        return result
    return wrapper

# Use with any executor
logged_executor = logging_executor(BenchmarkTrialExecutor())
```

---

## Testing with Ports

### Mock Implementations

```python
# tests/mocks.py
from harbor.ports import TrialExecutor, JobOrchestrator
from harbor.models.trial.result import TrialResult

class MockTrialExecutor:
    """Deterministic executor for testing."""

    def __init__(self, results=None):
        self.results = results or []
        self.call_count = 0

    def __call__(self, config) -> TrialResult:
        result = self.results[self.call_count]
        self.call_count += 1
        return result

class MockOrchestrator:
    """Simple orchestrator for testing."""

    async def run(self, configs, executor):
        return [executor(config) for config in configs]
```

### Unit Test Example

```python
# tests/test_benchmark.py
import pytest
from mocks import MockTrialExecutor
from helios_bench import BenchmarkRunner

@pytest.mark.asyncio
async def test_benchmark_runner():
    """Test benchmark with mock executor."""
    mock_executor = MockTrialExecutor(results=[...])
    runner = BenchmarkRunner(executor=mock_executor)

    results = await runner.run(configs)
    assert len(results) == len(configs)
```

---

## Troubleshooting

### Configuration not loading from env

Check the env var prefix matches your settings class:
```python
class MySettings(BaseConfig):
    model_config = SettingsConfigDict(env_prefix="MYSERVICE_")

    # Then use MYSERVICE_* environment variables
```

### Port type hints not working

Ensure you're using Protocol from typing:
```python
from typing import Protocol

class MyPort(Protocol):
    def method(self) -> None: ...
```

### Logging not JSON formatted

Ensure `json_output=True` in configure_logging:
```python
configure_logging(
    level="INFO",
    service_name="my-service",
    json_output=True,  # Enable JSON
)
```

---

## Next Steps

1. **Integrate phenotype-py-kit** into existing Python services
2. **Create benchmark adapters** for helios_bench
3. **Implement API adapters** for Harbor HTTP API
4. **Add integration tests** verifying port contracts
5. **Document adapter patterns** for future contributors

For more information, see:
- `/Users/kooshapari/CodeProjects/Phenotype/repos/template-commons/phenotype-py-kit/README.md`
- `/Users/kooshapari/CodeProjects/Phenotype/repos/PHASE_3_SUMMARY.md`
