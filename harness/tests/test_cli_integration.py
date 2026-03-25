import os
import shutil
import subprocess
import json
from pathlib import Path


def test_execute_phase_2_harness_script_smoke(tmp_path: Path) -> None:
    source_root = Path(__file__).resolve().parents[1]
    workspace = tmp_path / "phase2-harness-workspace"
    workspace.mkdir()

    (workspace / "commands").mkdir(parents=True)
    (workspace / "clones").mkdir(parents=True)
    (workspace / "artifacts" / "phase-2").mkdir(parents=True)
    (workspace / "harness").mkdir(parents=True)

    command_src = source_root.parent / "commands" / "execute-phase-2-harness.sh"
    harness_src = source_root.parent / "harness"

    shutil.copy2(command_src, workspace / "commands" / "execute-phase-2-harness.sh")
    shutil.copytree(harness_src, workspace / "harness", dirs_exist_ok=True)

    repo_root = workspace / "clones" / "toyrepo"
    repo_root.mkdir()
    (repo_root / "package.json").write_text(
        '{\n  "scripts": {\n    "quality": "echo quality",\n    "test": "echo test"\n  }\n}\n'
    )

    env = os.environ.copy()
    env["HELIOS_HARNESS_ROOT"] = str(workspace)

    proc = subprocess.run(
        ["bash", "commands/execute-phase-2-harness.sh"],
        cwd=workspace,
        env=env,
        text=True,
        capture_output=True,
    )
    assert proc.returncode == 0

    evidence = json.loads((workspace / "artifacts" / "phase-2" / "evidence-all.json").read_text())
    targets = evidence["targets"]
    assert any(t["repo_name"] == "toyrepo" for t in targets)
    toy_target = next(t for t in targets if t["repo_name"] == "toyrepo")
    assert toy_target["run"]["result_code"] == "PASS"

    matrix_text = (workspace / "artifacts" / "phase-2" / "lane-d" / "integration-matrix.md").read_text()
    assert "toyrepo" in matrix_text
