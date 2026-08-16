# Feature: Day RFC foundations and process-model specification, correction round 4

## Summary

Establish a reviewed RFC and ADR discipline for day, then use its first
substantive RFC to specify day's central process ontology before further v0.13
implementation. RFC 1 will distinguish an aspirational category-theoretic
denotational model, a precise operational profile for current day, and an
explicit approximation map between them; acceptance of that contract blocks
v0.13 implementation, while implementation of the full formal target does not.

This work serves `telos/composable-process`, `telos/vocabulary-substrate`,
`telos/legible-process`, `telos/honest-reads`, and
`telos/affordance-not-enforcement`.

## Requirements

- REQ-1: Day must adopt an RFC 0 defining when an RFC or ADR is required, their
  distinct truth conditions, statuses, numbering, review and acceptance rules,
  supersession, implementation status, and relationship to `.design/`
  documents and kan claims. Its initial structure follows kan RFC 0 while
  adapting authority, repository, and publication rules to day.

- REQ-2: RFCs must live as individually addressable files under `rfcs/`, use a
  shared template and index, and pass an executable structural validator in
  ordinary CI. ADRs must have their own template and index; this change must
  not manufacture reconstructed ADRs from historical prose that has not been
  audited for faithful migration.

- REQ-3: The merged RFC file is the publicly reviewed normative content. Each
  accepted or implemented RFC must also be published as a kan claim carrying
  an exact committed repository artifact address. The claim supplies durable
  identity, provenance, and graph relations without creating a competing copy
  of the RFC text. Normative RFC bytes must not contain their own claim CID;
  publication discovery belongs in kan's `.claims/` projection or another
  derived index, so publication cannot recursively change its addressed bytes.

- REQ-4: RFC 1 must define the central terms `Frame`, `Artifact`, `Evidence`,
  `Evidence Context`, `Assessment`, `Certificate`, `Witness`, `Probe`, `Telos`,
  `Witness System`, `Atom`, `Bridge`, `Vocabulary`, `Pack`, `Present Predicate`,
  `Identity Process`, and `Realizability Cell`, including their identities,
  relationships, authority, provenance, equivalence rules, and lifecycle.

- REQ-5: RFC 1 must contain three visibly separate layers: an aspirational
  denotational model, a normative operational profile implementable by current
  day, and a mapping that identifies exactly what information the operational
  profile forgets. Operational vocabulary must not masquerade as the ontology.

- REQ-6: The denotational target must specify frames as dependent contexts and
  describe the candidate structure as an indexed symmetric monoidal equipment
  over a category of frames. Teloi are frame-dependent predicates invariant
  under declared weak equivalences; atoms are open process 1-cells; bridges
  are their horizontal composites; and assessment-backed witness-bearing
  realizability arguments are 2-cells.

- REQ-7: RFC 1 must distinguish the identity process from the present
  predicate. Given present predicate $P_0$, bridge $B$, and target telos $T$,
  the target realizability shape is a bridge-and-certificate pair $(B,\eta)$
  with $\eta:P_0\Rightarrow T\odot B$ under the declared right-to-left
  composition convention. Intermediate teloi mediate local cells
  $\eta_i:P_{i-1}\Rightarrow P_i\odot A_i$ that paste into the global argument.

- REQ-8: RFC 1 must define artifacts as things in the world that need not live
  in kan or day; evidence as typed claims making artifacts legible;
  assessments as frame-local, scoped delineations of evidence produced by
  witness-bearing procedures; probes as operational procedures constructing
  or approximating witness-bearing relations; and witnesses as diagrammatic
  observational decompositions of teloi.

- REQ-9: RFC 1 must not silently identify witness systems with teloi. It must
  distinguish sufficient, necessary, and exact witness-to-telos relationships
  and state the soundness map from assembled witness semantics to the
  frame-local observable semantics of a telos. Failure to form a sufficient
  witness certificate must not be rendered as proof that the telos is false.
  In operational profile v1, a legacy witness list is only a flat
  component-evidence report: it does not certify or refute a telos. A versioned
  `sufficient` relationship and certificate schema are specified for new
  writes. `Necessary` and `exact` remain denotational relationships until a
  later profile specifies their refutation algorithms.

- REQ-10: RFC 1 must describe frame migration as dependent reindexing of
  teloi, evidence, assessments, witnesses, atoms, bridges, and realization
  cells. It must state the desired monoidal, gluing, and naturality conditions,
  including an explicit base-change square and mate transformation where
  adjoints are assumed, and distinguish invertible transport
  from lax, lossy, unsupported, or incomparable frame relationships.

