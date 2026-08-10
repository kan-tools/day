# Feature: Cross-harness packaging — day targets Agent Plugins 1.0.0, and stops pre-executing its own context

## Summary

day is packaged for exactly one harness. Agent Plugins 1.0.0, published
2026-08-06 by OpenAI, AWS, Cursor, GitHub, VS Code and Vercel with Google since
joining as a core maintainer, is a vendor-neutral packaging standard whose two
component types — Agent Skills and MCP servers — are the two things day already
is. This document decides that **day targets it**, and specifies the shape: the
five atoms become portable skills, the hook moves into a reverse-domain
extension directory, and `` !`command` `` context pre-execution is removed
outright rather than ported.

The last of those is the substance. It reads as a portability concession and is
not one: `!` pre-execution is the mechanism behind day#99 and day#100, it sits
in the one channel `telos/honest-reads`' source scan cannot reach, and removing
it is a correctness fix that portability merely makes urgent. `docs/ROADMAP.md`
already schedules non-Claude-Code harnesses for v0.9 as "packaging rather than
redesign"; this is the vehicle, and the redesign is smaller than the roadmap
feared in every respect but this one.

## Requirements

- REQ-1: day ships a conformant `plugin.json` at the repo root, carrying the
  required `$schema` (`https://agent-plugins.org/schemas/1.0.0/plugin.schema.json`)
  and `name`. The manifest schema is **closed** — the only permitted top-level
  fields are `$schema`, `name`, `version`, `description`, `author`, `homepage`,
  `repository`, `license`, `keywords`, and `extensions` — so any Claude
  Code-specific key belongs under `extensions`, never beside them.

- REQ-2: day ships a conformant `mcp.json` at the repo root. **This is not a
  rename of the existing `.mcp.json`.** Agent Plugins requires a `$schema` field
  and permits no other top-level field beside `mcpServers`; day's current
  `.mcp.json` has no `$schema` and would be rejected outright, because §7.2.2
  makes a malformed `mcp.json` disable MCP for the whole plugin. The claim in
  day#157 that "the MCP half is done and already portable" is false and this
  requirement exists to correct it.

- REQ-3: each of day's five atoms is discoverable at `skills/<name>/SKILL.md`,
  with frontmatter `name` matching its parent directory and a `description`.
  `skills/` is the only location Agent Plugins searches, and it does not recurse
  past the immediate child.

- REQ-4: **no `` !`command` `` remains in any atom body.** Every read those
  preambles performed becomes an explicit instruction in the body, executed by
  the agent as a tool call. This is a behavioural change under Claude Code too,
  where `!` does still expand — it is not conditioned on the target harness.

- REQ-5: a read that fails in a converted atom is **reported, never silently
  empty**. This is `telos/honest-reads` applied to the channel its source scan
  (`a_failed_kan_read_is_never_swallowed` in `tests/plugin.rs`) structurally
  cannot see, because command bodies are not `src/`. day#100 is the instance:
  a telos grep matched nothing, `|| echo "none"` fired, and every adversarial
  review in this repo silently measured against `CLAUDE.md` instead of against
  nine live teloi, exit zero throughout.

- REQ-6: the `SessionStart` and `UserPromptSubmit` hooks move under a
  reverse-domain extension directory. Agent Plugins v1 defines exactly two
  component types and hooks are neither, so a client that does not implement
  the namespace ignores the directory rather than failing on it. Mid-session
  context injection is a harness affordance with no portable equivalent, and
  this is the correct place for it rather than a gap to be lobbied about.

- REQ-7: Claude Code behaviour is unchanged by the packaging move. `/design`,
  `/adversarial-review`, `/handoff`, `/wakeup` and `/witness-interview` still
  resolve, and both hooks still fire. Claude Code merged custom commands into
  skills — a file at `commands/design.md` and a skill at `skills/design/SKILL.md`
  both create `/design` — so the conversion is invisible from that side.

