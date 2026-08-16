# Day's Process Model: A Category-Theoretic Sketch

**Status:** Exploratory pre-RFC mathematical target. This is an approximate
pseudo-formal denotational semantics, not an adopted specification. RFC 1
should distinguish this ideal from the smaller operational calculus day
implements today.

## 1. Frames as dependent contexts

Let $\mathcal F$ be a category of frames.

- An object $f \in \mathcal F$ is a frame.
- A morphism $u:g\to f$ says that evidence and judgments available in $f$ can
  be reinterpreted in $g$.
- Each frame has its own internal logic, evidence vocabulary, admissible
  procedures, and standards of witness-bearing.

Rather than one global category of teloi or assessments, categories vary over
frames:

$$
\mathcal C : \mathcal F^{op}\to\mathbf{Cat}
$$

or, more structurally, form a fibration:

$$
p:\int_{\mathcal F}\mathcal C\longrightarrow\mathcal F.
$$

The fiber $\mathcal C_f$ contains the things meaningful in frame $f$. A frame
map $u:g\to f$ induces reindexing:

$$
u^*:\mathcal C_f\to\mathcal C_g.
$$

This is the dependent-type reading:

```text
f : Frame
T : Telos(f)
W : Witness(f, T)
A : Assessment(f)
```

A telos is not merely tagged with a frame. Its type depends on that frame.
Transporting it requires an explicit substitution or reindexing operation.

The present implementation effectively has one implicit context,
$\Gamma_{\mathrm{local}}:\mathrm{Frame}$, and silently evaluates everything in
its fiber. RFC 1 should make that approximation explicit.

## 2. World states and teloi

Let $\mathcal S_f$ be the category of world states legible in frame $f$. Its
morphisms are frame-recognized evolutions or refinements of state.

A telos is not a target state. It is a frame-relative predicate on states:

$$
T_f:\mathcal S_f^{op}\to\mathcal V_f,
$$

where $\mathcal V_f$ is the frame's category of truth values, evidence spaces,
or modes of support.

The weak-equivalence claim becomes:

$$
s\simeq_f s'\implies T_f(s)\simeq T_f(s').
$$

Thus a telos factors through a localization:

$$
\mathcal S_f\longrightarrow\mathcal S_f[W_f^{-1}],
$$

where $W_f$ is the frame's chosen class of weak equivalences. “A state of the
world up to weak equivalence” becomes an invariant predicate on the localized
state category.

## 3. Artifacts and evidence

Artifacts remain external:

$$
a\in\mathcal A_{\mathrm{world}}.
$$

Day and kan do not need to contain $a$. They may only know an address or handle
referring to it.

Evidence is a typed, attributable presentation of an artifact within a frame.
A rough dependent form is:

$$
e:\mathrm{Evidence}_f(P,a),
$$

where $f$ is the frame, $P$ is the proposition or evidential role, $a$ is the
artifact being made legible, and $e$ carries provenance, scope, and an address.

This permits:

- one artifact to support multiple evidence claims;
- contradictory evidence claims about one artifact;
- an artifact to exist without being evidence;
- an evidence claim to be validly authored but inadmissible in a frame.

Kan stores the presentation; day interprets its process role.

## 4. Evidence contexts

An assessment rarely uses one evidence claim. Let $\mathcal E_f$ be a category
of frame-local evidence contexts. An object might be a finite,
provenance-preserving diagram:

$$
E=\{e_1,e_2,\ldots,e_n;\text{ relations}\}.
$$

Morphisms preserve evidential structure while allowing refinement,
restriction, or reindexing. A monoidal product combines contexts:

$$
E_1\otimes_f E_2.
$$

The correct monoidal structure is unresolved. It must account for at least
these facts:

- evidence may be referenced more than once;
- repeated reference to one claim does not create an independent observation;
- provenance and dependency must survive combination;
- distinct claims about one artifact remain distinct;
- contradictions must remain legible rather than collapse;
- frame reindexing must interact coherently with combination.

Cartesian, cocartesian, provenance-preserving union, pushout-like gluing, and
resource-sensitive candidates remain to be compared rather than selected by
intuition.

## 5. Assessments

An assessment is a procedure-bearing delineation of evidence in a frame:

$$
A:\mathrm{Assessment}_f(E,p,\sigma),
$$

where $E$ is the selected evidence context, $p$ is the assessment procedure,
$\sigma$ is the scope, and $f$ supplies the internal logic.

An assessment is closer to

$$
(E,\sigma,p,p(E),\text{limitations},\text{provenance})
$$