- REQ-11: The evidence-context tensor, enrichment category, ordinary versus
  relative Day convolution, representability of witness-bearing, exact
  equipment presentation, and permissible frame morphisms must remain named
  unresolved questions until justified. RFC 1 may specify constraints and
  candidate structures but must not claim these choices are settled merely to
  complete the document.

- REQ-12: RFC 1 must map current day behavior to the operational profile:
  free-form atom `in`/`out` types approximate process boundaries; `next` and
  `revisits` approximate process relations; `done` names witness concepts;
  `day bridge check` checks typeability rather than realizability; and
  `day assess atom` renders a lossy probe result rather than preserving a
  realization cell.

- REQ-13: Concrete executor kinds, dynamic CLI projection, pack transport,
  explicit frame identifiers, cross-frame reconciliation, richer assessment
  certificates, and structured witness diagrams must be scoped into GitHub
  implementation issues derived from RFC 1. They are not silently specified
  by RFC 1's denotational target.

- REQ-14: Acceptance of RFC 1 is a prerequisite for further v0.13 feature
  implementation. Full implementation of the indexed equipment, frame
  migration, or unresolved formal choices is not a v0.13 prerequisite. The
  next v0.13 design correction must cite RFC 1 and express release, trial, and
  reconstruction behavior through project-declared atoms, witnesses, and
  repository procedures rather than new release-specific day verbs.

- REQ-15: The RFC/ADR validator must enforce every guarantee it reports. It
  validates both templates structurally, complete ADR metadata,
  file/title/index number agreement, bidirectional exact index coverage,
  index/file status agreement, an append-only number registry, Accepted-state
  discussion and review metadata, and a structured target/profile relationship
  field. Its self-test mutates each guarantee and observes rejection.

- REQ-16: RFC 1 must ship machine-readable finite reference vectors and an
  executable checker for composition typing, witness-result semantics,
  shared-coordinate mismatch, and frame-migration outcomes. A checklist of
  future examples is not a reference vector.

## Acceptance Criteria

- [ ] AC-1: (REQ-1, REQ-2) `rfcs/0-rfc-and-adr-process.md`, `rfcs/template.md`,
      `rfcs/README.md`, `adrs/template.md`, and `adrs/README.md` contain every
      required lifecycle and metadata section, and the RFC/ADR validator
      rejects a missing section, invalid status, duplicate number, reused
      number, or unindexed record.

- [ ] AC-2: (REQ-1, REQ-2) RFC 0 defines a public review period, a documented
      unanimous override that waives time only, substantive-change restart
      rules, and explicit Accepted, Implemented, Rejected, Withdrawn, and
      Superseded transitions. The validator and contributor documentation use
      the same recognized statuses.

- [ ] AC-3: (REQ-3) A fixture publishes an accepted RFC claim whose artifact
      address resolves in a fresh clone to the exact merged RFC bytes. Changing
      the CID, commit, path, or file bytes fails resolution; the index and RFC
      status remain derived from the merged repository file. No normative RFC
      field contains the claim CID, so republishing cannot change the addressed
      RFC bytes.

- [ ] AC-4: (REQ-4, REQ-8) RFC 1 contains normative definitions for all
      seventeen central terms and a conformance table showing which current kan
      subjects, fenced blocks, files, or runtime structures encode each term.
      `Artifact`, `Evidence`, `Assessment`, `Witness`, and `Probe` are never
      defined as synonyms.

- [ ] AC-5: (REQ-5, REQ-6) RFC 1 has separately reviewable sections named
      `Denotational target`, `Operational profile v1`, and `Approximation map`.
      A structural validator requires all three and rejects language claiming
      that the operational profile implements the full target.

- [ ] AC-6: (REQ-6, REQ-7) RFC 1 includes typed reference diagrams and test
      vectors for the identity process, present predicate, atom 1-cell, bridge
      composite, intermediate-telos cells, and pasted realization cell
      $\eta:P_0\Rightarrow T\odot B$. At least one vector has a typeable bridge
      with no realizability cell.

- [ ] AC-7: (REQ-8, REQ-9) Reference vectors distinguish one artifact with two
      evidence claims, one evidence claim reused by two witness components,
      two independent evidence claims, and two individually passing components
      whose shared coordinate disagrees. Flat conjunction passes the mismatch
      vector while coherent witness assembly rejects it.

