#!/usr/bin/env node
// Unified entry point for the Codex CLI.

import { spawn } from "node:child_process";
import { existsSync, readFileSync } from "fs";
import { createRequire } from "node:module";
import path from "path";
import { fileURLToPath } from "url";

// __dirname equivalent in ESM
const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const require = createRequire(import.meta.url);
const argv = process.argv.slice(2);
const previewOnly = argv.includes("--preview-loader");
const STARTUP_DELAY_MS = 5000;
const SPINNER_FRAMES = ["|", "/", "-", "\\"];
const LOADER_FRAME_MS = 80;
const MORPH_DENSE = "@#%*+=-:. ";
const MORPH_LIGHT = " .:-=+*#%@";
const HASH_PULSE = "#*+=-";
const ANSI = {
  reset: "\x1b[0m",
  bgMain: "\x1b[48;2;31;32;34m", // #1f2022
  bgSecondary: "\x1b[48;2;53;58;64m", // #353a40
  textPrimary: "\x1b[38;2;246;245;245m", // #f6f5f5
  accent: "\x1b[38;2;126;186;181m", // #7ebab5
};
const FINAL_WORDMARK_LINES = [
  "HH   HH  EEEEEEEE  LL       IIIIIIII   OOOOOO    SSSSSS",
  "HH   HH  EE        LL          II     OO    OO  SS    SS",
  "HH   HH  EE        LL          II     OO    OO  SS",
  "HHHHHHH  EEEEEE    LL          II     OO    OO   SSSSS",
  "HH   HH  EE        LL          II     OO    OO       SS",
  "HH   HH  EE        LL          II     OO    OO  SS    SS",
  "HH   HH  EEEEEEEE  LLLLLLLL  IIIIIIII   OOOOOO    SSSSSS",
];

const PLATFORM_PACKAGE_BY_TARGET = {
  "x86_64-unknown-linux-musl": "@openai/codex-linux-x64",
  "aarch64-unknown-linux-musl": "@openai/codex-linux-arm64",
  "x86_64-apple-darwin": "@openai/codex-darwin-x64",
  "aarch64-apple-darwin": "@openai/codex-darwin-arm64",
  "x86_64-pc-windows-msvc": "@openai/codex-win32-x64",
  "aarch64-pc-windows-msvc": "@openai/codex-win32-arm64",
};

const { platform, arch } = process;

let targetTriple = null;
switch (platform) {
  case "linux":
  case "android":
    switch (arch) {
      case "x64":
        targetTriple = "x86_64-unknown-linux-musl";
        break;
      case "arm64":
        targetTriple = "aarch64-unknown-linux-musl";
        break;
      default:
        break;
    }
    break;
  case "darwin":
    switch (arch) {
      case "x64":
        targetTriple = "x86_64-apple-darwin";
        break;
      case "arm64":
        targetTriple = "aarch64-apple-darwin";
        break;
      default:
        break;
    }
    break;
  case "win32":
    switch (arch) {
      case "x64":
        targetTriple = "x86_64-pc-windows-msvc";
        break;
      case "arm64":
        targetTriple = "aarch64-pc-windows-msvc";
        break;
      default:
        break;
    }
    break;
  default:
    break;
}

if (!targetTriple) {
  throw new Error(`Unsupported platform: ${platform} (${arch})`);
}

const platformPackage = PLATFORM_PACKAGE_BY_TARGET[targetTriple];
if (!platformPackage) {
  throw new Error(`Unsupported target triple: ${targetTriple}`);
}

if (previewOnly) {
  await showAnimatedLoader({ waitForEnterToClose: true });
  process.exit(0);
}

const codexBinaryName = process.platform === "win32" ? "codex.exe" : "codex";
const localVendorRoot = path.join(__dirname, "..", "vendor");
const localBinaryPath = path.join(
  localVendorRoot,
  targetTriple,
  "codex",
  codexBinaryName,
);

let vendorRoot;
try {
  const packageJsonPath = require.resolve(`${platformPackage}/package.json`);
  vendorRoot = path.join(path.dirname(packageJsonPath), "vendor");
} catch {
  if (existsSync(localBinaryPath)) {
    vendorRoot = localVendorRoot;
  } else {
    const packageManager = detectPackageManager();
    const updateCommand =
      packageManager === "bun"
        ? "bun install -g @openai/codex@latest"
        : "npm install -g @openai/codex@latest";
    throw new Error(
      `Missing optional dependency ${platformPackage}. Reinstall Codex: ${updateCommand}. ` +
        "To preview loader only: node codex-cli/bin/codex.js --preview-loader",
    );
  }
}

