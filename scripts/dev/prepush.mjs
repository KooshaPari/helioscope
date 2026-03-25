#!/usr/bin/env node

import { spawn } from "node:child_process";

const SKIP_ENV = "SKIP_LOCAL_PREPUSH";

function run(command, args) {
  return new Promise((resolve, reject) => {
    const child = spawn(command, args, {
      stdio: "inherit",
      env: process.env,
    });
    child.on("error", reject);
    child.on("exit", (code, signal) => {
      if (signal) {
        reject(new Error(`${command} terminated by signal ${signal}`));
        return;
      }
      resolve(code ?? 1);
    });
  });
}

async function main() {
  if (process.env[SKIP_ENV] === "1") {
    console.log(`[prepush] skipped because ${SKIP_ENV}=1`);
    return;
  }

  const steps = [
    {
      label: "codex smoke",
      command: "just",
      args: ["codex", "--version"],
    },
    {
      label: "bazel codex prepush",
      command: "just",
      args: ["bazel-codex-prepush"],
    },
  ];

  for (const step of steps) {
    console.log(`[prepush] running: ${step.label}`);
    const exitCode = await run(step.command, step.args);
    if (exitCode !== 0) {
      process.exit(exitCode);
    }
  }

  console.log("[prepush] all checks passed");
}

main().catch((error) => {
  console.error(`[prepush] failed: ${error.message}`);
  process.exit(1);
});
