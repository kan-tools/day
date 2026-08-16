# Day as an indexed process equipment

Status: normative denotational companion to Draft RFC 1

This document is incorporated by the `Denotational target` section of
[`RFC 1`](../1-frame-indexed-process-model.md). It gives one coherent reading
of that target. RFC 1 remains authoritative for lifecycle, operational-profile,
and conflict-resolution questions. The adjacent HTML file is a derivative
reading copy; this Markdown-with-LaTeX file is the claim-addressed source.

## 1. The thesis

Day is an indexed process bicategory—or, more precisely, a candidate symmetric
monoidal equipment—fibred over frames. Teloi are frame-dependent predicates,
atoms are process 1-cells, bridges are composites of atoms, and realizability is
witnessed by 2-cells relating a present predicate to the predicate induced by a
bridge and a target telos.

The essential picture in a frame $f$ is

$$
\begin{array}{c}
X_0 \overset{\;B\;}{\rightsquigarrow} X_n\\[-2pt]
P_0\Downarrow\eta\qquad\Downarrow T
\end{array}
\qquad
\eta:P_0\Longrightarrow T\odot B.
$$

Here $P_0:X_0\rightsquigarrow I$ describes the presently inhabited region,
$B:X_0\rightsquigarrow X_n$ is a bridge, and
$T:X_n\rightsquigarrow I$ is the target telos. Composition is written
right-to-left, so $T\odot B:X_0\rightsquigarrow I$ is the target predicate
pulled back along the process represented by $B$.

This is the abstraction the rest of the vocabulary exists to make operational.
Day does not identify a command succeeding, an artifact existing, a witness
being observed, an atom completing, and a telos being realized. Those are
different cells or claims in this geometry.

## 2. Frames are the base of dependence

Let $\mathcal F$ be a category of frames. A frame is not merely a bag of
configuration. It is a dependent context fixing:

- which world states and artifacts are legible;
- which evidence claims and procedures are admitted;
- which equivalences are intentionally ignored;
- how witnesses are interpreted;
- which authority and provenance relations count; and
- which process compositions are meaningful.

The long-term structure is an equipment-valued pseudofunctor

$$
\mathbb D:\mathcal F^{op}\longrightarrow\mathbf{Equip}.
$$

For each $f\in\mathcal F$, the fiber $\mathbb D_f$ contains the predicates,
processes, evidence semantics, and realization cells meaningful in that frame.
A frame morphism $u:g\to f$ induces reindexing

$$
u^*:\mathbb D_f\longrightarrow\mathbb D_g.
$$

Calling this structure “fibred” states the design obligation: meaning is local
to a frame, and migration is an explicit typed operation. It does not assert
that an ordinary Grothendieck fibration already captures every horizontal and
vertical component of an equipment. The exact double-categorical
Grothendieck construction remains an open formal choice.

When they exist, adjoints

$$
\Sigma_u\dashv u^*\dashv\Pi_u
$$

separate existential forgetting, substitution, and universal transport. If no
frame morphism is known, day may not silently copy a verdict between frames.

## 3. The process equipment in one frame

Fix a frame $f$. The candidate fiber $\mathbb D_f$ has:

- objects $X,Y,\ldots$ representing typed state or artifact contexts;
- horizontal 1-cells $A:X\rightsquigarrow Y$ representing open processes;
- predicate-like proarrows $P:X\rightsquigarrow I$;
- 2-cells or squares relating predicates through processes; and
- a monoidal product for genuinely parallel composition.

The equipment language matters because day needs both process composition and
predicate semantics. A plain category of states makes processes easy but hides
the squares that certify them. A plain logic of predicates makes satisfaction
easy but hides open process boundaries. The double-dimensional picture keeps
both and distinguishes:

1. horizontal composition of atoms into bridges;
2. vertical pasting of realization cells;
3. monoidal juxtaposition of independent processes;
4. convolution or gluing of witness semantics; and
5. reindexing along frame morphisms.

These operations may satisfy coherence and interchange laws, but they are not
synonyms and must not share one operational verb merely because each can be
described informally as “composition.”

## 4. Teloi are predicates, not points

