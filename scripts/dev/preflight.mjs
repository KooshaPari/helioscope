#!/usr/bin/env node

import fs from "node:fs";
import path from "node:path";

const INSTALL_HINTS = {
  "process-compose": "brew install process-compose",
  node: "mise install node@22 && mise reshim",
  pnpm: "mise install pnpm@10 && mise reshim",
  cargo: "rustup toolchain install 1.93.0",
  watchexec: "brew install watchexec",
  bazel: "brew install bazel",
  ibazel: "brew install bazelbuild/tap/ibazel",
};

const PROFILE_TOOLS = {
  fast: ["process-compose", "node", "pnpm", "cargo", "watchexec"],
  full: ["process-compose", "node", "pnpm", "cargo", "watchexec", "bazel", "ibazel"],
};

function parseProfile(argv) {
  const index = argv.findIndex((arg) => arg === "--profile");
  if (index === -1) {
    return "fast";
  }
  const value = argv[index + 1];
  if (!value) {
    throw new Error("Missing value for --profile");
  }
  return value;
}

function executableExists(name) {
  const pathValue = process.env.PATH ?? "";
  const pathEntries = pathValue.split(path.delimiter).filter(Boolean);
  const windowsExts =
    process.platform === "win32"
      ? (process.env.PATHEXT ?? ".EXE;.CMD;.BAT")
          .split(";")
          .filter(Boolean)
      : [""];

  for (const directory of pathEntries) {
    for (const extension of windowsExts) {
      const candidate = path.join(directory, `${name}${extension}`);
      if (!fs.existsSync(candidate)) {
        continue;
      }
      try {
        fs.accessSync(candidate, fs.constants.X_OK);
        return true;
      } catch {
        // Continue scanning PATH.
      }
    }
  }
  return false;
}

function requiredPaths() {
  return [
    ".process-compose/dev-fast.yaml",
    ".process-compose/dev-full.yaml",
    "sdk/typescript/package.json",
  ];
}

function main() {
  const profile = parseProfile(process.argv.slice(2));
  if (!PROFILE_TOOLS[profile]) {
    const validProfiles = Object.keys(PROFILE_TOOLS).join(", ");
    throw new Error(`Unknown profile '${profile}'. Expected one of: ${validProfiles}`);
  }

  const missingTools = PROFILE_TOOLS[profile].filter(
    (toolName) => !executableExists(toolName),
  );
  const missingPaths = requiredPaths().filter((requiredPath) => !fs.existsSync(requiredPath));

  if (missingTools.length === 0 && missingPaths.length === 0) {
    console.log(`[preflight] profile=${profile}: OK`);
    return;
  }

  if (missingTools.length > 0) {
    console.error("[preflight] missing tools:");
    for (const toolName of missingTools) {
      const hint = INSTALL_HINTS[toolName] ?? "Install and add to PATH";
      console.error(`  - ${toolName}: ${hint}`);
    }
  }

  if (missingPaths.length > 0) {
    console.error("[preflight] missing required files:");
    for (const requiredPath of missingPaths) {
      console.error(`  - ${requiredPath}`);
    }
  }

  process.exit(1);
}

main();
