# Feature: position reports that can represent their own failure (day#98, day#103, day#97)

## Summary

Three open findings against position inference, filed weeks apart from three
different causes, are one defect class: **a report that cannot represent its own
failure mode.**

| issue | what position says | what is true |
|---|---|---|
| day#98 | an inputless atom is never named current | it is the next thing to build |
| day#103 | `release` is a candidate | a release was cut and never recorded |
| day#97 | the status line's string | position as of session start, hours stale |

Each was found by using day, none by the suite, and all three are cheap because
the data is already in hand — no new probe, no extra kan read, no new
declaration. What is missing is a *distinction* in the output.

The through-line is a rule day already applies correctly one module over.
`src/compat.rs` keeps `TooOld` and `Newer` as separate variants rather than one
"mismatch", and says why:

> collapsing them into one "mismatch" would make the benign case as loud as the
> real one — which is how a warning gets ignored.

Position collapses exactly that distinction three times. `release` as a
candidate means both "you may release" and "you released and lost the record",
and says one word for both.

day#103 is the sharpest because the omission is self-concealing: the detector of
a skipped `release` atom (`day assess docs`) is *downstream of* the release atom
and is manual-only. Skip the atom and you skip the alarm. The record then looks
complete from the inside indefinitely — it did, for two consecutive releases,
until the verb was run for an unrelated reason.

## Requirements

- REQ-1: An inputless atom whose outputs are some other atom's declared inputs —
  a **convergent root** — remains eligible to be named current. A genuine source,
  whose outputs nobody consumes, stays excluded, preserving what the present rule
  protects. The discriminator is computed from the atom set already loaded in
  `infer_with`; no probe and no read is added.

- REQ-2: An artifact type may declare **two witnesses rather than one**: a
  `material` witness (what exists in the world — a tag, a path, a command) and a
  `record` witness (what the log says about it — a `claim` probe). Their
  disagreement is the finding: **material present + record absent = "done, but
  unrecorded."**

  This generalises day#103 rather than fixing its instance. The release case is
  one artifact type — `published-artifact`, material `{tag: "v*"}`, record
  `{claim: {subject: "release"}}` — but nothing about the rule is
  release-specific. Any atom whose product can be both *seen* and *written down*
  gets the same distinction, and `docs::reconcile_boundary` becomes the special
  case of a general rule rather than the only place the question is asked.

  It is also the model day already has. Witnesses are declared, not hardcoded;
  the `claim` probe (v0.7) already reads the log; `path`/`tag` already read the
  world. What was missing is that a type could only pick one, so "the thing
  exists" and "the thing is recorded" could never be compared.

- REQ-3: The paired form is **backward compatible and refuses what it cannot
  read**. `"published-artifact": {"tag": "v*"}` keeps its present meaning —
  material only, no record claim, no new finding — so every existing
  `day-witness` block is untouched and no project is forced to declare a record
  witness. A malformed pair is reported as an unsupported witness the way a
  malformed probe already is, and does not silently degrade to material-only,
  which is `telos/honest-reads` applied to the new shape.

- REQ-8: The comparison is evaluated **wherever position is computed**, not at a
  call site. Per day#101 — *a guarantee about reads belongs in the mechanism,
  never in a caller* — it lives with the witness evaluation every channel already
  goes through, so no channel can be added later that reports position without it.

