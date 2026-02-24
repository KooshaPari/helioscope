"""Teammate Subagent System - Core Implementation

Implements teammate registry, delegation protocol, and execution for heliosHarness.
"""

from __future__ import annotations

import asyncio
import hashlib
import json
import logging
import os
import subprocess
import tempfile
import uuid
from dataclasses import dataclass, field
from datetime import datetime, timezone
from enum import Enum
from pathlib import Path
from typing import Any, Callable
from uuid import UUID

# Lazy psutil - only load when needed for process monitoring
_psutil = None

def _get_psutil():
    global _psutil
    if _psutil is None:
        import psutil
        _psutil = psutil
    return _psutil

import yaml
from pydantic import BaseModel, ConfigDict

logger = logging.getLogger(__name__)


# =============================================================================
# Enums
# =============================================================================


class Priority(str, Enum):
    CRITICAL = "critical"
    HIGH = "high"
    NORMAL = "normal"
    LOW = "low"


class DelegationStatus(str, Enum):
    PENDING = "pending"
    RUNNING = "running"
    COMPLETED = "completed"
    FAILED = "failed"
    CANCELLED = "cancelled"


class HealthStatus(str, Enum):
    HEALTHY = "healthy"
    SLOW = "slow"
    UNHEALTHY = "unhealthy"
    CRASHED = "crashed"
    UNKNOWN = "unknown"


# =============================================================================
# Data Models
# =============================================================================


class Teammate(BaseModel):
    """Teammate definition."""
    model_config = ConfigDict(use_enum_values=True)

    id: str
    name: str
    role: str
    description: str
    system_prompt: str = ""
    tools: list[str] = []
    max_concurrent: int = 1
    timeout_seconds: int = 300


class DelegationRequest(BaseModel):
    """Delegation request."""
    model_config = ConfigDict(use_enum_values=True)

    teammate_id: str
    task_description: str
    priority: Priority = Priority.NORMAL
    timeout_seconds: int = 300
    context_files: list[str] = []


class DelegationResult(BaseModel):
    """Delegation result."""
    model_config = ConfigDict(use_enum_values=True)

    delegation_id: str
    teammate_id: str
    status: DelegationStatus
    result: str | None = None
    error: str | None = None
    duration_ms: int = 0
    evidence: list[str] = []


# =============================================================================
# Teammate Registry
# =============================================================================


class TeammateRegistry:
    """Registry for teammate discovery and management."""

    def __init__(self, agents_dir: Path | None = None):
        self.agents_dir = agents_dir or Path.cwd() / "agents"
        self._teammates: dict[str, Teammate] = {}

    def discover(self) -> dict[str, Teammate]:
        """Auto-discover teammates from agents/ directory."""
        if not self.agents_dir.exists():
            logger.warning(f"Agents directory not found: {self.agents_dir}")
            return {}

        for agent_file in self.agents_dir.glob("*.md"):
            try:
                teammate = self._load_teammate(agent_file)
                self._teammates[teammate.id] = teammate
            except Exception as e:
                logger.error(f"Failed to load teammate {agent_file}: {e}")

        return self._teammates

    def _load_teammate(self, path: Path) -> Teammate:
        """Load teammate from markdown file with YAML frontmatter."""
        content = path.read_text()

        # Parse YAML frontmatter
        if content.startswith("---"):
            parts = content.split("---", 2)
            if len(parts) >= 3:
                frontmatter = yaml.safe_load(parts[1])
                return Teammate(**frontmatter)

        # Fallback: create from filename
        return Teammate(
            id=path.stem,
            name=path.stem.replace("-", " ").title(),
            role="general",
            description=f"Auto-discovered from {path.name}",
        )

    def get(self, teammate_id: str) -> Teammate | None:
        """Get teammate by ID."""
        return self._teammates.get(teammate_id)

    def list_all(self) -> list[Teammate]:
        """List all teammates."""
        return list(self._teammates.values())

    def list_by_role(self, role: str) -> list[Teammate]:
        """List teammates by role."""
        return [t for t in self._teammates.values() if t.role == role]


# =============================================================================
# Delegation Protocol
# =============================================================================


