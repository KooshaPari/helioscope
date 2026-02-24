"""Session Persistence - Save and resume agent sessions.

Provides session state persistence for long-running agent tasks
that span multiple sessions.
"""

import json
import os
import threading
import time
import uuid
from dataclasses import dataclass, field
from pathlib import Path
from typing import Any, Optional


@dataclass
class SessionState:
    """Complete session state."""
    session_id: str
    goal: str
    created_at: float
    updated_at: float
    completed_at: Optional[float] = None
    status: str = "running"  # running, completed, failed, cancelled
    
    # Progress tracking
    progress: float = 0.0  # 0.0 - 1.0
    milestones: list[dict] = field(default_factory=list)
    
    # Data
    context: list[dict] = field(default_factory=list)
    artifacts: dict = field(default_factory=dict)
    metadata: dict = field(default_factory=dict)
    
    # Checkpoints
    checkpoint_id: Optional[str] = None


@dataclass
class Checkpoint:
    """A checkpoint in the session."""
    id: str
    session_id: str
    timestamp: float
    data: dict


class SessionStore:
    """Persistent storage for session state.
    
    Usage:
        store = SessionStore(base_dir=\"/data/sessions\")
        
        # Create session
        session = store.create(\"Build a todo app\")
        
        # Save progress
        store.update(session.session_id, progress=0.5, context=[...])
        
        # Resume later
        session = store.load(session.session_id)
    """
    
    def __init__(self, base_dir: Optional[Path] = None):
        self._base_dir = base_dir or Path.home() / ".helios" / "sessions"
        self._base_dir.mkdir(parents=True, exist_ok=True)
        self._lock = threading.RLock()
        self._cache: dict[str, SessionState] = {}
    
    def create(self, goal: str, metadata: Optional[dict] = None) -> SessionState:
        """Create a new session."""
        session_id = str(uuid.uuid4())[:12]
        
        now = time.time()
        session = SessionState(
            session_id=session_id,
            goal=goal,
            created_at=now,
            updated_at=now,
            metadata=metadata or {},
        )
        
        with self._lock:
            self._cache[session_id] = session
            self._save(session)
        
        return session
    
    def update(
        self,
        session_id: str,
        progress: Optional[float] = None,
        context: Optional[list] = None,
        artifacts: Optional[dict] = None,
        metadata: Optional[dict] = None,
        milestone: Optional[dict] = None,
    ) -> Optional[SessionState]:
        """Update session state."""
        with self._lock:
            session = self._cache.get(session_id) or self._load(session_id)
            if not session:
                return None
            
            session.updated_at = time.time()
            
            if progress is not None:
                session.progress = progress
            
            if context is not None:
                session.context = context
            
            if artifacts is not None:
                session.artifacts.update(artifacts)
            
            if metadata is not None:
                session.metadata.update(metadata)
            
            if milestone:
                session.milestones.append(milestone)
            
            self._save(session)
            self._cache[session_id] = session
            
            return session
    
    def load(self, session_id: str) -> Optional[SessionState]:
        """Load session state."""
        with self._lock:
            # Check cache first
            if session_id in self._cache:
                return self._cache[session_id]
            
            return self._load(session_id)
    
    def _load(self, session_id: str) -> Optional[SessionState]:
        """Load from disk."""
        session_file = self._base_dir / f"{session_id}.json"
        
        if not session_file.exists():
            return None
        
        try:
            data = json.loads(session_file.read_text())
            session = SessionState(**data)
            self._cache[session_id] = session
            return session
        except Exception:
            return None
    
    def _save(self, session: SessionState) -> None:
        """Save to disk."""
        session_file = self._base_dir / f"{session.session_id}.json"
        session_file.write_text(json.dumps({
            'session_id': session.session_id,
            'goal': session.goal,
            'created_at': session.created_at,
            'updated_at': session.updated_at,
            'completed_at': session.completed_at,
            'status': session.status,
            'progress': session.progress,
            'milestones': session.milestones,
            'context': session.context,
            'artifacts': session.artifacts,
            'metadata': session.metadata,
            'checkpoint_id': session.checkpoint_id,
        }, indent=2))
    
    def complete(self, session_id: str) -> bool:
        """Mark session as completed."""
        with self._lock:
            session = self._cache.get(session_id)
            if not session:
                return False
            
            session.status = "completed"
            session.completed_at = time.time()
            session.progress = 1.0
            self._save(session)
            return True
    
    def fail(self, session_id: str, error: str) -> bool:
        """Mark session as failed."""
        with self._lock:
            session = self._cache.get(session_id)
            if not session:
                return False
            
            session.status = "failed"
            session.metadata['error'] = error
            session.updated_at = time.time()
            self._save(session)
            return True
    
    def delete(self, session_id: str) -> bool:
        """Delete a session."""
        with self._lock:
            session_file = self._base_dir / f"{session_id}.json"
            if session_file.exists():
                session_file.unlink()
            
            self._cache.pop(session_id, None)
            return True
    
    def list_sessions(self, status: Optional[str] = None) -> list[SessionState]:
        """List all sessions."""
        sessions = []
        
        for session_file in self._base_dir.glob("*.json"):
            try:
                data = json.loads(session_file.read_text())
                session = SessionState(**data)
                
                if status is None or session.status == status:
                    sessions.append(session)
            except:
                pass
        
        # Add cached sessions
        for session in self._cache.values():
            if session not in sessions:
                if status is None or session.status == status:
                    sessions.append(session)
        
        # Sort by updated_at
        sessions.sort(key=lambda s: s.updated_at, reverse=True)
        return sessions
    
    def create_checkpoint(self, session_id: str, data: dict) -> Optional[str]:
        """Create a checkpoint."""
        with self._lock:
            session = self._cache.get(session_id) or self._load(session_id)
            if not session:
                return None
            
            checkpoint_id = str(uuid.uuid4())[:8]
            checkpoint_dir = self._base_dir / session_id / "checkpoints"
            checkpoint_dir.mkdir(parents=True, exist_ok=True)
            
            checkpoint_file = checkpoint_dir / f"{checkpoint_id}.json"
            checkpoint_file.write_text(json.dumps(data, indent=2))
            
            session.checkpoint_id = checkpoint_id
            self._save(session)
            
            return checkpoint_id
    
    def load_checkpoint(self, session_id: str, checkpoint_id: str) -> Optional[dict]:
        """Load a checkpoint."""
        checkpoint_file = self._base_dir / session_id / "checkpoints" / f"{checkpoint_id}.json"
        
        if not checkpoint_file.exists():
            return None
        
        try:
            return json.loads(checkpoint_file.read_text())
        except:
            return None
    
    def get_stats(self) -> dict:
        """Get store statistics."""
        sessions = self.list_sessions()
        
        by_status = {}
        for s in sessions:
            by_status[s.status] = by_status.get(s.status, 0) + 1
        
        return {
            'total_sessions': len(sessions),
            'by_status': by_status,
        }


# Global store instance
_session_store: Optional[SessionStore] = None
_store_lock = threading.Lock()


def get_session_store(base_dir: Optional[Path] = None) -> SessionStore:
    """Get the global session store."""
    global _session_store
    
    with _store_lock:
        if _session_store is None:
            _session_store = SessionStore(base_dir)
        return _session_store
