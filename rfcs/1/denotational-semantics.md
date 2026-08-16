# Day as an indexed process equipment

Status: normative denotational companion to Draft RFC 1

This document is incorporated by the `Denotational target` section of
[`RFC 1`](../1-frame-indexed-process-model.md). RFC 1 remains authoritative for
lifecycle, operational-profile, and conflict-resolution questions. The adjacent
HTML file is a derivative reading copy; this Markdown-with-LaTeX file is the
claim-addressed source.

## 1. Thesis and reading discipline

Day is an indexed process bicategory—or, more precisely, a candidate symmetric
monoidal equipment—fibred over frames. Teloi are frame-dependent predicates,
atoms are process 1-cells, bridges are composites of atoms, and realizability is
witnessed by 2-cells relating a present predicate to the predicate induced by a
bridge and a target telos.

The essential picture in a frame $f$ is the equipment square

$$
\begin{array}{ccccc}
X_0 & \overset{\;B\;}{\rightsquigarrow} & X_n
  & \qquad & \eta:P_0\Longrightarrow T\odot B\\[4pt]
{\scriptstyle P_0}\Downarrow & \underset{\eta}{\Rightarrow} &
  \Downarrow{\scriptstyle T} & & \\[2pt]
I & = & I & &
\end{array}
$$

Here $P_0:X_0\rightsquigarrow I$ describes the presently inhabited region,
$B:X_0\rightsquigarrow X_n$ is a bridge, and
$T:X_n\rightsquigarrow I$ is the target telos. Composition is written
right-to-left, so $T\odot B:X_0\rightsquigarrow I$ is the target predicate
induced at the source boundary by $B$.

The presentation uses three strengths of statement. A definition fixes an
intended type. A proposition follows from assumptions stated here and receives
a proof sketch. An open obligation names structure still required before a
claim becomes a settled theorem. Suggestive notation therefore does not
quietly decide the tensor, enrichment, or exact equipment.

## 2. The indexed ambient structure

### Definition 2.1 — Frame

A frame $f$ is a dependent context determining legible states, admissible
artifacts and evidence, procedures and scopes, selected weak equivalences,
witness interpretation, authority, and meaningful process boundaries. Let
$\mathcal F$ be the category whose objects are frames and whose morphisms are
declared context translations. A frame morphism is evidence that migration is
meaningful; the absence of one is meaningful incomparability.

### Definition 2.2 — Indexed process equipment

The long-term ambient structure is an equipment-valued pseudofunctor

$$
\mathbb D:\mathcal F^{op}\longrightarrow\mathbf{Equip}.
$$

For every $f\in\mathcal F$, the fiber $\mathbb D_f$ contains the states,
predicates, processes, and squares meaningful in $f$. A frame morphism
$u:g\to f$ supplies reindexing

$$
u^*:\mathbb D_f\longrightarrow\mathbb D_g.
$$

For $v:h\to g$, pseudofunctoriality supplies a coherent comparison

$$
\begin{array}{ccccc}
\mathbb D_f & \xrightarrow{\;u^*\;} & \mathbb D_g
  & \xrightarrow{\;v^*\;} & \mathbb D_h\\[7pt]
v^*u^* & \underset{\phi_{u,v}}{\overset{\cong}{\Longrightarrow}} &
  (u\circ v)^* & :\mathbb D_f\longrightarrow & \mathbb D_h
\end{array}
$$

The identity comparison similarly identifies $(1_f)^*$ with
$1_{\mathbb D_f}$. These comparisons are data, not equality by typography.

### Open obligation 2.A — Grothendieck construction

An ordinary Grothendieck fibration captures category-valued shadows of
$\mathbb D$, but need not capture horizontal arrows, vertical arrows, and
squares together. The exact double-categorical Grothendieck construction and
class of admissible equipment morphisms remain to be selected.

## 3. One fiber: processes, predicates, and teloi

Fix a frame $f$ for this section.

### Definition 3.1 — State context and open process