- REQ-8: no atom's prose exists in two places. A single body per atom serves both
  harnesses. `CLAUDE.md` records rule-drift-across-two-locations as a defect
  class twice, and ten prose files holding five atoms would be a third instance;
  it would also collide, since Claude Code folds `commands/` and `skills/` into
  one inventory keyed by name.

- REQ-9: day's MCP server entry declares `"command": "day"`, a bare executable
  name resolved by the platform's search rules, and day states plainly that the
  binary must be installed first (`cargo install day`). Agent Plugins defines no
  install step, and a plugin that bundles an executable must use a
  plugin-relative command — which day does not do, because the binary is a Rust
  crate published to crates.io rather than package data.

## Acceptance Criteria

- [x] AC-1: `plugin.json` validates against the published 1.0.0 plugin schema,
      and a test asserts every top-level key it carries is one of the ten the
      closed schema permits. (REQ-1)
- [x] AC-2: `mcp.json` validates against the published 1.0.0 MCP schema, and a
      test asserts it carries `$schema` and that its only other top-level key is
      `mcpServers`. A test also asserts `mcp.json` and `.mcp.json` describe the
      same server, so the two cannot drift. (REQ-2, REQ-9)
- [x] AC-3: a test enumerates `skills/` by reading the directory — never a
      literal list — and asserts each child holds a `SKILL.md` whose frontmatter
      `name` equals the directory name. The count is asserted exactly and
      separately, because a count catches a parser that stopped matching and a
      derived list catches a member never added. (REQ-3)
- [x] AC-4: a source scan over every `SKILL.md` fails the build on any
      occurrence of the `!`-backtick form, with no exemption hatch, and is
      demonstrated to fire by reintroducing one. (REQ-4)
- [x] AC-5: each converted atom's **`## Context` section** names, for every read
      it instructs there, what to do when that read fails — and a test asserts
      the failure-handling clause is present for each, with the bullet count
      asserted exactly. Verified against day#100 specifically: the telos read
      must state that an unreadable log is to be reported, not treated as "no
      teloi". (REQ-5)

      **Narrowed to what is actually asserted.** This read "each converted atom's
      body … for every read it instructs", which is broader than the test: the
      Steps and Phases instruct further commands that nothing checks. It was
      ticked against the broader reading, which is the requirement-that-fails-
      nothing shape `CLAUDE.md` names. Two known limits, stated rather than
      implied: the marker is a token, so `**If this read fails:** ignore it`
      would pass; and only the Context section is scanned. Broadening it is
      follow-up work, not a claim this milestone gets to make.
- [ ] AC-6: `hooks/hooks.json` is reachable only from the reverse-domain
      directory, and `tests/plugin.rs`'s existing assertion that no hook emits a
      blocking decision still runs against it at its new path. (REQ-6)
- [x] AC-7: `claude plugin details` on the converted tree reports five skills,
      two hooks and one MCP server — the same inventory it reports today — and
      the five names are unchanged. (REQ-7)
- [x] AC-8: a test asserts no atom name appears both under `commands/` and under
      `skills/`, so the single-source rule cannot be broken by adding back a file
      rather than by editing one. (REQ-8)

## Architecture

**What moves, and what is added rather than moved.** The Claude Code manifest at
`.claude-plugin/plugin.json` and the Claude Code MCP config at `.mcp.json` stay
where they are and keep their current contents; Agent Plugins' `plugin.json` and
`mcp.json` are new files at the repo root, not renames. Two manifests is the
intended shape — Agent Plugins explicitly "leaves installation, distribution,
policy … to each client" and says nothing about discovery, so it does not replace
the marketplace entry that makes `/plugin install day@kan-tools` work.

The five atom bodies lived one file each under the old `commands/` directory
(design, adversarial-review, handoff, wakeup, witness-interview) and move to
`skills/adversarial-review/SKILL.md`, `skills/design/SKILL.md`,
`skills/handoff/SKILL.md`, `skills/wakeup/SKILL.md` and
`skills/witness-interview/SKILL.md`. Their frontmatter already carries
`description` and `allowed-tools`, and only `name` is added.

