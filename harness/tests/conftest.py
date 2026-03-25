import sys
from pathlib import Path

# Ensure harness package resolves from source layout in harness/src.
src_root = Path(__file__).resolve().parents[1] / "src"
if str(src_root) not in sys.path:
    sys.path.insert(0, str(src_root))