- [ ] AC-8: (REQ-9) RFC 1 gives separate denotational examples of sufficient,
      necessary, and exact witness systems. Operational profile v1 specifies
      versioned `sufficient` declarations and distinguishes `certified`, `not
      certified`, and `uncheckable`; legacy lists remain component reports.
      Refutation, necessary, and exact declarations are unavailable in v1
      rather than accepted with unspecified behavior.

- [ ] AC-9: (REQ-10) Frame-migration vectors cover invertible reindexing,
      evidence transport with an unsupported procedure, lossy lax-monoidal
      comparison, successful shared-boundary gluing, failed gluing after a
      coordinate is forgotten, and two incomparable frames. Each outcome
      exposes what transported and what did not.

- [ ] AC-10: (REQ-11) Every unresolved formal choice appears in RFC 1's
      `Unresolved questions` section with its required laws and discriminating
      examples. RFC 1 cannot reach Accepted while a question blocks the
      operational profile, but may defer a question shown not to affect that
      profile.

- [ ] AC-11: (REQ-12) An approximation table maps every current `day-atom`
      field, `day-bridge`, `day-telos`, `day-witness`, probe outcome, and
      assessment command to the denotational structure it approximates, the
      information it preserves, and the information it loses.

- [ ] AC-15: (REQ-15) Hostile validator mutations covering RFC and ADR
      templates, ADR metadata, stale and status-mismatched index rows, title and
      number mismatch, reused historical numbers, invalid Accepted metadata,
      and a target/profile relationship claiming full implementation all fail
      for their intended reasons.

- [ ] AC-16: (REQ-16) The RFC 1 vector checker parses committed vectors and
      rejects a reversed composition boundary, a shared-coordinate mismatch
      labelled certified, a missing sufficient witness labelled refuted, and a
      lossy frame migration labelled equivalent.

- [ ] AC-12: (post-acceptance rollout gate; REQ-13) Every deferred rollout area has a GitHub issue citing
      RFC 1 and naming the relevant semantic obligation. Closing or deferring
      an issue cannot change RFC 1's accepted semantics without a superseding
      RFC or ADR explaining an implementation departure.

- [ ] AC-13: (post-acceptance rollout gate; REQ-14) The v0.13 roadmap and canonical design identify RFC 1
      acceptance as an implementation prerequisite, contain no bespoke
      `day release verify` or release-specific core verb, and assign concrete
      release/trial execution to project-declared vocabulary plus
      repository-owned procedures.

- [ ] AC-14: (review/acceptance gate; REQ-1, REQ-3, REQ-14) RFC 0 and RFC 1 each receive their own
      cold adversarial review and published kan claim. The v0.13 design cites
      the accepted RFC 1 claim and exact committed file rather than an
      unreviewed working sketch.

## Architecture

This correction answers BLOCK verdict
`bafyreickogt7nsnfoqqutdfz5qknfgsdwlyfhwmmp3salyl5ricde2zeci`. The blocked
draft remains append-only history; the corrected design and RFC bytes receive a
new Plan and fresh cold review.

`docs/day-process-model-category-sketch.md` is the exploratory mathematical
source for RFC 1. It remains explicitly non-normative and may change as the
correct categories, enrichment, tensor, and coherence laws become apparent.
`docs/day-process-model-category-sketch.html` is only an initial reading copy;
it is not a second authority.

RFC 0 will adapt the process established by kan's
`rfcs/0-rfc-and-adr-process.md`, `rfcs/template.md`, and RFC validator. Day's
new `rfcs/` and `adrs/` trees will be repository-native review surfaces.
`CONTRIBUTING.md` will define when work moves from `.design/` into an RFC or
ADR. The validator will be invoked from existing CI rather than relying on a
maintainer to remember structural rules.

RFC 1 will be the first substantive proposal. Its denotational target is an
indexed symmetric monoidal equipment
$\mathbb D:\mathcal F^{op}\to\mathbf{Equip}$. This is a trajectory, not a claim
that current Rust types implement an equipment. The operational profile must
be stated using independently implementable rules and finite reference vectors.

`docs/TELOS.md` remains explanatory foundations: motivation, plausible
fiction, weak equivalence, and the longer category-theoretic trajectory.
`docs/CONVENTIONS.md` becomes the concrete kan encoding and CLI projection of
the accepted operational profile. Where either conflicts with accepted RFC 1,
RFC 1 governs and the documentation must be reconciled visibly.

The approximation map will reference the existing implementations in
`src/atoms.rs`, `src/bridge.rs`, `src/probe.rs`, `src/telos.rs`,
`src/blocks.rs`, and `src/position.rs`. It must distinguish typeability,
execution, assessment, witness-bearing, and telos certification rather than
using `done` or an exit code as a synonym for all five.

