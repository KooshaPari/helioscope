from __future__ import annotations

import json
from pathlib import Path

from .interfaces import CanonicalCommand, DiscoverInput, DiscoverOutput, EvidenceBucket, RepoManifest


class Discoverer:
    def discover(self, input: DiscoverInput) -> DiscoverOutput:
        root = Path(input.repo_root).resolve()
        manifest = RepoManifest.from_repo(root, root.name)
        discovered_files = list(self._collect_files(root, input.max_scan_depth))
        scripts = self._extract_scripts(root)
        buckets = self._bucket_commands(scripts)
        return DiscoverOutput(
            manifest=manifest,
            signals=self._extract_signals(root),
            buckets=buckets,
            files=[str(path) for path in discovered_files],
            raw_events=[{"event": "discover.complete", "repo": manifest.repo_id}],
        )

    def _collect_files(self, root: Path, max_depth: int):
        max_parts = root.parts
        for path in root.rglob("*"):
            if path.is_file():
                depth = len(path.relative_to(root).parts)
                if depth <= max_depth or path.name == "AGENTS.md":
                    yield path

    def _extract_signals(self, root: Path) -> list[str]:
        signals: list[str] = []
        interesting = [
            root / "AGENTS.md",
            root / ".github" / "workflows",
            root / "package.json",
            root / "go.mod",
            root / "Cargo.toml",
            root / "pyproject.toml",
            root / "Makefile",
            root / "Justfile",
            root / "Taskfile",
            root / "Taskfile.yaml",
        ]
        for file in interesting:
            if file.exists():
                signals.append(str(file))
        return sorted(signals)

    def _extract_scripts(self, root: Path) -> list[dict]:
        script_hits = []
        package_json = root / "package.json"
        if package_json.exists():
            data = json.loads(package_json.read_text())
            for name, cmd in data.get("scripts", {}).items():
                script_hits.append({"name": name, "command": cmd, "source": "package.json"})
        for fname in ["Makefile", "Justfile", "Taskfile", "Taskfile.yaml", "justfile"]:
            path = root / fname
            if path.exists():
                script_hits.append({"name": fname, "command": f"{fname}:custom", "source": str(path)})
        for fname in [f for f in root.glob("Makefile*") if f.is_file() and f.name.lower() != "makefile"]:
            script_hits.append({"name": fname.name, "command": f"make -f {fname.name} -n", "source": str(fname)})

        if (root / "go.mod").exists():
            script_hits.extend(
                [
                    {
                        "name": "go-check",
                        "command": "go test ./...",
                        "source": "go.mod",
                    },
                    {
                        "name": "go-fmt",
                        "command": "go fmt ./...",
                        "source": "go.mod",
                    },
                    {
                        "name": "go-build",
                        "command": "go build ./...",
                        "source": "go.mod",
                    },
                ]
            )

        if (root / "Cargo.toml").exists():
            script_hits.extend(
                [
                    {
                        "name": "cargo-check",
                        "command": "cargo test --all -- --list",
                        "source": "Cargo.toml",
                    },
                    {
                        "name": "cargo-build",
                        "command": "cargo build --all-targets",
                        "source": "Cargo.toml",
                    },
                    {
                        "name": "cargo-lint",
                        "command": "cargo fmt --check",
                        "source": "Cargo.toml",
                    },
                    {
                        "name": "cargo-clippy",
                        "command": "cargo clippy --all-targets",
                        "source": "Cargo.toml",
                    },
                ]
            )

        if (root / "pyproject.toml").exists() and not (root / ".pytest_cache").exists():
            script_hits.extend(
                [
                    {
                        "name": "python-test",
                        "command": "python -m pytest",
                        "source": "pyproject.toml",
                    },
                    {
                        "name": "python-lint",
                        "command": "python -m ruff check .",
                        "source": "pyproject.toml",
                    },
                    {
                        "name": "python-build",
                        "command": "python -m pytest --co",
                        "source": "pyproject.toml",
                    },
                ]
            )
        return script_hits

    def _bucket_commands(self, scripts: list[dict]) -> dict[EvidenceBucket, list[CanonicalCommand]]:
        buckets: dict[EvidenceBucket, list[CanonicalCommand]] = {b: [] for b in EvidenceBucket}
        for hit in scripts:
            cmd_name = hit["name"]
            cmd = hit["command"]
            source = hit["source"]
            lowered = cmd_name.lower()
            bucket = self._bucket_for_command_name(lowered)
            required = lowered not in {"docs", "fmtcheck", "format:fix", "coverage"}
            buckets[bucket].append(
                CanonicalCommand(
                    command=cmd,
                    bucket=bucket,
                    cwd=".",
                    required=required,
                    rationale=f"script '{cmd_name}' discovered from {source}",
                    source=source,
                )
            )
        return buckets

    def _bucket_for_command_name(self, cmd_name: str) -> EvidenceBucket:
        if any(k in cmd_name for k in ("lint", "fmt", "format", "typecheck", "clippy", "ruff", "eslint", "check")):
            return EvidenceBucket.STATIC
        if any(k in cmd_name for k in ("test", "unit", "integration")):
            return EvidenceBucket.TEST
        if any(k in cmd_name for k in ("build", "package", "pack", "release", "compile")):
            return EvidenceBucket.BUILD
        if "quality" in cmd_name or "strict" in cmd_name:
            return EvidenceBucket.QUALITY
        if "api" in cmd_name or "openapi" in cmd_name or "schema" in cmd_name:
            return EvidenceBucket.API
        return EvidenceBucket.RUNTIME
