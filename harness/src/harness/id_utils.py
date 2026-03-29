"""Identity utilities for stable, collision-resistant IDs.

This module keeps two families of IDs:
- canonical IDs: full UUID strings for durable storage/lookup across sessions
- short IDs: deterministic short aliases derived from canonical IDs
"""

from __future__ import annotations

import hashlib
import uuid


def new_id() -> str:
    """Generate a canonical UUID string."""

    return str(uuid.uuid4())


def short_alias(canonical_id: str, namespace: str = "id", length: int = 12) -> str:
    """Create a deterministic short alias for a canonical ID.

    Stable across sessions because it is derived from the canonical identifier.
    """

    if length <= len(namespace) + 1:
        raise ValueError("short alias length must be greater than namespace prefix")

    digest = hashlib.blake2b(f"{namespace}:{canonical_id}".encode(), digest_size=16).hexdigest()
    return f"{namespace}_{digest[: length - len(namespace) - 1]}"


def artifact_slug(value: str, length: int = 12) -> str:
    """Deterministic filename-safe artifact slug for command/content identities."""

    digest = hashlib.sha256(value.encode("utf-8")).hexdigest()
    return digest[:length]


def command_id(command: str) -> str:
    """Generate a canonical command id."""

    return new_id()
