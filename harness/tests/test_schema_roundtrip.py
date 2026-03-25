import json
from pathlib import Path


def test_schema_exists():
    base = Path("harness/schemas")
    if not base.exists():
        base = Path("schemas")
    assert (base / "harness-evidence.schema.json").exists()
    assert (base / "phase2-harness.schema.json").exists()
    json.loads((base / "harness-evidence.schema.json").read_text())