Let $\mathcal S_f$ be the category of world states legible in $f$, and let
$W_f$ be the selected weak equivalences. A telos is a frame-dependent predicate

$$
T_f:\mathcal S_f^{op}\longrightarrow\mathcal V_f
$$

that factors through the localization $\mathcal S_f[W_f^{-1}]$. Thus a telos
describes an invariant region of possible states rather than one privileged
endpoint. Different concrete states may satisfy the same telos precisely
because the frame declares their differences irrelevant to that purpose.

The current observables do not define the telos. A witness system can be
incomplete, unavailable, or replaced without changing what is wanted. This is
why day distinguishes the predicate $T$ from an observational presentation of
$T$ and why failure to obtain a sufficient certificate is not a refutation.

## 5. Present predicate and identity process

Two identity-like ideas must remain type-distinct:

$$
1_X:X\rightsquigarrow X,
\qquad
P_0:X\rightsquigarrow I.
$$

$1_X$ is the identity process: it changes nothing. $P_0$ is the present
predicate: it describes the region currently inhabited according to the frame.
The first is a process; the second is a predicate. Conflating them erases the
difference between “do nothing” and “the world is presently such-and-such.”

A telos is already realized without a nontrivial bridge when a cell exists:

$$
P_0\Longrightarrow T\odot1_X\cong T.
$$

That is an attainment judgment, not an assertion that $P_0$, $T$, and $1_X$
are the same object.

## 6. Atoms generate bridges

An atom is a generating open process

$$
A_i:X_{i-1}\rightsquigarrow X_i.
$$

A bridge is a horizontal composite

$$
B=A_n\odot\cdots\odot A_2\odot A_1:
X_0\rightsquigarrow X_n.
$$

The bridge records a typed path of possible process. It does not by itself say
that the processes ran, that their postconditions hold, or that the target
telos is realized. A typeable bridge can have no realization cell.

Intermediate predicates make local reasoning explicit. For
$P_0,P_1,\ldots,P_n=T$, local cells have type

$$
\eta_i:P_{i-1}\Longrightarrow P_i\odot A_i.
$$

When their boundaries agree, they paste to

$$
\eta:P_0\Longrightarrow
T\odot A_n\odot\cdots\odot A_1
=T\odot B.
$$

This is the precise sense in which “bridging states” are intermediate teloi.
They are predicates mediating adjacent process cells, not mutable workflow
statuses stored by day.

## 7. Realizability is the 2-dimensional judgment

The primary solution object is not a bridge $B$ but a pair $(B,\eta)$. Two
identical atom sequences with different witness-bearing cells are different
realizations because they carry different evidence, procedures, provenance,
scope, or limitations.

In a Hoare-style reading, the cell may be seen as

$$
P_0\leq\operatorname{wp}_B(T),
$$

or dually as

$$
\operatorname{sp}_B(P_0)\leq T.
$$

The equipment notation avoids choosing one predicate orientation too early.
What matters operationally is that a realization has typed source and target
boundaries and is assembled from witness-bearing assessments rather than
inferred from control flow.

## 8. Artifacts, evidence, and assessment

Artifacts are things in the world. They need not live in day or kan. Evidence
is a typed, attributable claim that makes an artifact legible for an evidential
role in frame $f$:

$$
e:\operatorname{Evidence}_f(P,a).
$$

An evidence context $E$ is a finite frame-local diagram retaining artifact
coordinates, provenance, dependencies, and correspondence. An assessment is a
dependent object

$$
\mathcal A:\operatorname{Assessment}_f(E,p,\sigma),
$$

where $p$ is the declared procedure and $\sigma$ its scope. A certificate is a
durable attributable presentation of that assessment; it is not global truth.

This separation prevents authority laundering. Artifact existence is not
evidence. A validly signed evidence claim is not automatically admitted by a
frame. An admitted claim does not determine an assessment procedure. A
certificate authenticates that an assessment was made; it does not force every
frame to accept its conclusion.

## 9. Witnesses, probes, and observational semantics

A witness is a frame-relative observable concept through which a telos or
process proposition can become evidenced. A probe is an operational procedure
that samples or approximates witness-bearing. The probe is not the witness.

Provisionally, witness-bearing is an enriched relation