class DelegationProtocol:
    """Handles delegation execution with handoff protocol."""

    def __init__(self):
        self._delegations: dict[str, DelegationRequest] = {}
        self._results: dict[str, DelegationResult] = {}

    async def delegate(
        self,
        request: DelegationRequest,
        executor: "CodexExecutor",
    ) -> DelegationResult:
        """Execute delegation with timeout and retry."""
        delegation_id = str(uuid.uuid4())[:8]

        # Create delegation record
        self._delegations[delegation_id] = request

        # Track start time
        start_time = datetime.now(timezone.utc)

        try:
            # Execute with timeout
            result = await asyncio.wait_for(
                executor.execute(request), timeout=request.timeout_seconds
            )

            # Store result
            delegation_result = DelegationResult(
                delegation_id=delegation_id,
                teammate_id=request.teammate_id,
                status=DelegationStatus.COMPLETED,
                result=result.get("output", ""),
                duration_ms=int(
                    (datetime.now(timezone.utc) - start_time).total_seconds() * 1000
                ),
                evidence=result.get("evidence", []),
            )

        except asyncio.TimeoutError:
            delegation_result = DelegationResult(
                delegation_id=delegation_id,
                teammate_id=request.teammate_id,
                status=DelegationStatus.FAILED,
                error="Timeout",
                duration_ms=int(
                    (datetime.now(timezone.utc) - start_time).total_seconds() * 1000
                ),
            )

        except Exception as e:
            delegation_result = DelegationResult(
                delegation_id=delegation_id,
                teammate_id=request.teammate_id,
                status=DelegationStatus.FAILED,
                error=str(e),
                duration_ms=int(
                    (datetime.now(timezone.utc) - start_time).total_seconds() * 1000
                ),
            )

        self._results[delegation_id] = delegation_result
        return delegation_result

    def get_status(self, delegation_id: str) -> DelegationResult | None:
        """Get delegation status."""
        return self._results.get(delegation_id)

    def cancel_delegation(self, delegation_id: str) -> bool:
        """Cancel a delegation."""
        if delegation_id in self._delegations:
            # Mark as cancelled
            self._results[delegation_id] = DelegationResult(
                delegation_id=delegation_id,
                teammate_id=self._delegations[delegation_id].teammate_id,
                status=DelegationStatus.CANCELLED,
                error="Cancelled by user",
            )
            return True
        return False


# =============================================================================
# Codex Executor
# =============================================================================


class CodexExecutor:
    """Executes tasks via Codex CLI."""

    def __init__(self, workdir: Path | None = None, profile: str = "default"):
        self.workdir = workdir or Path(tempfile.mkdtemp(prefix="helios_"))
        self.profile = profile

    async def execute(self, request: DelegationRequest) -> dict[str, Any]:
        """Execute delegation via codex exec."""

        # Build command
        cmd = [
            "codex",
            "exec",
            "--profile",
            self.profile,
            "--prompt",
            request.task_description,
        ]

        # Run in isolated workdir
        proc = await asyncio.create_subprocess_exec(
            *cmd,
            cwd=self.workdir,
            stdout=asyncio.subprocess.PIPE,
            stderr=asyncio.subprocess.PIPE,
        )

        stdout, stderr = await proc.communicate()

        return {
            "output": stdout.decode() if stdout else "",
            "error": stderr.decode() if stderr else "",
            "returncode": proc.returncode,
        }


# =============================================================================
# Health Monitor
# =============================================================================


class HealthMonitor:
    """Monitors agent health with resource tracking."""

    def __init__(self):
        self._agents: dict[str, dict] = {}
        self._interval = 10  # seconds

    async def check_health(self, agent_id: str) -> HealthStatus:
        """Check agent health."""
        if agent_id not in self._agents:
            return HealthStatus.UNKNOWN

        agent = self._agents[agent_id]

        try:
            proc = _get_psutil().Process(agent["pid"])

            # Check if running
            if not proc.is_running():
                return HealthStatus.CRASHED

            # Check memory
            mem = proc.memory_info()
            if mem.rss > 10 * 1024 * 1024 * 1024:  # 10GB
                return HealthStatus.UNHEALTHY

            # Check response time
            if agent.get("last_response_time", 0) > 5000:
                return HealthStatus.SLOW

            return HealthStatus.HEALTHY

        except _get_psutil().NoSuchProcess:
            return HealthStatus.CRASHED
        except Exception:
            return HealthStatus.UNHEALTHY

    def register_agent(self, agent_id: str, pid: int):
        """Register agent for monitoring."""
        self._agents[agent_id] = {
            "pid": pid,
            "last_response_time": 0,
            "registered_at": datetime.now(timezone.utc),
        }

    def unregister_agent(self, agent_id: str):
        """Unregister agent."""
        self._agents.pop(agent_id, None)
