---
name: handoff
allowed-tools: Bash(kan *), Bash(day *), Bash(git *), Bash(gh *), Bash(ls *), Read, Grep, Glob
description: Write this session's handoff onto its thread, stating claims the next wakeup can verify
---

> **day's "handoff" atom**, and the write half of a pair: `/wakeup` reads what
> this writes. Its interface: it consumes the session's work and produces a
> handoff, which the next session's orientation consumes — so the loop closes
> across a session boundary rather than inside one.
>
> ```day-atom
> {"in": ["code-change"], "out": ["handoff"], "next": ["orient"]}
> ```
>
> **No `done` criteria.** "The handoff is good" is a judgement about prose. What
> *is* checkable is whether the claims it makes survive `/wakeup`'s Phase 2, and
> that check belongs to the next session rather than to this one — a criterion
> asserting its own quality would be the witness-that-cannot-fail day#86 names.

## Context — gather this yourself, and report what fails

Run these before Phase 1. **A read that fails is a finding, not an empty
result**, and this section is instructions rather than pre-expanded output
precisely so that you get an exit code where the harness used to get a string.
It matters more here than anywhere else in day: an unavailable read that looks
like an empty one becomes a *claim* in the record, and `/wakeup` will then
report it CONFIRMED against the same absence.

- **Branch** — `git branch --show-current`.
  **If this read fails:** or prints nothing, you are on a detached HEAD. Write
  that in the handoff; do not name a branch you did not read.
- **HEAD** — `git log --oneline -1`.
  **If this read fails:** the repo has no commits. Say so rather than omitting
  the line, since a missing HEAD claim reads as an oversight.
- **Tree** — `git status --porcelain`.
  **If this read fails:** say so in the handoff. Empty output means clean; a
  failed command does not, and "clean" is a claim the next session will check.
- **Unpushed** — `git log --oneline @{u}..HEAD`.
  **If this read fails:** there is no upstream. Record *that*, not "nothing
  unpushed" — they are different facts and only one of them is reassuring.
- **day position** — `day status`.
  **If this read fails:** day is not on PATH or cannot reach kan. Say so
  instead of writing a position from memory; a remembered position is exactly
  the unfalsifiable prose this skill exists to keep out of the record.

Also note whether this is a git worktree (`git rev-parse --git-common-dir`
differing from `--git-dir`). kan#197: a worktree gets its own `.kan/`, so Phase
4's write would fork the log into one nobody reads. **Run the write from the
main checkout.**

## Your task

Write the handoff for this thread onto `agents/handoff/<thread>`.

### The premise, which decides everything below

**Write the claims the next `/wakeup` will check.** That command takes a
handoff's factual assertions and verifies each against git, `gh` and `day`,
reporting CONFIRMED, DRIFTED or UNCHECKABLE. A handoff full of unfalsifiable
prose survives that check by being immune to it, which is the same defect as a
telos whose witness cannot fail.

So prefer *"HEAD at 824586b, CI green, suite 30 targets / 0 failures"* — three
claims a machine can confirm or refute — over *"everything is in good shape"*,
which cannot be wrong and therefore says nothing.

### Phase 1: Gather, do not recall

Compute the state rather than remembering it. What you remember is what you
believe happened, and the gap between that and the record is exactly what a
handoff exists to close.

```bash
git log --oneline <last-handoff-sha>..HEAD
git status --porcelain
day status
day doctor
```

Also check: CI on the branch, any issue this session opened or closed, and any
claims recorded — read them back through `kan show --all --json` rather than
trusting your account of what you wrote.

### Phase 2: Read the previous handoff

Take the newest live claim on the subject. You are writing a *supersession*, not
an independent note:

- Say which of its "next" items are now done, so the next session does not redo
  them.
- Say explicitly that it supersedes, and name it. `kan show <subject>` does not
  surface inbound citations, so a reader arriving at the old claim sees no sign
  it was replaced unless the new one is newer on the same subject — which it is,
  and that is why handoffs live on one subject per thread.
- Carry forward anything still open. A handoff that drops an open item silently
  is how a thread loses work.

### Phase 3: Write it

Structure that makes the round trip work:

1. **State, with how it was verified.** Branch, HEAD, CI, suite, tree. Each a
   claim; each checkable.
2. **The decision a reader needs** to understand the work — the one or two
   choices without which the next step looks arbitrary. Not a changelog.
3. **What is next, in order**, and why that order. If something was promoted or
   demoted, say what promoted it.
4. **What is open elsewhere** — issues in this repo and others, blocked items,
   things waiting on someone.
5. **What is deliberately not being done**, and why. This is the section that
   stops the next session relitigating a settled decision.

**Do not carry anything derivable.** `day status` computes position, `git log`
carries history, `day doctor` counts atoms. Restating them makes a second copy
with no fold behind it, and every second copy in this repo has drifted.

Keep it to what a reader needs to *act*. A handoff long enough to skim is a
handoff that gets skimmed.

### Phase 4: Record it

```bash
kan observe agents/handoff/<thread> "<the handoff>"
```

`observe`, not `decide` — a handoff reports state; it does not settle a
question. Decisions belong on their own subjects, where the reader looking for
them will actually be.

Then **read it back** through `kan show --all --json` and confirm the text
arrived whole. Recording is not the same as having recorded, which this repo has
learned more than once.

### Phase 5: Say what you could not establish

Anything you asserted from memory rather than from a command, name it — in the
handoff itself, not only in conversation. An unverified claim sitting beside
verified ones inherits their credibility, and the next `/wakeup` will report it
as UNCHECKABLE at best and CONFIRMED-looking at worst.

## When to run this

At the end of a working session, and **before any context boundary you can
see coming**. A handoff written at 90% context is worth more than a perfect one
that never got written, because the failure mode is not a bad handoff — it is no
handoff, and a thread with no handoff is one the next session reconstructs by
guessing.

Also worth running mid-session after anything a future reader would not infer:
a reversed decision, a design superseded, a defect found and filed.