Objects $X,Y,\ldots$ of $\mathbb D_f$ are typed state or artifact contexts. A
horizontal 1-cell $A:X\rightsquigarrow Y$ is an open process with input
boundary $X$ and output boundary $Y$. Horizontal composition is defined
exactly when boundaries agree:

$$
\frac{A:X\rightsquigarrow Y\qquad C:Y\rightsquigarrow Z}
     {C\odot A:X\rightsquigarrow Z}.
$$

The horizontal identity $1_X:X\rightsquigarrow X$ is the process that changes
nothing at boundary $X$.

### Definition 3.2 — Predicate

A predicate on $X$ is a proarrow $P:X\rightsquigarrow I$, where $I$ is the
fiber's truth or observation boundary. Predicates restrict along processes:

$$
\frac{A:X\rightsquigarrow Y\qquad Q:Y\rightsquigarrow I}
     {Q\odot A:X\rightsquigarrow I}.
$$

### Definition 3.3 — Telos

Let $\mathcal S_f$ be the category of states legible in $f$, $W_f$ its selected
weak equivalences, and $\mathcal V_f$ a provisional category of truth values.
A telos is a predicate

$$
T_f:\mathcal S_f^{op}\longrightarrow\mathcal V_f
$$

that factors through $\mathcal S_f[W_f^{-1}]$. A telos denotes an invariant
region, not a privileged endpoint.

### Proposition 3.4 — Weakly equivalent states agree on a telos

If $w:s\to s'$ belongs to $W_f$, then $T_f(w)$ is invertible. Hence $s$ and
$s'$ cannot be distinguished by $T_f$.

### Proof sketch

Factoring through the localization sends every morphism inverted there to an
isomorphism in $\mathcal V_f$. This does not say $s=s'$; only that their
difference is irrelevant to this telos in this frame.

### Example 3.5 — A repository-state telos

Let objects of a finite $\mathcal S_f$ be repository snapshots and let $W_f$
identify snapshots differing only in ignored build output. The telos “the RFC
companion is source-linked and renders without MathJax errors” contains many
snapshots. It is invariant under ignored output changes, but not under deleting
the source link. The HTML file is an artifact in a satisfying state; it is not
itself the telos.

## 4. Present predicates, atoms, and bridges

### Definition 4.1 — Present predicate

The present predicate $P_0:X_0\rightsquigarrow I$ describes the presently
inhabited region according to $f$. It is type-distinct from
$1_{X_0}:X_0\rightsquigarrow X_0$. “The world is presently such-and-such” is a
predicate; “do nothing” is a process.

### Definition 4.2 — Atom and bridge

An atom is a generating open process $A_i:X_{i-1}\rightsquigarrow X_i$. A
bridge is a boundary-compatible horizontal composite

$$
B=A_n\odot\cdots\odot A_2\odot A_1:
X_0\rightsquigarrow X_n.
$$

For two atoms, the typing derivation is

$$
\frac{A_1:X_0\rightsquigarrow X_1\qquad
      A_2:X_1\rightsquigarrow X_2}
     {A_2\odot A_1:X_0\rightsquigarrow X_2}.
$$

A bridge records a possible process path. It does not assert execution,
postconditions, or realization.

### Definition 4.3 — Realization cell

For $B:X_0\rightsquigarrow X_n$, $P_0:X_0\rightsquigarrow I$, and
$T:X_n\rightsquigarrow I$, a realization is a square

$$
\eta:P_0\Longrightarrow T\odot B.
$$

The primary solution object is $(B,\eta)$, not $B$ alone. Equal atom sequences
carrying different assessments, provenance, scopes, or limitations may
determine different realization cells.

### Proposition 4.4 — Identity attainment

If a cell $\eta_0:P_0\Rightarrow T$ exists, the empty bridge realizes $T$ from
$P_0$.

$$
\begin{array}{ccccc}
X & \xrightarrow{\;1_X\;} & X
  & \qquad & \eta_0:P_0\Rightarrow T\odot1_X\cong T\\[4pt]
{\scriptstyle P_0}\Downarrow & \underset{\eta_0}{\Rightarrow} &
  \Downarrow{\scriptstyle T} & & \\[2pt]
I & = & I. & &
\end{array}
$$

