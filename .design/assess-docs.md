# Feature: `day assess docs` — the first assessment atom

## Summary

The v0.2 release shipped with a README that described a feature the same
release removed the need for. Nothing caught it: the contradiction was
already in the record, and nothing compared the record to the docs. This adds
`day assess docs`, the first instance of v0.4's assessment machinery, built
against the release boundary because a tag and a published artifact are the
most crystallized substrate day has to assess against.

It also gives day its second substrate — git, for reads only — and makes the
mechanical checks per-project, because not every project day might serve is a
CLI.

## Motivation

`README.md` said declaring a telos was "still a calling convention you apply
by hand with kan" while kan's log held a claim recording that `day telos
declare` had shipped. Both statements were in the same repo, one in prose and
one in the record, and they contradicted each other for the length of a
release. It was caught by a human reading the file during a version bump.

A PR and a crates.io release are the right place to check for this: they are
the moments where the work becomes externally visible and externally
witnessed, and — in `docs/TELOS.md`'s terms — the most **crystallized** end of
the regularity spectrum. Either the version is on the registry and runs, or it
is not. Assessment against that is cheap and hard to game, which is exactly
why it is the right first assessment to build rather than the most
interesting one.

## Requirements

- REQ-1: day may invoke `git` as a **read-only** substrate — `git tag`, `git
  diff --name-only`, `git log` — and never a mutating one (no `add`, `commit`,
  `tag`, `checkout`, `push`). This is day's second substrate and a deliberate
  widening of "day talks to kan and nothing else"; `CLAUDE.md` records it as
  such rather than letting it arrive unannounced.
- REQ-2: `day assess docs` establishes the release boundary from **both**
  sources — the newest claim on the `release` subject, and the most recent
  `v*` git tag — and **reconciles them**. Divergence is itself a finding: a
  tag with no recorded claim means a release nobody recorded, and a claim with
  no tag means a release nobody cut. Neither is an error to suppress; both are
  drift worth naming.
- REQ-3: When the two boundaries disagree, the assessment runs from the
  **earlier** of the two, so nothing between them escapes review. `--since
  <cid-or-git-ref>` overrides both and skips reconciliation.
- REQ-4: The mechanical tier is **version consistency**: every file the
  project declares as carrying the version must contain the version string
  from the declared source of truth. A file that carries a stale version fails.
- REQ-5: Which files those are is **declared per project** in kan, on a
  `schema/docs` subject carrying a fenced `day-docs` JSON block — the same
  mechanism `schema/design-doc` already uses. day ships a sensible default
  (`Cargo.toml`'s `version` as the source, `README.md` as the carrier) that it
  *suggests* rather than assumes, exactly as `Schema::starter` does. Not every
  project day serves is a Rust CLI, so day must not hard-code where a version
  lives or what a "surface" is.
- REQ-6: The evidence tier reports, without judging: the claims recorded since
  the boundary grouped by subject, and the files changed since the boundary.
  A subject with claims whose docs went untouched is surfaced as a **prompt to
  reconcile**, not a failure — deciding whether a change needed documenting
  requires reading both, which is a judgment and stays with the command.
- REQ-7: `day assess docs` is advisory. It exits non-zero when the mechanical
  tier fails so CI can use it, and it never blocks, gates, or writes.
- REQ-8: The surface is a new `assess` verb group with a `docs` leaf, leaving
  room for `day assess telos` to sit beside it in the rest of v0.4.
- REQ-9: The MCP server exposes `assess_docs`, dispatching to the same
  function the CLI verb calls, following the pattern `doctor` and
  `design_check` already established.
- REQ-10: `docs/CONVENTIONS.md` documents the `schema/docs` convention and the
  `day-docs` block, and `CLAUDE.md` records the git-substrate widening.

## Acceptance Criteria

- [ ] AC-1: Every `git` invocation in the codebase is a read subcommand,
      asserted by a test that greps for the mutating ones — the same guardrail
      style `tests/plugin.rs` uses for blocking hooks. (REQ-1)
- [ ] AC-2: Given a `release` claim and a `v*` tag at different points,
      `day assess docs` names both boundaries and reports the divergence.
      (REQ-2)
- [ ] AC-3: With divergent boundaries, the claims reported are those since the
      **earlier** boundary, verified by a fixture where a claim falls between
      the two. (REQ-3)
- [ ] AC-4: `--since` overrides both sources and produces no divergence
      finding. (REQ-3)
- [ ] AC-5: A declared version-carrying file whose version string is stale
      fails and is named; with all declared files current, the mechanical tier
      passes. (REQ-4)
- [ ] AC-6: Changing the `schema/docs` claim to name a different carrier file
      changes what is checked, with no code or config file edited — the same
      property `schema/design-doc` already has. (REQ-5)
- [ ] AC-7: With no `schema/docs` declared, `day assess docs` explains and
      prints a runnable command recording the starter, rather than assuming a
      layout. (REQ-5)
- [ ] AC-8: Given claims on a subject since the boundary and no doc file
      changed, the output names that subject as unreconciled; given docs
      changed, it does not. (REQ-6)
- [ ] AC-9: `day assess docs` exits non-zero on a mechanical failure and zero
      when only evidence-tier prompts remain, and writes nothing to kan or to
      the working tree in either case. (REQ-7)
- [ ] AC-10: `day assess docs` exists as a subcommand of `assess`, and
      `day assess --help` lists it. (REQ-8)
- [ ] AC-11: An MCP `tools/list` includes `assess_docs`, and calling it
      returns the same text as the CLI verb for the same repository state —
      the equivalence assertion, not merely presence. (REQ-9)
- [ ] AC-12: `docs/CONVENTIONS.md` contains the `schema/docs` prefix usage and
      the `day-docs` fence token, checked against the code's own constants;
      `CLAUDE.md` states that git is a read-only substrate. (REQ-10)

## Architecture

**`src/git.rs` (new)** wraps the read-only git surface, mirroring
`src/kan_client.rs`'s shape: a struct holding the repo path, subprocess calls
returning parsed output, and no method that mutates. Keeping every git call
behind one module is what makes AC-1's grep a meaningful guarantee rather
than a spot check.

**`src/docs.rs` (new)** holds the `DocsSchema` type (loaded from kan via the
existing `atoms::newest_fenced`, the same path `src/schema.rs` uses), the
version-consistency check, and the evidence assembly. The two tiers stay
separate types so the exit code depends only on the mechanical one.

**`src/assess.rs` (new)** is the orchestration: reconcile boundaries, run the
mechanical tier, assemble evidence, render. `src/cli/mod.rs` gains the
`assess` group, and `src/mcp.rs` gains one tool dispatching to the same
function.

**Boundary reconciliation** is the piece with real substance. `release`'s
newest claim gives a CID but not a commit; the newest `v*` tag gives a commit
but not a claim. They are compared by asking git for the tag's commit date and
treating the claim's position in the log as its order — deliberately coarse,
because the goal is to notice they disagree, not to build a precise
cross-substrate clock. Anything finer would be inventing a synchronization
model kan does not have, and kan's own sync design (in kan's repo, not this
one) is where that belongs if it is ever needed.

**Nothing here writes.** `day assess docs` reads kan, reads git, reads files,
and prints. It appends no claim — recording an assessment is a separate act,
and conflating "I checked" with "I recorded that I checked" would let the tool
manufacture its own evidence.

## Resolved Questions

- **git becomes day's second substrate, read-only.** Chosen over inferring
  changed files from kan claim artifacts (which `kan show` does not currently
  expose) and over doing without. The cost is honest: day now depends on two
  substrates instead of one, and `CLAUDE.md` says so.
- **Both boundary sources are primary, and the default reconciles them.**
  Rather than picking the claim log or git tags as authoritative, day reads
  both and treats disagreement as a finding — a release tagged but never
  recorded, or recorded but never cut, is exactly the drift this atom exists
  to surface. `--since` remains available to skip the question.
- **Mechanical tier is version consistency only, and per-project.** A
  verb-surface check was considered and rejected as a built-in: it assumes the
  project is a CLI, and day must not impose its own shape on what it serves.
  Which files carry a version — and, later, what else is mechanically
  checkable — is declared in kan on a `schema/docs` subject, the same way
  design-doc shape already is.
- **`day assess docs`, under a new `assess` group**, so `day assess telos` has
  somewhere to live as the rest of v0.4 lands.

## Out of Scope

- Telos assessment (`day assess telos`) — the larger half of v0.4, its own
  design pass. This builds the surface it will share.
- Any mechanical check beyond version consistency. Additional per-project
  rules are a follow-up once there is evidence about which ones earn their
  keep; guessing at a rule engine now would be building the cathedral.
- Judging whether a change *should* have been documented. That requires
  reading both the change and the prose, and stays with the command.
- Writing assessments back into kan. Reading and recording are separate acts.
- Any git mutation, and any change to kan.