if (!vendorRoot) {
  const packageManager = detectPackageManager();
  const updateCommand =
    packageManager === "bun"
      ? "bun install -g @openai/codex@latest"
      : "npm install -g @openai/codex@latest";
  throw new Error(
    `Missing optional dependency ${platformPackage}. Reinstall Codex: ${updateCommand}. ` +
      "To preview loader only: node codex-cli/bin/codex.js --preview-loader",
  );
}

const archRoot = path.join(vendorRoot, targetTriple);
const binaryPath = path.join(archRoot, "codex", codexBinaryName);

// Use an asynchronous spawn instead of spawnSync so that Node is able to
// respond to signals (e.g. Ctrl-C / SIGINT) while the native binary is
// executing. This allows us to forward those signals to the child process
// and guarantees that when either the child terminates or the parent
// receives a fatal signal, both processes exit in a predictable manner.

function getUpdatedPath(newDirs) {
  const pathSep = process.platform === "win32" ? ";" : ":";
  const existingPath = process.env.PATH || "";
  const updatedPath = [
    ...newDirs,
    ...existingPath.split(pathSep).filter(Boolean),
  ].join(pathSep);
  return updatedPath;
}

/**
 * Use heuristics to detect the package manager that was used to install Codex
 * in order to give the user a hint about how to update it.
 */
function detectPackageManager() {
  const userAgent = process.env.npm_config_user_agent || "";
  if (/\bbun\//.test(userAgent)) {
    return "bun";
  }

  const execPath = process.env.npm_execpath || "";
  if (execPath.includes("bun")) {
    return "bun";
  }

  if (
    __dirname.includes(".bun/install/global") ||
    __dirname.includes(".bun\\install\\global")
  ) {
    return "bun";
  }

  return userAgent ? "npm" : null;
}

const additionalDirs = [];
const pathDir = path.join(archRoot, "path");
if (existsSync(pathDir)) {
  additionalDirs.push(pathDir);
}
const updatedPath = getUpdatedPath(additionalDirs);

const env = { ...process.env, PATH: updatedPath };
const packageManagerEnvVar =
  detectPackageManager() === "bun"
    ? "CODEX_MANAGED_BY_BUN"
    : "CODEX_MANAGED_BY_NPM";
env[packageManagerEnvVar] = "1";

function shouldShowAnimatedLoader(argv) {
  if (!process.stdout.isTTY || !process.stdin.isTTY) {
    return false;
  }

  const headlessLikeArg = argv.some(
    (arg) =>
      arg === "exec" ||
      arg === "--headless" ||
      arg === "--json" ||
      arg === "--quiet" ||
      arg === "--output-last-message",
  );

  return !headlessLikeArg;
}

function loadAsciiArtLines() {
  const asciiPath = path.join(__dirname, "..", "..", "ascii.md");
  if (!existsSync(asciiPath)) {
    return [];
  }

  const raw = readFileSync(asciiPath, "utf8");
  const lines = raw
    .split(/\r?\n/)
    .filter((line) => line.trim().length > 0)
    .filter((line) => !line.startsWith("The ASCII Art generated"))
    .map((line) => line.replace(/\s+$/g, ""))
    .slice(0, 56);

  if (lines.length === 0) {
    return lines;
  }

  // Normalize source indentation so centering uses the true glyph bounds.
  const commonIndent = lines.reduce((min, line) => {
    const match = line.match(/^ */);
    const indent = match ? match[0].length : 0;
    return Math.min(min, indent);
  }, Number.POSITIVE_INFINITY);

  const normalized = lines.map((line) => line.slice(commonIndent)).map((line) => line.slice(0, 120));
  return normalized;
}

function sleep(ms) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

