# RFC 0: RFC and ADR process

- Status: Draft
- Authors: day maintainers
- Created: 2026-08-15
- Discussion: Not opened
- Review-period-ends: Not scheduled
- Review-override: None
- Supersedes: The implicit design and decision process in `CONTRIBUTING.md`
- Superseded-by: None
- Kan-claim: Not published
- Implementation: Proposed in this RFC

## Summary

Day uses Requests for Comments for forward-looking public contracts and
Architecture Decision Records for decisions actually taken. This RFC defines
their formats, lifecycles, review, repository authority, publication as kan
claims, and relationship to working `.design/` documents.

## Motivation

Day's process ontology now affects independently changeable pieces: the Rust
binary, kan conventions, plugins, skills, vocabulary packs, and project-defined
procedures. Working design documents are effective for iteration but do not
provide a stable public review lifecycle or distinguish proposals from adopted
contracts. The frame-indexed process model therefore needs an RFC discipline
before its vocabulary becomes another implementation-defined convention.

Kan RFC 0 has already exercised a suitable structure. Day adopts that process
shape while adding its own claim-publication rule and without manufacturing a
historical ADR sequence.

## Terminology

- **RFC:** A forward-looking proposal for a public process model, interface,
  durable convention, compatibility promise, governance rule, or cross-cutting
  architectural commitment.
- **ADR:** An immutable account of a decision actually taken and the evidence
  available at that time.
- **Design document:** A working, mechanically validated proposal under
  `.design/`. It may produce an RFC, ADR, both, or neither.
- **Normative file:** The merged RFC or ADR Markdown file whose bytes constitute
  the reviewed repository record.
- **Published claim:** A signed kan claim pointing to the normative file at an
  exact commit and path.
- **Maintainer:** A person authorized to merge into the day repository.

Normative words such as MUST, SHOULD, and MAY have their RFC 2119 meanings.

## Denotational target

Not applicable. This RFC governs specification records; RFC 1 supplies the
first process-model denotational target.

## Operational profile

### When an RFC is required

An RFC is required for a new or changed public process primitive, durable data
or block format, compatibility contract, governance or trust rule, execution
authority boundary, or architecture spanning independently changeable
components. A maintainer MAY require one for a similarly broad commitment.

An RFC is not required for a local reversible implementation choice, ordinary
bug fix, release record, or documentation correction. Those may proceed from a
validated design directly to implementation and, where lasting rationale
matters, an ADR.

### Numbering and files

RFCs live at `rfcs/N-slug.md`; ADRs live at `adrs/N-slug.md`. Numbers use their
shortest ASCII decimal representation with no leading zeroes. An RFC number is
allocated when its proposal pull request opens and MUST never be reused.
Permanent gaps are valid historical evidence. ADR numbers are allocated
monotonically when proposed for merge.

### RFC lifecycle

Recognized statuses are:

```text
Draft -> Review -> Accepted -> Implemented
                -> Rejected
Draft/Review -> Withdrawn
Accepted/Implemented -> Superseded
```

Acceptance requires all blocking questions to be resolved and no fewer than 72
continuous hours of public review. Every current maintainer MAY waive only the
remaining time by reacting with a rocket approval to the proposal pull request
after its latest substantive commit. The override never waives unresolved
questions, evidence, validation, or CI. A substantive change restarts the
review period and invalidates earlier override reactions; editorial corrections
do not.

`Implemented` requires linked shipped evidence. Acceptance is a decision about
the contract, not a claim that the implementation exists.

### ADR lifecycle

Recognized ADR statuses are `Proposed`, `Accepted`, `Rejected`, `Deprecated`,
and `Superseded`. An ADR MUST contain context, decision, rationale,
consequences, evidence, alternatives, and supersession sections. It records
what was decided, not what is merely proposed.

### Designs, RFCs, ADRs, and claims

