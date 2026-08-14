# Getting started

This guide covers installation and first use. The README stays focused on what
day is and the shortest useful workflow.

## Install a tested pair

All current day releases are prereleases, so Cargo needs explicit versions.
Use the pair measured by this repository:

```bash
cargo install kan --version 0.13.0-beta.1
cargo install day --version 0.12.1-beta.3
```

Do not omit the versions. A plain `cargo install day` will not select a
prerelease. A plain `cargo install kan` can select kan's old stable release,
which installs successfully but is below day's supported range.

The Cargo commands are the same in macOS, Linux, WSL, PowerShell, and Windows
Command Prompt. If `cargo --version` fails, install Rust from
[rustup.rs](https://rustup.rs/) first: use the shell installer on macOS, Linux,
or WSL, and `rustup-init.exe` on native Windows. Do not switch a native Windows
installation into WSL merely to install day. Matching prebuilt day and kan
artifacts may be used when a release provides both; otherwise Cargo is required.

Verify both binaries before integrating a harness:

```bash
kan --version
day --version
day doctor
```

`day doctor` prints the measured kan range and checks the project's declared
process graph. A newer-kan message is advisory: it means that exact pairing
has not shipped in day yet, not that kan is known to be incompatible.

## Start in a repository

```bash
day init
day config
day doctor
```

`day init` appends the baseline schema through kan and prints integration
instructions. It does not edit project or user configuration. `day config`
shows effective configuration and provenance; it is the first diagnostic when
the project behaves differently from the shipped defaults.

## Claude Code plugin

From Claude Code:

```text
/plugin install <path to this repository>
```

The plugin provides the design, review, handoff, wakeup, and witness-interview
skills, plus advisory hooks and the MCP server. The plugin wiring is inert
configuration; kan remains the durable memory layer.

A practical first cycle is:

1. `/design`
2. implement the checked design
3. `/adversarial-review`
4. `/handoff` before leaving the session
5. `/wakeup` when returning

## Worktrees

day treats linked worktrees as views of one repository while leaving kan's
storage ownership with kan. If a worktree cannot see the main checkout's kan
record, day reports the mismatch instead of claiming that the project has no
teloi or atoms. Run `day doctor` from both the main checkout and the worktree
when diagnosing a split.

## Common diagnostics

- **A command is missing:** rerun the two pinned `cargo install` commands.
- **kan is newer than measured:** normally safe and advisory; check the
  compatibility table if behavior is surprising.
- **Configuration is unexpected:** run `day config` or `day config --json`.
- **The process graph does not compose:** run `day doctor`, then inspect the
  named atoms with `day next <atom>`.
- **A hook is quiet:** run the underlying `day hook ...` command directly and
  include its output in a field report.

The measured compatibility record is
[`tests/fixtures/kan-compat.tsv`](../tests/fixtures/kan-compat.tsv). Record
formats and configuration declarations are documented in
[CONVENTIONS](CONVENTIONS.md).
