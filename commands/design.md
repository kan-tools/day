---
allowed-tools: Bash(kan *), Bash(day *), Bash(git *), Bash(gh *), Bash(ls *), Bash(mkdir *), Bash(shasum *), Read, Grep, Glob, Write
description: Interactive, iterative design document authoring grounded in codebase exploration, recorded into kan
---

> **day's "generative closed-loop design" atom** (`docs/TELOS.md`). Its
> interface: it consumes a rough intent plus the codebase, and produces a
> validated design document plus the kan claims that make it findable
> later. It composes into a build step, and the document it produces is
> still what `/adversarial-review` audits against further downstream.
>
> ```day-atom
> {"in": ["intent"], "out": ["design-doc"], "next": ["generative-build"]}
> ```
>
> This command lived in kan's own repo until day existed to receive it —
> kan's ADR-18 flagged it as workflow creep, since interactive multi-turn
> authoring is process, not durable fact-recording. This is that migration.

## Context

- Current repo root: !`git rev-parse --show-toplevel`
- Current branch: !`git branch --show-current`
- kan available: !`command -v kan >/dev/null 2>&1 && echo yes || echo "no (spine not built yet — write plain files, skip kan integration steps)"`
- kan subjects (if kan exists): !`command -v kan >/dev/null 2>&1 && kan status 2>/dev/null || true`
- day process state: !`command -v day >/dev/null 2>&1 && day doctor 2>&1 || echo "day not on PATH"`
- Existing design docs: !`ls .design/*.md 2>/dev/null`
- Orientation files: !`ls README.md CLAUDE.md docs/SPEC.md docs/HANDOFF.md docs/SETUP-TODO.md 2>/dev/null`

## Your task

You are an interactive design document author. You help the user go from a rough
feature idea to a validated, codebase-grounded design document.

The design doc *is* a claim (or a small chain of claims) in the acting agent's own
kan log: exploration becomes `observe`, the draft becomes `plan`, resolved open
questions become `decide`. The fold — not a separate knowledge base — is what makes
the doc findable later. There is no pipeline to hand off to and no shared mutable
store to write into; a design that is never recorded is a design that the next
session cannot find. If `kan` isn't available in this repo, fall back to writing
plain files under `.design/` and note in the doc that it still needs to be
back-filled into the log (see Phase 5).

### Arguments

The user may pass these after `/design`:

- A quoted feature description: `/design "add the RelationProvider trait for git anchors"`
- `--gh-issue <number>`: pull context from a GitHub issue (`gh issue view <number>`)
- `--continue <slug>`: resume iteration on an existing draft in `.design/<slug>.md`

If no arguments are given, ask the user what they want to design.

### Phase 1: Explore & Interview (skip if `--continue`)

1. **Gather context** from all available sources:
   - If `--gh-issue <number>`: run `gh issue view <number>`
   - Read this repo's orientation files if they exist (`CLAUDE.md`, `docs/SPEC.md`,
     `docs/HANDOFF.md`, or their local equivalents). Where a repo has an authoritative
     spec, a feature that contradicts it needs an explicit open question, not a silent
     override.
   - If any teloi are recorded (`telos/*` subjects in kan), read them: a design that
     serves no recorded telos, or quietly trades one off against another, is exactly
     the drift day exists to surface. Name which telos the feature serves.
   - Search for related code using `Grep` and `Glob` — find modules, types, functions, and
     test patterns related to the feature.
   - If `kan` is built and has a log: `kan issues` and `kan show <subject>` for anything
     that looks related, so you don't re-derive a decision that's already on record.
   - Check existing drafts in `.design/`.

2. **Ask 3-5 clarifying questions** grounded in what you found:
   - Reference specific files, functions, spec sections, or prior kan claims you discovered.
   - Ask about ambiguities that affect architecture decisions.
   - Ask about scope boundaries — kan's smell test applies: if the local-only path isn't
     dramatically simpler than the multi-actor path, something's wrong; flag it as an
     open question rather than quietly picking the complex option.
   - Do NOT ask generic questions — every question must reference something concrete.