### Proof sketch

Whisker $\eta_0$ by the horizontal unit and compose with the right unitor
$T\odot1_X\cong T$ in the required orientation. This uses the equipment's unit
law and does not identify $P_0$, $T$, and $1_X$.

### Construction 4.5 — Two-atom vertical pasting

Let $P_i:X_i\rightsquigarrow I$ and suppose

$$
\eta_1:P_0\Rightarrow P_1\odot A_1,
\qquad
\eta_2:P_1\Rightarrow P_2\odot A_2.
$$

Their boundaries form the pasted diagram

$$
\begin{array}{ccccc}
X_0 & \xrightarrow{\;A_1\;} & X_1
    & \xrightarrow{\;A_2\;} & X_2\\[3pt]
{\scriptstyle P_0}\Downarrow & \underset{\eta_1}{\Rightarrow} &
{\scriptstyle P_1}\Downarrow & \underset{\eta_2}{\Rightarrow} &
\Downarrow{\scriptstyle P_2}\\[2pt]
I & = & I & = & I.
\end{array}
$$

Whiskering $\eta_2$ by $A_1$ and vertically composing gives

$$
P_0
\overset{\eta_1}{\Longrightarrow}P_1\odot A_1
\overset{\eta_2\ast A_1}{\Longrightarrow}
(P_2\odot A_2)\odot A_1
\cong P_2\odot(A_2\odot A_1).
$$

### Proposition 4.6 — Finite local realizations paste globally

Given boundary-compatible atoms $A_1,\ldots,A_n$ and local cells
$\eta_i:P_{i-1}\Rightarrow P_i\odot A_i$, their vertical paste determines

$$
\eta:P_0\Longrightarrow
P_n\odot A_n\odot\cdots\odot A_1.
$$

### Proof sketch

Induct on $n$. Proposition 4.4 gives the zero case. The successor case applies
Construction 4.5 to the induction hypothesis and $\eta_{n+1}$, then uses the
associator. Bicategorical coherence makes reassociations canonical. Taking
$P_n=T$ yields the central realization cell. This is the precise sense in which
bridging states are intermediate teloi rather than workflow statuses.

### Example 4.7 — Design followed by build

Take $A_1$ to be `atom/design` and $A_2$ to be `atom/generative-build`. The
declared boundary `design-doc` makes $A_2\odot A_1$ typeable from `intent` to
`code-change`. A design assessment can approximate $\eta_1$ and a build
assessment can approximate $\eta_2$. The CLI verifies the shared artifact type;
it does not construct the 2-cell or prove the procedures sound.

## 5. Artifacts, evidence, assessment, and certificates

### Definition 5.1 — Artifact and evidence claim

An artifact $a$ is a thing in the world and need not live in day or kan.
Evidence is a typed attributable claim

$$
e:\operatorname{Evidence}_f(P,a)
$$

presenting $a$ for evidential role $P$ in frame $f$. Changing $P$, $f$, the
author, or the addressed artifact gives a different evidence claim even when
$a$ is unchanged.

### Definition 5.2 — Evidence context

An evidence context $E$ is a finite frame-local diagram, not merely a set. It
retains artifact coordinates, provenance, dependencies, and correspondence. A
typical span-shaped context is

$$
\begin{array}{ccccc}
& e_{\mathrm{build}} & & e_{\mathrm{review}} & \\
& \searrow & & \swarrow & \\
& & c:\operatorname{Commit} & &
\end{array}
$$

where both claims address the same commit $c$. Replacing one leg by a claim
about $c'\neq c$ changes the diagram even if both claims pass locally.

### Definition 5.3 — Assessment and certificate

An assessment is a dependent object

$$
\mathcal A:\operatorname{Assessment}_f(E,p,\sigma),
$$

where $E$ is admitted evidence, $p$ the declared procedure, and $\sigma$ its
scope. A certificate is a durable attributable presentation of that assessment.

### Proposition 5.4 — No authority laundering

Artifact existence, evidence validity, evidence admission, assessment, and
certificate validity are separate judgments; no implication between adjacent
judgments exists without a frame-declared rule.

