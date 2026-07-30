# Feature: Declared block schemas — a project can invent a fenced vocabulary day validates

## Summary

day owns seven fenced block types and a project can invent none. A research
program tried to instantiate day's process for a non-software domain and needed
exactly one more — `research-claim`, carrying a claim's evidential station — and
had nowhere to put it (day#74). This adds a **declared block schema**: a project
records, as a kan claim, a block type's name and its required and optional
fields, and day then validates that block wherever it reads one, under the same
honesty contract `v0.7.0-beta.2` gave day's built-ins.

It is beta.3's mechanism, and the other declarations in this milestone ride it:
review verdict vocabularies (day#77), the cycle boundary (day#76), and the
injection cadence. It is also what makes `v0.8`'s packs buildable at all — a pack
transports only what is declarable, which is the premise `docs/ROADMAP.md` used
to assert and this milestone makes true.

## Requirements

- REQ-1: A project declares a block type by recording a `day-blocks` block on a
  `schema/blocks` subject: a map from block name to its field spec. The spec is
  **names and required/optional only** — no value types, no pattern language.
  A schema arrives from a claim, and day#34's rule holds: a richer language is a
  wider surface than the declared need. `research-claim`'s five fields
  (`medium`, `scope_coords`, `anchor_ref`, `decay_note`, `situated_verdict`) are
  the shape this must fit.
- REQ-2: day **validates** a declared block wherever it reads one, and a
  violation is refused rather than reported loosely — the same treatment a
  built-in gets. Extending `v0.7.0-beta.2`'s contract rather than leaving the
  declarable path the tolerant one is the point: an inconsistency there would be
  day#78 again, with day's own seven strict and everything a project invents lax.
- REQ-3: A declared block carries the same **`_version` gate** as a built-in, via
  the same `atoms::parse_block` path, so a project can version its own vocabulary
  and an older day says *"this day reads `research-claim` v1, this block declares
  v2"* rather than reporting the project's claim as malformed.
- REQ-4: An **unreadable declared block is reported, not dropped** — through
  `status::Unreadable` and therefore on both hook channels, exactly as a built-in
  is. A project that declares a vocabulary day silently ignores has a decorative
  declaration.
- REQ-5: **day's seven built-in blocks stay struct-defined.** Their Rust struct
  *is* the schema, `deny_unknown_fields` and `Versioned` keep it strict, and
  `tests/block_corpus.rs` keeps it backward-compatible. day#74 asks for
  `day-atom` to become the first instance of the new mechanism; it should not,
  and the reason is concrete rather than conservative — a declaration beside the
  struct is a second source of truth with no compiler between them, which is the
  `extract_fenced` defect this milestone's own review found (F3). One mechanism
  for what day writes, one for what a project invents, and neither pretends to be
  the other.
- REQ-6: The **injection cadence becomes declarable** on a `schema/injection`
  subject, riding this mechanism. It is the scalar `v0.7.0-beta.2` shipped as a
  constant with day#82 filed to measure it; making it declared is what the
  roadmap promised beta.3 would do.
- REQ-7: `docs/CONVENTIONS.md` documents the declared-block mechanism, its field
  spec, and — stated plainly because it is the question a reader will have — **why
  day's own blocks are not declared this way.**

## Acceptance Criteria

- [ ] AC-1: A project recording a `schema/blocks` declaration for
      `research-claim` with three required and two optional fields can then
      record a claim carrying a `research-claim` block, and day resolves it. A
      block missing a **required** field is refused, naming the field; a block
      omitting an **optional** one is not. (REQ-1)
- [ ] AC-2: A `research-claim` block carrying a field the declaration does not
      name is **refused**, and the refusal names the field — the same behaviour
      `day-atom` has for an undeclared field since `v0.7.0-beta.2`. **Negative
      control:** the same block against a declaration that *does* name that field
      resolves, so the refusal tracks the declaration rather than a fixed list.
      (REQ-2)
- [ ] AC-3: A declared block carrying `_version: 2` against a declaration this
      build reads at v1 is refused with the version-skew message ("upgrade day"),
      distinguishably from a block that violates its field spec ("the claim needs
      fixing"). Asserted on the rendered text of both. (REQ-3)
- [ ] AC-4: With one unreadable declared block in the log, `day hook
      session-notice` reports it and `day hook session-start` marks its context
      partial — the same two channels a built-in reaches. **Negative control:**
      with every declared block valid, both channels are silent. (REQ-4)
- [ ] AC-5: The seven built-in fences are **not** resolvable through the declared
      mechanism, and a project declaring a `schema/blocks` entry named `day-atom`
      is told that name is reserved rather than silently shadowing the built-in.
      `tests/block_corpus.rs` still passes unchanged, proving the built-ins'
      struct-defined path is untouched. (REQ-5)
- [ ] AC-6: A `schema/injection` declaration setting the cadence changes how many
      prompts pass before a standing condition is re-displayed, and an absent
      declaration keeps `cache::DEFAULT_CADENCE`. (REQ-6)
- [ ] AC-7: `docs/CONVENTIONS.md` documents the mechanism and the field spec, and
      states why day's built-ins are struct-defined — asserted by a
      `tests/plugin.rs` doc check, as the conventions page's other invariants
      are. (REQ-7)

## Architecture

**The mechanism is a second reader beside `parse_block`, not a replacement for
it.** `atoms::parse_block` in `src/atoms.rs` deserializes into a type that
implements `Versioned`, which is what makes the built-ins strict at compile time.
A declared block has no such type, so it parses to a `serde_json::Value` and is
checked against the declared field spec — but through the *same* version gate and
into the *same* `BlockError`, so `TooNew` / `Malformed` / `Invalid` and their
diagnostics are shared rather than reimplemented. That sharing is the whole reason
REQ-3 and REQ-4 are nearly free.

**`src/schema.rs` is the model to follow and the place this most resembles.** It
already does the thing this generalises: a `schema/design-doc` subject carries a
`day-schema` block declaring what a design document must contain, and
`src/design.rs` validates against it. The new part is that the *block type* is
declared rather than the document's sections — one level up, same shape.

**Where the built-ins and the declared path must not meet.** The seven fence
constants (`atoms::FENCE_INFO`, `bridge::FENCE_INFO`, `bridge::TELOS_FENCE`,
`telos::FENCE_INFO`, `schema::FENCE_INFO`, `docs::FENCE_INFO`,
`tension::FENCE_INFO`) become a reserved set the declared mechanism refuses to
shadow (AC-5). Without that, a project could declare `day-atom` and get two
readers for one fence with no rule for which wins — and `v0.7.0-beta.2` removed
exactly that ambiguity from `extract_fenced` by deleting its `fence` parameter.

**`src/status.rs` needs no new concept.** `Unreadable` and `Finding::unreadable`
already carry "day could not read a declaration, so this report is partial", and
both hook channels already render it. A declared block that fails validation
produces the same `Unreadable`, which is why REQ-4 is a wiring requirement rather
than a feature.

**`src/cache.rs`'s `DEFAULT_CADENCE` becomes a fallback** rather than the value.
`schema/injection` is read where the witness schema already is, and an absent
declaration keeps the constant — the same "declared, with a starter day offers
and never applies" contract `WitnessSchema::starter` holds.

**Nothing here writes a claim day did not already write, and no new substrate
appears.** The declared spec is read through the same `kan show` path; no process
is spawned, so `src/probe.rs`'s `run_command` remains day's only spawn site;
`telos/no-store-of-its-own` is untouched because a declaration is a claim.

## Resolved Questions

- **day's built-ins stay struct-defined (REQ-5), and day#74's "one less special
  case" is declined.** The issue asks for `day-atom` to become the first instance
  of the mechanism. That would trade compile-time field checking — which
  `v0.7.0-beta.2` added and `tests/block_corpus.rs` protects against every shape a
  released version wrote — for a declaration beside the struct with no compiler
  between them. This milestone's own adversarial review found precisely that
  defect in `extract_fenced`, where a `fence` parameter and `T::FENCE` were two
  sources of truth for one fact. Declining the symmetry keeps the mechanism
  honest: one path for what day writes, one for what a project invents.
- **Field specs are names plus required/optional, with no value types.** day#34
  established that a schema arriving from a claim should not bring a pattern
  language, and day#70 held the same line for `subject`. Value types would let day
  catch `medium: 7` where a station name was meant — the research loop's
  highest-value rule — but a type language is a language, and every addition to it
  is a decision day then owns for every project. The loop's linter interprets its
  own values; day validates that the fields it declared are present and that
  nothing undeclared is.
- **day validates declared blocks rather than merely registering them.** A
  registry would be smaller, and the loop's linter already validates
  `research-claim` for itself. But leaving the declarable path tolerant while
  day's own seven are strict is day#78's inconsistency reintroduced at the exact
  moment the surface widens — and `v0.7.0-beta.2`'s contract is only a contract if
  it covers what projects declare.
- **The cadence lives on `schema/injection` rather than on `practice`.**
  `practice` is prose items projected into a model's context; the cadence is typed
  config. Putting the first structured value there would make `practice` two
  things. A `schema/injection` subject matches `schema/witness` and `schema/docs`,
  and gives later injection settings somewhere to go.

## Out of Scope

- **Generating declared blocks from flags.** day#74 asks for a verb that emits a
  block "never hand-written". `v0.7.0-beta.2` settled why that guarantee cannot
  come from the writer: generation-only is unenforceable once a second key can
  append, so the guarantee is reader-side validation, which REQ-2 delivers. A
  generator remains a reasonable *affordance* and is deliberately deferred — it is
  new CLI surface, and `CLAUDE.md` holds that a new verb needs its own design.
- **`day assess citations`** (day#75) — agreed to ship in the research repo, with
  day supplying this mechanism and the `kan show --json` contract.
- **Value types, enums, or JSON Schema** for field specs — see Resolved
  Questions. Revisitable if a concrete need appears that presence-checking cannot
  serve.
- **Frames.** Needs kan's v0.8 REQ-3; day's requirements are in
  `.design/kan-read-contract.md` and the design pass runs against what kan ships.