3. **Wait for the user to answer** before proceeding to Phase 2.

### Phase 2: Draft

4. **Create the `.design/` directory** if it doesn't exist: `mkdir -p .design`

5. **Derive the slug** from the feature title: lowercase, spaces to hyphens, strip special
   chars. Example: "Add the RelationProvider trait" → `add-relationprovider-trait`.

6. **Write the design document** to `.design/<slug>.md`:

```markdown
# Feature: <title>

## Summary
1-3 sentence overview of what this feature does and why.

## Requirements
- REQ-1: <specific, measurable requirement grounded in codebase/spec>
- REQ-2: ...

## Acceptance Criteria
- [ ] AC-1: <mechanically testable criterion>
- [ ] AC-2: ...

## Architecture
Freeform prose referencing actual files, modules, types, and patterns. Note any
touchpoint with the fold, the append-only log, or anchors — kan's non-negotiable
invariant (no operation destroys a subject) constrains this section directly.

## Open Questions

<!-- OPEN: Q1 -->
### Q1: <question title>
<context and options>
**To resolve**: Edit this section with your decision and remove the `<!-- OPEN -->` marker.
<!-- /OPEN -->

## Out of Scope
- <explicit exclusion to prevent scope creep>
```

**Quality standards — enforce all of these:**
- Requirements reference real codebase/spec concepts (not generic "should handle errors").
- Acceptance criteria are mechanically testable.
- Architecture references actual file paths discovered during exploration.
- No placeholder text (`<...>`, `TODO`, `TBD`).
- Every requirement maps to at least one acceptance criterion.
- Genuine ambiguities become `<!-- OPEN -->` blocks, not guesses.

### Phase 3: Resolve open questions (interactive)

7. **Present each open question to the user directly** in conversational text. Do NOT
   require the user to edit the file manually.
8. **Wait for the user's answer** to each question. If they say "skip" or "later", leave
   the `<!-- OPEN -->` block in place.
9. **Update the document**: replace resolved `<!-- OPEN -->` blocks, adjust
   requirements/acceptance criteria, and re-explore code if the answer changes scope.

### Phase 4: Iterate (when `--continue` is used)

10. Read the existing draft: `Read .design/<slug>.md`.
11. Detect remaining `<!-- OPEN -->` blocks; if any remain, run Phase 3 on them. If none
    remain, proceed to strengthening.
12. Update requirements/acceptance criteria if scope changed; add new `<!-- OPEN -->`
    blocks if new ambiguities surfaced.
13. Write the updated document back to `.design/<slug>.md`.

### Validation

**Do not validate by hand. Run:**

```bash
day design check .design/<slug>.md
```

It checks the document against this project's live schema (declared in kan,
`docs/CONVENTIONS.md`): required sections, requirement and criterion counts,
every requirement covered by a criterion, placeholder text, referenced file
paths existing on disk, and unresolved open questions. Print its output as-is.

This used to be a list of rules for you to apply. It is a linter now because
counting is exactly what a language model does worst and reports most
confidently — and because a check you perform on yourself is not a check. If
`day` is unavailable, say so plainly rather than substituting your own count.

Fix what it reports and re-run until it is clean, or until what remains is a
deliberate open question (those warn; they do not fail).

### Phase 5: Record into kan

**Do not assemble the claim chain by hand. Run:**

```bash
day design record .design/<slug>.md
```

It appends an `observe` carrying the validation result, a `plan` for the design
citing it, and one `decide` per bullet under the schema's resolved-questions
section citing the plan — building `--cites` from CIDs it captured itself.

That last part is the whole reason this is a command and not instructions. The
prose this replaces told you to pass file paths to `--cites`, which takes claim
CIDs and errors on a path. It was wrong for as long as it was prose.

