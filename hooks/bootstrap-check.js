#!/usr/bin/env node
// Cross-platform bootstrap hook. Claude Code itself requires Node, so this
// avoids registering a POSIX-shell program on Windows.
const { spawnSync } = require("node:child_process");

function available(command) {
  const result = spawnSync(command, ["--version"], { stdio: "ignore", shell: false });
  return !result.error;
}

const forced = new Set((process.env.DAY_BOOTSTRAP_FORCE_MISSING || "").split(","));
const missing = ["day", "kan"].filter((command) => forced.has(command) || !available(command));
if (missing.length) {
  const installs = [];
  if (missing.includes("kan")) installs.push("cargo install kan --version 0.12.0-beta.4");
  if (missing.includes("day")) installs.push("cargo install day --version 0.12.1-beta.3");
  process.stdout.write(JSON.stringify({
    systemMessage: `day bootstrap: missing ${missing.join(", ")}\n${installs.join("\n")}`,
  }) + "\n");
}
// Advisory on every platform: missing tools never block session startup.
process.exit(0);
