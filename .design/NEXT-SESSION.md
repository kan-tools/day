# Startup prompt — next session

Paste the block below. Everything above the line is context for whoever is
pasting; everything below it is the prompt.

This file is disposable. Replace it at the end of each session; it is not a
second roadmap. `docs/ROADMAP.md` is authoritative for direction, the GitHub
milestones are authoritative for scope, and kan is authoritative for why.

---

Start on **v0.10 — the graph tells the truth**, beginning with **day#113**.

Read first: `CLAUDE.md`, then `docs/ROADMAP.md`'s "The next three, and why in
that order", then `gh issue view 113`. The design decision for day#113 is
already made and recorded in the issue's comments — read them before designing
anything, because the obvious fix was considered and rejected with evidence.

**What day#113 is.** `next` currently carries two different relations: sequence
("review follows build") and feedback ("a review sends you back to fix"). The
off-sequence check treats both as "upstream must precede downstream", which
assumes a DAG. day's own vocabulary is not one, so the check fires through the
entire build phase of every milestone — it is on the board right now.

The decided shape: `next` becomes forward-only and **guaranteed acyclic**;
feedback edges move to a new optional `revisits` field. Additive, so every
existing `day-atom` block stays valid. `doctor` reports a cycle in `next` as a
**finding, not a failure** — an existing project gets told, not broken.

**Do not** "fix" it by excluding cyclic edges from the off-sequence check. That
was proposed, and it is wrong: both directions of the cycle carry findings and
only one is noise. Excluding them silences a real one — *a verdict exists with
no code change*, i.e. a rubber stamp. Reproduced; see the issue comments.

The DAG guarantee is the actual prize. Reachability, topological ordering and
partial-order reporting all go from unavailable to trivial once `next` is
acyclic by construction. The warning is the symptom.

**Then** day#115, #112, #108, #95 — all in the same milestone, all report
honesty on surfaces a person reads.

## How to work here

Follow `CLAUDE.md`. The parts that cost the most time last session, stated
plainly because knowing them in advance is worth more than rediscovering them:

- **Dogfood before trusting a test.** Every real defect in day was found by
  running it. A green suite has never been sufficient here, and last session it
  was actively misleading three times.
- **Mutation and reversion are different tests.** Mutation asks "does any test
  assert this line". Reversion asks "does the test I wrote for this finding fail
  when the finding returns". Before claiming a fix is covered, **revert the fix
  and confirm the named test fails.** A vacuous regression test survived a whole
  review round last session; reverting is what caught it.
- **A fixture must reach the mode the defect lives in.** Three separate tests
  passed against the defects they were named for because their fixtures could
  not produce the failing state. Assert the *premise* — that the fixture
  produces the state — not that the code ran.
- **`scripts/mutate.py` has known gaps** (day#114): it does not check the
  baseline is green, it leaves `target/` built from the mutant, and `cargo test`
  stops at the first failing binary so the reported catcher list is truncated.
  Rebuild after a mutation run before any manual probe.
- **Pass long claim text via `--text "$(cat file)"`,** never inline. Backticks
  in an inline shell argument run as command substitution and silently eat
  words; a decision was recorded mangled that way last session.

## State as of this file

- `v0.9.0-beta.1` shipped, published, artifact-verified. `main` clean.
- Releases go through `scripts/cut-release.sh <tag>` — it records the release
  claim **before** tagging and refuses on anything it cannot verify. Never tag
  by hand.
- Known gap in that script, filed as day#118 and **not** fixed: it does not add
  the migration-expectations row for the version just released, so the next
  release trips over the omission. If you release before day#118 lands, add the
  row by hand: `./scripts/run-migration-cell.sh <path-to-that-binary>` and put
  the outcome in `tests/fixtures/migration-expectations.tsv`.
- day works with kan `0.10.0-beta.1` (measured, all 8 conformance cells), but
  `src/compat.rs` still records `0.9.1` as newest-measured, so day tells anyone
  on a newer kan they are past the tested edge. Conservative, not wrong;
  unscheduled.

## The thing to hold onto

Three consecutive cold reviews last session each found real defects, and the
severity moved from code, to fixes-that-introduced-bugs, to **the tests
themselves**. The code converged; the verification did not. That is day#116, and
it is why v0.11 exists.

The corollary worth carrying into v0.10: when you fix something here, the
likeliest remaining defect is not in the fix — it is in the thing you wrote to
prove the fix works.

Run `/adversarial-review` before the PR, and dispatch it to a **cold** agent
(`subagent_type: "general-purpose"`, never a fork). In-session review has a
false premise — you wrote the code — and day#100's Step 0 says so. Three rounds
of cold review last session found things the author could not.
