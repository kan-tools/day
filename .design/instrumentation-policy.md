# Policy: Proportionate instrumentation and evidence

## Summary

day uses the least powerful instrument that can falsify the claim being made.
Deterministic product behavior, repository integration, external conformance,
observational trials, and release reconciliation are distinct evidence layers;
none silently inherits the guarantees of a stronger layer. Adversarial
attestation is an explicit security mode with a declared threat model, not the
default interpretation of “verified.”

This policy governs new validation and evidence machinery, including the v0.13
release work currently composed in `xtask/src/release/v013.rs`. It serves
`telos/honest-reads`, `telos/legible-process`,
`telos/affordance-not-enforcement`, and `telos/composable-process` by making the
strength and limits of each instrument explicit without turning every review
finding into a stronger release gate.

## Requirements

- REQ-1: Every material claim used by CI, review, or release qualification must
  declare one evidence layer: deterministic behavior, repository integration,
  external conformance, observational trial, or release reconciliation. The
  declaration states what the instrument establishes and what it does not.

- REQ-2: Instrumentation must use the lowest layer capable of falsifying the
  claim. A stronger layer requires a reason the lower layer cannot answer the
  question; “more rigorous” without a distinct falsification condition is not
  a reason.

- REQ-3: Deterministic behavior belongs in Rust unit or integration tests. This
  includes parsing, validation, folds, serialization, provenance invariants,
  command construction, and pure graders over supplied bytes. Tests use
  fixtures and temporary repositories and must not depend on future release or
  mutable network state.

- REQ-4: Repository integration belongs in reusable `xtask` validation or
  evidence modules when it evaluates the real checkout, git history, kan view,
  or commit trailers. These commands preserve the
  `Passed`/`Finding`/`CouldNotCheck` distinction in `xtask/src/outcome.rs` and
  are exercised by deterministic tests.

- REQ-5: External conformance checks exercise a real dependency or service
  only when compatibility with that external surface is itself the claim. They
  pin the relevant version or coordinate, separate unreadable from negative,
  and keep local fixture tests for the code that interprets the result. The
  real-kan exception documented in `CLAUDE.md` and
  `tests/kan_conformance.rs` is the model.

- REQ-6: Observational trials measure model, harness, human, or workflow
  behavior that deterministic tests cannot establish. A trial preregisters its
  scenario and rubric, retains enough raw material for a human to inspect, and
  reports the runner, model/harness, candidate, and known limitations. It does
  not claim resistance to a deceptive participant unless REQ-9 is invoked.

- REQ-7: An observational trial is a release gate only when the release thesis
  explicitly concerns the observed model or harness behavior and the gating
  choice was recorded before implementation. Otherwise it is advisory retained
  evidence or a post-merge witness. In v0.13, deterministic `/askme` and event
  invariants gate the source candidate; the real `/askme` run and reconstruction
  trial are observational evidence, with reconstruction remaining a post-merge
  witness.

- REQ-8: Release reconciliation evaluates actual external coordinates that do
  not exist in an ordinary test: candidate SHA, required workflow conclusions,
  tag, crate, GitHub Release, and release claim. Version-specific release code
  may select those coordinates and compose generic validators, but it must not
  introduce a new parser, evidence protocol, trust model, or authentication
  mechanism.

- REQ-9: Adversarial attestation requires a separately recorded threat model
  naming the adversary, assets, authority, trust root, and security boundary.
  The design must explain why ordinary deterministic, conformance, or
  observational evidence is insufficient. Same-UID or model-workspace
  resistance is out of scope unless explicitly named there.

- REQ-10: A version-specific module under `xtask/src/release/` is composition,
  not a permanent home for mechanisms. It may contain release constants,
  manifests, selected Plan and publication coordinates, and calls into generic
  validators. If a release needs a new schema, parser, lifecycle validator,
  evidence authenticator, or removal-control engine, that mechanism requires a
  version-neutral design and module before the release adapter may use it.

- REQ-11: Cold-review findings are judged against the declared layer and threat
  model. A finding blocks when it reproduces a false product behavior, a false
  evidence claim, or an in-scope integrity failure. A request for guarantees
  belonging to a stronger undeclared layer becomes a named follow-up, not a
  blocker. Downgrading an overstated claim is as valid a correction as building
  a stronger instrument.

- REQ-12: Instrumentation must not recurse merely to authenticate itself. When
  a review attacks the producer rather than the claimed behavior, the response
  is to check REQ-9, narrow the claim, or design an explicit trust boundary—not
  to add another receipt, signature, or wrapper by default.