`.design/` remains the working requirements surface. An RFC is the reviewed
public proposal produced when the subject crosses the threshold above. An ADR
records a decision actually taken. An accepted RFC is itself the governing
decision; a duplicate ADR is discouraged. An ADR MUST record a material
implementation departure from an accepted RFC.

The merged repository file is authoritative for RFC or ADR text and status.
An accepted or implemented record MUST also be published as a kan claim with an
exact committed artifact address. The claim provides identity, authorship,
citations, and graph reachability; it does not replace or duplicate the file's
normative content.

### Validation

`scripts/check-rfcs-adrs.sh` validates recognized statuses, required metadata
and sections, shortest-decimal unique filenames, and exact index coverage. It
runs in ordinary CI. The validator checks structure, not truth or acceptance.

## Approximation map

The RFC lifecycle is represented by repository metadata and review state. Kan
publication projects the accepted file into the claim graph but does not make
the mutable claim fold the RFC status authority. `.design/` Plan claims remain
working proposals and do not become accepted RFCs merely by being published.

## Canonicalization and equivalence

The RFC or ADR number is its canonical identity. A changed title or filename
slug does not create a new identity; reusing a number does. The normative
version is identified by repository, commit, and path. Two renderings are
equivalent only when they resolve to identical normative bytes.

## Resolution or processing algorithm

1. Open or identify the motivating issue and validated design.
2. Copy `rfcs/template.md` and author a Draft.
3. Open the proposal pull request, allocate the next unused number, and update
   `rfcs/README.md`.
4. Change status to Review and record the discussion and review deadline.
5. Resolve blocking questions; restart review after substantive changes.
6. After the review period or valid unanimous override, a maintainer accepts,
   rejects, or requests revision.
7. Merge the normative file, publish a kan claim addressing its exact commit
   and path, and record that CID in a follow-up metadata change.
8. Change status to Implemented only when shipped evidence is linked.

ADRs follow the corresponding template, numbering, index, review, merge, and
publication steps but record a decision rather than request comments.

## Authority and trust model

The merged repository is authoritative for text and status. Pull-request
discussion is supporting evidence. Maintainer merge authority establishes
acceptance. A kan signature establishes who published the artifact reference,
not that the RFC was accepted; repository admission and view trust remain kan
read results rather than properties inferred by day.

## Security considerations

RFC text, links, examples, pack declarations, and command examples are
untrusted review inputs. No RFC metadata may contain secrets. A published claim
MUST resolve to the exact reviewed bytes. Executable behavior described by an
RFC remains subject to day's command-probe authorization boundaries; an RFC or
pack does not confer execution authority.

## Compatibility

Existing `.design/` documents and kan decisions remain historical inputs and
are not silently promoted to RFCs or ADRs. `docs/TELOS.md` remains explanatory;
`docs/CONVENTIONS.md` remains authoritative for shipped behavior until an
accepted RFC is implemented and those conventions are reconciled. No
historical ADR migration is performed by this RFC.

## Alternatives considered

- Continue with design documents only: rejected because working proposals have
  no explicit public lifecycle or stable acceptance state.
- Make kan claims the sole normative RFC content: rejected pending kan's
  official claim-addressed content flow and because public repository review
  needs directly inspectable files.
- Duplicate every accepted RFC as an ADR: rejected as two authorities for one
  decision.
- Infer historical ADRs: rejected because completing a template would
  manufacture rationale and evidence not recorded contemporaneously.

## Reference test vectors

The executable validator MUST accept RFC 0, RFC 1, and both templates. Its
self-test MUST reject at least a missing section, invalid status, duplicate
number, leading-zero number, and missing index entry.

## Unresolved questions

None.

## Deferred questions

- Whether kan later provides a first-class RFC publication projection is
  deferred to kan's claim-addressed content work.

## Implementation status

Draft. The accompanying change supplies the file structure, templates,
validator, contributor guidance, and CI integration. Acceptance and claim
publication require the review lifecycle above.

