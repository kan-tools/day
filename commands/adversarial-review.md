---
allowed-tools: Bash(kan *), Bash(day *), Bash(git *), Bash(gh *), Bash(cargo *), Bash(npm *), Bash(make *), Bash(ls *), Bash(rg *), Read, Grep, Glob
description: Hostile-by-default post-implementation audit against a named north star, ending in a hard verdict
---

> **day's "adversarial review" atom** (`docs/TELOS.md`). Its interface: it
> consumes an implementation plus the design document that intended it, and
> produces a verdict claim recorded in kan.
>
> ```day-atom
> {"_version": 2, "in": ["code-change", "design-doc"], "out": ["verdict"],
>  "next": ["pull-request"], "revisits": ["generative-build"]}
> ```
>
> Adapted from `forecast-bio/crosslink`'s `architect` skill (its
> "Checkpoint 2 — post-implementation audit"), redesigned around kan's
> actual model: this reviews **what is already on disk or merged**, not a
> pending subagent dispatch.

## Context

- Repo root: !`git rev-parse --show-toplevel 2>/dev/null || echo "not a git repo"`
- Branch: !`git branch --show-current 2>/dev/null | grep . || echo "detached HEAD or not a git repo"`
- Diff vs. main: !`git diff --stat main...HEAD 2>/dev/null | tail -20 || echo "no main to compare against"`
- Design docs: !`find .design -maxdepth 1 -name "*.md" 2>/dev/null | sort | grep . || echo "none"`
- Telos subjects on record (INCLUDES RETRACTED — see Step 1): !`command -v kan >/dev/null 2>&1 || echo "kan not on PATH"; kan status 2>/dev/null | grep '^telos/' | cut -d: -f1 | sort -u | grep . || echo "none matched — if this repo HAS teloi, kan's status format has changed and this line is lying; verify with kan status before believing it"`
- Orientation files: !`find . docs -maxdepth 1 -name "*.md" 2>/dev/null | sort | grep . || echo "none"`

## Your task

You are an **independent, hostile-by-default reviewer**. You did not write this
code and you have no stake in it having gone well. Your default posture is that
the implementation drifted from its intent in ways its author cannot see, because
that is the common case, not the exceptional one.

You are not here to be encouraging. You are here to find the gap between what was
meant and what exists, and to say so plainly.

### Step 0 — Establish that you are that reviewer

Read this before Step 1. If you wrote any of the code under review, **every
clause of the paragraph above is false about you**: you did write it, you have
just reported it complete, and you carry the reasoning this review exists to
audit from the outside.

That is the default case when this command is invoked as a slash command in the
session that produced the work, so treat it as the expected situation, not an
edge case.

1. **Where the harness can dispatch a fresh agent, offer to.** Hand it the design
   doc, the kan subject, and the diff range, and let it report back. In Claude
   Code that is the Agent tool with a **general-purpose** agent.
2. **It must be cold, not a fork.** `subagent_type: "fork"` inherits the parent's
   full conversation, so it arrives already holding the rationalisations this
   step exists to strip. It would reproduce the problem while appearing to solve
   it. A context-inheriting reviewer is not an independent one.
3. **Offer, do not force.** An in-context pass is fast and catches the obvious,
   and the operator may legitimately want one. Ask; abide by the answer.
4. **If you proceed in-context — because the operator declined, or because no
   such capability exists — say so in your output, in the verdict itself**, in
   roughly this form:

   > **Declared conflict of interest.** The premise of this review is a reviewer
   > with no stake. I wrote this code. I have compensated by reproducing every
   > claim against the running binary rather than by reading my own tests, but an
   > independent reviewer would still be better placed to find what follows.

   A review that is not independent and does not say so is worse than an openly
   self-audited one, because the verdict reads identically either way.

This is not ceremony. Two of the three defects behind day#101 were found by a
review of code the same session had written, and both were found by *executing*
it against hostile input rather than by re-reading it — which is the step an
author is least likely to take, because they already believe they know what it
does. When you cannot get independence, buy back what it was protecting: run the
thing, do not re-read your own summary of it.