$$
\begin{array}{ccccc}
a\ \mathrm{exists}
& \not\Rightarrow & e:\operatorname{Evidence}_f(P,a)
& \not\Rightarrow & e\in E\\[3pt]
& & \Downarrow\scriptstyle\text{declared }p,\sigma & & \\
& & \mathcal A:\operatorname{Assessment}_f(E,p,\sigma)
& \not\Rightarrow & \text{global truth.}
\end{array}
$$

### Proof sketch

Each expression has different dependent parameters and authority. An artifact
does not choose a role or claimant. A signed claim does not choose admission
policy. An admitted diagram does not choose a procedure. A certificate records
an assessment under a scope; scope prevents promotion to global truth.

### Example 5.5 — One artifact, two evidence claims

The same commit $c$ can support
$e_1:\operatorname{Evidence}_f(\mathrm{build},c)$ and
$e_2:\operatorname{Evidence}_f(\mathrm{review},c)$. They share an artifact
coordinate but differ in role and possibly author. Treating them as one Boolean
“commit exists” loses the correspondence needed for a release assessment.

## 6. Witnesses, probes, and telos observation

### Definition 6.1 — Witness and probe

A witness $W$ is a frame-relative observable concept through which a telos or
process proposition can become evidenced. A probe $q$ is a procedure that
samples or approximates witness-bearing. The procedure is not the concept.

Provisionally, witness-bearing has enriched type

$$
\mathsf{Bear}_f:
\mathsf{Assess}_f^{op}\otimes\mathsf{Witness}_f
\longrightarrow\mathcal V_f.
$$

Running $q$ may produce an assessment $\mathcal A$ and a value approximating
$\mathsf{Bear}_f(\mathcal A,W)$. An exit code does not define $W$ or establish
that the approximation is sound.

### Construction 6.2 — Observable semantics of a telos

Choose an evidence-to-state proarrow
$R_f:\mathcal E_f\nrightarrow\mathcal S_f$. Define

$$
\operatorname{Obs}_f(T)(E)
=\int^{s\in\mathcal S_f}R_f(E,s)\otimes T(s).
$$

The coend composes evidence compatible with $s$ with the truth value of $T$ at
$s$, then quotients over changes of the state coordinate. It is well-typed when
$R_f$ and $T$ have the displayed variance and $\mathcal V_f$ admits the coend.

### Open obligation 6.A — Representability and enrichment

The categories $\mathcal E_f$ and $\mathcal V_f$, variance of all evidence
coordinates, and existence of the coend remain provisional. This construction
states an interface; it does not claim current day computes the coend.

### Definition 6.3 — Witness system and assembly

A witness system for $T$ is a diagram

$$
W_T:J_T\longrightarrow
[\mathcal E_f^{op},\mathcal V_f].
$$

Its assembly $\lVert W_T\rVert$ is a specified weighted colimit, convolution,
or relative gluing preserving the coordinates declared shared by $J_T$. The
choice of assembly is part of the witness-system declaration.

### Definition 6.4 — Sufficient, necessary, and exact systems

A sufficient system carries a soundness transformation
$\alpha_T:\lVert W_T\rVert\Rightarrow\operatorname{Obs}_f(T)$. A necessary
system carries the reverse transformation. An exact presentation carries
mutually inverse transformations.

$$
\begin{array}{ccccc}
\lVert W_T\rVert & \overset{\alpha_T}{\Longrightarrow} &
\operatorname{Obs}_f(T) & & \text{sufficient}\\[3pt]
\lVert W_T\rVert & \overset{\beta_T}{\Longleftarrow} &
\operatorname{Obs}_f(T) & & \text{necessary}\\[3pt]
\lVert W_T\rVert & \underset{\beta_T}{\overset{\alpha_T}{\rightleftarrows}} &
\operatorname{Obs}_f(T),\quad
\beta_T\alpha_T=1,\ \alpha_T\beta_T=1
& & \text{exact.}
\end{array}
$$

### Proposition 6.5 — What a sufficient system permits

