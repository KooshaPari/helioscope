"""Helios Harness - Lazy-loaded for low memory"""


# Lazy imports to reduce startup memory (<10MB target)
def __getattr__(name: str):
    """Lazy load modules on first access"""

    # Core interfaces - always needed
    if name == "Discoverer":
        from . import discoverer

        return discoverer.Discoverer
    if name == "Runner":
        from . import runner

        return runner.Runner
    if name == "RunnerConfig":
        from . import runner

        return runner.RunnerConfig
    if name == "QualityNormalizer":
        from . import normalizer

        return normalizer.QualityNormalizer
    if name == "evidence_payload":
        from . import schema

        return schema.evidence_payload

    # Teammate system - loaded on demand
    if name == "Teammate":
        from . import teammates

        return teammates.Teammate
    if name == "TeammateRegistry":
        from . import teammates

        return teammates.TeammateRegistry
    if name == "DelegationRequest":
        from . import teammates

        return teammates.DelegationRequest
    if name == "DelegationResult":
        from . import teammates

        return teammates.DelegationResult
    if name == "DelegationProtocol":
        from . import teammates

        return teammates.DelegationProtocol
    if name == "CodexExecutor":
        from . import teammates

        return teammates.CodexExecutor
    if name == "Priority":
        from . import teammates

        return teammates.Priority
    if name == "DelegationStatus":
        from . import teammates

        return teammates.DelegationStatus
    if name == "HealthStatus":
        from . import teammates

        return teammates.HealthStatus
    if name == "HealthMonitor":
        from . import teammates

        return teammates.HealthMonitor

    # Dynamic scaling - loaded on demand
    if name == "ScalingConfig":
        from . import scaling

        return scaling.ScalingConfig
    if name == "ResourceSampler":
        from . import scaling

        return scaling.ResourceSampler
    if name == "ResourceSnapshot":
        from . import scaling

        return scaling.ResourceSnapshot
    if name == "DynamicLimitController":
        from . import scaling

        return scaling.DynamicLimitController
    if name == "MemoryPressureHandler":
        from . import scaling

        return scaling.MemoryPressureHandler
    if name == "FDManager":
        from . import scaling

        return scaling.FDManager
    if name == "CircuitBreaker":
        from . import scaling

        return scaling.CircuitBreaker

    # Caching - loaded on demand
    if name == "L1Cache":
        from . import cache

        return cache.L1Cache
    if name == "L2Cache":
        from . import cache

        return cache.L2Cache
    if name == "L1CacheStats":
        from . import cache

        return cache.L1CacheStats
    if name == "RequestCoalescer":
        from . import cache

        return cache.RequestCoalescer
    if name == "CachePreWarmer":
        from . import cache

        return cache.CachePreWarmer
    if name == "SpeculativeExecutor":
        from . import cache

        return cache.SpeculativeExecutor

    raise AttributeError(f"module {__name__!r} has no attribute {name!r}")


__all__ = [
    # Core
    "Discoverer",
    "Runner",
    "RunnerConfig",
    "QualityNormalizer",
    "evidence_payload",
]
