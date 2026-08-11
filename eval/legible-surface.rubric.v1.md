# legible-surface eval — rubric v1

**Preregistered.** This file is the "specified version of the eval rubric"
that the `legible-surface-eval` witness anchors on: an attestation claim
(`kan result` on `eval/legible-surface`) counts only when it starts with
`legible-surface eval PASSED (rubric v1)`, and this file is what "rubric v1"
means. It is committed before any run it scores, so the band cannot be
fitted to the result afterwards — the same preregistration discipline
`telos/cross-harness-attained` uses, recorded in the witness interview on
`telos/legible-surface-attained`.

**Revised before any run, and that is why it is still v1.** A cold review
found the first draft's fixture degenerate on five of these eight questions
— it declared no atoms, had no upstream, was always the main checkout and
rendered no tray, so a footer that could not have got them wrong scored five
free points against a 7-of-8 bar. The fixture and this rubric were corrected
**before any evaluation was run and with no attestation in existence**, so
nothing was fitted to a result: there is no result. Preregistration forbids
moving the band after seeing the outcome, not fixing a band that cannot be
missed. Had a pass been attested against the first draft, this would be v2
instead — that is what the version inside the anchored marker is for.

## What is being evidenced

`telos/legible-surface-attained`: a person running a session sees where the
process sits — position, signing identity, repo/branch/sync context — from
the harness surface alone, without invoking day.

## Setup (standardized)

- A tmux pane, `80×24`, `LANG=en_US.UTF-8`, `TERM=xterm-256color`, no
  `NO_COLOR`, no `DAY_FOOTER` override — the detection path, not a forced
  style.
- **The same environment on the `day hook session-start` invocation**, which
  is where the style is decided and cached. Setting it only on the pane
  controlled nothing: a `TERM=dumb` ambient shell produced a plain capture
  inside a pane declaring UTF-8, and the harness could not tell — so an
  artifact attested as "the detection path" might have been a forced one.
  The pane keeps the same values because the *width* is read there.
- A fixture repository in a **known and adversarial state** (the ground
  truth), prepared by `scripts/capture-footer.sh`: a real git repo with a
  named branch, an upstream it is **3 ahead and 1 behind** (asymmetric and
  non-zero, so a swapped mark is visible), an uncommitted file so the tree is
  genuinely dirty, tooling workspaces gitignored the way a real repo has
  them, a kan log declaring a witness schema and **two atoms** so the
  position is genuinely ambiguous, and a declared role whose DID is active.
  The script **exits 2 rather than capturing** if any of those degenerate —
  no ambiguity, a clean tree, a field it could not derive.
- `day hook session-start` runs in the fixture (this is where the footer is
  computed — REQ-10), then the footer is rendered into the pane and captured
  with `tmux capture-pane`. The script waits for the pane to render and exits
  2 if it never does, rather than writing an empty capture at exit 0.

v1 captures **the footer surface itself** rendered in the pane. It does not
capture the surrounding harness chrome; a rubric version that drives the full
harness supersedes this one, and because the rubric version sits inside the
witness's anchored marker, adopting it re-opens the witness rather than
inheriting v1's passes.

## Scoring

The scorer is an agent given **only the capture** — not day's documentation,
not this repository, not the fixture. It answers the operator questions:

1. Where does the process sit (which atom, or that none/several are in play)?
2. Is that position singular or ambiguous?
3. What repository is this, and on what branch?
4. Is the working tree clean or dirty? Ahead of or behind its upstream?
5. Is this the main checkout or a worktree?
6. What identity is the session signing as, if any is shown?
7. Are there outstanding warnings, and of what kind?
8. Has anything been withheld from the view?

Each answer is marked against the fixture's ground truth
(`ground-truth.json`, written by the capture script from the same state that
produced the footer). **Every field is derived from the substrate**, and a
field the script cannot derive exits 2 rather than being written by hand —
six of the first draft's nine were literals in a file whose own header
forbade exactly that, including a repo name hand-copied from a URL set
thirty-seven lines above it. The position is derived from the **kan log**,
never from day's reading of it: scoring day against day's own answer would
make the eval circular. Q7's `warnings` field exists so "none" is a markable
answer rather than a question with nothing to mark against.

## Pass condition (preregistered)

- Question 1 **must** be correct — a footer whose position cannot be read is
  the failure the telos names, whatever else scores.
- At least **7 of 8** correct in total.
- The capture must come from the standardized setup above; a pass under a
  forced style or a different geometry is a different claim and must not be
  attested as this one.

## Attesting a pass

A pass is recorded from a session (the log is not writable from CI):

```
kan result eval/legible-surface \
  "legible-surface eval PASSED (rubric v1): <capture ref, scorer, 8-question tally>" \
  --cites <CID of the claim holding or pointing at the capture and answers>
```

A failed run is recorded on the same subject **without** the anchored marker
— `day assess telos legible-surface-attained` then still reports absent,
which is the falsifiability the witness interview established.
