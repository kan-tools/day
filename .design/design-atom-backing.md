# Feature: CLI backing for the design and review atoms

## Summary

Split day's two commands into the layer that orchestrates and the layer that
verifies: `/design` and `/adversarial-review` stay slash commands (only a
command can conduct a multi-turn interview), and day's CLI gains the
mechanically-checkable parts they currently ask a language model to perform —
schema validation, the kan claim chain, verdict shape, and successor lookup.
The design-doc schema itself is declared in kan as process vocabulary, the
same way atoms are, so it is per-project and revisable rather than hard-coded.

## Motivation, stated once

`/adversarial-review` instructs a reviewer to run the tests itself and quote
code rather than trust a design doc's claims — and nothing checks that it
did. A model can emit a complete REQ/AC coverage table without reading a line
of code and the output is indistinguishable. That is the exact failure mode
the review exists to catch, reproduced one level up.

The same holds for `/design`. All seven of its validation rules
(`commands/design.md`, "Validation") are mechanically checkable, including
"Architecture references at least one real file path (verify with `ls`)". It
is a linter specification currently executed by asking a model to count
things. Models are poor at counting and excellent at reporting that they
counted.

There is direct evidence this matters: the `--cites` bug. kan's copy of
`design.md` instructed agents to pass file paths to `--cites`, which takes
claim CIDs and errors on a path. It survived because the instruction was
prose. A CLI cannot get its own flag wrong.

## Requirements

- REQ-1: A design-doc schema is declared in kan as process vocabulary on a
  `schema/<slug>` subject (default `schema/design-doc`), carrying a fenced
  `day-schema` JSON block — the same mechanism `atom/<slug>` subjects already
  use (`src/atoms.rs`, `docs/CONVENTIONS.md`). It declares required section
  headings, the ID patterns for requirements and acceptance criteria, and the
  placeholder tokens that count as unfinished. Per-project and revisable by
  appending a superseding claim; day ships no hard-coded document shape.
- REQ-2: `day design check <path>` validates a design document against the
  live schema and reports one line per rule with a pass/warn/fail verdict:
  required sections present and non-empty, at least the declared minimum
  number of requirements and acceptance criteria, every requirement ID
  referenced by at least one acceptance criterion, no placeholder tokens,
  every file path mentioned in the architecture section existing on disk, and
  a count of unresolved open-question blocks. Reads only.
- REQ-3: `day design record <path>` parses the document and appends the kan
  claim chain: an `observe` for exploration, a `plan` for the design itself
  citing it, and one `decide` per resolved question citing the plan. day
  assembles the chain and passes CIDs, so the `--cites`-takes-paths class of
  error is unreachable rather than merely documented.
- REQ-4: `day design record` runs REQ-2's validation and **records
  regardless of the result**, embedding the validation summary in the plan
  claim's text. An under-specified design becomes visible in the graph as
  data rather than being blocked at a gate — `telos/affordance-not-enforcement`
  applied to day's own surface, and the option that keeps rough designs
  recorded instead of unrecorded.
- REQ-5: `day review record <path>` appends an adversarial-review verdict:
  the verdict value must be one of exactly `APPROVE`, `APPROVE-WITH-FOLLOW-UPS`,
  `REDIRECT`, `BLOCK`, and the claim must cite the design claim being audited.
  A verdict outside that set, or with no citation, is rejected as a malformed
  argument — argument validation, not a workflow gate.
- REQ-6: `day next <atom>` reads the atom graph from kan and reports an atom's
  declared successors together with the inputs each successor needs and where
  those inputs come from in the upstream closure — reusing `atoms::ancestors`
  and the same transitive-coverage rule `day doctor` already applies.
- REQ-7: `commands/design.md` and `commands/adversarial-review.md` call the
  CLI for every step it can perform, keeping prose only for the parts that
  genuinely require a model: interviewing, exploring the codebase, writing the
  document, and forming a judgment. Neither command names the other; both end
  by calling `day next` and reporting what the graph says.
- REQ-8: The MCP server exposes `design_check` and `next` as tools
  (`src/mcp.rs`), dispatching to the same functions the CLI verbs call, so the
  two surfaces cannot disagree. The interview itself is not exposed as a tool
  — a multi-turn interview is not a function call.
- REQ-9: `docs/CONVENTIONS.md` documents the `schema/<slug>` subject
  convention and the `day-schema` block format alongside the existing telos
  and atom conventions.

## Acceptance Criteria

- [ ] AC-1: Given a kan log containing a `schema/design-doc` subject whose
      live claim declares required sections, `day design check` on a document
      missing one of them names the missing section and exits non-zero; with
      all sections present it exits zero. (REQ-1, REQ-2)
- [ ] AC-2: Revising the schema by appending a second claim with different
      required sections changes `day design check`'s result without any file
      being edited — the newest claim wins, as with atoms. (REQ-1)
- [ ] AC-3: `day design check` on a document with `REQ-3` never referenced by
      any acceptance criterion reports that requirement as uncovered and names
      it. (REQ-2)
