# Security policy

## Reporting a vulnerability

Please report privately, through GitHub's private vulnerability reporting:

**<https://github.com/kan-tools/day/security/advisories/new>**

(Or: the repository's **Security** tab → **Report a vulnerability**.)

That channel is enabled on this repository. There is deliberately no email
address here — a published address is a spam target and, for a project with
one maintainer, a slower path than the tab.

Please do not open a public issue for anything you believe is exploitable.
For everything else — including the known limitations below — a public issue
is the right place and is more useful.

**What to expect.** day has one maintainer and no SLA. You should get an
acknowledgement within a few days. If a report is valid, the fix and the
disclosure will be coordinated with you, and you will be credited unless you
ask otherwise.

## Supported versions

**Only the most recent release.** Everything day has published is a
prerelease — there is no stable line to backport to, and fixes land on `main`
and go out in the next release rather than being patched into an older one.

day also declares a supported range of `kan` versions, measured rather than
asserted (`tests/fixtures/kan-compat.tsv`); `day doctor` prints that range
next to the kan you have installed.

## What day actually does, which is where to look

day is a local CLI. It has **no network access of its own** and no server
component; `day mcp` speaks MCP over stdio to a local client. It publishes to
crates.io only — there are no prebuilt binaries to sign or verify.

It has exactly **three substrates**, and only one of them executes anything:

1. **kan**, reached by shelling out to the `kan` binary — never by linking it,
   never by touching kan's storage, its signing, or its log format. day only
   ever appends, and never retracts or rejects.
2. **git, read-only.** All git access lives in `src/git.rs`, and there is
   deliberately no method that stages, commits, tags, checks out, or pushes.
   `tests/assess.rs` greps that module for the mutating subcommands
   (`ac1_day_never_invokes_a_mutating_git_subcommand`), which is a guarantee
   rather than a spot check precisely because all git access is behind the one
   module.
3. **Project-declared commands**, in `src/probe.rs`. This is the interesting
   one.

### The command-probe boundary

A `command` probe's argv **comes from a kan claim**, so a claim is untrusted
input that names something to execute. That is the sharp edge, and it is
bounded by four rules, each with a test:

- **No shell, ever.** The argv is split and exec'd directly, so shell
  metacharacters in a claim stay literal.
- **`--run` opt-in, per invocation.** Command probes do not execute unless
  you ask on that specific run.
- **Never reachable over MCP.** An agent talking to `day mcp` cannot trigger
  command execution at all.
- **A timeout that kills.**

Not every probe is a command. The `claim` probe reads the kan log through the
same public read verbs and is bound by none of those four, deliberately —
there is nothing to shell-escape and nothing for `--run` to gate. The line is
**read vs. execute**, and a test asserts the `claim` probe never reaches
`run_command`.

If you have found a way around any of the four rules — a shell reached, a
command run without `--run`, execution triggered over MCP, or a probe that
outlives its timeout — that is a vulnerability and the private channel is the
right place for it.

## Known limitations — please file these publicly, not privately

These are already known and public. Reporting them privately just delays a
real report.

- **day is pre-1.0 and every release is a prerelease.** The conventions are
  v0 and expected to change.
- **A `command` probe cannot express an argument containing a space**
  ([#140](https://github.com/kan-tools/day/issues/140)). This is a
  consequence of splitting argv without a shell — the safe direction — but it
  is a real expressiveness limit and is filed.
- **A positive `command` probe collapses "could not answer" into "found
  nothing"** ([#142](https://github.com/kan-tools/day/issues/142)), and
  several read paths still report a false absence
  ([#160](https://github.com/kan-tools/day/issues/160),
  [#161](https://github.com/kan-tools/day/issues/161),
  [#162](https://github.com/kan-tools/day/issues/162)). These are honesty
  defects in what day reports, not access-control defects, and they are
  tracked in the open.
- **`day init` prints wiring for you to apply; it does not mutate your
  config.** Anything that would change Claude Code settings is opt-in
  ([#109](https://github.com/kan-tools/day/issues/109)). If you find day
  writing config you did not ask it to write, that *is* worth reporting —
  privately if you think it is exploitable.
- **The first kan write after a kan upgrade can hang on the macOS keychain**
  ([#153](https://github.com/kan-tools/day/issues/153)). This is upstream in
  kan; day looks broken rather than blocked. Availability only.
- **Anything day executes, you declared.** day runs what a project's own kan
  claims name, under the four rules above. It is not a sandbox, and a
  project whose claim log an attacker can already write to is a project whose
  build they can already influence.

## Scope

In scope: the command-probe boundary above, anything that lets day write
outside `.day/` or write kan's storage directly, anything that makes an
advisory hook block or deny, and any path that leaks a credential into a
claim, a log, or the status line.

Out of scope: vulnerabilities in `kan` itself (report those to
[kan-tools/kan](https://github.com/kan-tools/kan/security/advisories/new)),
in `git`, or in Claude Code; and anything requiring an attacker who can
already write to your repository or run code as you.