A document that fails validation is **still recorded**, with the result embedded
in the plan claim. Do not withhold a recording because a design is rough — an
unrecorded design serves the record worse than a visibly incomplete one.

Report the CIDs it prints. If `day` or `kan` is unavailable, say so and leave the
document as a plain file rather than improvising a chain.

### Phase 6: Name the target, and plan the path to it

A design says what to build. It does not say what state of the world the work
is *for*, or how the steps get there — and when those go unrecorded, the record
cannot afterwards say what the work was aiming at.

**This phase exists because of measured behaviour, not principle.** Across
day's first five milestones, every capability with a slash command was used
every time, and every capability that was only a bare CLI verb was used once,
by its author, in the milestone that built it. `day bridge check` ran for the
milestone that shipped it and never again; exactly one milestone telos was ever
declared. The fix is not to ask people to remember a verb. It is to attach the
verb to a step that already happens — this one. (Recorded on `process-model`;
`docs/ROADMAP.md` v0.5 carries the evidence.)

1. **Establish the target telos.** Read the teloi already on record
   (`kan status | grep 'telos/'`). Either this work serves one that exists — say
   which, and why — or it needs a new one. A milestone-shaped design usually
   does: the state of the world where this work is done and visible.

   Declare it with **witnesses**, the artifact *types* that would evidence it.
   Without witnesses a telos cannot be a checkable bridge target, and day will
   say so rather than guess:

   ```bash
   day telos declare <slug> "<the state of the world this reaches>" \
     --witness <artifact-type>
   ```

2. **Declare the bridge** — the arrangement of atoms that gets from here to
   there. Ask the vocabulary what exists (`day doctor`) rather than assuming a
   pipeline; `>` is sequence, `&` concurrent, `|` alternatives, and `--have`
   names what is already available at the start:

   ```bash
   day bridge declare <slug> --telos <telos-slug> --have intent \
     --plan "design > generative-build > adversarial-review > pull-request > release"
   ```

3. **Check it, and report what it says verbatim.** `day bridge declare` runs
   this for you, so a separate `day bridge check <slug>` is only needed when
   revising.

   If it reports the plan cannot reach the telos, **do not quietly adjust the
   telos until it passes.** An unreachable plan is information: either a step is
   missing, or the target needs a witness no declared atom produces. Say which.

**If the user declines any of this, record that they declined and move on.**
This phase is advisory like everything else day does — `telos/affordance-not-enforcement`
governs day's own commands too, and a design pass that refuses to finish
without a bridge would be the gate this project exists to avoid.

### Phase 7: What comes next

Do not tell the user to run a particular command next. Ask the graph:

```bash
day next design
```

Composition is declared in kan, so what follows a design differs by project —
one may go straight to build, another through formal verification first. Report
what it says. If you name a fixed next step from memory, you have hard-coded a
pipeline that this project may not have.

### Summary Output

Print this summary after every invocation:

```
Design document written: .design/<slug>.md

Validation: N requirements, N acceptance criteria, N open questions
kan record:  <recorded as observe/plan/decide claims | skipped, kan not built>
Target:      telos/<slug> <witnesses> | <named existing telos> | declined
Bridge:      bridge/<slug> — <reaches | does not reach | not declared>

Next steps:
  - Edit in your editor:  $EDITOR .design/<slug>.md
  - Continue iterating:   /design --continue <slug>
```

### Rules

- Do NOT modify any source code files. You only write to `.design/`.
- Do NOT block on a design being "complete" — an unresolved open question, explicitly
  marked, is more useful than a confident guess.
- Do NOT invent kan subcommands beyond CLAUDE.md's CLI vocabulary.
- Do NOT auto-create GitHub issues. The user manages issue lifecycle.
- Every question you ask must be grounded in specific codebase/spec findings.
- A document with unresolved `<!-- OPEN -->` blocks is valid but flagged.
