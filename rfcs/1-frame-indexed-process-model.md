# RFC 1: Frame-indexed process model

- Status: Review
- Authors: day maintainers
- Created: 2026-08-15
- Discussion: https://github.com/kan-tools/day/pull/224
- Review-started-at: 2026-08-17T05:20:00Z
- Review-period-ends: 2026-08-20T05:20:00Z
- Review-override: None
- Supersedes: The implicit ontology in `docs/TELOS.md` and concrete vocabulary in `docs/CONVENTIONS.md` where this RFC is explicitly normative
- Superseded-by: None
- Profile-relationship: approximation
- Implementation: Operational profile partially implemented; conformance reconciliation not started

## Summary

Day is a general process layer whose project vocabulary is declared rather than
built into release-specific commands. This RFC defines its central objects and
their relationships, gives an aspirational category-theoretic semantics,
specifies an independently implementable operational profile for the current
tool, and states exactly how that profile loses information from the target.

The semantic target is an indexed symmetric monoidal equipment over frames.
Within a frame, teloi are weak-equivalence-invariant predicates, atoms are open
process 1-cells, bridges are their horizontal composites, and assessments use
evidence to construct or approximate witness-bearing realization 2-cells. The
v1 profile has one implicit local frame, free-form artifact types, declared
atom interfaces, witness lists, and finite probe outcomes. It MUST describe
these as approximations rather than as the ontology itself.

## Motivation

Day's existing vocabulary grew from successful concrete designs: teloi,
atoms, bridges, witnesses, probes, assessments, and packs. The concepts compose
usefully, but their definitions have remained distributed between theory,
conventions, Rust structs, and accumulated design decisions. That ambiguity is
now producing concrete design errors: release policy becomes a proposed core
verb, a witness becomes synonymous with its probe, successful commands become
completion, independent witness checks join evidence from unrelated commits,
and frame-local assessment reads as global truth.

The problem is not missing workflow features. It is that decategorified
projections have begun masquerading as definitions. A denotational target makes
the forgotten information explicit and guides incremental operational choices
without requiring day to become a theorem prover.

## Terminology

- **Frame:** A dependent context supplying an internal logic, admissible
  evidence, procedures, equivalences, and witness interpretations.
- **Artifact:** A thing in the world. It need not be stored in kan or day.
- **Evidence:** A typed, attributable claim that makes an artifact legible in a
  frame for an evidential role.
- **Evidence context:** A finite frame-local diagram of evidence claims,
  artifact coordinates, provenance, dependencies, and relations.
- **Assessment:** A frame-local, scoped delineation of evidence formed by a
  declared procedure designed to be witness-bearing.
- **Certificate:** A durable attributable presentation of an assessment,
  including procedure, scope, inputs, outcome, provenance, and limitations.
- **Witness:** A frame-relative description of an observable concept through
  which a telos or process-completion proposition may become evidenced.
- **Witness system:** A diagram of witness concepts and their correspondence,
  alternative, dependency, and coherence relationships.
- **Probe:** An operational procedure that constructs, samples, or approximates
  how assessments bear a witness. A probe is not the witness.
- **Telos:** A frame-dependent predicate on possible world states, invariant
  under the frame's declared weak equivalences.
- **Atom:** A composable open process relating input and output contexts.
- **Bridge:** A diagram composed from atoms toward a telos.
- **Vocabulary:** The live, project-scoped declarations of process concepts and
  their operational encodings.
- **Pack:** A transportable manifest of vocabulary declarations. A pack is not
  their authority and does not itself execute the declared process.
- **Present predicate:** The frame-local predicate describing the currently
  inhabited region of state space.
- **Identity process:** The process that leaves a context unchanged. It is not
  synonymous with the present predicate.
- **Realizability cell:** A witness-bearing 2-cell establishing that a process
  relates a source predicate to a target predicate as claimed.

## Denotational target