$$
\mathsf{Bear}_f:
\mathsf{Assess}_f^{op}\otimes\mathsf{Witness}_f
\longrightarrow\mathcal V_f.
$$

A candidate observation semantics chooses an evidence-to-state proarrow
$R_f:\mathcal E_f\nrightarrow\mathcal S_f$ and forms

$$
\operatorname{Obs}_f(T)(E)
=\int^{s\in\mathcal S_f}R_f(E,s)\otimes T(s).
$$

A witness system is a diagram

$$
W_T:J_T\longrightarrow[\mathcal E_f^{op},\mathcal V_f]
$$

whose assembly $\lVert W_T\rVert$ preserves shared coordinates and coherence.
Its relationship to the telos must be explicit. A sufficient system carries a
soundness map

$$
\alpha_T:\lVert W_T\rVert
\Longrightarrow\operatorname{Obs}_f(T).
$$

A necessary system reverses that direction; an exact observational
presentation gives an equivalence. These three strengths support different
inferences. In particular, absence of a sufficient witness-bearing assessment
means “not certified,” not “the telos is false.”

## 10. Why convolution appears

Witness components do not always combine by Cartesian conjunction. Evidence
may be reused, independently reproduced, resource-sensitive, or required to
agree on a shared candidate coordinate. Consequently the evidence tensor is a
real design choice rather than decorative notation.

Day convolution is a candidate operation when witness semantics compose along
the monoidal structure of evidence contexts. Relative or promonoidal
convolution is a candidate when components must glue along a boundary. The
critical operational lesson is already stable even while the final tensor is
not: two individually material components cannot certify one assembled witness
when their required shared coordinates disagree.

## 11. Frame migration reindexes the whole diagram

A frame morphism migrates more than a rendered verdict. It reindexes teloi,
evidence, assessments, procedures, witnesses, atoms, bridges, and realization
cells together. For a pullback square

$$
\begin{matrix}
g'&\xrightarrow{v'}&g\\
\downarrow{u'}&&\downarrow{u}\\
f'&\xrightarrow{v}&f,
\end{matrix}
$$

the candidate Beck--Chevalley obligation is invertibility of

$$
\Sigma_{u'}(v')^*\Longrightarrow v^*\Sigma_u
$$

when the relevant adjoints exist. Strong monoidal comparison and preserved
shared-boundary gluing express lossless transport. Lax comparison, an
unsupported procedure, a forgotten coordinate, or absence of a frame morphism
must remain visible as loss or incomparability.

This is why frame migration is expected to be “nice” only conditionally. The
fibred structure supplies the place to state the laws; it does not make every
change of context invertible.

## 12. The operational profile is a decategorification

Profile v1 has one implicit local frame. Project strings stand in for artifact
types; `day-atom` blocks stand in for open process interfaces; `day-bridge`
expressions approximate horizontal and monoidal composition; probe outcomes
approximate witness-bearing; and kan Result claims carry assessment
certificates.

This is a deliberate decategorification. It preserves enough distinctions to
avoid false claims while forgetting general state categories, explicit frame
morphisms, enrichment, arbitrary witness diagrams, and realization cells as
first-class data. The approximation must always be described in the weaker
language:

- bridge checking establishes typeability, not execution or realizability;
- a probe result is evidence for an assessment, not a witness definition;
- a legacy witness list is a component report, not a logical conjunction;
- a sufficient coherent certificate supports a telos without defining it; and
- migration not established is not equivalent transport.

## 13. What remains open

The abstract shape is firmer than several formal choices inside it. RFC 1
therefore leaves open:

- the exact objects and morphisms of $\mathcal E_f$;
- whether its tensor is Cartesian, cocartesian, resource-sensitive, open-diagram
  union, or part of a duoidal structure;
- the enrichment category $\mathcal V_f$;
- representability of $\mathsf{Bear}_f$;
- ordinary versus relative or promonoidal convolution;
- the exact equipment and double Grothendieck construction; and
- the class of frame morphisms carrying adjoints and invertible coherence.

These choices refine the theory; they do not undo its central geometry. Day is
organized around frame-local predicates and open processes, with bridges as
horizontal composites and witness-bearing realizability as the 2-dimensional
judgment connecting what is present to what is sought.