function morphAsciiLine(line, frame, rowIndex, highlightRow) {
  const chars = line.split("");
  const phase = frame + rowIndex * 3;
  const inBand = Math.abs(rowIndex - highlightRow) <= 1;

  return chars
    .map((char, colIndex) => {
      if (char === " ") {
        return char;
      }

      // Make dense `#` portraits visibly animate: pulse many hash cells.
      if (char === "#" && (colIndex + phase) % (inBand ? 2 : 3) === 0) {
        return HASH_PULSE[(Math.floor((colIndex + phase) / (inBand ? 1 : 2))) % HASH_PULSE.length];
      }

      if ((colIndex + phase) % (inBand ? 3 : 6) !== 0) {
        return char;
      }

      const ramp = inBand ? MORPH_DENSE : MORPH_LIGHT;
      const normalized = MORPH_DENSE.includes(char)
        ? char
        : MORPH_LIGHT.includes(char)
          ? char
          : "@";
      const currentIndex = ramp.indexOf(normalized);
      const nextIndex = (currentIndex + 1 + (phase % 2)) % ramp.length;
      return ramp[nextIndex];
    })
    .join("");
}

function buildWordmarkCanvas(height, width) {
  const canvas = Array.from({ length: height }, () => " ".repeat(width).split(""));
  const startRow = Math.max(0, Math.floor((height - FINAL_WORDMARK_LINES.length) / 2));

  for (let row = 0; row < FINAL_WORDMARK_LINES.length; row += 1) {
    const line = FINAL_WORDMARK_LINES[row];
    const startCol = Math.max(0, Math.floor((width - line.length) / 2));
    for (let col = 0; col < line.length && startCol + col < width; col += 1) {
      canvas[startRow + row][startCol + col] = line[col];
    }
  }

  return canvas.map((row) => row.join(""));
}

function mergeLine(sourceLine, targetLine, rowIndex, totalRows, frame, mergeProgress) {
  const heightHint = 56;
  const width = Math.max(sourceLine.length, targetLine.length);
  const source = sourceLine.padEnd(width, " ");
  const target = targetLine.padEnd(width, " ");
  const centerRow = Math.floor(heightHint / 2);
  const centerCol = Math.floor(width / 2);

  return source
    .split("")
    .map((char, colIndex) => {
      const targetChar = target[colIndex];
      if (mergeProgress <= 0) {
        return char;
      }

      const rowNorm = totalRows <= 1 ? 0 : rowIndex / (totalRows - 1);
      const colNorm = width <= 1 ? 0 : colIndex / (width - 1);
      const isTopHalf = rowNorm <= 0.5;

      // Stage 1: each half collapses into a dense rotating blob/polygon.
      const collapseProgress = Math.min(1, mergeProgress / 0.35);
      if (collapseProgress < 1) {
        const topCx = centerCol - width * 0.12;
        const topCy = centerRow - heightHint * 0.14;
        const botCx = centerCol + width * 0.12;
        const botCy = centerRow + heightHint * 0.14;
        const blobCx = isTopHalf ? topCx : botCx;
        const blobCy = isTopHalf ? topCy : botCy;
        const rx = Math.max(2, width * (0.36 - collapseProgress * 0.19));
        const ry = Math.max(2, heightHint * (0.26 - collapseProgress * 0.13));
        const nx = (colIndex - blobCx) / rx;
        const ny = (rowIndex - blobCy) / ry;
        const angle = Math.atan2(ny, nx);
        const radial = Math.sqrt(nx * nx + ny * ny);
        const polygonWave = 0.11 * Math.cos(angle * 6 + frame * 0.22);
        const blobEdge = 1 - collapseProgress * 0.28 + polygonWave;

        if (radial <= blobEdge && (char !== " " || targetChar !== " ")) {
          const glyphs = isTopHalf ? "@#%*" : "%#*@";
          return glyphs[(Math.floor((frame + colIndex + rowIndex) / 2)) % glyphs.length];
        }

        if (collapseProgress > 0.42) {
          return " ";
        }
        return char;
      }

      const dx = colIndex - centerCol;
      const dy = rowIndex - centerRow;
      const angle = Math.atan2(dy, dx);
      const radius = Math.sqrt(dx * dx + dy * dy);
      const normalizedAngle = (angle + Math.PI) / (2 * Math.PI);
      const radialBias = Math.min(1, radius / Math.max(1, centerCol));
      const phase = ((frame % 120) / 120 + normalizedAngle + radialBias * 0.35) % 1;
      const topSweep = (1 - colNorm) * 0.65 + rowNorm * 0.35; // down-left feel
      const bottomSweep = colNorm * 0.65 + (1 - rowNorm) * 0.35; // up-right feel
      const directional = isTopHalf ? topSweep : bottomSweep;
      const revealProgress = Math.max(0, (mergeProgress - 0.30) / 0.70);
      const threshold = Math.min(1, revealProgress * 1.4 + directional * 0.22);

      if (phase < threshold) {
        return targetChar;
      }

      if (targetChar !== " " && revealProgress > 0.2 && (colIndex + frame + rowIndex) % 5 === 0) {
        return "+";
      }

      return char;
    })
    .join("");
}

