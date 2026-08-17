---
name: askme
allowed-tools: Bash(kan *), Bash(day *), Bash(git *), Bash(gh *), Read, Grep, Glob
description: Gather semi-structured human input one question at a time, without recording unless explicitly requested
---

> **A driver affordance, deliberately not a process atom.** `/askme` pauses an
> existing flow so a person can supply context, facts, and choices efficiently.
> It consumes no process artifact and promises none, so inventing a `day-atom`
> interface would make the graph say something untrue (day#193).

## Context — gather only what the topic needs

Before asking, establish the topic from the argument or the user's request.
Read available repository context that can make the first question informed:
relevant files, the current diff, issue state, and `kan show --all --json` when
the topic names recorded work.

- **Repository reads** — use the narrowest relevant `git`, `gh`, `Read`, or
  `Grep` operation. **If this read fails:** say which context is unavailable and
  continue with that gap visible; do not convert a failed read into "nothing is
  recorded".
- **kan context** — use one `kan show --all --json` bulk read and filter it
  locally. **If this read fails:** say the record is unavailable and do not
  infer facts, decisions, or absence from it.

Do not read every open issue merely because one exists. This is a general input
affordance, not the open-subject walker proposed in the original #193.

## Arguments

- `/askme <topic>` — gather input about that topic.
- `/askme` — use the topic in the user's request when it is unambiguous;
  otherwise ask what they want to work through, and wait.

## The interaction contract

Ask exactly **one question at a time**, then wait for the answer. Each next
question responds to what the person actually supplied; do not print a survey
or answer your own questions.

Keep four buckets distinct throughout:

1. **Facts supplied** — assertions the person says are true, with sources when
   they provide them.
2. **Decisions made** — choices the person actually settles.
3. **Unresolved items** — unknowns, disagreements, or choices left open.
4. **Material effect** — what this input changes about the work, if anything.

Use this fixed prompting shape adaptively, skipping a question whose answer is
already known:

1. What outcome or decision would make this conversation useful?
2. What facts or constraints should the work treat as given, and what supports
   them?
3. What remains uncertain or should explicitly not be decided here?
4. Given those answers, what should change in the work now?

The person may say **skip** to omit the current question or **stop** to end the
interview immediately. Never reinterpret silence, skip, stop, or apparent
completion as consent to write a claim.

## Close and optionally record

First present a compact summary under the four buckets above. Separate supplied
facts from your inferences, and identify any repository or claim coordinates
used as basis. Do not include a chat transcript.

Then ask one final question and wait:

> Record this acquired input as an Observation now?

If the answer is not an explicit yes, append nothing and end with the summary.
If it is yes, use `day acquired-input record` with the work subject, topic,
facts, decisions, unresolved items, material effect, and every basis CID.

Human input relayed in this conversation is **reported provider provenance**;
the recording remains authored by the active signer. Use `--provider-claim`
only when a separately signed claim from the provider is visible and is the
actual source. A person's name in prose, a payload field, or the user's request
cannot manufacture first-hand authorship.

Read the new claim back through `kan show --all --json` and confirm its block,
signer, subject, and citations. **If this read fails:** say recording succeeded
but read-back verification is UNCHECKABLE; do not claim that the durable bytes
were verified.

## Boundaries

- No automatic recording, including from hooks, session prompts, completion,
  or model confidence.
- No raw transcript in kan.
- No declarable prompt loader in v0.13; that remains #194.
- No claim that every fact is true merely because it was supplied. Provenance
  and truth are different questions.
- No requirement that a decision be reached. A useful result may consist only
  of facts and explicitly unresolved items.
