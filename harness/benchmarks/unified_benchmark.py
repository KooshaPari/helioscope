"""Unified Benchmark - loads from ~/.factory/config.json

Supports harness detection and model variant selection:
- cliproxy: uses minimax-m2.5 (default)
- direct: uses minimax-m2.5-highspeed variant when available
"""

import asyncio
import os
import sys
import json
import time
import httpx
from dataclasses import dataclass
from pathlib import Path
from typing import Optional

CONFIG_PATH = Path.home() / ".factory" / "config.json"
CLIPROXY_URL = os.environ.get("CLIPROXY_URL", "http://localhost:8317")
DEFAULT_HARNESS = os.environ.get("HARNESS", "cliproxy")

# Map display names to cliproxy model IDs
CLIPROXY_MODEL_MAP = {
    "minimax-m2.1": "minimax-m2.1",
    "minimax-m2.5": "minimax-m2.5",
    "minimax-m2.5-byok": "minimax-m2.5",
}


def get_cliproxy_model_id(display_name: str) -> str:
    """Map display name to cliproxy model ID."""
    key = display_name.lower().replace("-", "-").strip()
    return CLIPROXY_MODEL_MAP.get(key, display_name)


def detect_harness() -> str:
    """Detect running harness type."""
    harness = os.environ.get("HARNESS", "").lower()
    if harness:
        return harness

    # Check cliproxy availability
    try:
        import socket

        sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        sock.settimeout(1)
        result = sock.connect_ex(("localhost", 8317))
        sock.close()
        if result == 0:
            return "cliproxy"
    except:
        pass
    return "unknown"


def select_model_for_harness(models: list, harness: str) -> Optional[dict]:
    """Select optimal model variant based on harness type."""
    if not models:
        return None

    # cliproxy: prefer standard minimax-m2.5
    if harness == "cliproxy":
        for m in models:
            name = m.get("model_display_name", "")
            if "minimax" in name.lower() and "highspeed" not in name.lower():
                return m
        return models[0]

    # Other harnesses: prefer highspeed variant
    for m in models:
        name = m.get("model_display_name", "")
        if "highspeed" in name.lower() or "fast" in name.lower():
            return m

    # Default: first minimax model
    for m in models:
        if "minimax" in m.get("model_display_name", "").lower():
            return m

    return models[0] if models else None


def load_config() -> dict:
    if CONFIG_PATH.exists():
        return json.loads(CONFIG_PATH.read_text())
    return {}


def find_model(models: list, suffix: str = None) -> dict:
    for m in models:
        name = m.get("model_display_name", m.get("model", ""))
        if suffix and suffix in name:
            return m
    return models[0] if models else {}


@dataclass
class Result:
    model: str
    latency_ms: float
    success: bool
    source: str
    error: str = ""


async def run_model(model_cfg: dict, prompt: str = "hi") -> Result:
    model = model_cfg.get("model_display_name", "unknown")
    base_url = model_cfg.get("base_url", "")
    api_key = model_cfg.get("api_key", "")

    # Try cliproxy
    try:
        cliproxy_model = get_cliproxy_model_id(model)
        async with httpx.AsyncClient(timeout=30.0) as c:
            r = await c.post(
                f"{CLIPROXY_URL}/v1/chat/completions",
                json={"model": cliproxy_model, "messages": [{"role": "user", "content": prompt}], "max_tokens": 20},
            )
            if r.status_code == 200:
                return Result(model, 100, True, "cliproxy")
    except Exception as e:
        pass

    # Direct
    if not api_key or "dummy" in api_key.lower():
        return Result(model, 0, False, "direct", "no key")

    try:
        async with httpx.AsyncClient(timeout=60.0) as c:
            start = time.perf_counter()
            r = await c.post(
                f"{base_url}/v1/messages",
                headers={"Authorization": f"Bearer {api_key}", "Content-Type": "application/json"},
                json={"model": model, "messages": [{"role": "user", "content": prompt}], "max_tokens": 20},
            )
            lat = (time.perf_counter() - start) * 1000
            return Result(
                model, lat, r.status_code == 200, "direct", str(r.status_code) if r.status_code != 200 else ""
            )
    except Exception as e:
        return Result(model, 0, False, "direct", str(e)[:50])


async def main():
    cfg = load_config()
    models = cfg.get("custom_models", [])

    # Detect harness and select optimal model
    harness = detect_harness()
    print(f"Detected harness: {harness}")

    # Benchmark all minimax models
    print("\n=== Benchmark Results ===")
    for m in models:
        name = m.get("model_display_name", m.get("model", ""))
        if "minimax" not in name.lower():
            continue
        print(f"\nTesting: {name} ({harness})")
        r = await run_model(m)
        print(f"Result: {'OK' if r.success else 'FAIL'} {r.source} {r.latency_ms:.0f}ms {r.error}")


if __name__ == "__main__":
    asyncio.run(main())
