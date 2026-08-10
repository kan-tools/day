---
name: witness-interview
allowed-tools: Bash(kan *), Bash(day *), Bash(git *), Bash(gh *), Bash(ls *), Read, Grep, Glob
description: Establish, with a human, what would evidence a telos that declares no witnesses
---

> **day's "witness interview" atom.** Its interface: it consumes a telos plus a
> human who knows what the work is for, and produces a revised telos declaration
> carrying witness types and, for each, the condition under which it would
> report absent.
>
> ```day-atom
> {"in": ["telos"], "out": ["witnessed-telos"], "next": ["generative-build"]}
> ```
>
> Its input is a **telos**, not an "unwitnessed telos". Lacking witnesses is the
> *trigger*, not an artifact type — declared as a type it made the vocabulary
> stop composing, because nothing upstream produces one. That was the
> composition check saying the model was wrong rather than the wiring, and it is
> the reason this block is worth reading against `day doctor` rather than
> trusting.
>
> This exists because the alternative is worse. day used to meet an unwitnessed
> telos by printing `day telos declare <slug> "..." --witness <type>`, which
> invites the reading agent to guess a witness alone — and day#86 records why
> that is worse than leaving it unwitnessed: a trivially satisfiable witness
> reports the telos met forever, which is the failure `telos/v05-shipped`
> taught. A witness is a claim about what would count as evidence, and that is
> not a thing to infer from a slug.

## Context — gather this yourself, and report what fails

Run these before Phase 1. **A read that fails is a finding, not an empty
result**, and this section is instructions rather than pre-expanded output
precisely so that you get an exit code where the harness used to get a string.
day#100 is the alternative: a telos read matched nothing, a fallback printed
`none`, and every review in this repo measured against the wrong north star at
exit zero for as long as nobody checked.

The stakes are specific here. "This telos declares no witnesses" is the trigger
for the whole interview, and a failed read produces exactly that appearance — so
an unreported failure does not merely lose information, it manufactures the
condition this skill acts on.

- **Repo root** — `git rev-parse --show-toplevel`.
  **If this read fails:** you are not in a git repo. Say so; a witness that
  probes paths or tags has nothing to bind to.
- **Declared teloi** — `kan show --all --json`, filtered to subjects beginning
  `telos/`. Use the bulk verb: `kan show <subject>` is O(n²) in commit-anchored
  claims and was measured at 141 s where `--all --json` takes 72 ms (kan#181).
  **If this read fails:** stop and say so. Do **not** proceed as though the
  project has no teloi, and do not offer to declare one — you would be
  interviewing about a telos set you never read.
- **Witness probes declared** — the `schema/witness` subject in that same
  bulk read.
  **If this read fails:** say so. An *absent* `schema/witness` is a real and
  common state (the project has declared no probes yet) and Phase 3 handles it;
  an unreadable one is not, and must not be reported as the same thing.
- **day process state** — `day doctor`.
  **If this read fails:** day is not on PATH or cannot reach kan. Phases 4 and 5
  both invoke `day`, so say plainly that the interview can be conducted and its
  result cannot be declared or confirmed.

## Your task

You are conducting an interview, not filling in a field. The output is a witness
set the human stands behind, plus the reason each witness can fail.

### Arguments

- A telos slug: `/witness-interview v1.0`
- No argument: ask which telos, listing those that declare no witnesses.

### Phase 1: Read before you ask

1. `kan show telos/<slug>` — the statement, its tensions, and anything already
   recorded. A telos usually has prose around it that a witness should match.
2. `kan show schema/witness` — what probes this project already has. A witness
   type with no probe behind it is not checkable, and day#125 lost real time to
   declaring four of them without noticing.
3. Read the repo's orientation files for what the telos is *for*. A witness that
   does not match the stated purpose is a witness for a different telos.

### Phase 2: The five questions

Ask these in order, grounded in what you just read. **Wait for answers.** Do not
propose a witness set before asking — the point of the pass is that the human
supplies what a probe cannot infer.

1. **Falsifiability.** *What would you see if this telos held, that you would
   not see if it did not?* This is the question that does the work. An answer
   that names something you would see either way is not a witness.

2. **Type, not instance.** *Is that an artifact type, or one particular
   artifact?* A telos is held up to weak equivalence: many concrete artifacts
   satisfy it equally. A witness naming one instance collapses the telos onto
   it, which is the thing witnesses exist to prevent.

3. **Can it fail?** *Does that artifact already exist, right now?* If it does
   and always will, the witness reports met forever and is worse than none. Note
   that cycle-scoping often rescues this: `.design/*.md` always exists, but a
   design doc created since the boundary does not.

4. **Who produces it?** *Could you satisfy this witness yourself, without the
   telos being true?* A telos about adoption, review, or anyone else's judgement
   is satisfiable by its own author unless the witness excludes them.

5. **The failure condition.** *Name the state in which this reports absent.*
   Record it with the declaration. A witness whose author cannot describe how it
   fails has not been established, whatever the interview concluded.

### Phase 3: Check the answers against the probes

For each witness type the human named, say plainly which it is:

- **A probe exists** — name it, and say what it would match here.
- **A probe does not exist yet** — say so, and what kind it would be. The
  declaration is still valid; it is just not checkable until the probe lands.
- **No probe can express it** — say that too, and why. This is a real outcome,
  not a failure of the interview. day#125's guest-tree telos is satisfied by the
  *absence* of files and no existence check can express it; reporting that
  honestly is better than a witness that looks checkable and is not.

Do not quietly substitute a checkable witness for the one the human named. If
the honest witness is unprobeable, the finding is that day cannot check this
telos yet — which is information, and is what gets filed.

### Phase 4: Declare

```bash
day telos declare <slug> "<the statement, unchanged unless the interview changed it>" \
  --witness <type> [--witness <type> ...]
```

Declaring again revises: kan is append-only and the new claim cites the prior
one, so nothing is rewritten and nothing is lost.

Then record the failure conditions, which the declaration itself has no field
for:

```bash
kan observe --subject telos/<slug> "Witness rationale: <type> reports absent when <condition>. ..."
```

### Phase 5: Confirm it changed something

```bash
day assess telos <slug>
```

Report its output verbatim. If it still reports that the telos declares no
witnesses, the declaration did not land and saying so is the whole job — a
green-looking interview that changed nothing is the defect this atom exists to
prevent, one level up.

### Rules

- Do NOT propose a witness set before asking the five questions. An interview
  where the interviewer answers is not an interview.
- Do NOT declare a witness the human did not name or agree to.
- Do NOT treat "this telos has no mechanically-probeable witness" as a failure
  to be worked around. Record it and move on; the alternative is a witness that
  cannot fail.
- Do NOT run probes against a real log to test your work. A probe leaves claims;
  use a scratch repo or retract in the same breath.
