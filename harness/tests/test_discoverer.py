from harness.discoverer import Discoverer
from harness.interfaces import DiscoverInput
from pathlib import Path


def test_discover_minimal():
    root = Path(__file__).resolve().parents[1]
    out = Discoverer().discover(DiscoverInput(repo_root=str(root)))
    assert out.manifest.repo_id
    assert isinstance(out.files, list)
    assert isinstance(out.signals, list)