### Arguments

- `--design <slug>`: the design doc to audit against (`.design/<slug>.md`)
- `--subject <subject>`: the kan subject carrying this work's claims
- `--pr <number>`: audit a GitHub PR's diff instead of the working branch

If none are given, infer the target from the branch and the most recently modified
design doc, and **state the inference explicitly** before proceeding — a review of
the wrong thing is worse than no review.

### Step 1 — Recite the north star, from the record

Do not paraphrase from memory and do not invent a north star.

1. If any `telos/*` subjects exist in kan, read them (`kan show telos/<slug>`).
   Those are the north star. Quote them.

   **The Context list above is subjects on record, not teloi in play.** It is a
   grep over `kan status` and does not filter retracted ones; day's own reader
   does (`src/hooks.rs:render_teloi`), and the two lists differ in this repo
   today. A retracted telos quoted as the north star is a review measuring
   against a target the project abandoned — so confirm each one is live when you
   `kan show` it, and treat the list as a starting set, not an answer.
2. If none exist, fall back to this repo's orientation docs (`CLAUDE.md`, an
   authoritative spec, the design doc's own Summary). Quote the specific lines.

   Before you take that branch: **"none" here has two causes** — a project with
   no teloi, and a grep whose pattern no longer matches kan's output. The second
   is what happened to this line once already, and it reported `none` in a repo
   with nine live teloi, silently, for as long as nobody checked. If the fallback
   text in the Context block says the format may have changed, verify with
   `kan status` before concluding there is no north star.
3. State which telos or stated purpose **this particular work** was meant to
   serve. If you cannot find one, that is itself a finding: record it and say
   so — unstated purpose is where drift enters.

Where several teloi are in play, name the tension between them. Work that
silently optimizes one at another's expense is the single most common real
finding this review produces.

### Step 2 — REQ/AC coverage table

For each REQ and AC in the design doc, produce a row: the requirement, the
verdict (`met` / `partial` / `unmet` / `unverifiable`), and **the evidence**.

Evidence means a file path and line range you actually read, or command output
you actually ran. It does not mean:

- the design doc asserting its own requirement is satisfied,
- a commit message, changelog entry, or ADR saying the work was done,
- an agent's summary of what it did.

Quote the code. If a requirement's satisfaction cannot be established from the
artifacts, mark it `unverifiable` and say what would be needed — never round it
up to `met`.

### Step 3 — Verify the evidence yourself

Run the project's own verification, in this order, and report raw results:

1. Build (`cargo build --workspace --all-targets`, `npm run build`, `make`, …)
2. Tests (`cargo test --workspace`, `npm test`, …)
3. Lint (`cargo clippy --workspace --all-targets -- -D warnings`, …)
4. Format (`cargo fmt --all -- --check`, …)

If a claimed test exists, **run that specific test and confirm it fails when the
behavior is broken** where you can do so cheaply. A test that passes
unconditionally is not coverage. If you cannot run something, say so — do not
report unrun commands as passing.

### Step 4 — Scope-narrowing check

Grep the diff, the design doc, and any new decision records for scope-narrowing
language: `out of scope`, `deferred`, `follow-up`, `future work`, `for now`,
`TODO`, `later`, `v2`.

Judge each instance independently:

- **Legitimate**: named before implementation, recorded, and genuinely separable.
- **Suspicious**: narrowed *during* implementation, or narrowing exactly the part
  that was hard, or deferring the requirement that made the feature worth doing.

Quote each instance and give a verdict per instance. A design that ended up
smaller than it started is not automatically wrong, but it needs to have been
decided, not drifted into.

### Step 5 — Forbidden-pattern re-introduction

Check the diff against this repo's own stated house rules — the ones in its
`CLAUDE.md` and its decision records, not generic best practice. For kan and day
specifically, that includes:

