"""Context Compaction Engine - Manage context window efficiently.

Provides context compaction strategies to handle long sessions without
overflowing the context window.
"""

import json
import threading
import time
from dataclasses import dataclass, field
from enum import Enum
from typing import Any, Callable, Optional


class CompactionStrategy(Enum):
    """Context compaction strategies."""

    SUMMARIZE = "summarize"  # Summarize old messages
    TRUNCATE = "truncate"  # Keep first N messages
    PRIORITY = "priority"  # Keep high-priority messages
    SLIDING = "sliding"  # Sliding window of recent


@dataclass
class ContextMessage:
    """A message in the context."""

    role: str
    content: str
    timestamp: float = field(default_factory=time.time)
    priority: int = 5  # 1-10, higher is more important
    token_count: int = 0
    metadata: dict = field(default_factory=dict)


@dataclass
class CompactionConfig:
    """Configuration for context compaction."""

    max_tokens: int = 100000
    avg_token_per_char: float = 4.0  # Approximate tokens per character
    strategy: CompactionStrategy = CompactionStrategy.SUMMARIZE
    preserve_recent: int = 10  # Always keep this many recent
    min_priority: int = 3  # Minimum priority to keep


class ContextCompactor:
    """Manages context window through compaction strategies.

    Usage:
        compactor = ContextCompactor(
            max_tokens=80000,
            strategy=CompactionStrategy.SUMMARIZE
        )

        # Add messages
        compactor.add_message(ContextMessage(role=\"user\", content=\"Hello\"))

        # Get compacted context
        compacted = compactor.get_compacted_context()
    """

    def __init__(self, config: Optional[CompactionConfig] = None):
        self._config = config or CompactionConfig()
        self._messages: list[ContextMessage] = []
        self._lock = threading.RLock()
        self._total_tokens = 0

    def add_message(self, message: ContextMessage) -> None:
        """Add a message to context."""
        # Estimate tokens
        message.token_count = len(message.content) // self._config.avg_token_per_char

        with self._lock:
            self._messages.append(message)
            self._total_tokens += message.token_count

    def add_messages(self, messages: list[ContextMessage]) -> None:
        """Add multiple messages."""
        for msg in messages:
            self.add_message(msg)

    def get_compacted_context(self) -> list[dict]:
        """Get compacted context within token limit."""
        with self._lock:
            if self._total_tokens <= self._config.max_tokens:
                return self._to_dict_list(self._messages)

            return self._apply_strategy()

    def _apply_strategy(self) -> list[dict]:
        """Apply the configured compaction strategy."""
        strategy = self._config.strategy

        if strategy == CompactionStrategy.TRUNCATE:
            return self._strategy_truncate()
        elif strategy == CompactionStrategy.PRIORITY:
            return self._strategy_priority()
        elif strategy == CompactionStrategy.SUMMARIZE:
            return self._strategy_summarize()
        elif strategy == CompactionStrategy.SLIDING:
            return self._strategy_sliding()

        return self._to_dict_list(self._messages[: self._config.preserve_recent])

    def _strategy_truncate(self) -> list[dict]:
        """Keep first N messages until token limit."""
        result = []
        tokens = 0

        for msg in self._messages:
            if tokens + msg.token_count > self._config.max_tokens:
                break
            result.append(msg)
            tokens += msg.token_count

        return self._to_dict_list(result)

    def _strategy_priority(self) -> list[dict]:
        """Keep high-priority messages."""
        # Sort by priority (descending) then timestamp (descending)
        sorted_msgs = sorted(
            self._messages,
            key=lambda m: (m.priority, m.timestamp),
            reverse=True,
        )

        result = []
        tokens = 0

        for msg in sorted_msgs:
            if msg.priority < self._config.min_priority:
                continue
            if tokens + msg.token_count > self._config.max_tokens:
                break
            result.append(msg)
            tokens += msg.token_count

        # Sort back by timestamp
        result.sort(key=lambda m: m.timestamp)
        return self._to_dict_list(result)

    def _strategy_summarize(self) -> list[dict]:
        """Keep recent and add summary of old."""
        # Keep recent messages
        recent = self._messages[-self._config.preserve_recent :]
        recent_tokens = sum(m.token_count for m in recent)

        # Create summary of older messages
        old = self._messages[: -self._config.preserve_recent]

        if old:
            summary_msg = ContextMessage(
                role="system",
                content=f"[Previous {len(old)} messages summarized - {sum(m.token_count for m in old)} tokens]",
                priority=10,
            )
            return self._to_dict_list([summary_msg] + recent)

        return self._to_dict_list(recent)

    def _strategy_sliding(self) -> list[dict]:
        """Sliding window of most recent messages."""
        result = []
        tokens = 0

        # Start from most recent
        for msg in reversed(self._messages):
            if tokens + msg.token_count > self._config.max_tokens:
                break
            result.insert(0, msg)
            tokens += msg.token_count

        return self._to_dict_list(result)

    def _to_dict_list(self, messages: list[ContextMessage]) -> list[dict]:
        """Convert messages to dict list."""
        return [
            {
                "role": m.role,
                "content": m.content,
                "metadata": m.metadata,
            }
            for m in messages
        ]

    def get_stats(self) -> dict:
        """Get context statistics."""
        with self._lock:
            return {
                "message_count": len(self._messages),
                "total_tokens": self._total_tokens,
                "max_tokens": self._config.max_tokens,
                "utilization": self._total_tokens / max(1, self._config.max_tokens),
                "strategy": self._config.strategy.value,
            }

    def clear(self) -> None:
        """Clear all messages."""
        with self._lock:
            self._messages.clear()
            self._total_tokens = 0


class ContextWindowManager:
    """Manages multiple context windows with prioritization.

    Usage:
        manager = ContextWindowManager()

        # Add context from different sources
        manager.add_context(\"system\", system_prompt)
        manager.add_context(\"user\", user_messages)
        manager.add_context(\"tools\", tool_results)

        # Get optimized context
        final_context = manager.get_optimized_context()
    """

    def __init__(self, max_tokens: int = 100000):
        self._max_tokens = max_tokens
        self._contexts: dict[str, list[ContextMessage]] = {
            "system": [],
            "user": [],
            "assistant": [],
            "tools": [],
        }
        self._lock = threading.RLock()
        self._compactor = ContextCompactor(CompactionConfig(max_tokens=max_tokens))

    def add_context(self, source: str, messages: list[dict]) -> None:
        """Add context from a source."""
        with self._lock:
            for msg in messages:
                self._contexts[source].append(
                    ContextMessage(
                        role=msg.get("role", "user"),
                        content=msg.get("content", ""),
                        priority=msg.get("priority", 5),
                        metadata=msg.get("metadata", {}),
                    )
                )

    def get_optimized_context(self) -> list[dict]:
        """Get optimized context across all sources."""
        with self._lock:
            # Combine all contexts
            all_messages = []

            # Priority order: system > tools > assistant > user
            priority_order = ["system", "tools", "assistant", "user"]

            for source in priority_order:
                all_messages.extend(self._contexts[source])

            # Use compactor to get within limit
            self._compactor._messages = all_messages
            self._compactor._total_tokens = sum(m.token_count for m in all_messages)

            return self._compactor.get_compacted_context()

    def clear(self, source: Optional[str] = None) -> None:
        """Clear context."""
        with self._lock:
            if source:
                self._contexts[source].clear()
            else:
                for ctx in self._contexts.values():
                    ctx.clear()