This section is an explicit trajectory. It constrains the direction and the
distinctions implementations MUST preserve, but notation here is not a claim
that current Rust types implement the named categorical structures. The
exploratory derivation lives in
`docs/day-process-model-category-sketch.md`.

The standalone companion
[`rfcs/1/denotational-semantics.md`](1/denotational-semantics.md) is
incorporated into this denotational target. It gives the coherent mathematical
reading organized around the cell $P_0\Rightarrow T\odot B$. This RFC remains
authoritative where the companion, the exploratory sketch, lifecycle rules, or
the operational profile conflict.

### Frames and dependent structure

Let $\mathcal F$ be a category of frames. The long-term target is an
equipment-valued pseudofunctor:

$$
\mathbb D:\mathcal F^{op}\to\mathbf{Equip}.
$$

This is not claimed to be equivalent to an ordinary Grothendieck fibration:
the exact double-categorical Grothendieck construction is unresolved. Its
category-valued shadows MUST behave fibrationally. In particular, a frame
morphism $u:g\to f$ induces typed reindexing functors such as
$u^*:\mathcal S_f\to\mathcal S_g$ and corresponding reindexing of evidence,
assessments, witnesses, processes, and realization cells. When adjoints exist,

$$
\Sigma_u\dashv u^*\dashv\Pi_u
$$

distinguishes existential forgetting, dependent substitution, and universal
transport. No frame morphism means no inferred migration.

Reindexing SHOULD carry explicit monoidal comparison maps and preserve witness
assembly and shared-boundary gluing where the frame map admits them. For a
pullback square of frames