A witness-bearing element of $\lVert W_T\rVert(E)$ maps through $\alpha_T$ to
an element of $\operatorname{Obs}_f(T)(E)$. Failure to obtain an element of
$\lVert W_T\rVert(E)$ supplies no element of the complement of
$\operatorname{Obs}_f(T)(E)$.

### Proof sketch

The first claim is application of $\alpha_T$ at $E$. The second would require a
refutation map, excluded-middle structure, or an exact presentation. None
follows from a one-way natural transformation. “Not certified” is not
“refuted.”

### Example 6.6 — Finite truth-value models

Take $\mathcal V_f=\mathbf{Set}$ and suppress $E$. If
$\lVert W\rVert=\{s_1\}$ and
$\operatorname{Obs}(T)=\{s_1,s_2\}$, inclusion is sufficient but not
necessary. Reversing the sets is necessary but not sufficient. Equal sets with
identity maps give an exact presentation. These examples select direction
without pretending profile v1 implements all three relationships.

## 7. Witness assembly and convolution

### Definition 7.1 — Evidence tensor

Suppose evidence contexts carry a monoidal product
$(\mathcal E_f,\otimes_f,I_f)$. The tensor says how evidence diagrams coexist.
Cartesian product permits unrestricted copying and deletion. A
resource-sensitive tensor may forbid them. A gluing tensor may require equality
on a shared boundary. The tensor changes what a composite witness means.

### Construction 7.2 — Day convolution

For presheaves $F,G:\mathcal E_f^{op}\to\mathcal V_f$, the candidate Day
convolution is

$$
(F\star G)(E)
=\int^{E_1,E_2}
\mathcal E_f(E,E_1\otimes_f E_2)
\otimes F(E_1)\otimes G(E_2).
$$

The hom term witnesses a decomposition of total evidence $E$ into contexts
supporting $F$ and $G$. Associativity of $\star$ is induced by associativity of
$\otimes_f$ plus Fubini interchange for the coends.

### Proposition 7.3 — Convolution respects shared coordinates only when the tensor does

If $E_1\otimes_f E_2$ is defined by a pullback over a shared coordinate $C$,
then a term of $(F\star G)(E)$ requires compatible maps from both components to
the same $C$. Material terms with coordinates $c_1\neq c_2$ do not compose.

$$
\begin{array}{ccccc}
E_1 & \longrightarrow & C & \longleftarrow & E_2\\
\big\downarrow & & \big\Vert & & \big\downarrow\\
c_1 & \longrightarrow & C & \longleftarrow & c_2
\end{array}
\qquad
c_1\neq c_2\ \Longrightarrow\
E_1\times_CE_2\ \text{has no matching pair.}
$$

### Proof sketch

Objects of the pullback are pairs whose images in $C$ agree. An unequal pair
is not in the pullback, so it cannot supply the decomposition morphism in the
convolution coend. Flat Boolean conjunction forgets this hom term.

### Example 7.4 — Build and review of different commits

Let $F$ say “tests pass for commit $c$” and $G$ say “review approves commit
$c$.” A test at $c_1$ and review at $c_2$ each pass. When the evidence tensor
glues over `Commit`, their composite exists only if $c_1=c_2$. Profile v1
preserves this through shared artifact coordinates; it does not expose Day
convolution as a runtime value.

### Open obligation 7.A — Final assembly structure

Ordinary Day convolution, relative convolution, promonoidal convolution, and a
duoidal combination of parallel and gluing products remain candidates. The
chosen structure must discriminate copying, independent reproduction,
resource use, and shared-coordinate agreement.

## 8. Frame migration

### Definition 8.1 — Reindexing a realization

For $u:g\to f$, suppose $u^*$ preserves horizontal composition through an
invertible comparison

$$
\chi_{T,B}:u^*(T\odot B)
\overset{\cong}{\Longrightarrow}u^*T\odot u^*B.
$$

Applying $u^*$ to a realization and composing with $\chi_{T,B}$ gives

$$
u^*P_0\overset{u^*\eta}{\Longrightarrow}
u^*(T\odot B)\overset{\chi_{T,B}}{\Longrightarrow}
u^*T\odot u^*B.
$$

The commuting diagram is