than to a verdict. Its procedure must be designed to produce something capable
of bearing witness, but the assessment does not certify by itself that it bears
a particular witness.

```text
procedure executed successfully
!= assessment formed honestly
!= assessment bears witness
!= telos holds
```

A durable certificate is a presentation of the assessment:

$$
c:\mathrm{Certificate}_f(A).
$$

The certificate is what kan may record.

## 6. Witnesses as observables of a telos

A witness is not a probe. It is a frame-relative description of an observable
aspect through which a telos can become evidenced.

For a telos $T$, take a diagram:

$$
W_T:J_T\to\mathcal W_f.
$$

Each $W_T(j)$ describes one evidential concept involved in making $T$ legible.
The shape $J_T$ carries relationships such as conjunction-like dependency,
alternatives, sequencing, shared scope, correspondence, and compatibility
constraints.

The telos need not equal the limit of this diagram. More cautiously, the
diagram is an observational decomposition of $T$. It may be:

- sufficient but not necessary;
- necessary but incomplete;
- an approximation at a declared operational profile;
- jointly conservative only under stated assumptions.

An incomplete witness system must not redefine the telos to whatever happens
to be measurable.

## 7. Witness-bearing as an enriched profunctor

Assessments and witnesses need not initially inhabit the same ordinary
category. Their most general relationship is an enriched profunctor:

$$
\mathsf{Bear}_f:
\mathsf{Assess}_f^{op}\otimes\mathsf{Witness}_f
\to\mathcal V_f.
$$

For an assessment $A$ and witness $W$,

$$
\mathsf{Bear}_f(A,W)\in\mathcal V_f
$$

is the structured space of ways $A$ bears $W$.

This becomes an actual enriched hom when representability is established,
perhaps by embedding assessments and witnesses into one enriched category:

$$
\mathsf{Bear}_f(A,W)
\cong
\underline{\mathrm{Hom}}_{\mathcal X_f}(A,W).
$$

A useful trajectory is:

1. Today: a finite verdict enum.
2. Near term: a structured witness-bearing relation.
3. Later: an enriched profunctor.
4. Stronger result: a representable enriched hom-object.

A probe is an operational procedure

$$
q_{f,A,W}\leadsto\mathsf{Bear}_f(A,W)
$$

that computes, samples, or approximates that object. The probe is not the
relationship itself. Current outputs such as `MATERIAL`, `MISSING`, `VACUOUS`,
`ERROR`, `NOT RUN`, and `TIMEOUT` are coarse observations of
$\mathsf{Bear}_f(A,W)$.

## 8. Where Day convolution may enter

For each frame, suppose witness semantics are presheaves over evidence
contexts:

$$
F:\mathcal E_f^{op}\to\mathcal V_f.
$$

Here $F(E)$ is the structured space of ways evidence context $E$ can bear a
witness. Given an appropriate monoidal structure on evidence contexts, Day
convolution combines witness semantics:

$$
(F\star G)(E)
=
\int^{E_1,E_2}
\mathcal E_f(E,E_1\otimes E_2)
\otimes
F(E_1)
\otimes
G(E_2).
$$

Informally: to determine how a combined evidence context bears the composite
witness, consider all coherent decompositions into contexts bearing the
component witnesses.

This may capture what a flat “all witnesses pass” implementation cannot:

- evidence may jointly bear witness without partitioning uniquely;
- one item may participate in several assessments;
- independent witness components may combine monoidally;
- the result can retain the space of witness-bearing relationships rather than
  collapse immediately to a Boolean.

Two independent probes can both pass while their evidence fails to compose
coherently. If two witness components require the same candidate identity, a
trial from one commit and a tag from another provide no coherent gluing map.

The placement of Day convolution remains a hypothesis pending a more precise
choice of $\mathcal E_f$, $\mathcal V_f$, and $\otimes_f$.

## 9. Atoms and bridges

For artifact-context types $X$ and $Y$, an atom is unlikely to be an ordinary
function $X\to Y$. Execution may be nondeterministic, interactive, multi-actor,
or fail to produce an output. A better target may be an enriched profunctor:

$$
A:X\nrightarrow Y.
$$

Sequential composition is coend composition:

$$
(B\circ A)(x,z)
=
\int^y A(x,y)\otimes B(y,z).
$$

Parallel composition uses a monoidal product:

$$
A\otimes B:
X_1\otimes X_2\nrightarrow Y_1\otimes Y_2.
$$

A bridge is then a string diagram in a symmetric monoidal bicategory of these
profunctor-like processes.

This separates two related coend constructions:

- **Profunctor composition** describes how process possibilities compose
  through intermediate artifact contexts.
- **Day convolution** describes how witness-bearing semantics compose over
  combined evidence contexts.

Whether enriched profunctors are the correct final model for atoms remains a
target to test, not a settled choice.

## 10. Telos assessment as coherence

Given a telos witness diagram $W_T$ and a frame-local assessment context $A$,
assessing the telos should construct

$$
\mathsf{Bear}_f(A,W_T)
$$

with coherence across the diagram. The operational result is not merely

$$
\bigwedge_j \mathrm{probe}(A,W_T(j)).
$$

It must preserve shared coordinates and commutative constraints. If two
witnesses refer to a candidate SHA, their maps into the shared candidate object
must agree.

The mismatched-trial-and-tag bug class is therefore a noncommuting diagram, not
merely a missing Boolean check.

## 11. A plausible formal stack

| Role | Candidate structure |
| --- | --- |
| Frames | Base category $\mathcal F$ |
| World states | Localized category $\mathcal S_f[W_f^{-1}]$ |
| Values / enrichment | Frame-local category $\mathcal V_f$ |
| Evidence contexts | Symmetric monoidal category $(\mathcal E_f,\otimes_f,I_f)$ |
| Assessments | Category $\mathsf{Assess}_f$ |
| Witnesses | Category $\mathsf{Witness}_f$ |
| Witness-bearing | Enriched profunctor $\mathsf{Bear}_f$ |
| Telos decomposition | Witness diagram $J_T\to\mathsf{Witness}_f$ |
| Atoms | Enriched process profunctors $X\nrightarrow Y$ |
| Bridges | String diagrams composed from atoms |
| Witness composition | Candidate Day convolution on $[\mathcal E_f^{op},\mathcal V_f]$ |

## Open formal questions

1. What are the objects and morphisms of $\mathcal E_f$?
2. Is $\otimes_f$ cartesian, cocartesian, resource-sensitive, or obtained by a
   gluing construction over shared provenance?
3. What independence relation, if any, must evidence carry?
4. What is the enrichment category $\mathcal V_f$?
5. Is witness-bearing merely a profunctor, or representable as an enriched
   hom-object?
6. In what precise sense does a witness diagram decompose rather than redefine
   a telos?
7. Which coherence laws must reindexing along frame morphisms preserve?
8. Are atoms best modeled as enriched profunctors, polynomial functors, or
   another class of open systems?
9. Where exactly does Day convolution enter once the preceding structures are
   fixed?

## 12. Frame migration

The candidate structure is indexed or fibred over frames:

$$
\pi:\mathbb D\to\mathcal F,
$$

where each fiber $\mathbb D_f$ contains the state and artifact contexts,
evidence diagrams, assessments, witness semantics, telos predicates, atoms,
bridges, and witness-bearing cells meaningful in frame $f$.

A frame morphism $u:g\to f$ induces dependent reindexing:

$$
u^*:\mathbb D_f\to\mathbb D_g.
$$

Migration transports the whole dependent construction, not a rendered verdict:

$$
T_f\mapsto u^*T_f,
\qquad
E_f\mapsto u^*E_f,
\qquad
A_f\mapsto u^*A_f,
\qquad
W_f\mapsto u^*W_f.
$$

When they exist, adjoints

$$
\Sigma_u\dashv u^*\dashv\Pi_u
$$

distinguish existential forgetting, interpretation by substitution, and
universal transport. A migrated object may therefore retain its evidence while
losing an admissible procedure, a scope distinction, an identity judgment, or
a witness interpretation. Migration is not equivalence unless the relevant
comparison maps are invertible.

Reindexing should preserve evidence composition where the frame morphism
permits it. In the strongest case it is monoidal:

$$
u^*(E_1\otimes_f E_2)
\cong
u^*E_1\otimes_g u^*E_2
$$

and commutes with witness convolution:

$$
u^*(F\star_f G)
\cong
u^*F\star_g u^*G.
$$

If these are only lax comparison maps, their direction and information loss
must remain visible. Likewise, gluing along a shared boundary should obey a
Beck--Chevalley-like condition when possible:

$$
u^*(E_1\sqcup_B E_2)
\cong
u^*E_1\sqcup_{u^*B}u^*E_2.
$$

Failure of this comparison may mean that the destination frame forgot,
identified, or ceased to recognize the shared coordinate. It is a migration
limitation, not evidence that the telos failed.

The witness-to-telos soundness map must also be indexed naturally. For

$$
\alpha_T:
\lVert W_T\rVert
\Longrightarrow
\operatorname{Obs}_f(T),
$$

