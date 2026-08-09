# Feature: day's consumer contract for kan's v0.8 read surface

## Summary

kan's v0.8 milestone makes the `PeerContested` trust base reachable from a read
surface (its REQ-3) — the substrate day's Frames folds under, and the half of a
frame kan owns. That milestone is specced and ready to build; its architecture
deliberately leaves the shape open ("a way to name several authors"). day is the
consumer, and this document states what day needs that surface to do, while it
is still cheap to say.

Nothing here asks kan to change its model. Four of the five requirements are
about *which* surface carries the selection and *what the response says about
itself*; the fifth is a read-cost ask (day#71) that is far cheaper now, while
kan is already rewiring reads, than as a later retrofit.

The requirement worth arguing over is REQ-3. day's doctrine is that an
assessment is a certificate valid inside some frame, never an absolute fact — so
a view that does not say which frame produced it is a view day cannot honestly
label. That is not a preference; it is the same defect day just fixed one level
down, at the boundary between the two repos instead of inside one.

## Requirements

- REQ-1: The `PeerContested` selection is reachable from **`kan show --json`**.
  day talks to kan only by executing the binary and parsing `--json` — never by
  linking it — so a selector that exists only on MCP, only on the human-rendered
  output, or only in the library is unreachable from day. kan's v0.8 AC-3 says
  "a CLI/MCP read verb" without naming one; day needs `show --json` to be among
  them.
- REQ-2: Trust selection is a **per-invocation** parameter, not workspace-global
  state that a prior command sets. day resolves several teloi in one sweep, and
  Frames means the same subject is folded under different frames within a single
  day command. A global setting would force day to mutate shared state between
  reads — racy under concurrent sessions, and the sort of durable side-channel
  `telos/no-store-of-its-own` exists to keep day out of.
- REQ-3: A `--json` view **identifies the trust base that produced it**. Given a
  response, day must be able to tell a `Solo` view from a `PeerContested` one,
  and which authors and weights the latter used, by *reading the response* rather
  than by assuming kan honoured what day asked for.
- REQ-4: A trust selector is **never accepted and ignored**. Either it changes
  the view, or the invocation fails — day must never receive an exit-0 `Solo`
  view in reply to a request for a frame. This asks for no work: clap already
  rejects unknown arguments, so every kan predating the feature satisfies it
  today. It is stated so the property is not later traded away for a tolerant
  parameter, which is the single change that would break it.
- REQ-5: A **bulk read** returns the live claims of many subjects — ideally all
  of them — in one invocation (day#71). day's claim probe is not tied to a
  subject, so answering one witness means reading the whole log: today that is
  one `kan show` per subject, and every vocabulary day makes declarable adds
  reads to that path. Per kan's ADR-18 a fold over the claim graph is kan's, so
  the fix belongs upstream rather than as caching in day.

  **The ask is specifically to reduce the invocation *count*, not to make reads
  faster.** Measured on day's own 40-subject, 4.2 MB log: `day status` takes
  2.76 s, of which 1.99 s is 41 `kan` invocations at roughly 40 ms each. That
  40 ms is almost entirely fixed per-process cost — an *empty* log costs 30 ms
  per call, a one-claim subject costs the same as the largest one, and
  `kan identity did`, which reads no log at all, costs the same again. Binary
  startup is ~0 ms, so the cost is everything `Workspace::open`
  (`kan/src/workspace.rs`) does before any read happens. The consequence for kan is worth stating plainly: **no
  optimisation inside the read substitutes for this.** A faster fold or a
  better index leaves day paying 41 setups; only a bulk verb collapses them to
  one.

- REQ-6: A read **discloses when the trust base excluded claims**. A count is
  enough; day does not need the hidden content, and asking for it would be
  asking kan to defeat the trust semantics it is applying. Without this, day
  cannot distinguish a subject holding one claim from a subject holding three of
  which two were filtered — so every report day builds on a kan read inherits a
  completeness day never verified. This is a consumer requirement independent of
  *what the default trust base is*: whatever kan defaults to, day has to be able
  to tell whether the view it received was complete under it.

## Acceptance Criteria

- [ ] AC-1: A `kan show <subject> --json` invocation selecting `PeerContested`
      over two named authors returns a view containing both authors' live
      claims, where the same invocation without the selector returns only the
      active identity's. Asserted in `tests/kan_conformance.rs`, against the
      real binary. (REQ-1)
- [ ] AC-2: Two `show --json` invocations in the same session, naming different
      author sets, each return the view for the set they named — with no
      intervening configuration command, and in either order. (REQ-2)
- [ ] AC-3: The `--json` response distinguishes the two views by content, not
      only by day's memory of what it requested: a `Solo` response and a
      `PeerContested` response over the same subject differ in a field day can
      read, and that field names the authors (and weights) in force. (REQ-3)
- [ ] AC-4: Against whatever kan is installed, passing a trust selector yields
      **either** a differently-trusted view **or** a non-zero exit — never
      exit 0 with the `Solo` view. The forbidden third outcome is the
      assertion, so the test is meaningful against an old kan (which takes the
      second branch) and a new one (the first) without day holding two kan
      versions. (REQ-4)
- [ ] AC-5: One bulk-read invocation returns the live claims of every subject a
      per-subject sweep would return, with the same claim fields, and day's
      `ClaimLog` populated from it agrees claim-for-claim with the per-subject
      path. (REQ-5)
- [ ] AC-6: On a subject holding claims from two authors, a read trusting only
      one reports both the claims it returned **and** that others were excluded.
      **Negative control:** a subject genuinely holding one claim, read under the
      same trust base, reports no exclusion — so the signal distinguishes
      *filtered* from *absent* rather than warning unconditionally. (REQ-6)

## Architecture

**Nothing in this document is day-side work beyond a read.** day's contact with
kan is `src/kan_client.rs`, which executes the binary and parses `--json`;
`src/probe.rs`'s `ClaimLog` is the one place that reads every subject, and it is
the sole consumer of REQ-5. Frames will thread a selected trust base from a
day-side frame declaration down to those calls, and nowhere else in day changes.

**The cost REQ-1 is about lives in `kan/src/workspace.rs`**, not on day's side
of the boundary — the per-invocation setup, not the read. This citation was
removed once and replaced with the symbol name alone, to silence
`referenced path does not exist yet: kan/src/workspace.rs`, which is the
degradation day#84 was filed about: a linter argued a document into being less
precise than it would have been with no check at all. It is restored now that
`schema/design-doc` declares `kan/` external, and this section is the reason the
declaration exists.

**Verification lands in `tests/kan_conformance.rs`.** That file is day's one
deliberate exception to the stub rule: it talks to the real kan binary and skips
when kan is absent, because a stub accepts whatever day sends it and therefore
validates day against day's own idea of kan's CLI. That is not hypothetical —
`docs/CONVENTIONS.md` documented a `kan result` invocation that did not run,
through several releases, and the same file caught kan's subject-argument change
when it landed. Every AC above is written to sit there, so this contract is
checked against kan rather than asserted about it.

**REQ-3 is the one that needs its rationale stated, because it is the one kan's
own spec does not already imply.** day's reports are frame-internal by
construction — day's MCP surface says so to every agent that calls it. If day
folds a subject under a frame, renders "this telos is satisfied", and cannot
tell from the response whether kan actually applied that frame, then day is
labelling a view it did not verify. day already has the shape of this bug on
record twice: a witness probe that could not tell which claims it had matched
(day#70, fixed in `src/probe.rs`), and a documented kan invocation that never
ran. Both were invisible to a green test suite, because both were about what an
output *meant* rather than what it *said*. REQ-3 is the cheap structural fix —
the response carries its own frame — and it costs kan a field, not a design.

**REQ-5 is a blocker on a designed feature, not performance hygiene, and the
measurement says which kan issue is the right one.** day's roadmap already
designs *situated injection*: off-sequence findings injected to the agent
through `UserPromptSubmit` **only when the state changes**, distinguished there
from the ambient standing rules that failed in day#30 by being specific and
state-triggered. That hook fires per prompt, so it needs position recomputed per
prompt. At 2.0 s of kan reads per recomputation it cannot be built at all; the
git half of the same computation costs 30 ms, so `path`- and `tag`-driven
transitions are affordable today and only the `claim`-probe half is gated. REQ-5
is what ungates it.

The same measurement rules a candidate *out*, which is the more useful half.
kan#25 — "incremental identity/state fold (currently full recompute every
call)" — names this problem almost exactly and is not it: an identity-only
command that folds nothing costs the same 35 ms, and only ~10 ms of a 40 ms
call is proportional to a 4.2 MB log. Reaching for kan#25 to fix day's read
cost would be effort spent on the ~15% that is not the problem.

**REQ-4 is the same defect at the repo boundary, and it asks for a default to be
kept rather than a feature to be built.** day#70 shipped a narrowed witness
shape whose new fields an older day silently drops, so an older binary reads a
*narrowed* probe as a *broad* one and reports evidence that is not there
(day#78). A kan that accepted and ignored a trust selector would reproduce that
one layer up: day asks for a frame, gets solo, cannot tell. clap's default
rejection of unknown arguments already prevents it, which is why this is a
non-regression note rather than a requirement competing for scope.

**REQ-6 comes from a reproduction, not a worry.** Two role identities created
against one empty workspace both append successfully to one subject. Each then
reads that subject and sees **only its own claim**, rendered as
`1 live claim(s)` — identically through the human output, `--json`, and
`status`. Nothing is lost on disk; nothing indicates a second claim exists.
`fold::TrustBase::Solo` filters rather than weights, and every read surface
passes `Solo(my_author)`, so this is the designed behaviour rather than a bug.

kan has already judged this exact failure unacceptable once. Removing
`KAN_AGENT` is described in kan's own workspace source as fixing a shipped
configuration that "made the agent surface and the human surface read disjoint
views of one log — each reporting a complete-looking view, neither mentioning
the other's." That is verbatim what reproduces here; the cause was fixed, the
class was not, and per-role identities reintroduce it deliberately.

**Whether Solo is the right default is explicitly not this document's
question** — see Out of Scope. REQ-6 is the narrower and strictly separable
half: whatever the default, a consumer must be able to tell that the view it
was handed was partial. day needs it to satisfy its own reporting doctrine,
which is that day never asserts a completeness it did not verify.

Worth stating why day does not take, for its own declared blocks, the softer
route REQ-3 lets kan take here — so this contract does not read as one rule for
kan and another for day. An unknown field on a day block *is* a narrowing
predicate: ignoring it widens the match and leaves no trace in the output, so
detection after the fact is impossible and the reader has to reject. A trust
selector is different precisely because REQ-3 puts the answer in the response —
the information survives, so detection can replace rejection. The rule is the
same in both places; only the availability of evidence differs.

## Resolved Questions

- **day does not need enrichment-as-a-signed-record (kan#117's Ask B) *for its
  own reads*, and this contract does not ask for it.** A frame's *name* is
  process vocabulary, which is day's under kan's ADR-18; the fold under it is
  kan's. So a frame can be a day claim on a `frame/<slug>` subject carrying the
  author weights, passed to kan per read. That keeps the ownership split day
  already recorded on the `frames` subject, and it means **day's own reporting
  is unblocked by v0.8 REQ-3 alone**.

  The scope of that claim is worth stating precisely, because it is narrower
  than it first looks. It covers what *day* reports. It does **not** cover what
  a person running `kan` directly sees, and a repo that has declared a
  multi-role frame will still hand that person a complete-looking partial view.
  Closing *that* needs the declared frame to steer the fold, which means the
  fold must read it, which means it must be a kan-typed record — Ask B, and a
  kan verb to write it, since a day fenced block is by construction not
  something kan's fold reads. So Ask B is off day's critical path and on the
  end-state path, and those are different things.

  In the meantime day can close the human-facing half **advisorily and with
  nothing new**: knowing the declared frame, day can report that kan's default
  view of a subject is narrower than the frame the repo declared. That is day
  surfacing drift as data rather than day changing another tool's behaviour,
  which is the posture `telos/affordance-not-enforcement` already requires.
- **Weights, not just a set of authors.** kan's `PeerContested` is defined over
  per-author weights in [0,1], and an author with no entry is invisible rather
  than merely down-weighted. day's frames are the research loop's role hierarchy
  ("verdict claims authoritative only from the director's key"), which is a
  weighting, not a membership test. A surface that accepted only a set of
  authors would be a narrower thing wearing the same name.
- **Per-invocation rather than a session or workspace setting.** Stated as REQ-2
  rather than left to implementation because the alternative is not merely
  inconvenient for day: cross-frame comparison — folding one subject under two
  frames and reporting that they disagree — is the entire point of frames, and a
  global setting makes it a sequence of mutations rather than two reads.

## Out of Scope

- **How kan spells any of this.** Flag names, whether weights arrive as
  `--trust author=0.5` or a JSON argument, and whether the bulk read is a new
  verb or a flag on an existing one are kan's to choose. This document states
  what day must be able to *do* and *read*, not the syntax.
- **Enrichment as a signed, shareable record** (kan#117 Ask B) — see Resolved
  Questions.
- **Whether `Solo` is the right default, and what a workspace should do when a
  second identity appears.** Raised separately as a kan issue, with the
  reproduction behind REQ-6 and the observation that `Solo` defends against
  foreign claims arriving over sync — a threat kan cannot yet face, since
  consuming another actor's published claims is v0.8's REQ-1 and does not exist
  today. The default, and an init/config flow that makes the trust posture a
  visible choice rather than an inherited silence, are kan's design questions
  and deserve their own thread rather than riding a consumer contract. This
  document asks only that whatever is excluded is *disclosed* (REQ-6).
- **Consuming another actor's published claims** (kan#114, kan's v0.8 REQ-1/2).
  day needs it eventually for genuinely multi-actor frames, but every frame day
  can express today is over role identities in one workspace, which kan's REQ-4
  and REQ-6 already cover.
- **day-side Frames itself.** Its design pass runs after day's v0.7.0-beta.3,
  against whatever kan actually ships, rather than being guessed ahead of it.