$$
\begin{matrix}
g'&\xrightarrow{v'}&g\\
\downarrow{u'}&&\downarrow{u}\\
f'&\xrightarrow{v}&f,
\end{matrix}
$$

the candidate Beck--Chevalley obligation is invertibility of the well-typed
mate $\Sigma_{u'}(v')^*\Rightarrow v^*\Sigma_u$ whenever the displayed
adjoints exist. These comparisons are target obligations, not profile-v1
operations. Invertible comparisons express lossless migration. Lax or absent
comparisons MUST expose loss or incomparability rather than yield an
equivalent-looking verdict.

### States and teloi

For frame $f$, let $\mathcal S_f$ be its category of legible world states and
$W_f$ its selected weak equivalences. A telos is a predicate

$$
T_f:\mathcal S_f^{op}\to\mathcal V_f
$$

that factors through $\mathcal S_f[W_f^{-1}]$. Distinct concrete states can
therefore satisfy one telos without being identical. A telos is not a point
target and is never reduced to the currently measurable witness set.

### Evidence, assessment, and witness-bearing

Artifacts remain external. Evidence is a typed claim
$e:\operatorname{Evidence}_f(P,a)$ presenting artifact $a$ for role $P$.
Evidence contexts form a candidate symmetric monoidal category
$(\mathcal E_f,\otimes_f,I_f)$ whose exact tensor remains unresolved.

An assessment has dependent shape

$$
A:\operatorname{Assessment}_f(E,p,\sigma),
$$

where $E$ is its evidence context, $p$ its procedure, and $\sigma$ its scope.
Witness-bearing is provisionally modeled as an enriched profunctor

$$
\mathsf{Bear}_f:
\mathsf{Assess}_f^{op}\otimes\mathsf{Witness}_f\to\mathcal V_f.
$$

A probe approximates $\mathsf{Bear}_f(A,W)$; it is neither that relation nor a
Boolean definition of $W$.

### Witness systems and teloi

A candidate equipment-level observation semantics chooses an evidence-to-state
proarrow $R_f:\mathcal E_f\nrightarrow\mathcal S_f$ and defines

$$
\operatorname{Obs}_f(T)(E)=
\int^{s\in\mathcal S_f}R_f(E,s)\otimes T(s),
$$

an object of $[\mathcal E_f^{op},\mathcal V_f]$. This choice is explicitly
provisional: it states the types required of a future construction without
selecting the final equipment or tensor.

A telos $T$ has a witness diagram indexed by a category $\mathcal I_T$ of
witness components and their declared relations:

$$
W_T:\mathcal I_T\to[\mathcal E_f^{op},\mathcal V_f]
$$

whose assembly $\lVert W_T\rVert$ retains shared coordinates and coherence.
The relationship to the telos is explicit:

$$
\alpha_T:\lVert W_T\rVert\Longrightarrow\operatorname{Obs}_f(T).
$$

A sufficient witness system has the displayed soundness direction. A necessary
system has the reverse direction. An exact observational presentation has an
equivalence. Missing a sufficient certificate does not refute the telos.

The three relationships are discriminated in the finite poset
$0<1$, regarded as a category with a unique arrow $x\to y$ exactly when
$x\le y$. Taking $\lVert W\rVert=0$ and
$\operatorname{Obs}(T)=1$ is sufficient but not necessary: the forward arrow
exists and the reverse does not. Reversing the values is necessary but not
sufficient. Equal values give identity arrows in both directions and hence an
exact system. Arbitrary nonempty sets are not used for this distinction,
because functions can exist in both directions without expressing the intended
logical order. Profile v1 implements only the sufficient case.

Candidate assembly operations include Day convolution for juxtaposed witness
semantics and relative or promonoidal convolution for components constrained to
glue along a shared boundary. The exact construction depends on the unresolved
evidence tensor.

### Atoms, bridges, and realization

An atom is an open process 1-cell $A:X\rightsquigarrow Y$. A bridge is a
horizontal composite

$$
B=A_n\odot\cdots\odot A_1:X_0\rightsquigarrow X_n.
$$

Let $P_0:X_0\rightsquigarrow I$ be the present predicate and
$T:X_n\rightsquigarrow I$ the target telos. Composition is written
right-to-left: if $A:X\rightsquigarrow Y$ and $C:Y\rightsquigarrow Z$, then
$C\odot A:X\rightsquigarrow Z$. Realization seeks a pair $(B,\eta)$ with

$$
\eta:P_0\Rightarrow T\odot B.
$$

For intermediate teloi $P_0,P_1,\ldots,P_n=T$, local cells

$$
\eta_i:P_{i-1}\Rightarrow P_i\odot A_i
$$

paste into the global cell. This gives exact meaning to “bridging states are
intermediate teloi.” The empty bridge is the identity process; $T$ is already
attained when $P_0\Rightarrow T\odot 1_X\cong T$.

Parallel process composition is monoidal. Sequential atom composition,
vertical pasting of realization cells, witness convolution, evidence gluing,
and frame reindexing are distinct operations subject to coherence laws.

## Operational profile v1

### Frame

Profile v1 has exactly one implicit local frame: the current repository,
checkout, kan view, acting identity, environment, and invocation scope as day
can read them. Every assessment MUST identify itself as single-frame. Profile
v1 defines no explicit frame identifier, transport, or reconciliation verb.
Serialized repository and commit fields are assessment provenance, not an
equality key for this larger context. Even equal serialized fields do not prove
that two assessments inhabit the same frame.

### Durable representation

Day stores no durable state of its own. Vocabulary, certificates, and process
records are ordinary kan claims using subject and fenced-block conventions.
Kan establishes claim validity, authorship, repository admission, and view
trust. Day MUST NOT collapse those into one authority judgment.

### Teloi

A `telos/<slug>` subject carries the current telos declaration and a
`day-telos` block listing witness names. An unversioned declaration without a
relationship field is a **legacy flat component report**. It does not state a
logical relationship to the telos and cannot produce a telos certificate.

Profile v1 adds one explicit declaration shape. Its assembly and procedure are
fixed before component results exist:

```day-telos
{
  "_version": 3,
  "subject": "telos/releasable",
  "relationship": "sufficient",
  "components": [
    {"name": "candidate", "coordinates": ["candidate"]},
    {"name": "tests", "coordinates": ["candidate"]}
  ],
  "assembly": {
    "kind": "all",
    "shared_coordinates": [
      {"name": "candidate", "components": ["candidate", "tests"]}
    ]
  },
  "procedure_spec": {
    "repository": "REPO", "commit": "OID", "path": "PATH",
    "sha256": "HEX", "version": "VERSION"
  }
}
```

`_version` MUST be `3`; `subject` MUST equal the containing telos subject;
`components` MUST be a nonempty array of distinct project witness names and
their required coordinate names; `assembly.kind` MUST be `all`; and every
shared-coordinate constraint MUST name declared components that require that
coordinate. `relationship` MUST be `sufficient`. `procedure_spec` is an exact
repository artifact address: canonical repository, full commit, path,
lowercase SHA-256 digest, and nonempty schema version. It MUST resolve to those
bytes from a fresh clone. Unknown keys MUST be preserved by rewriting tools.
`necessary` and `exact` are reserved; profile-v1 readers MUST report them as
unsupported rather than guess an algorithm. Failure to obtain a sufficient
certificate never refutes a telos.

### Evidence and assessments

An artifact is addressed by existing kan artifact coordinates or by coordinates
inside a typed claim block. Evidence is an ordinary typed claim referencing the
artifact. A profile-v1 certificate is a Result claim containing exactly one
`day-assessment` JSON block with this required shape (additional keys are
preserved but have no profile-v1 semantics):

```day-assessment
{
  "_version": 1,
  "frame": {"kind": "implicit-local", "repository": "REPO", "commit": "OID"},
  "procedure_spec": {"repository": "REPO", "commit": "OID", "path": "PATH", "sha256": "HEX", "version": "VERSION"},
  "scope": {"subject": "SUBJECT"},
  "evidence": [
    {"cid": "CID", "role": "ROLE", "artifact": {"repository": "REPO", "commit": "OID", "path": "PATH", "sha256": "HEX"}}
  ],
  "witness_system": {"subject": "TELOS", "declaration_sha256": "HEX"},
  "components": [
    {"name": "candidate", "outcome": "material", "evidence_cids": ["CID-A"], "coordinates": {"candidate": "VALUE"}},
    {"name": "tests", "outcome": "material", "evidence_cids": ["CID-B"], "coordinates": {"candidate": "VALUE"}}
  ],
  "outcome": "certified",
  "limitations": []
}
```

Required scalar strings MUST be nonempty. `commit` is the full repository
object ID and `sha256` is lowercase hexadecimal. Certificate bytes contain no
freshness assertion: once stored, a certificate is historical evidence. Only
the operation that executes the declared procedure may produce a current
verdict; reading a Result claim never does. The procedure address MUST exactly
equal the predeclared address and the witness-system subject and digest MUST
bind the declaration bytes used.
`declaration_sha256` is the lowercase SHA-256 of the declaration object encoded
with the JSON Canonicalization Scheme (RFC 8785), so independent readers hash
the same bytes.
Component names MUST be distinct and equal the declared components. Every
component records one closed probe outcome, at least one evidence CID present
in the certificate evidence array, and every coordinate required by its
declaration. Certification enforces the predeclared shared-coordinate
constraints by exact canonical JSON scalar equality. `outcome` is one of
`certified`, `not-certified`, or `uncheckable`. Legacy Results remain readable
but are not profile-v1 certificates. Kan supplies claim attribution and
provenance outside this block; day MUST preserve that distinction.

### Witnesses and probes

`schema/witness` maps project-defined witness names to operational probe
specifications. The name identifies the witness concept; the probe is only its
profile-v1 evaluator. Path, tag, claim, every, absent, and command probes retain
the rules in `docs/CONVENTIONS.md` until a later RFC changes them.

Probe results distinguish material evidence, missing evidence, vacuity,
not-run execution, timeout, and error. Could-not-check outcomes outrank
checked-and-clean. A command requires explicit `--run`, uses argv without a
shell, is bounded, and is unavailable through MCP.

Where witness components share a coordinate, the declaration's assembly entry
MUST name that coordinate and every constrained component. The certificate
supplies the observed value separately for each component. Independent
component success without exact equality cannot certify the assembled witness
system. An empty shared-coordinate array is valid only when the addressed
procedure specification declares the components independent.

### Atoms

An `atom/<slug>` declaration contains:

- `in`: free-form required artifact-type names;
- `out`: free-form produced artifact-type names;
- `next`: possible forward composition edges;
- `revisits`: negative-outcome return edges;
- `done`: witness names used to assess completion.

Type names are project vocabulary. Day compares identity but does not interpret
their domain meaning. `day atom declare` appends declarations and reports
composition findings without rejecting the project vocabulary.

An atom being applicable, an execution returning zero, an output being present,
a completion witness being certified, and a telos being certified are five
different statements. No one implies another without a declared relationship.

### Bridges

A `bridge/<slug>` declares a target telos, initial artifact types, and a diagram
using sequence, concurrency, and alternatives. `day bridge check` checks only
syntactic validity, atom existence, artifact-type availability, and whether
the arrangement can produce the target's declared witness types. It MUST say
that this is typeability inside one frame, not execution or realizability.

Sequence accumulates outputs. Concurrent branches cannot consume each other's
new outputs. An alternative offers downstream only output types common to all
branches. `revisits` is not forward order and contributes nothing to bridge
reachability.

### Vocabulary and packs

Vocabulary is the live project-scoped fold of declarations. Packs transport
those declarations as data and apply them with consent. A pack does not become
the authority for the resulting claims, supply a fourth execution substrate,
or turn atom names into built-in day semantics.

Concrete release, trial, reconstruction, and deployment procedures remain
project vocabulary and repository automation. General CLI projection from atom
execution specifications requires a later RFC.

## Approximation map

| Denotational object | Profile-v1 representation | Preserved | Forgotten |
|---|---|---|---|
| Frame | implicit invocation context | locality disclosure | identity, morphisms, migration, overlaps |
| Telos predicate | prose declaration plus witness names | stable subject, intended invariant | state category and explicit weak equivalences |
| Artifact | external thing addressed by coordinates | repository bytes and digest when supplied | the thing beyond its address |
| Artifact type | free-form type string | exact type-name equality | domain meaning and semantic equivalence |
| Evidence | typed kan claim referencing an artifact | CID, attribution, role, artifact address | truth outside the frame |
| Evidence context | finite set of certificate evidence entries | cited coordinates and roles | general open-diagram structure and tensor |
| Assessment | probe execution plus optional Result | procedure outcome and some scope | full dependent assessment object |
| Certificate | Result claim | durable attribution and cited evidence | canonical realization-cell encoding |
| Witness | project name resolved to a probe | project vocabulary and operational evaluator | witness concept independent of evaluator |
| Witness system | predeclared sufficient components, assembly constraints, and procedure address | components, relationship, exact evidence/coordinate bindings | alternatives and general coherence diagrams |
| Probe | built-in probe specification and bounded run | declared procedure and finite outcome | the witness-bearing relation itself |
| Atom 1-cell | `day-atom` block | input/output names and graph relations | process semantics and execution binding |
| Bridge composite | `day-bridge` expression | typeability and coarse availability | realization cell and execution trace |
| Vocabulary | live project declaration fold | scoped declared names and encodings | global ontology or authority |
| Pack | transport manifest applied with consent | declaration transport | authority and execution substrate |
| Present predicate | implicit current checkout/context | current source scope | explicit predicate object |
| Identity process | empty bridge | absence of process steps | categorical identity laws as data |
| Realizability cell | not represented directly | partial evidence through certificates | pasted correctness argument |
| Monoidal process composition | `&` plus output union | branch distinction | general tensor and interchange laws |
| Alternative | `|` plus downstream intersection | conservative common outputs | coproduct injections and choice evidence |
| Frame migration | prose-only limitation | acknowledgement of absence | reindexing, adjoints, Beck--Chevalley data |

No profile-v1 command may claim more than the corresponding `Preserved` column.

### Operational surface census

| Surface | Approximates | Preserves | Forgets or does not imply |
|---|---|---|---|
| `day-atom.in` / `.out` | open-process boundaries | exact project type names | artifact existence or semantic subtyping |
| `day-atom.next` | possible process composition | permitted forward graph edge | successful execution |
| `day-atom.revisits` | negative process transition | return edge | forward reachability |
| `day-atom.done` | completion witness system | witness names | atom applicability or telos truth |
| `day-bridge` sequence / `&` / `\|` | horizontal, monoidal, and alternative composition | conservative type propagation | realization or execution trace |
| `day-telos` legacy list | observational components | component identities and results | logical sufficiency |
| `day-telos` v3 sufficient declaration | witness-to-telos soundness map | predeclared components, assembly, and procedure | necessity, exactness, or refutation |
| `day-witness` path/tag/claim/every/absent/command | probes of witness-bearing | evaluator specification | witness identity with its evaluator |
| `MATERIAL` / `MISSING` / `VACUOUS` | checked bearing sample | checked component result | telos truth or falsity |
| `ERROR` / `NOT RUN` / `TIMEOUT` | unavailable bearing sample | reason evaluation was unavailable | checked failure |
| `day atom declare` | vocabulary extension | appended declaration and findings | vocabulary authorization |
| `day assess atom` / `day assess telos` | frame-local assessment | component or sufficient-system result | global judgment |
| `day bridge check` | bridge typeability | single-frame syntactic composition | execution or realizability |

## Canonicalization and equivalence

Kan CIDs identify evidence claims and certificates. Repository, commit, path,
and digest identify addressed artifact bytes. Slugs identify declared
vocabulary subjects but do not define semantic equality of their prose.

Profile-v1 artifact types are equal by exact string equality. Witness names and
atom slugs are exact within the project vocabulary. This syntactic equality is
an approximation and MUST NOT be described as equivalence of underlying world
artifacts or processes.

Telos equivalence is frame-relative and declared conceptually through its
invariant; profile v1 does not compute it. Reusing one evidence CID in two
places remains reuse of one claim, not two independent observations.

## Resolution or processing algorithm

### Atom and bridge planning

1. Load the live project vocabulary through kan's public bulk read.
2. Refuse or isolate unreadable declarations without treating them as absent.
3. Check atom graph and type-name composition.
4. Parse the bridge diagram and propagate available type names according to
   sequence, concurrency, and alternative rules.
5. Report whether target witness type names are reachable.
6. State that neither execution nor realizability was established.

### Assessment and certification

1. Identify the implicit frame and assessment scope. Never import a stored
   verdict as the current result; a current result requires a fresh procedure
   execution. Report a repository or commit mismatch as provenance mismatch.
2. Resolve the telos relationship and the exact procedure-spec artifact. For an unversioned legacy list, stop after
   rendering component probe results and label the aggregate `COMPONENT REPORT`.
   Refuse unsupported versions and `necessary` or `exact` relationships.
3. Resolve each witness name to its probe without identifying the two.
4. Report executable probes before authorization; execute commands only under
   explicit bounded authorization.
5. Preserve material, missing, vacuous, unavailable, and error outcomes.
6. For an explicit sufficient system, bind each declared component to its
   closed outcome, evidence CIDs, and required coordinate values. Require every
   component to be material and enforce the predeclared assembly constraints.
7. Render a sufficient coherent assembly as `CERTIFIED`; render checked
   absence, vacuity, or shared-coordinate mismatch as `NOT CERTIFIED`; render
   not-run, timeout, error, unreadable input, or unavailable evaluation as
   `UNCHECKABLE`. Profile v1 has no `REFUTED` outcome.
8. Record a certificate only through an explicit kan Result write carrying its
   required `day-assessment` block. Validate it before publication.

### Frame migration

Profile v1 performs none. Repository and commit disclose provenance but do not
decide frame equality. A reader MUST render every stored assessment as
historical evidence and MUST NOT copy its verdict into a current assessment,
even when those fields match. When they differ from the current checkout, the
reader additionally reports a provenance mismatch. Only a fresh execution of
the addressed procedure can produce a current-frame verdict.

## Authority and trust model

Artifacts do not derive authority from day. Evidence claim validity,
authorship, admission, and view inclusion come from kan and remain separate.
The frame determines which valid and admitted evidence and procedures it
recognizes. A certificate authenticates that its signer made the assessment;
it does not make the assessment globally true.

Project vocabulary is authoritative only inside the project and view that
admits its claims. A pack proposes declarations; applying it creates locally
authored/admitted claims under the adopter's authority. Day's compiled probe
guardrails bound execution but do not endorse a declared command's purpose.

## Security considerations

Separating artifacts, evidence, assessments, witnesses, and certificates
prevents authority laundering: existence is not evidence, authentic evidence
is not admitted evidence, an assessment is not global truth, and a certificate
does not prove the target telos independently of its frame and procedure.

Project-declared commands are untrusted. They MUST remain shell-free, opt-in,
bounded, visible before execution, and unavailable over read-shaped MCP tools.
Pack transport MUST NOT silently broaden execution authority.

Frame migration can cause false confidence by forgetting scope, provenance,
identity distinctions, or unsupported procedures. Noninvertible transport must
remain visibly lossy. Individually successful witness components must not be
joined when required shared coordinates disagree.

## Compatibility

Existing `day-atom`, `day-bridge`, `day-telos`, and `day-witness` blocks remain
readable. A legacy witness list is rendered only as a flat component report;
it is not silently strengthened to sufficient. Existing
`MATERIAL`, `MISSING`, `VACUOUS`, `ERROR`, `NOT RUN`, and `TIMEOUT` renderings
remain input evidence for the new certification vocabulary but MUST not be
silently relabeled as telos truth or falsity.

`docs/CONVENTIONS.md` remains the shipped encoding reference until the RFC is
accepted and implemented. Reconciliation changes must preserve old block bytes
and use versioned additive fields. `docs/TELOS.md` remains explanatory and will
identify this RFC as the normative model once accepted.

RFC 1 acceptance is a prerequisite for further v0.13 feature implementation.
Implementing the full denotational target is not. Release-specific verification
continues as project-declared procedures rather than new core verbs.

## Alternatives considered

- Specify only the current Rust structures: rejected because it would bless the
  information loss causing the present design failures.
- Require the full formal model before shipping: rejected because the model is
  a trajectory and several correct categorical choices remain research
  questions.
- Define witnesses as probes: rejected because one concept may have several
  evaluators and a probe result is not its semantics.
- Define teloi as witness conjunctions: rejected because incomplete
  observability would redefine desired states as whatever day can measure.
- Model atoms as built-in commands: rejected because atom vocabulary is
  project-scoped and many atoms are interactive or human.
- Treat the present predicate as the identity process: rejected because a
  predicate and a process have different types and roles.

## Reference test vectors

Normative machine-checked vectors live in `rfcs/vectors/1-process-model.json`
and are validated by `scripts/check-rfc1-vectors.py`. They cover:

1. One artifact addressed by two distinct evidence claims.
2. One evidence claim reused by two witness components without becoming two
   independent observations.
3. Two independent evidence claims supporting separate components.
4. Two passing components with unequal required candidate coordinates; flat
   conjunction passes and coherent assembly rejects certification.
5. A sufficient witness absent while the telos remains `NOT CERTIFIED`, not
   false.
6. Necessary and exact declarations refused as unsupported by profile v1.
7. Identity and present-predicate boundaries, local intermediate-telos cells,
   their pasted global cell, and a typeable bridge whose designated local cell
   is absent so no realization exists.
8. A correctly typed realization composite and rejection of its reversed form.
9. Lossless frame reindexing, lossy migration, unsupported procedure,
   forgotten gluing coordinate, and incomparable frames.
10. A legacy `day-telos` block rendered as a flat component report rather than
   silently strengthened.
11. The normative v3 declaration and certificate shapes, including exact
    procedure addressing, per-component outcomes, evidence-CID bindings,
    coordinate bindings, predeclared assembly, and derived certification.
12. Stored same-provenance and mismatched-provenance assessments remaining
    historical, contrasted with a fresh assessment that may certify.
13. Sufficient-only, necessary-only, and exact relationships in the finite
    poset $0<1$, with arrow existence derived from order.

Every migration vector names both what transported and what was lost. The
checker derives composition boundaries, pasting compatibility, witness
coherence, certificate outcomes, historical-read treatment, relationship
direction, and migration classification from vector data rather than accepting
precomputed labels.

## Unresolved questions

| Choice | Required law before selection | Discriminating example |
|---|---|---|
| Objects and morphisms of $\mathcal E_f$ | identities/composition preserve artifact coordinates and evidential roles | two claims about one artifact versus one claim reused twice |
| Evidence tensor | associativity/unit plus an explicit diagonal or its absence | independent test runs versus reuse of one run |
| Enrichment $\mathcal V_f$ | complete/cocomplete enough for the stated coends and invariant under selected weak equivalences | Boolean pass/fail versus ordered unavailable/material information |
| Representability of $\mathsf{Bear}_f$ | a natural representing isomorphism, not only pointwise matching | two probes for one witness that agree on current evidence but diverge after extension |
| Witness convolution | associativity, unit, and exact shared-boundary gluing | two material components with equal versus unequal candidate coordinates |
| Equipment presentation | typed companions/conjoints and horizontal/vertical interchange for predicates and processes | local realization squares pasting to the global bridge square |
| Frame morphisms and adjoints | functorial reindexing, monoidal comparison, and the displayed Beck--Chevalley mate where applicable | invertible transport, unsupported procedure, forgotten coordinate, and incomparable frames |
| Epistemic site and telos-relative topology | coverage axioms, reindexing stability, and proof that local equivalence preserves distinctions material to $T$ | a commit-preserving cover versus a topology that unsoundly identifies releases at different commits |
| Realization prestack, descent, and model structure | pseudofunctorial reindexing, effective descent for declared covers, and coherent weak equivalences | local test and review realizations that agree pairwise but fail or satisfy higher overlap coherence |
| Obstruction coefficients and cohomology theory | a named coefficient object and sound obstruction/vanishing theorem for each claimed degree or generalized theory | a missing local realizer versus incompatible gluing versus higher path coherence |
| Effective realization fragment and provability ledger | computable presentations plus explicit soundness, completeness, termination, and `unknown` boundaries for every algorithmic judgment | bounded bridge search that returns `unknown` rather than a false impossibility certificate |

These questions block claims about the full denotational model. They do not
block acceptance of profile v1 once reviewers verify that no profile-v1 rule
depends on selecting among them.

## Deferred questions

- Concrete frame identifiers and cross-frame reconciliation.
- Generic atom execution descriptors and executor registries.
- Dynamic CLI projection from atom specifications.
- Pack transport of execution bindings.
- A canonical kan encoding for realization cells beyond profile-v1 assessment
  certificates.
- Automatic bridge search over bridge-and-certificate pairs.
- Formal mechanization in a proof assistant.

Each deferred implementation area must receive a GitHub issue citing the
accepted RFC and naming the semantic obligation it implements.

## Implementation status

Review. Current day partially implements operational profile v1 but uses older
rendering and flat witness assembly. No claim is made that the denotational
target is implemented. Acceptance precedes the v0.13 implementation restart;
implementation work will be decomposed into issues after review.
