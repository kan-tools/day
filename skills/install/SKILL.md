---
name: install
allowed-tools: Bash(cargo *), Bash(day *), Bash(kan *), Read
description: Install or repair the external day and kan prerequisites with platform-appropriate guidance, then verify the tested pair.
---

# Install day and kan

The plugin ships skills and MCP configuration; it does not bundle the `day` or
`kan` executables. Ask before installing software. Do not infer that Node,
Bash, PowerShell, Cargo, or a package manager exists merely because an agent
harness is running.

## Context

- **Cargo** — `cargo --version`.
  **If this read fails:** report that Cargo is unavailable and offer the
  platform-specific alternatives below; do not treat the failure as permission
  to install a toolchain.
- **day** — `day --version`.
  **If this read fails:** report the exact failure and continue checking kan;
  one missing prerequisite does not establish the state of the other.
- **kan** — `kan --version`.
  **If this read fails:** report the exact failure and continue checking day.
  Do not substitute a version inferred from documentation for an executable
  version you could not read.

## 1. Inspect without changing the machine

Check the operating system and whether `cargo`, `day`, and `kan` are available.
If a read fails, report that failure rather than treating the tool as absent.

## 2. Install the tested pair

When Cargo is available, these commands are identical on macOS, Linux, WSL,
and native Windows terminals:

```text
cargo install kan --version 0.13.0-beta.1
cargo install day --version 0.12.1-beta.3
```

Ask for approval before running them. Do not omit the versions: current day
releases are prereleases, while an unpinned kan install can select an old stable
release outside day's measured range.

When Cargo is unavailable, do not silently install a toolchain:

- macOS, Linux, and WSL: offer the official Rust toolchain installer or a
  prebuilt day/kan release artifact when one exists. State which source and
  version would be installed before asking for approval.
- Native Windows: offer the official Rustup Windows installer from rustup.rs,
  or matching prebuilt Windows release artifacts when they exist. Do not route
  the user through WSL unless they chose a WSL environment.

If no prebuilt artifact exists for the platform, say that Cargo is currently
required. Never substitute a source checkout binary for a released artifact
without saying so.

## 3. Verify

Run all three commands and report any failure:

```text
kan --version
day --version
day doctor
```

`day doctor` may report that kan is newer than the measured range. That is an
advisory compatibility finding, not proof of incompatibility and not a reason
to downgrade without the user's direction.

## Harness boundary

Installation is portable plugin content, not a Claude Code SessionStart hook.
A harness may display its own missing-command diagnostic, but this skill is the
recovery path shared by Agent Skills consumers. Missing prerequisites never
authorize a blocking decision or an automatic download.