**`allowed-tools` does not survive unchanged, and this document said it would.**
The sentence above used to read "`allowed-tools` survives, since the Agent
Skills specification defines it (as experimental) and Claude Code supports it".
Both halves are true and the conclusion does not follow: the specification
defines the field as a *space-separated* string (`Bash(git:*) Read`), and day's
five bodies carry Claude Code's *comma-separated* form (`Bash(kan *), Read`), so
a strict client splitting on spaces gets tokens matching no tool. That is
`CLAUDE.md`'s named failure — a justification about a mechanism whose own
specification says otherwise — caught by reading the spec rather than by
reasoning about it. Resolved as RQ-5.

**The `!` removal is the work.** Each body opened with a `## Context` block whose
lines were `` !`git rev-parse --show-toplevel` ``, `` !`kan status` ``,
`` !`day doctor` ``, and similar. Under REQ-4 these become instructions the agent
executes. The `allowed-tools` grant already covers them —
`skills/design/SKILL.md` declares `Bash(kan *)`, `Bash(day *)`, `Bash(git *)` —
so no permission surface widens, and the agent gets an exit code where the
harness previously got a string.

That difference is the whole argument. `src/probe.rs` states that a subject day
cannot read is an error and never a silently empty result, and `CLAUDE.md`
records five separate violations of that rule in `src/`, each closed by the scan
in `tests/plugin.rs`. The command channel has the same rule and no scan, and it
has failed the same way twice:

- day#99 — `ls CLAUDE.md docs/SPEC.md docs/HANDOFF.md docs/DECISIONS.md` exits 1
  because three of those files have never existed in this repo, and the harness
  treats a non-zero `!` command as a **load failure**. `/adversarial-review`
  could not load at all.
- day#100 — the telos line grepped a shape `kan status` no longer emits, matched
  nothing, and its `|| echo "none"` fired. The review reported no teloi in a repo
  with nine, and measured against `CLAUDE.md` instead. Exit zero, no warning.

Both are the failure mode of pre-execution rather than of any particular command:
the preamble assumes state that may not be there, and has no way to say so. An
agent instructed to run `kan status` and told what an empty result means can
report it; a `!` line cannot. Removing `!` therefore closes a defect class, and
the fact that the Agent Skills specification has no `!` is a consequence worth
noticing rather than the reason.

**The hook.** `hooks/hooks.json` registers `SessionStart` and `UserPromptSubmit`,
and `src/hooks.rs` implements them. Neither is an Agent Plugins component type,
so both move under a reverse-domain directory per §8.2 — the extension directory
for a namespace is the top-level directory named after it. Which namespace
Claude Code claims is not something this document may invent; it is Open
Question Q1. The invariant that survives unchanged is the one
`tests/plugin.rs` enforces against the shipped hook config: no hook may emit a
blocking decision, per `telos/affordance-not-enforcement`.

**Why the binary is not bundled.** `docs/CONVENTIONS.md` and `CLAUDE.md` both
fix day's substrates, and day talks to kan by shelling out to the `kan` binary
rather than linking it. The same reasoning applies one level out: `mcp.json`
names `day` as a bare command and the user installs it from crates.io, where
both crates are published at the versions this repo expects. Bundling
per-platform binaries as package data would make the plugin the distribution
channel for a Rust crate, which is a second release path that can drift from
`scripts/cut-release.sh`.

## Resolved Questions

- RQ-1: Q: Does day target Agent Plugins 1.0.0 at all? **Yes.** `docs/ROADMAP.md` v0.9
  already schedules non-Claude-Code harnesses and calls the CLI core
  harness-agnostic; this is the standard that makes that item concrete rather
  than a new direction.
