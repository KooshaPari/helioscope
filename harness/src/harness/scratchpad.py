"""File Buffering System - Scratchpad for large context data.

Provides a session-specific filesystem for buffering large data that would
overflow the context window.
"""

import json
import os
import shutil
import tempfile
import threading
import uuid
from dataclasses import dataclass, field
from pathlib import Path
from typing import Any, Optional


@dataclass
class BufferEntry:
    """A buffered file entry."""
    id: str
    name: str
    path: Path
    size_bytes: int
    description: str
    created_at: float = field(default_factory=os.path.getmtime)
    accessed_at: float = field(default_factory=os.path.getmtime)
    metadata: dict = field(default_factory=dict)


class ScratchpadFileSystem:
    """Session-specific scratchpad filesystem for context buffering.
    
    Usage:
        scratchpad = ScratchpadFileSystem(session_id=\"session-123\")
        
        # Buffer large data
        file_id = await scratchpad.write(\"results.csv\", large_data, \"Query results\")
        
        # Read back later
        data = await scratchpad.read(file_id)
        
        # Get pointer for context
        pointer = scratchpad.get_pointer(file_id)
        # -> \"Use file_id:abc123 - Query results for session-123\"
    """
    
    def __init__(self, session_id: str, base_dir: Optional[Path] = None):
        self.session_id = session_id
        self._base_dir = base_dir or Path(tempfile.gettempdir()) / "helios_scratchpad" / session_id
        self._base_dir.mkdir(parents=True, exist_ok=True)
        
        self._entries: dict[str, BufferEntry] = {}
        self._lock = threading.RLock()
        self._index_file = self._base_dir / "index.json"
        self._load_index()
    
    async def write(
        self,
        name: str,
        content: Any,
        description: str = "",
        metadata: Optional[dict] = None,
    ) -> str:
        """Write content to scratchpad."""
        entry_id = str(uuid.uuid4())[:8]
        
        # Determine file extension
        if isinstance(content, bytes):
            ext = ".bin"
            content_bytes = content
        elif isinstance(content, str):
            if content.strip().startswith('<'):
                ext = ".html"
            elif content.strip().startswith('{') or content.strip().startswith('['):
                ext = ".json"
            else:
                ext = ".txt"
            content_bytes = content.encode('utf-8')
        else:
            # Serialize to JSON
            ext = ".json"
            content_bytes = json.dumps(content, indent=2).encode('utf-8')
        
        # Write file
        file_path = self._base_dir / f"{entry_id}_{name}{ext}"
        file_path.write_bytes(content_bytes)
        
        # Create entry
        entry = BufferEntry(
            id=entry_id,
            name=name,
            path=file_path,
            size_bytes=len(content_bytes),
            description=description,
            metadata=metadata or {},
        )
        
        with self._lock:
            self._entries[entry_id] = entry
            self._save_index()
        
        return entry_id
    
    async def read(self, entry_id: str) -> Optional[bytes]:
        """Read content from scratchpad."""
        with self._lock:
            entry = self._entries.get(entry_id)
            if not entry:
                return None
            
            if not entry.path.exists():
                return None
            
            entry.accessed_at = os.path.getmtime(entry.path)
            return entry.path.read_bytes()
    
    def get_pointer(self, entry_id: str) -> str:
        """Get a pointer string for context."""
        with self._lock:
            entry = self._entries.get(entry_id)
            if not entry:
                return f"file_id:{entry_id} - Not found"
            
            return f"[file_id:{entry_id}] - {entry.name}: {entry.description} ({entry.size_bytes} bytes)"
    
    def list_entries(self) -> list[BufferEntry]:
        """List all buffered entries."""
        return list(self._entries.values())
    
    def get_stats(self) -> dict:
        """Get scratchpad statistics."""
        total_size = sum(e.size_bytes for e in self._entries.values())
        return {
            'session_id': self.session_id,
            'entry_count': len(self._entries),
            'total_size_bytes': total_size,
            'total_size_mb': total_size / 1024 / 1024,
        }
    
    async def delete(self, entry_id: str) -> bool:
        """Delete a buffered entry."""
        with self._lock:
            entry = self._entries.pop(entry_id, None)
            if not entry:
                return False
            
            if entry.path.exists():
                entry.path.unlink()
            
            self._save_index()
            return True
    
    async def clear(self) -> int:
        """Clear all buffered entries."""
        with self._lock:
            count = len(self._entries)
            
            for entry in self._entries.values():
                if entry.path.exists():
                    entry.path.unlink()
            
            self._entries.clear()
            self._save_index()
            return count
    
    def _save_index(self) -> None:
        """Save index to disk."""
        data = {
            'session_id': self.session_id,
            'entries': [
                {
                    'id': e.id,
                    'name': e.name,
                    'path': str(e.path),
                    'size_bytes': e.size_bytes,
                    'description': e.description,
                    'metadata': e.metadata,
                }
                for e in self._entries.values()
            ],
        }
        self._index_file.write_text(json.dumps(data, indent=2))
    
    def _load_index(self) -> None:
        """Load index from disk."""
        if not self._index_file.exists():
            return
        
        try:
            data = json.loads(self._index_file.read_text())
            self._entries = {
                e['id']: BufferEntry(
                    id=e['id'],
                    name=e['name'],
                    path=Path(e['path']),
                    size_bytes=e['size_bytes'],
                    description=e['description'],
                    metadata=e.get('metadata', {}),
                )
                for e in data.get('entries', [])
            }
        except Exception:
            pass  # Start fresh if load fails


class ContextPointer:
    """Pointer to data in scratchpad or external store."""
    
    def __init__(
        self,
        source: str,  # 'scratchpad', 'memory', 'external'
        reference: str,
        description: str = "",
        size_bytes: int = 0,
    ):
        self.source = source
        self.reference = reference
        self.description = description
        self.size_bytes = size_bytes
    
    def to_context_string(self) -> str:
        """Convert to context-friendly string."""
        return f"[{self.source}:{self.reference}] {self.description}"
    
    def __str__(self) -> str:
        return self.to_context_string()
