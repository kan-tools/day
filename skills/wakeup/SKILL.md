---
name: wakeup
allowed-tools: Bash(kan *), Bash(day *), Bash(git *), Bash(gh *), Bash(ls *), Bash(cargo *), Read, Grep, Glob
description: Wake into a working thread by reading its recorded handoff and verifying what the handoff claims before trusting it
---

> **day's "orientation" atom**, and the read half of a pair: `/handoff` writes
> what this reads. Its interface: it consumes a recorded handoff and produces
> the *intent* a session can act on — which is what `/design` takes as its own
> input, so the three compose into a loop that crosses session boundaries.
>
> ```day-atom
> {"in": ["handoff"], "out": ["intent"], "next": ["design"]}
> ```
>
> **No `done` criteria, deliberately.** "The session is oriented" is not
> mechanically checkable, and day#86 holds that a witness which cannot fail is
> worse than none — so inventing a probe to clear the caution would be the
> failure this repo has recorded twice. `day status` will say completion cannot
> be checked for this atom, which is the honest reading.

## Context — gather this yourself, and report what fails

Run these before Phase 1. **A read that fails is a finding, not an empty
result**, and this section is instructions rather than pre-expanded output
precisely so that you get an exit code where the harness used to get a string.
day#100 is the alternative: a telos read matched nothing, a fallback printed
`none`, and every adversarial review in this repo measured against the wrong
north star at exit zero for as long as nobody checked.

- **Repo root** — `git rev-parse --show-toplevel`.
  **If this read fails:** you are not in a git repo, so every Phase 2 check
  against git is UNCHECKABLE rather than CONFIRMED. Say so and continue.
- **Branch** — `git branch --show-current`.
  **If this read fails:** or prints nothing, you are on a detached HEAD. Report
  that rather than substituting the branch the handoff names.
- **HEAD** — `git log --oneline -1`.
  **If this read fails:** the repo has no commits, which makes the handoff's
  `HEAD is at <sha>` claim UNCHECKABLE. Do not report it as CONFIRMED.
- **Working tree** — `git status --porcelain`.
  **If this read fails:** say so. Empty output means clean; a failed command
  does not, and the two must not be reported the same way.
- **kan** — `kan --version`.
  **If this read fails:** kan is not on PATH, and a handoff lives in kan, so
  this skill has nothing to read. Say that plainly and stop rather than
  orienting from git alone and calling it a wakeup.
- **day** — `day doctor`.
  **If this read fails:** day is not on PATH or cannot reach kan. Phase 3 is
  then UNCHECKABLE; report it as such rather than skipping it silently.

Also note whether this is a git worktree (`git rev-parse --git-common-dir`
differing from `--git-dir`). kan#197: a worktree gets its own `.kan/`, so kan
reads there report "no subjects yet" against an empty log rather than failing —
a success that means nothing. If you are in one, run kan from the main checkout.

## Your task

Take a session from zero to working by reading the thread's recorded handoff —
**and by checking what it says, rather than believing it.**

### The premise, which is the whole design

A handoff is a *claim about state at the moment it was written*. State moves:
commits land, CI runs, someone else pushes, an issue gets closed. A command that
reads a handoff and reports it as current is copy-paste with extra steps, and it
fails in the direction that matters — a stale handoff read as fresh is worse
than no handoff, because it carries the authority of the record.

So every phase below separates **what the handoff asserts** from **what is true
now**, and the output says which is which.

### Arguments

- `/wakeup` — wake into `agents/handoff/main`, the thread that interfaces with
  the human.
- `/wakeup <thread>` — wake into `agents/handoff/<thread>`.
- `/wakeup --list` — show which handoff threads exist and when each was last
  written.

### Phase 1: Read the handoff

