"""Teammate Registry - Manages subagent/ teammate configurations.

Provides a registry for managing multiple teammates with different capabilities,
specializations, and resource limits.
"""

import json
import threading
import time
from dataclasses import dataclass, field, asdict
from enum import Enum
from pathlib import Path
from typing import Any, Optional


class TeammateType(Enum):
    """Types of teammates."""
    CODER = "coder"
    REVIEWER = "reviewer"
    TESTER = "tester"
    RESEARCHER = "researcher"
    ORCHESTRATOR = "orchestrator"
    GENERAL = "general"


class TeammateState(Enum):
    """Teammate availability state."""
    IDLE = "idle"
    BUSY = "busy"
    OFFLINE = "offline"
    ERROR = "error"


@dataclass
class TeammateConfig:
    """Configuration for a teammate."""
    name: str
    type: TeammateType
    model: str = "default"
    max_tokens: int = 8192
    temperature: float = 0.7
    system_prompt: str = ""
    capabilities: list[str] = field(default_factory=list)
    resource_limit_mb: int = 512
    timeout_seconds: int = 300
    priority: int = 5  # 1-10, higher is more priority


@dataclass
class TeammateStatus:
    """Runtime status of a teammate."""
    name: str
    state: TeammateState = TeammateState.IDLE
    current_task: Optional[str] = None
    tasks_completed: int = 0
    tasks_failed: int = 0
    avg_latency_ms: float = 0.0
    last_used: float = 0.0
    error_message: Optional[str] = None


class TeammateRegistry:
    """Registry for managing teammates.
    
    Usage:
        registry = TeammateRegistry(persist_path=\"teammates.json\")
        
        # Register a teammate
        registry.register(TeammateConfig(
            name=\"coder-1\",
            type=TeammateType.CODER,
            capabilities=[\"python\", \"rust\"]
        ))
        
        # Get available teammate
        teammate = registry.get_available(TeammateType.CODER)
    """
    
    def __init__(self, persist_path: Optional[Path] = None):
        self._persist_path = persist_path
        self._configs: dict[str, TeammateConfig] = {}
        self._status: dict[str, TeammateStatus] = {}
        self._lock = threading.RLock()
        
        # Load existing teammates
        if persist_path and persist_path.exists():
            self._load()
    
    def register(self, config: TeammateConfig) -> None:
        """Register a new teammate."""
        with self._lock:
            self._configs[config.name] = config
            self._status[config.name] = TeammateStatus(
                name=config.name,
                state=TeammateState.IDLE,
            )
            self._persist()
    
    def unregister(self, name: str) -> bool:
        """Unregister a teammate."""
        with self._lock:
            if name in self._configs:
                del self._configs[name]
                del self._status[name]
                self._persist()
                return True
            return False
    
    def get(self, name: str) -> Optional[TeammateConfig]:
        """Get teammate config by name."""
        return self._configs.get(name)
    
    def get_all(self) -> list[TeammateConfig]:
        """Get all teammate configs."""
        return list(self._configs.values())
    
    def get_by_type(self, type: TeammateType) -> list[TeammateConfig]:
        """Get all teammates of a specific type."""
        return [c for c in self._configs.values() if c.type == type]
    
    def get_available(self, type: Optional[TeammateType] = None) -> Optional[TeammateConfig]:
        """Get an available teammate (idle state)."""
        with self._lock:
            candidates = []
            if type:
                candidates = [c for c in self._configs.values() 
                            if c.type == type and 
                            self._status.get(c.name, TeammateStatus(c.name)).state == TeammateState.IDLE]
            else:
                candidates = [c for c in self._configs.values() 
                            if self._status.get(c.name, TeammateStatus(c.name)).state == TeammateState.IDLE]
            
            if not candidates:
                return None
            
            # Return highest priority
            return max(candidates, key=lambda c: c.priority)
    
    def update_status(self, name: str, status: TeammateStatus) -> None:
        """Update teammate status."""
        with self._lock:
            if name in self._status:
                self._status[name] = status
    
    def get_status(self, name: str) -> Optional[TeammateStatus]:
        """Get teammate status."""
        return self._status.get(name)
    
    def mark_busy(self, name: str, task_id: str) -> bool:
        """Mark teammate as busy with a task."""
        with self._lock:
            if name in self._status:
                self._status[name].state = TeammateState.BUSY
                self._status[name].current_task = task_id
                self._status[name].last_used = time.time()
                return True
            return False
    
    def mark_idle(self, name: str) -> bool:
        """Mark teammate as idle."""
        with self._lock:
            if name in self._status:
                self._status[name].state = TeammateState.IDLE
                self._status[name].current_task = None
                return True
            return False
    
    def mark_error(self, name: str, error: str) -> bool:
        """Mark teammate in error state."""
        with self._lock:
            if name in self._status:
                self._status[name].state = TeammateState.ERROR
                self._status[name].error_message = error
                return True
            return False
    
    def get_all_status(self) -> dict[str, TeammateStatus]:
        """Get status of all teammates."""
        return dict(self._status)
    
    def get_stats(self) -> dict[str, Any]:
        """Get registry statistics."""
        with self._lock:
            states = {}
            for status in self._status.values():
                state = status.state.value
                states[state] = states.get(state, 0) + 1
            
            return {
                'total_teammates': len(self._configs),
                'by_state': states,
                'by_type': {
                    t.value: len([c for c in self._configs.values() if c.type == t])
                    for t in TeammateType
                },
            }
    
    def _persist(self) -> None:
        """Persist to disk."""
        if not self._persist_path:
            return
        
        data = {
            'teammates': [asdict(c) for c in self._configs.values()],
        }
        
        # Convert enum to string for JSON
        for teammate in data['teammates']:
            teammate['type'] = teammate['type'].value if isinstance(teammate['type'], TeammateType) else teammate['type']
        
        self._persist_path.parent.mkdir(parents=True, exist_ok=True)
        self._persist_path.write_text(json.dumps(data, indent=2))
    
    def _load(self) -> None:
        """Load from disk."""
        if not self._persist_path or not self._persist_path.exists():
            return
        
        try:
            data = json.loads(self._persist_path.read_text())
            for teammate_data in data.get('teammates', []):
                # Convert type string to enum
                if isinstance(teammate_data.get('type'), str):
                    teammate_data['type'] = TeammateType(teammate_data['type'])
                
                config = TeammateConfig(**teammate_data)
                self._configs[config.name] = config
                self._status[config.name] = TeammateStatus(name=config.name)
        except Exception:
            pass  # Start fresh if load fails
    
    def create_default_teammates(self) -> None:
        """Create default teammate configurations."""
        defaults = [
            TeammateConfig(
                name="coder-1",
                type=TeammateType.CODER,
                capabilities=["python", "rust", "javascript"],
                system_prompt="You are an expert coder.",
                priority=10,
            ),
            TeammateConfig(
                name="reviewer-1",
                type=TeammateType.REVIEWER,
                capabilities=["code-review", "security"],
                system_prompt="You are an expert code reviewer.",
                priority=8,
            ),
            TeammateConfig(
                name="tester-1",
                type=TeammateType.TESTER,
                capabilities=["testing", "pytest"],
                system_prompt="You are an expert tester.",
                priority=7,
            ),
            TeammateConfig(
                name="researcher-1",
                type=TeammateType.RESEARCHER,
                capabilities=["research", "documentation"],
                system_prompt="You are an expert researcher.",
                priority=5,
            ),
        ]
        
        for config in defaults:
            if config.name not in self._configs:
                self.register(config)