$$
\begin{array}{ccc}
u^*P_0 & \overset{u^*\eta}{\Longrightarrow} & u^*(T\odot B)\\[4pt]
\big\Vert & & \big\Downarrow\scriptstyle\chi_{T,B}\\[4pt]
u^*P_0 & \overset{\eta_g}{\Longrightarrow} & u^*T\odot u^*B.
\end{array}
$$

### Proposition 8.2 — Strong reindexing preserves realizability

Under this invertible comparison, every realization in $f$ induces one in $g$
for the reindexed present predicate, bridge, and telos.

### Proof sketch

Equipment morphisms map squares to squares. The comparison fixes the target
boundary so the mapped square has the required type. Invertibility means this
comparison loses no information. This proves preservation, not reflection: a
cell found only in $g$ need not descend to $f$.

### Definition 8.3 — Adjoints and the Beck--Chevalley mate

When they exist, adjoints
$\Sigma_u\dashv u^*\dashv\Pi_u$ separate existential forgetting,
substitution, and universal transport. For a pullback square

$$
\begin{array}{ccc}
g' & \xrightarrow{\;v'\;} & g\\
{\scriptstyle u'}\downarrow & & \downarrow{\scriptstyle u}\\
f' & \xrightarrow{\;v\;} & f,
\end{array}
$$

both paths below have type $\mathbb D_g\to\mathbb D_{f'}$:

$$
\begin{array}{ccccc}
\mathbb D_g & \xrightarrow{\;(v')^*\;} & \mathbb D_{g'}
  & \xrightarrow{\;\Sigma_{u'}\;} & \mathbb D_{f'}\\[5pt]
\mathbb D_g & \xrightarrow{\;\Sigma_u\;} & \mathbb D_f
  & \xrightarrow{\;v^*\;} & \mathbb D_{f'}.
\end{array}
$$

The Beck--Chevalley mate is well-typed as

$$
\operatorname{BC}_{u,v}:
\Sigma_{u'}(v')^*\Longrightarrow v^*\Sigma_u.
$$

### Proposition 8.4 — Invertible Beck--Chevalley comparison makes pullback transport path-independent

If $\operatorname{BC}_{u,v}$ is invertible, existentially transporting after
pulling back agrees, up to the specified isomorphism, with pulling back after
existential transport.

### Proof sketch

The functor composites have the common source and target displayed above. An
invertible natural transformation supplies path independence. Compatibility
with horizontal composition and witness assembly requires further
monoidal/equipment coherence; Beck--Chevalley alone does not provide it.

### Example 8.5 — Lossless and lossy migration

Suppose $f$ records a commit coordinate and procedure version, while $g$
renames both through bijections. Reindexing preserves evidence gluing,
procedure support, and the realization square; $\chi$ can be invertible.

Now suppose $g$ forgets the commit coordinate. Individual test and review
assessments transport, but their shared-commit pullback cannot be reconstructed.
The monoidal comparison is merely lax, so migration must report the loss. If
$g$ also lacks the procedure, evidence may transport while assessment is
uncheckable.

### Open obligation 8.A — Admissible migration laws

The target must identify which frame morphisms carry $\Sigma_u$ or $\Pi_u$,
which comparisons are strong versus lax, and which preserve witness assembly.
Absence of a morphism or adjoint, and every noninvertible comparison, must
remain visible.

## 9. A conditional compositionality theorem

### Theorem sketch 9.1 — Realizability composes in an indexed fiber

Assume a fiber with coherent horizontal units and associators, and local cells
$\eta_i:P_{i-1}\Rightarrow P_i\odot A_i$. Then
$B=A_n\odot\cdots\odot A_1$ has a global realization
$\eta:P_0\Rightarrow P_n\odot B$.

If $u:g\to f$ is a strong equipment morphism, reindexing local squares and
pasting in $g$ agrees with reindexing the global paste in $f$:

$$
\begin{array}{ccc}
u^*P_0 & \overset{u^*(\eta_n\circ\cdots\circ\eta_1)}{\Longrightarrow} &
u^*(P_n\odot B)\\[5pt]
\big\Vert & & \big\Downarrow\scriptstyle\chi\\[5pt]
u^*P_0 & \overset{(u^*\eta_n)\circ\cdots\circ(u^*\eta_1)}{\Longrightarrow} &
u^*P_n\odot u^*B.
\end{array}
$$

### Proof sketch

The first statement is Proposition 4.6. Functoriality preserves vertical
pasting, the strong comparison preserves horizontal whiskering, and the
equipment-morphism coherence axiom equates the composites. Without strength or
coherence, the lower cell need not equal the image of the upper one.

## 10. Concrete operational instances

### Instance 10.1 — Present identity telos

Take a repository already satisfying “working tree has no RFC validation
errors.” The present predicate entails the telos under the validator assessment.
The bridge is $1_X$ and the certificate approximates
$\eta_0:P_0\Rightarrow T$. Day can report attainment without inventing an atom
execution. It forgets the full predicate hom and proof that the validator is
sound for $T$.

### Instance 10.2 — Design-to-build bridge

Let

$$
A_1=\texttt{atom/design}:\texttt{intent}\rightsquigarrow\texttt{design-doc},
$$

$$
A_2=\texttt{atom/generative-build}:
\texttt{design-doc}\rightsquigarrow\texttt{code-change}.
$$

The bridge checker establishes the middle-boundary equality and hence the type
of $A_2\odot A_1$. Design and build results provide candidate local
assessments. Current day forgets general contexts, actual realization squares,
and proof that procedures establish their postconditions.

### Instance 10.3 — Coherent release witness

Let $W_1$ be “tests pass for commit $c$” and $W_2$ be “review approves commit
$c$.” Their witness diagram shares `Commit`. Assessments at one commit assemble;
assessments at different commits do not. The profile stores artifact addresses
and rejects mismatch. It forgets the pullback's universal property and does not
expose $W_1\star W_2$ as a presheaf.

### Instance 10.4 — Migration with an unsupported procedure

Frame $f$ admits `cargo-test/v1`; frame $g$ addresses the same commit but does
not admit that procedure. Reindexing transports artifact and evidence
coordinates. It cannot construct
$\operatorname{Assessment}_g(E,p,\sigma)$ for unsupported $p$. The correct
profile outcome is `uncheckable`, not `not-certified`, and not a transported
realization cell.

## 11. Operational profile as decategorification

Profile v1 has one implicit local frame. Project strings approximate artifact
types. `day-atom` blocks approximate open-process interfaces. `day-bridge`
expressions approximate horizontal and monoidal composition. Probe outcomes
approximate witness-bearing assessments. Kan Result claims approximate durable
certificates.

The profile aims to preserve this typeability square:

$$
\begin{array}{ccc}
\text{denotational declarations} & \xrightarrow{\;\text{compose}\;} &
\text{typed bridge}\\[4pt]
\big\downarrow\scriptstyle\text{profile} & &
\big\downarrow\scriptstyle\text{profile}\\[4pt]
\text{blocks and strings} & \xrightarrow{\;\texttt{day bridge check}\;} &
\text{boundary-compatible expression.}
\end{array}
$$

The square commutes only for typeability. Bridge-check success is not execution,
assessment, witness-bearing, or realization. Probe success is evidence for an
assessment rather than a witness definition; a legacy witness list is a
component report rather than conjunction; migration not established is not
equivalent transport.

## 12. Remaining formal obligations

The evidence category still needs defined variance and morphisms. Its tensor
must distinguish copying, independence, resources, and boundary gluing. The
enrichment must support the claimed coends and transformations.
Witness-bearing may or may not be representable. Ordinary, relative, or
promonoidal convolution must be chosen. The exact equipment and double
Grothendieck construction must be identified. Admissible frame morphisms,
adjoints, and strong or lax comparisons must be classified.

Those are open obligations, not holes concealed by notation. The stable result
is the typed geometry: day is organized around frame-local predicates and open
processes; bridges are horizontal composites; witness-bearing realizability is
the 2-dimensional judgment connecting what is present to what is sought; and
frame migration acts on the entire diagram rather than copying a verdict.