- REQ-4: `UserPromptSubmit` **re-renders the status line** when it recomputes the
  position. Today it recomputes and writes the standing, and the line keeps
  serving the session-start render. The fix must not reintroduce the 3.03s
  regression `v0.7.0-beta.2`'s review blocked.

  **AMENDED DURING THE BUILD (day#111), and the amendment is the interesting
  part.** This originally read "gated on the same 0.03s git fingerprint … and
  costs zero additional kan invocations". That gate is *wrong*, not merely
  cheap: `Git::position_fingerprint` reads no kan, but position also depends on
  `claim` probes, so recording a claim moved the position and left the
  fingerprint byte-identical. The bar was stale for a whole session whenever
  work was recorded rather than edited — the dominant workflow in this repo.

  So the gate now hashes the kan log too, and the quiet path costs **two** kan
  invocations, not zero. Measured: 0.03s → 0.16s, against the 1.40s recompute it
  avoids and the 3.03s regression that was blocked. `kan status` was measured and
  rejected as a cheaper signal — it does not change when a claim is appended to
  an existing subject, which is exactly the case that matters. Raised with kan as
  kan#151; if a cheap log-generation signal lands, this returns to roughly its
  original cost.

  Recorded here rather than left to drift, per `docs/ROADMAP.md`'s rule: revise
  by editing and recording the change, never by pretending it always said
  something else.

- REQ-5: Each of the three is covered by a test **in the mode where it fails**,
  not the mode this repo is in. day#98 cannot be tested against day's own
  vocabulary at all — every day atom declares inputs — so it needs a fixture
  vocabulary with a convergent root. day#103 needs a fixture whose tag and
  `release` claim disagree.

- REQ-6: Every new assertion is verified by **reintroducing the defect it covers**
  and confirming the test fails on the exact line the finding was about, not on
  the feature around it.

- REQ-7: `day design check` stops reporting requirement coverage as satisfied when
  **no acceptance criteria were declared** (day#105). Coverage is currently
  computed from *mentions* of requirement ids inside the criteria section, so a
  document with six requirements and zero criteria is told every requirement is
  covered — in the same run that fails the criteria count at zero. With no
  criteria the question is unanswerable, so it reports *unchecked*, not *pass*;
  could-not-check outranks checked-and-clean. Folded into this milestone because
  it is the same defect class, and because `design check` is the gate every
  future design doc passes through, including this one.

## Acceptance Criteria

- [ ] AC-1: (REQ-1) Given a vocabulary with an inputless atom whose output is another
  atom's input, and that output absent, `infer` names it in `current`. Given an
  inputless atom whose outputs no atom declares as an input, `infer` does not.
  Both asserted on a fixture, since day's own seven atoms all declare inputs and
  the repo therefore cannot exercise either branch.

- [ ] AC-2: (REQ-2) Given an artifact type declaring a paired witness whose
  material probe is satisfied and whose record probe is not, position reports it
  as **done-but-unrecorded**, in wording distinct from both "this atom is next"
  and "this atom is finished". Given both satisfied, it reports finished; given
  neither, it reports the atom as still ahead. All three asserted on a fixture,
  because day's own log after the record cleanup satisfies both and can therefore
  only exercise one of the three.

- [ ] AC-3: (REQ-3) An existing single-probe declaration — every `day-witness`
  block written before this change, including the one on this repo's
  `schema/witness` — parses and behaves exactly as before, producing no
  done-but-unrecorded finding. A pair that cannot be read is reported as an
  unsupported witness naming the type, never dropped to material-only.

- [ ] AC-8: (REQ-8) A source-scanned test asserts the paired comparison is
  reached through the shared witness evaluation rather than from one verb, so a
  channel cannot be added that omits it. Modelled on
  `a_failed_kan_read_is_never_swallowed`, including its stated escape hatch,
  because a test with no way out gets deleted the first time it is wrong.

- [ ] AC-4: (REQ-4) After a fingerprint-moving change, `day hook user-prompt` leaves
  `.day/statusline` holding the recomputed position, not the session-start one.
  Asserted as **content changed**, and separately as a **bounded, constant kan
  invocation count** on a quiet prompt — an invocation count measures the design
  where a duration measures the machine. Amended from "zero" with REQ-4: reading
  the log is the only way to know the log moved, so the honest property is that
  the cost does not scale and never reaches the full inference.

- [ ] AC-5: (REQ-5, REQ-6) Each of AC-1, AC-2 and AC-4 is shown to fail when its
  defect is reintroduced, recorded in the milestone's claims with the mutation
  used.

- [ ] AC-6: (REQ-2) The new finding does not fire on a healthy repo. Run against
  day's own log after the record cleanup — where tag and claim now agree — the
  status output gains nothing.

- [ ] AC-7: (REQ-7) `day design check` on a document with requirements and zero
  parseable acceptance criteria emits **no** coverage `[PASS]`. The regression
  asserts the pair, not either line alone: the criteria count was already
  correct, so a test checking only the count would have passed throughout.

## Architecture

- `src/position.rs` — `Standing::is_source` (`:239`) and the `current` filter
  (`:343`). REQ-1 adds the consumed-outputs set inside `infer_with`; `is_source`
  gains the discriminator rather than the filter growing a second condition, so
  the concept stays named.
- `src/telos.rs` — `WitnessSchema` is `transparent` over a map from artifact type
  to `Probe`. REQ-2 widens the *value* to `Witness::Single(Probe)` or
  `Witness::Paired { material, record }`; the map stays transparent and its keys
  stay data, so the reasoning in the existing doc comment about why there is no
  `deny_unknown_fields` here is unchanged.
- `src/probe.rs` — `evaluate` already answers a single probe. The pair is two
  evaluations and a comparison, which belongs beside it rather than in a caller.
- `src/docs.rs` — `reconcile_boundary` is the release-specific instance of the
  general rule. It stays (it also reconciles the *reverse* case, a claim with no
  tag) but stops being the only place the question is asked.
- `src/status.rs` — `status::compute`, where position reaches every channel.
- `src/hooks.rs` — the `user-prompt` path for REQ-4, and `render_teloi` as the
  precedent for a reader that filters rather than reporting raw.
- `src/cache.rs` — the only module permitted to touch `.day/`; REQ-4's re-render
  goes through it, and `tests/plugin.rs` already proves nothing else does.
- `tests/cycle_position.rs` — where AC-1 and AC-2's fixtures belong.
- `tests/plugin.rs` — AC-3's source scan, beside the existing ones.

### Deliberately not in scope

- **Vocabulary packs (day#73).** Named as v0.8 in `docs/ROADMAP.md` and
  deferred: a pack transports a vocabulary, and transporting one whose position
  reporting is wrong three ways exports the defect to exactly the population
  `telos/v1.0`'s bar names.
- **Gating the release claim in CI.** Real, and day#103's second fix bullet, but
  it is a workflow change rather than a position change and wants its own commit.