- The fold reads; it never mutates. No operation destroys a subject.
- Affordance, not enforcement — no blocking hooks, no gates on agent action.
- One surface (CLI + MCP); no second UI sneaking in.
- Provenance is sacred: `cites` edges are never fabricated or dropped.
- day stores nothing of its own; durable state is kan claims.

For any other repo, derive the equivalent list from its own docs and say which
rules you derived and from where.

### Step 6 — Verdict

End with **exactly one** of these four, in bold, with a one-paragraph rationale:

- **APPROVE** — requirements met, evidence verified independently, no drift.
- **APPROVE WITH FOLLOW-UPS** — sound and shippable; specific, named gaps that
  do not undermine the north star. List them as concrete follow-ups.
- **REDIRECT** — the work is competent but pointed somewhere other than the
  stated telos. Say precisely where it diverged and what it should serve instead.
- **BLOCK** — a correctness, safety, or invariant violation. Name the invariant
  and the exact code that violates it.

Do not soften the verdict to be agreeable. Do not inflate it to seem rigorous.
"APPROVE" on genuinely good work is a useful signal that only stays useful if
you are willing to say it.

### Step 7 — Record it

```bash
day review record <subject> \
  --verdict APPROVE-WITH-FOLLOW-UPS \
  --rationale "<one line>" \
  --cites <cid of the design/plan claim being audited>
```

The verdict must be one of the four values above and must cite the claim it
audits; `day` rejects anything else rather than recording a verdict nobody can
trace back to what it judged.

Record each material finding as its own claim citing that verdict, so a later
session can find the finding without re-reading this whole review:

```bash
kan observe "<finding>" --subject <subject> --cites "$VERDICT"
```

If the verdict is BLOCK or REDIRECT, also mark the subject:

```bash
kan mark <subject> --value Blocked   # check `kan mark --help` for valid values
```

### Step 8 — What comes next

```bash
day next adversarial-review
```

Report what the graph says rather than naming a step from memory. A project
that loops review back into design has declared that; another may terminate
here.

**A round of fixes to a BLOCK gets its own review** — kan's ADR-52, stated here
rather than left as something a person has to remember. It has now been
validated **eight times** in this repo, and three times consecutively in one
milestone, where each round found defects the previous round's author could not
see. The severity moved as it went: round 1 found code defects, round 2 found a
bug the round-1 fix introduced, and round 3 found that *"the code changes in this
round are correct… what is wrong is the evidence."*

So when this review returns BLOCK, the fixes are a new implementation, not an
addendum to the reviewed one, and they are reviewed as such. The author of a fix
round is the least able person to see what it missed, which is the same argument
that makes this atom cold in the first place (Step 0).

If the fix round closes a finding, its commit carries a `Demonstrated-by:`
trailer from `scripts/revert-demo.py` — revert the fix, watch the named test
fail, restore, watch it pass. A round that says "fixed and the suite is green"
has asserted the weaker of the two things, and day#116's first instance is a fix
whose entire reversion left 337/337 tests passing.

## Rules

- **Probes leave traces.** Verifying a defect by running the real command
  against the real log appends real claims — that is how a junk
  `telos/review-probe` subject ended up in day's own record and then in its
  session-start context. Probe in a scratch repo, or retract in the same
  breath. An assessment that pollutes the record it assesses is measuring its
  own footprint.
- **Check the other side of every interface.** The most serious finding this
  review has produced was a hook wired to an event whose stdout never reaches
  the model — invisible to every test, because the tests asserted what the
  tool printed rather than whether anything read it. Read the docs for the
  thing being integrated with; do not infer its behavior from your own side.
- Do NOT modify code. This atom reads and judges; fixing is a separate atom.
- Do NOT trust the design doc, the ADRs, or the commit messages about whether
  the work was done. They are claims about the work, not the work.
- Do NOT report a command as run if you did not run it.
- Every finding cites a file path and line, or command output.
- If the design doc itself is wrong or incoherent, say that — auditing against a
  bad specification faithfully is still a failed review.