- REQ-13: Workflows and scripts orchestrate environment setup, external tools,
  artifact retention, and calls to checked mechanisms. They do not duplicate
  grading policy already present in Rust, and static tests over their text claim
  only wiring, never model behavior or external success.

- REQ-14: Each new instrument records its expected cost and lifecycle. A
  release-only adapter is frozen after publication except for a demonstrated
  correctness repair; a reusable mechanism has an owner module and ordinary CI
  coverage; retained trial artifacts have an explicit retention location and
  are not silently promoted into durable product state.

## Acceptance Criteria

- [ ] AC-1: (REQ-1, REQ-2) A checked inventory classifies every command under
      `xtask/src/command.rs`, every release workflow, and every v0.13 protocol by
      evidence layer, claimed conclusion, and explicit non-conclusions; the
      inventory has no unclassified entry.

- [ ] AC-2: (REQ-3) Deterministic v0.13 product invariants—including transcript
      refusal, provenance, exact citations, handoff scope, stream completeness,
      practice order, and compaction behavior—are enforced by Rust tests that
      run without GitHub, a model, or a future release coordinate.

- [ ] AC-3: (REQ-4) Repository-wide validators use `Outcome` and a fixture
      forces each of pass, finding, and could-not-check; no unreadable real
      checkout or kan view is normalized to an empty or passing result.

- [ ] AC-4: (REQ-5) Every real external conformance check names its pinned
      external coordinate and has a separate deterministic interpreter test.
      A changed or unreadable dependency produces a distinguishable result.

- [ ] AC-5: (REQ-6, REQ-7) The v0.13 `/askme` and reconstruction workflows label
      their outputs observational, retain raw evidence and declared coordinates,
      state that the participant is not treated as malicious, and are absent
      from source-candidate qualification unless the Plan is revised beforehand
      to make observed harness behavior the release thesis.

- [ ] AC-6: (REQ-8, REQ-10) The v0.13 release adapter contains only typed release
      composition and calls to version-neutral mechanisms. A source scan fails
      if a version adapter defines raw-event lifecycle parsing, shell parsing,
      claim authentication, evidence-bundle schemas, or attack/removal engines.

- [ ] AC-7: (REQ-9) No check claims adversarial producer resistance without a
      cited threat-model design that names adversary, assets, authority, trust
      root, and boundary. With no such design, same-UID receipt or workspace
      substitution attacks are classified outside the instrument’s guarantee.

- [ ] AC-8: (REQ-11) The adversarial-review instructions include a layer/threat
      classification for every material finding and distinguish a reproduced
      in-scope violation from a request for stronger instrumentation. A fixture
      or documented review example exercises both classifications.

- [ ] AC-9: (REQ-11, REQ-12) A review demanding malicious-producer resistance
      from an honestly labelled observational trial yields a follow-up rather
      than a BLOCK; the same attack against an explicit adversarial-attestation
      claim yields a BLOCK.

- [ ] AC-10: (REQ-13) Workflow tests assert wiring, addressed artifact retention,
      and invocation of the owning validator, while behavioral assertions live
      against the validator itself. No workflow-text test claims that a model
      adapted, a release exists, or an external command succeeded.

- [ ] AC-11: (REQ-14) The instrumentation inventory names execution cost,
      trigger (`local`, `CI`, `manual trial`, `post-merge`, or `release`), owner,
      and retirement/freeze rule for every non-test instrument.

- [ ] AC-12: (REQ-7, REQ-8, REQ-10) Applying the policy to the current branch
      produces an explicit v0.13 disposition: deterministic feature gates stay;
      ordinary candidate and publication reconciliation stay; observational
      trials remain retained evidence; and bespoke adversarial-attestation code
      is removed, downgraded, or moved behind a separately approved generic
      design.

- [ ] AC-13: (REQ-1, REQ-11) The v0.13 Plan cites this policy and states the
      evidence layer for each of its acceptance criteria, so a subsequent cold
      review can evaluate the declared threshold without inferring one.

- [ ] AC-14: (REQ-14) `just ci` remains the deterministic source-candidate gate
      and reports its bounded cost; observational and post-publication commands
      remain separately invocable and cannot silently join that profile.

## Architecture

### Evidence layers

The layers classify conclusions, not file extensions:

| Layer | Question | Typical home | Default authority |
| --- | --- | --- | --- |
| Deterministic behavior | Does supplied code/input satisfy a pure or hermetic invariant? | `src/` tests, `tests/` | Source-candidate gate |
| Repository integration | Does this real checkout/history/kan view satisfy a repository invariant? | version-neutral `xtask/src/validate/` or `xtask/src/evidence/` | CI gate when hermetic; otherwise explicit could-not-check |
| External conformance | Does day conform to a pinned real dependency or service? | conformance test plus capability adapter | Gate only for the named dependency claim |
| Observational trial | What did this model, harness, human, or workflow do in this run? | preregistered protocol, runner, retained artifact, human-readable report | Advisory by default; gate only when predeclared release thesis |
| Release reconciliation | Do real release coordinates agree on one candidate? | thin release adapter calling generic validators | Release operation, not ordinary test |

Adversarial attestation is a modifier on a layer, not a sixth default layer. It
adds an explicit threat model and trust boundary. Without that modifier,
instrumentation may assume its runner and evidence producer are honest while
still detecting bugs, omissions, stale coordinates, malformed evidence, and
ordinary corruption.

### Placement

`xtask/src/outcome.rs` remains the shared result algebra.
`xtask/src/capability/` remains the boundary for process, repository, and GitHub
reads. Version-neutral mechanisms live under `xtask/src/validate/` and
`xtask/src/evidence/`, with hostile fixtures under `xtask/tests/` or test-only
modules. `xtask/src/release/<version>.rs` selects a release contract and invokes
those mechanisms.

The existing `xtask/src/release/v013.rs` violates the intended final shape: it
contains reusable raw-event parsing, evidence schemas, origin authentication,
shell parsing, claim validation, and attack controls alongside v0.13
composition. Applying this policy is therefore a refactor and scope correction,
not a declaration that the current placement was already compliant.

### Review threshold

For each material finding, review records:

1. the claim under attack;
2. its declared evidence layer;
3. its declared threat model, or “ordinary honest producer”;
4. a reproduction within or outside those bounds; and
5. BLOCK, correction-by-narrowing, or follow-up.

A finding that shows an instrument asserting more than it checked is always
in-scope under `telos/honest-reads`. The correction need not maximize assurance:
it must make assertion and checked material equal. This is the stopping rule
that prevents an observational rubric from recursively becoming a security
protocol.

Two boundary examples are normative review fixtures. Replacing a model-writable
observational JSONL bundle with another internally consistent bundle is outside
the honest-producer trial guarantee and is recorded as a follow-up if stronger
attestation is desired. The same replacement is a BLOCK when an instrument
explicitly claims adversarial attestation against that producer. Conversely, a
trial labelled observational that emits an authoritative pass verdict violates
its own declared conclusion and is a BLOCK even without a malicious producer.

### Applying the policy to v0.13

The six shipped feature areas remain deterministic source-candidate work. The
claim-addressed Plan and exact candidate/publication reconciliation remain
release operations. The real `/askme` run measures adaptive harness behavior;
the reconstruction run measures whether a fresh session recovered the intended
record. They retain their protocols, raw artifacts, coordinates, and honest
limitations but do not claim to authenticate against a deceptive model sharing
the runner UID.

The v0.13 release design must be revised before implementation resumes. Its
current REQ-10 through REQ-13 and AC-11 through AC-13 mix observational,
adversarial-attestation, and release-reconciliation guarantees. The revision
will split those claims, remove observational runs from deterministic candidate
qualification, and either extract genuinely reusable validators or delete
bespoke authentication machinery that no remaining claim needs.

## Resolved Questions

- RQ-1: The default threat model detects defects, omissions, malformed evidence,
  stale coordinates, and accidental corruption; it does not resist an actively
  deceptive same-UID participant.
- RQ-2: Deterministic product invariants gate every source candidate. Real model
  and reconstruction trials are observational unless observed harness behavior
  was explicitly made the release thesis before implementation.
- RQ-3: Version-specific release code composes generic instrumentation and does
  not define new evidence mechanisms.
- RQ-4: A cold-review finding blocks only when it violates the declared layer or
  threat model, or exposes a claim stronger than the instrument honestly
  supports.
- RQ-5: When a stronger guarantee is out of scope, the review records a concrete
  follow-up rather than demanding recursive authentication in the current
  release.

## Open Questions

None.

## Out of Scope

- Designing a malicious-model or same-UID evidence-attestation system.
- Claiming cryptographic or sandbox isolation from ordinary GitHub Actions,
  Codex, shell, or filesystem artifacts.
- Replacing deterministic tests, revert demonstrations, or the demonstration
  census with observational trials.
- Making observational trials part of the public `day` CLI.
- Selecting the eventual generic architecture for adversarial attestation; that
  requires its own design and threat model if the project chooses to build it.