function renderCenteredFrame(lines, statusLine, footer = null) {
  const cols = process.stdout.columns || 120;
  const rows = process.stdout.rows || 40;
  const frameLines = [...lines, statusLine];
  const topPad = Math.max(0, Math.floor((rows - frameLines.length) / 2));
  const stripAnsi = (value) => value.replace(/\x1b\[[0-9;]*m/g, "");
  const visibleLen = (value) => stripAnsi(value).length;
  const fitLineToCols = (value) => {
    const plain = stripAnsi(value);
    if (plain.length <= cols) {
      return value;
    }

    const start = Math.floor((plain.length - cols) / 2);
    const cropped = plain.slice(start, start + cols);
    const prefix = (value.match(/^((?:\x1b\[[0-9;]*m)+)/) || [])[1] || "";
    return `${prefix}${cropped}${ANSI.reset}`;
  };
  const visibleTrimmedLen = (value) => stripAnsi(value).replace(/\s+$/g, "").length;

  // Paint the whole emulator viewport with the main background every frame.
  process.stdout.write("\x1b[H\x1b[2J");
  for (let row = 0; row < rows; row += 1) {
    process.stdout.write(`${ANSI.bgMain}${" ".repeat(cols)}${ANSI.reset}`);
    if (row < rows - 1) {
      process.stdout.write("\n");
    }
  }

  // Overlay centered content.
  for (let index = 0; index < frameLines.length; index += 1) {
    const line = fitLineToCols(frameLines[index]);
    const row = topPad + index + 1; // 1-based cursor row
    if (row > rows) {
      break;
    }

    const lineWidth = Math.min(cols, visibleTrimmedLen(line));
    const col = Math.max(1, Math.floor((cols - lineWidth) / 2) + 1);
    process.stdout.write(`\x1b[${row};${col}H${line}`);
  }

  if (!footer) {
    return;
  }

  const left = footer.left || "";
  const right = footer.right || "";
  const leftWidth = visibleLen(left);
  const rightWidth = visibleLen(right);
  const maxGap = Math.max(1, cols - leftWidth - rightWidth);
  const footerText = `${left}${" ".repeat(maxGap)}${right}`;
  process.stdout.write(
    `\x1b[${rows};1H${ANSI.bgSecondary}${footerText.slice(0, cols)}${ANSI.reset}`,
  );
}

function buildLoadingBar(progress, width = 34) {
  const clamped = Math.max(0, Math.min(1, progress));
  const filled = Math.round(width * clamped);
  const empty = Math.max(0, width - filled);
  const bar = `${"=".repeat(Math.max(0, filled - 1))}${filled > 0 ? ">" : ""}${"-".repeat(empty)}`;
  const pct = String(Math.round(clamped * 100)).padStart(3, " ");
  return `[${ANSI.accent}${bar}${ANSI.reset}] ${ANSI.textPrimary}${pct}%${ANSI.reset}`;
}

async function showAnimatedLoader(options = { waitForEnterToClose: false }) {
  const asciiLines = loadAsciiArtLines();
  if (asciiLines.length === 0) {
    await sleep(STARTUP_DELAY_MS);
    return;
  }
  const width = Math.max(
    ...asciiLines.map((line) => line.length),
    ...FINAL_WORDMARK_LINES.map((line) => line.length),
  );
  const sourceCanvas = asciiLines.map((line) => line.padEnd(width, " "));
  const targetCanvas = buildWordmarkCanvas(sourceCanvas.length, width);

  // Use alternate screen + full redraw to prevent line accumulation/wrap artifacts.
  process.stdout.write("\x1b[?1049h\x1b[?25l");
  const startedAt = Date.now();
  let frame = 0;
  try {
    while (Date.now() - startedAt < STARTUP_DELAY_MS) {
      const elapsed = Date.now() - startedAt;
      const highlightRow = frame % asciiLines.length;
      const percent = Math.min(
        99,
        Math.floor((elapsed / STARTUP_DELAY_MS) * 100),
      );
      const progress = elapsed / STARTUP_DELAY_MS;
      const mergeProgress = Math.max(0, (progress - 0.52) / 0.48);
      const spinner = SPINNER_FRAMES[frame % SPINNER_FRAMES.length];
      const art = sourceCanvas
        .map((line, index) =>
          mergeProgress > 0
            ? (() => {
                const transformed = mergeLine(
                  morphAsciiLine(line, frame, index, highlightRow),
                  targetCanvas[index],
                  index,
                  sourceCanvas.length,
                  frame,
                  mergeProgress,
                );
                return `${ANSI.textPrimary}${transformed}${ANSI.reset}`;
              })()
            : index === highlightRow
              ? `${ANSI.accent}${morphAsciiLine(line, frame, index, highlightRow)}${ANSI.reset}`
              : `${ANSI.textPrimary}${morphAsciiLine(line, frame, index, highlightRow)}${ANSI.reset}`,
        )
        .join("\n");
      const status = `${ANSI.accent}${spinner}${ANSI.reset} ${buildLoadingBar(percent / 100)}`;
      renderCenteredFrame(art.split("\n"), status);
      frame += 1;
      await sleep(LOADER_FRAME_MS);
    }

    renderCenteredFrame(
      targetCanvas.map((line, index) => {
        if (index < FINAL_WORDMARK_LINES.length + Math.floor((targetCanvas.length - FINAL_WORDMARK_LINES.length) / 2)
          && index >= Math.floor((targetCanvas.length - FINAL_WORDMARK_LINES.length) / 2)) {
          return `${ANSI.accent}${line}${ANSI.reset}`;
        }
        return `${ANSI.textPrimary}${line}${ANSI.reset}`;
      }),
      `${ANSI.accent}*${ANSI.reset} ${buildLoadingBar(1)}`,
      {
        left: `${ANSI.accent}powered by Phenotype${ANSI.reset}`,
        right: `${ANSI.textPrimary}vX.X.X-X.X.X${ANSI.reset}`,
      },
    );
    if (options.waitForEnterToClose && process.stdin.isTTY) {
      await waitForEnterToClose();
    } else {
      await sleep(350);
    }
  } finally {
    process.stdout.write("\x1b[?25h\x1b[?1049l");
  }
}

function waitForEnterToClose() {
  return new Promise((resolve) => {
    process.stdout.write(`\n${ANSI.accent}Press Enter to close preview...${ANSI.reset}`);
    process.stdin.resume();
    process.stdin.once("data", () => resolve());
  });
}

if (shouldShowAnimatedLoader(argv)) {
  await showAnimatedLoader();
}

const child = spawn(binaryPath, argv, {
  stdio: "inherit",
  env,
});

child.on("error", (err) => {
  // Typically triggered when the binary is missing or not executable.
  // Re-throwing here will terminate the parent with a non-zero exit code
  // while still printing a helpful stack trace.
  // eslint-disable-next-line no-console
  console.error(err);
  process.exit(1);
});

// Forward common termination signals to the child so that it shuts down
// gracefully. In the handler we temporarily disable the default behavior of
// exiting immediately; once the child has been signaled we simply wait for
// its exit event which will in turn terminate the parent (see below).
const forwardSignal = (signal) => {
  if (child.killed) {
    return;
  }
  try {
    child.kill(signal);
  } catch {
    /* ignore */
  }
};

["SIGINT", "SIGTERM", "SIGHUP"].forEach((sig) => {
  process.on(sig, () => forwardSignal(sig));
});

// When the child exits, mirror its termination reason in the parent so that
// shell scripts and other tooling observe the correct exit status.
// Wrap the lifetime of the child process in a Promise so that we can await
// its termination in a structured way. The Promise resolves with an object
// describing how the child exited: either via exit code or due to a signal.
const childResult = await new Promise((resolve) => {
  child.on("exit", (code, signal) => {
    if (signal) {
      resolve({ type: "signal", signal });
    } else {
      resolve({ type: "code", exitCode: code ?? 1 });
    }
  });
});

if (childResult.type === "signal") {
  // Re-emit the same signal so that the parent terminates with the expected
  // semantics (this also sets the correct exit code of 128 + n).
  process.kill(process.pid, childResult.signal);
} else {
  process.exit(childResult.exitCode);
}
