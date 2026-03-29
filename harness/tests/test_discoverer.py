import sys

sys.path.insert(0, "src")

from pathlib import Path

from harness.discoverer import Discoverer
from harness.interfaces import DiscoverInput


def test_discover_minimal():
    root = Path(__file__).resolve().parents[1]
    out = Discoverer().discover(DiscoverInput(repo_root=str(root)))
    assert out.manifest.repo_id
    assert isinstance(out.files, list)
    assert isinstance(out.signals, list)


def test_discover_with_custom_max_depth():
    """Test discover with custom max scan depth."""
    root = Path(__file__).resolve().parents[1]
    out = Discoverer().discover(DiscoverInput(repo_root=str(root), max_scan_depth=2))
    assert out.manifest.repo_id
    assert isinstance(out.files, list)


def test_discover_output_has_buckets():
    """Test that discover output has buckets."""
    root = Path(__file__).resolve().parents[1]
    out = Discoverer().discover(DiscoverInput(repo_root=str(root)))
    assert isinstance(out.buckets, dict)
    assert isinstance(out.raw_events, list)


def test_discover_signals():
    """Test signal extraction."""
    root = Path(__file__).resolve().parents[1]
    d = Discoverer()

    # Test _extract_signals method
    signals = d._extract_signals(root)
    assert isinstance(signals, list)


def test_discover_collect_files():
    """Test file collection."""
    root = Path(__file__).resolve().parents[1]
    d = Discoverer()

    files = list(d._collect_files(root, 3))
    assert isinstance(files, list)


def test_discover_extract_scripts():
    """Test script extraction."""
    root = Path(__file__).resolve().parents[1]
    d = Discoverer()

    scripts = d._extract_scripts(root)
    assert isinstance(scripts, list)


def test_discover_bucket_commands():
    """Test command bucketing."""
    root = Path(__file__).resolve().parents[1]
    d = Discoverer()

    scripts = [
        {"name": "test", "command": "pytest", "source": "package.json"},
        {"name": "lint", "command": "ruff check", "source": "package.json"},
    ]
    buckets = d._bucket_commands(scripts)
    assert isinstance(buckets, dict)