- [ ] AC-4: `day design check` on a document whose architecture section
      references a path that does not exist reports that path. (REQ-2)
- [ ] AC-5: `day design record` on a document with two resolved questions
      appends exactly one `observe`, one `plan` citing it, and two `decide`
      claims citing the plan; the resulting subject's fold shows that chain.
      (REQ-3)
- [ ] AC-6: `day design record` on a document that fails validation still
      appends the chain, and the plan claim's text contains the validation
      summary naming the failed rule. (REQ-4)
- [ ] AC-7: `day review record` rejects a verdict string outside the four
      permitted values, and rejects an invocation with no `--cites`, in both
      cases exiting non-zero without appending anything. (REQ-5)
- [ ] AC-8: `day next design`, against the current three-atom vocabulary,
      names `generative-build` as the successor and reports the inputs it
      needs; `day next` on an atom with no successors says so and exits zero.
      (REQ-6)
- [ ] AC-9: Neither `commands/design.md` nor `commands/adversarial-review.md`
      contains a hard-coded invocation of the other — enforced by a test
      grepping both files, the same guardrail style `tests/plugin.rs` already
      uses for blocking hooks. (REQ-7)
- [ ] AC-10: An MCP `tools/list` includes `design_check` and `next`, and a
      `design_check` call returns the same text as the CLI verb for the same
      input — extending the equivalence assertion in `tests/mcp_server.rs`.
      (REQ-8)
- [ ] AC-11: `docs/CONVENTIONS.md` contains the `schema/` prefix and the
      `day-schema` fence token, checked against the code's own constants the
      way `tests/plugin.rs::ac9_conventions_document_the_prefixes` already
      checks the atom and telos prefixes. (REQ-9)

## Architecture

**`src/schema.rs` (new)** holds the `Schema` type and its kan-backed load,
mirroring `src/atoms.rs` almost exactly: a `schema/<slug>` subject, a fenced
`day-schema` JSON block, newest interface-bearing claim wins. The parallel is
close enough that the fence-extraction logic in `atoms::extract_interface`
should be generalized to take a fence token rather than duplicated — one
function, two callers, no new concept.

**`src/design.rs` (new)** parses a design document into sections and IDs and
runs the schema's rules over it. Parsing is deliberately shallow: heading
lines, `REQ-N`/`AC-N` tokens, fenced-block-aware placeholder scanning, and
backtick-quoted paths in the architecture section. It is a linter, not a
Markdown AST — anything requiring real document understanding stays in the
command's prose.

**`src/kan_client.rs`** gains write methods (`observe`, `plan`, `decide`,
`result`) that shell `kan`'s corresponding verbs and return the printed CID.
This is the v0.2 invariant change already recorded in `docs/ROADMAP.md`: day
becomes a writer, but only through kan's public CLI. kan still signs,
content-addresses, and owns the log format; day never touches storage and
still has no destroy path, because kan has none to reach. `CLAUDE.md`'s
"day is a reader" rule is updated in this pass, not quietly worked around.

**`src/cli/mod.rs`** gains `design` (with `check` and `record` leaves),
`review record`, and `next`. The existing four verbs are unchanged.

**`src/mcp.rs`** gains two tools dispatching to the same functions, following
the pattern already established by `doctor` and `session_context`.

**Composition stays data, not control flow.** `/design` does not invoke
`/adversarial-review`. Both call `day next`, which reads the declared `next`
edges from kan. A project that inserts a formal-verification atom between
design and build changes its atom claims, not day's prompts — which is what
`telos/composable-process` requires, and what makes swapping process opinions
in and out real rather than aspirational.

**Nothing here destroys.** Every new write is an append through kan's own
verbs; schema and atom revisions supersede by appending. day still keeps no
store of its own: the schema lives in kan, the design doc lives in the
consuming repo's `.design/`, and day holds neither.

## Out of Scope

- The telos and atom declaration verbs (`day telos`, `day atom`) — same v0.2
  release, separate design pass. This doc assumes atoms are declared the way
  they are today.
- Any change to kan. The `RelationKind` "in tension with" gap and the
  hard-claims idea are both recorded on kan's side and neither blocks this.
- Validating that acceptance criteria are genuinely *mechanically testable*,
  or that requirements are well-formed prose. Not mechanically decidable;
  it stays a job for the adversarial-review atom and for the reader.
- Rendering or round-tripping design documents from kan claims. day reads
  documents and records claims about them; it does not generate them.
- A Markdown AST or full parser. Shallow line-oriented parsing only; if a doc
  needs real parsing to validate, the schema is too clever.

## Open Questions

None remaining — the four resolved during this pass are recorded as `decide`
claims on the `design-atom-backing` subject in day's log: schema declared in
kan rather than hard-coded or config-filed; `record` parses the document
rather than taking explicit arguments; validation failure records with the
result embedded rather than gating; and successor lookup gets its own `day
next` verb rather than folding into `doctor`.