1. **Read it through the bulk verb, not `kan show`.**

   ```bash
   kan show --all --json
   ```

   Filter to the subject in the tool of your choice. `kan show <subject>` is the
   obvious command and is the wrong one: it is O(n²) in commit-anchored claims
   with a subprocess per pair, measured at 141 s on a repo where `--all --json`
   takes 72 ms (kan#181). If a friction log exists at `workflows/kan-reads`, the
   current status is there.

2. **Take the newest live claim on the subject.** Earlier ones are history, and
   a handoff that supersedes another usually says so. If the newest claim says
   it supersedes an earlier one, do not also act on the earlier one.

3. **If there is no such subject**, say so plainly and stop trying to resume.
   Offer to orient from scratch instead — `day status`, `day doctor`, the
   README, recent commits — and to write the first handoff at the end of the
   session. A repo with no handoff is the normal first-time state, not an error.

### Phase 2: Verify what it asserts

**This is the phase that earns the command.** Go through the handoff's factual
claims and check each one. Typical claims and their checks:

| the handoff says | check it with |
|---|---|
| HEAD is at `<sha>` | `git log --oneline -1`, and `git log --oneline <sha>..HEAD` for what landed since |
| branch `<name>` | `git branch --show-current` |
| GitHub Actions run `<run-id>` at `<head-sha>` concluded success | `gh run view <run-id> --json databaseId,headSha,conclusion,workflowName,url`; require both the same run ID and head SHA |
| `<suite-command>` passes at `<full-sha>` | first require `git cat-file -e <full-sha>^{commit}`; run the exact command in a clean checkout of that commit, or say you did not |
| census has N unaccounted over `<base>..<head>` | require both commits with `git cat-file -e`, then run `just census-demonstrations <base>..<head>` — never its current default range |
| tree clean | `git status --porcelain` |
| issue #N is open/closed | `gh issue view N --json state` |
| a design is recorded | the claim is in the `--all --json` you already read |
| N atoms, composition ok | `day doctor` |

Classify each as **CONFIRMED**, **DRIFTED** (with what it is now), or
**UNCHECKABLE** (with why). Do not silently drop one you could not check —
could-not-check outranks checked-and-clean, and a verification report that omits
what it skipped asserts a completeness it did not establish.

The coordinates belong to the recorded measurement, not to the session doing
the checking. Advancing or merging HEAD does not change a suite SHA, census
base/head pair, or CI run/head pair. If a legacy handoff says only "suite
passes", "census clean", or "CI green", classify that assertion UNCHECKABLE:
do not silently substitute the current HEAD, a newly derived census range, or
the newest workflow run. If a scoped commit no longer exists locally and
cannot be fetched, or an exact CI run is unreadable, that assertion is likewise
UNCHECKABLE and the missing coordinate is named.

**Anything that drifted is the most valuable thing in the output.** It is the
work someone else did, or the thing that broke, since the handoff was written.

### Phase 3: Orient from day, not from the handoff

The handoff says where the work *was*. day computes where it *is*, from
artifacts:

```bash
day status
day doctor
```

If `day status` names an atom the handoff does not mention, or the handoff's
"next" step is already done, say so. These two disagreeing is information: the
handoff is a person's account and `day status` is inferred from the record, and
the gap between them is usually where something was finished and not written
down.

### Phase 4: Report, and stop

Produce, in this order:

1. **One paragraph: where this thread is.** Not a restatement of the handoff —
   the handoff as corrected by what you just checked.
2. **What drifted**, if anything, with what it is now.
3. **What the handoff says to do next**, and whether that is still the right
   next thing given the drift.
4. **What is open elsewhere** — issues, other repos, blocked items the handoff
   names.
5. **Anything you could not verify**, named.

**Then stop.** Do not start the work. The point of this command is that the
human reads five lines and says "yes, that" or "no, actually" — and both are
cheap only if nothing has been done yet.

### There is no Phase 5

Writing the handoff is `/handoff`, a separate command with its own atom. The
split is not tidiness: waking and handing off happen at opposite ends of a
session, consume different things, and fail differently. A single command doing
both would be one whose second half is only ever reached by a session that
remembered to invoke it again.

What the pair guarantees is the round trip — `/handoff` writes the claims
`/wakeup` checks, so a handoff that cannot be verified is a handoff that was
written wrong, and both halves say so.

## Why this is a command and not a hook

`day hook session-start` already injects teloi, atoms, position and practice
into every session, and it should not also inject a handoff: the hook is
advisory context for *any* session, and waking into a thread is a deliberate act
with a subject to choose. Making it automatic would put one thread's state into
every session in the repo, including sessions working on something else.

It is also a *read* of the record that ends in a proposal, which is what makes
stopping at Phase 4 safe. A hook that resumed work would be a hook that decides.