transport should make $u^*(\alpha_T)$ agree with
$\alpha_{u^*T}$ wherever the relevant structures migrate. With no frame
morphism, there is no migration; two assessments may remain incomparable. A
span through an overlap frame may permit comparison without pretending either
frame embeds wholly into the other.

This is the route toward the sheaf-gluing half of realizability: frame-local
assessments are local sections, and a global assessment exists only when their
restrictions agree on overlaps.

## 13. Atoms, bridges, and realizability cells

There are two identity-like constructions that must not be conflated:

- the identity process $1_X:X\rightsquigarrow X$;
- the present predicate $P_0:X\rightsquigarrow I$, describing the currently
  inhabited region of state space.

A target telos is another predicate:

$$
T:Y\rightsquigarrow I.
$$

An atom is a process 1-cell:

$$
A:X\rightsquigarrow Y.
$$

A bridge is a horizontal composite of atoms:

$$
B=A_n\odot\cdots\odot A_2\odot A_1:X_0\rightsquigarrow X_n.
$$

Composing a bridge with the target predicate gives a predicate on starting
states:

$$
B\odot T:X_0\rightsquigarrow I.
$$

A realization or correctness certificate is a 2-cell

$$
\eta:P_0\Rightarrow B\odot T.
$$

Depending on predicate orientation, this is the Hoare-style refinement

$$
P_0\leq\operatorname{wp}_B(T)
$$

or equivalently

$$
\operatorname{sp}_B(P_0)\leq T.
$$

Atoms are therefore generating process 1-cells, bridges are their horizontal
composites, and assessments construct witness-bearing 2-cells establishing
that those processes relate predicates as claimed.

For intermediate teloi

$$
P_0,P_1,\ldots,P_n=T,
$$

each atom may carry a local realizability cell:

$$
\eta_i:P_{i-1}\Rightarrow A_i\odot P_i.
$$

These cells paste vertically:

$$
P_0
\Rightarrow A_1\odot P_1
\Rightarrow A_1\odot A_2\odot P_2
\Rightarrow\cdots
\Rightarrow B\odot T.
$$

This gives precise content to the statement that bridging states are
intermediate teloi. The bridge is the horizontal path of process 1-cells;
realizability is the pasted path of 2-cells relating successive predicates.

The empty bridge is the identity process. A target telos is already attained
when there is a cell

$$
P_0\Rightarrow 1_X\odot T\cong T.
$$

Otherwise bridge search seeks a pair $(B,\eta)$, not merely an atom sequence.
Two identical sequences with different witness-bearing realization cells are
different solutions.

A double category or equipment makes the geometry explicit:

- objects are state or artifact contexts;
- horizontal arrows are atoms and bridges;
- predicate-like proarrows represent present and target teloi;
- squares are assessment-backed, witness-bearing realization cells;
- frame migration reindexes the whole construction.

The current `day-atom` interface is a decategorified approximation:

- `in` approximates a process source boundary;
- `out` approximates a process target boundary;
- `next` approximates possible horizontal composition;
- `done` names witness concepts used to construct a local 2-cell;
- `day bridge check` verifies only that the horizontal string diagram is
  typeable;
- `day assess atom` searches coarsely for evidence of a local square but does
  not preserve the square itself.

Horizontal atom composition, vertical pasting of realization cells, monoidal
parallel composition, witness convolution, and frame reindexing are distinct
operations that must satisfy coherence laws rather than be collapsed into one
workflow graph.

## 14. Consolidated target

The emerging target is an indexed symmetric monoidal equipment:

$$
\mathbb D:\mathcal F^{op}\to\mathbf{Equip}.
$$

Within each frame:

- teloi are weak-equivalence-invariant predicates;
- atoms are open process 1-cells;
- bridges are horizontal composites of atoms;
- intermediate teloi mediate local process-correctness cells;
- evidence claims make external artifacts legible;
- assessments delineate evidence through declared procedures;
- witnesses form diagrams observationally decomposing teloi;
- probes construct or approximate witness-bearing enriched relations;
- realization is a pasted 2-cell from the present predicate through a bridge
  to the target predicate;
- Day or relative convolution composes the evidence semantics supporting those
  cells;
- frame morphisms reindex the entire construction and disclose any loss.

The present implementation is intentionally a finite syntactic approximation:
free-form artifact type names, atom adjacency, witness lists, independent probe
verdicts, one implicit frame, and Boolean-like reachability. RFC 1 should state
both the target and the exact approximation map rather than allowing the
projection to masquerade as the ontology.