RFC 1's witness model will describe an assembled witness diagram
$\lVert W_T\rVert$ and an explicit relationship to observable telos semantics,
such as a soundness transformation
$\alpha_T:\lVert W_T\rVert\Rightarrow\operatorname{Obs}_f(T)$. Candidate Day
or relative convolution belongs in the denotational target until the evidence
category and tensor are selected. The operational profile need only preserve
the coordinates and distinctions required by its reference vectors.

GitHub issues will carry implementation rollout after the RFC establishes the
semantic obligations. Packs transport project vocabulary; they do not become
the authority for RFC semantics or silently execute unfamiliar procedures.
Release and behavioral-trial automation remains repository-owned and is
connected to day through general atom and witness contracts.

## Resolved Questions

- RQ-1: Day adopts separate RFC and ADR disciplines modeled on kan RFC 0;
  working designs, forward-looking public contracts, and decisions actually
  taken retain different truth conditions.
- RQ-2: RFC 1 defines the ontology and operational approximation but defers
  concrete executor kinds, dynamic CLI projection, and pack transport.
- RQ-3: Frames are normative dependent contexts in RFC 1; operational profile
  v1 has one implicit local frame and must not present its assessments as
  globally settled truth.
- RQ-4: Artifacts, evidence, assessments, certificates, witnesses, and probes
  are distinct objects with the relationships stated in REQ-8.
- RQ-5: The category-theoretic model is an explicit aspirational target with a
  convergence trajectory, not an implementation claim or a prerequisite that
  the full formalism ship.
- RQ-6: RFC 1 acceptance precedes further v0.13 feature implementation. The
  full denotational target does not.
- RQ-7: Rollout details are scoped through GitHub issues derived from RFC 1;
  they are not baked into the foundational RFC merely because the current
  repository needs them.
- RQ-8: Markdown-with-LaTeX is the canonical exploratory mathematical document;
  rendered HTML is a derivative reading surface.
- RQ-9: The initial draft proposed reading legacy `day-telos` witness lists as
  `sufficient`; correction round 1 rejects that strengthening. Legacy lists
  carry component evidence only unless an explicit versioned relationship is
  present.
- RQ-10: Operational profile v1 supports an explicit versioned `sufficient`
  relationship. Legacy lists remain flat component reports, while `necessary`,
  `exact`, and sound refutation wait for a later profile with defined algorithms.
- RQ-11: Composition is conventional right-to-left. A bridge
  $B=A_n\odot\cdots\odot A_1$ realizes $T$ through
  $\eta:P_0\Rightarrow T\odot B$ and local cells
  $P_{i-1}\Rightarrow P_i\odot A_i$.
- RQ-12: RFC claim discovery is non-normative and external to RFC bytes; no RFC
  embeds the CID of a claim addressing itself.
- RQ-13: Draft readiness, formal review and acceptance, and post-acceptance
  rollout are separate delivery stages. AC-12 through AC-14 are lifecycle
  gates, not claims that a Draft build has already been accepted.
- RQ-14: RQ-10 directly supersedes the earlier RQ-9 decision claim
  `bafyreiaafsq7dnugfdc5qyldwlrylug6ms2moz7rnolsrj5otj5gdhpxqq` on this
  subject. Legacy witness lists are flat component reports and are not
  sufficient systems without an explicit versioned relationship.
- RQ-15: Accepted-state validation is evidence-bearing rather than
  presence-only: review timestamps must demonstrate 72 elapsed hours (or name
  a structured unanimous override), and the exact day GitHub proposal PR must
  be readable and contain the RFC file.
- RQ-16: Review chronology is anchored to GitHub's server-recorded push time
  for the PR head, not author-controlled commit dates. Every finite reference
  vector uses closed vocabularies and validates complete case structure before
  relationship-specific outcome logic.

## Open Questions

None.

## Out of Scope

- Selecting the final evidence-context tensor, enrichment category, or exact
  categorical presentation before the discriminating examples justify it.
- Implementing explicit frame identities, cross-frame reconciliation, or
  sheaf gluing in RFC 0 or RFC 1.
- Implementing generic atom executors, skill dispatch, or pack-carried command
  bindings as part of the RFC process change.
- Rewriting current Rust modules to resemble the denotational notation before
  the operational profile establishes a behavioral need.
- Treating the exploratory Markdown or HTML sketch as an accepted RFC.
- Completing the v0.13 feature implementation before RFC 1 acceptance.
- Creating or closing GitHub rollout issues during this design pass.
