from __future__ import annotations

from .interfaces import QualityNormalization, RunResult


class QualityNormalizer:
    def __init__(self):
        self._required_tokens = {
            "QUALITY": ["lint", "test", "build", "fmt", "clippy", "ruff", "eslint", "golangci"],
            "STATIC": ["lint", "fmt", "typecheck", "clippy", "ruff", "eslint", "gofmt", "gofumpt"],
            "TEST": ["test", "go test", "cargo test", "pytest", "turbo test", "pnpm test"],
            "BUILD": ["build", "buildx", "compile", "pkg"],
        }

    def normalize(self, run_results: list[RunResult], discovered_commands: list) -> QualityNormalization:
        normalization = QualityNormalization(inferred_profile="strict-full", mapped_buckets={})
        observed = {r.command.lower() for r in run_results}

        mapped_buckets = {"STATIC": [], "TEST": [], "BUILD": [], "API": [], "RUNTIME": [], "QUALITY": []}
        for result in run_results:
            cmd = result.command
            lower_cmd = cmd.lower()
            if any(marker in lower_cmd for marker in self._required_tokens["QUALITY"]):
                mapped_buckets.setdefault("QUALITY", []).append(cmd)
            if "test" in cmd:
                mapped_buckets["TEST"].append(cmd)
            if "build" in cmd:
                mapped_buckets["BUILD"].append(cmd)
            if "lint" in cmd or "fmt" in cmd:
                mapped_buckets["STATIC"].append(cmd)

        normalization.mapped_buckets = mapped_buckets
        normalization.discovered_rules = sorted({"strict-full"})

        for bucket_name, required_markers in self._required_tokens.items():
            if bucket_name == "QUALITY":
                continue
            if not any(marker in c for c in observed for marker in required_markers):
                normalization.blockers.append(
                    {
                        "bucket": bucket_name,
                        "severity": "block",
                        "reason": "missing command marker in executed profile",
                    }
                )
        for bucket in ("STATIC", "TEST", "BUILD"):
            if not mapped_buckets[bucket]:
                normalization.warnings.append(
                    {
                        "bucket": bucket,
                        "severity": "warn",
                        "reason": "partial evidence only",
                    }
                )

        return normalization