- RQ-2: Q: Convert the atoms, dual-ship them, or ship the manifests alone? **Convert.**
  Dual-shipping puts five prose bodies in two places, which `CLAUDE.md` records
  as a defect class twice, and would collide on name because Claude Code folds
  `commands/` and `skills/` into one inventory. Manifest-only would leave the
  five atoms — day's actual process contribution — single-vendor while shipping
  only the MCP server.
- RQ-3: Q: Keep `` !`command` `` for Claude Code, where it still works, and degrade
  elsewhere? **No — remove it everywhere.** A portable client does not strip the
  syntax, it renders it literally, so the agent would receive backtick text
  where day promised data: a silent wrong answer, which is day#100's exact shape
  exported to every other harness. Removing it outright is also the correct fix
  on its own terms, since `!` assumes state that may not be present and cannot
  report that it was not.
- RQ-5: Q: `allowed-tools` is comma-separated in day's bodies and space-separated
  in the Agent Skills specification. Convert it, drop it, or keep the divergence?
  **Keep the comma form.** REQ-7 (Claude Code behaviour unchanged) has an
  acceptance criterion and conformance of this field has none: it is optional,
  marked experimental, validated by no schema, and Agent Plugins defers the skill
  format entirely — so nothing rejects a plugin over it. Dropping the field would
  degrade Claude Code from pre-approved to prompting, which is a REQ-7 change for
  no conformance gain. And the divergence fails safe: a strict client splitting on
  spaces grants *fewer* tools and prompts, which is what
  `telos/affordance-not-enforcement` would choose anyway. Pinned by
  `the_allowed_tools_divergence_from_the_spec_is_deliberate` in
  `tests/agent_plugins.rs`, so changing it is a decision rather than a tidy-up.

- RQ-4: Q: Does this serve an existing telos? **No — it needs a new one.**
  `telos/v1.0`'s bar is that a non-author ships with day on a third project, and
  it says nothing about harnesses. Reading harness reach into it would widen a
  bar without re-checking it, which `CLAUDE.md` records as its own defect.

  **The telos this produced is `telos/cross-harness-attained`**, declared with
  the witness `cross-harness-trial` after a `/witness-interview` pass. It existed
  three and a half hours before this document was last edited and was not named
  here, so a reader could not find the north star from the document that argued
  for it — the same findability failure the handoff thread records for RQ-4's own
  supersession. Named now.

## Open Questions

<!-- OPEN: Q1 -->
### Q1: Which reverse-domain namespace does Claude Code claim?

§8.2 fixes the mechanism — files for a namespace live in a top-level directory
named after it — but the namespace itself is client-defined, and Claude Code was
not among Agent Plugins' launch clients (ChatGPT, Codex, Cursor, GitHub Copilot,
Kiro, VS Code). `com.anthropic.claude-code` is the obvious guess and a guess is
exactly what this document must not record: a wrong directory name is silently
ignored by every client, which is the failure mode that looks like success.

Resolving this needs a published Anthropic statement or a Claude Code release
that reads Agent Plugins packages. Until then REQ-6 is specified and not
implementable, and the hook stays at `hooks/hooks.json`.

**To resolve**: name the namespace once Claude Code documents one, or record
that day ships no extension directory until it does.
<!-- /OPEN -->

## Out of Scope

- The `kan-tools/plugins` marketplace repository. It is Claude Code-specific
  distribution, complementary to this and already built; Agent Plugins defines
  no discovery mechanism, so the marketplace entry keeps working unchanged.
- Submitting to the official Claude Code plugin directory or to any registry the
  Agent Plugins clients stand up. That is a reach decision, downstream of this
  one.
- Bundling per-platform `day` binaries as package data, per the Architecture
  section's reasoning.
- Converting kan's plugin packaging. kan ships one MCP server and no skills, so
  its conformance cost is a single `mcp.json` and it is a separate repo's call.
- Any change to what the five atoms *do*. This document moves and rewrites their
  context-gathering; the process each one describes is untouched.
