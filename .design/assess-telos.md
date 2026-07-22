# Feature: `day assess telos` — witnesses against material evidence

## Summary

The larger half of v0.4. `day assess docs` asks whether the docs still match
what shipped; this asks the question the framework exists for — **did work land
inside a telos's equivalence class**, judged against material evidence rather
than against an agent's account of what it did. It binds a telos's declared
witnesses to checkable probes declared per project, and in doing so accepts a
third substrate: day may execute project-declared commands, under guardrails
that are the substance of this design.

It also lands `day telos tension` as a real `in-tension-with` edge (day#18),
now unblocked, because the same milestone needs queryable tension and because
the prose form is actively degrading day's own session context.

## Motivation

`day bridge check` computes whether a plan **could** reach a telos: it walks the
arrangement and asks whether the target's declared witnesses are produced
somewhere along it. That is a statement about a plan, not about the world. The
question nobody can currently ask day is whether the witnesses were **actually
produced**, and by what evidence.

The gap is that a witness is a *type*. `telos/v03-shipped` declares
`published-artifact`, and many concrete artifacts of that type satisfy it
equally — that is the weak equivalence the whole model is built to preserve.
Assessing means binding the type to an instance without collapsing the telos to
that instance.

kan-tools/kan#61 is open: `kan show` exposes no artifacts and no anchors, so day
cannot ask the record which files a claim was about. The binding has to come
from substrates day can check itself. That is the same wall `day assess docs`
hit, and it produced the same answer — read the material world directly.

Two findings from this session sharpen the tension half:

- **kan#60 is closed.** The installed `kan relate --help` lists
  `in-tension-with`. day#18's blocker is gone.
- **The prose form is already costing something.** `src/hooks.rs` renders the
  newest narrative claim on each telos subject, and for four of day's five
  teloi that claim is now a `Tension: ...` decide — so session-start injects
  tension text where the telos statement belongs. Verified against the live
  log: `kan show telos/legible-process` returns the tension as its last
  `Decision`, two claims newer than the statement. This is the eighth defect
  found by running day rather than testing it, and invisible to the suite for
  the usual reason: the output is exactly what the code intends, and the defect
  is in what it means to the reader.

## Requirements

- REQ-1: `day assess telos <slug>` reads the target telos's declared witnesses
  from its newest `day-telos` block — the existing `bridge::Witnesses` type,
  loaded through `atoms::newest_fenced` — and reports per witness whether it is
  materially evidenced.
- REQ-2: The witness-to-probe mapping is **declared per project** on a
  `schema/witness` subject carrying a fenced `day-witness` block, the same
  mechanism `schema/design-doc` and `schema/docs` already use. day ships a
  starter it prints as a runnable command and never applies, exactly as
  `DocsSchema::starter_command` does.
- REQ-3: Three probe kinds. `path` — a git pathspec matching at least one
  tracked file. `tag` — a git tag pattern matching at least one tag. `command`
  — an argv whose exit status is the evidence, zero meaning satisfied.
- REQ-4: A `command` probe executes **only** when `--run` is passed. Without
  it, the probe is reported as not run, printing the exact argv that would have
  executed, so a reader sees what they are authorizing before authorizing it.
- REQ-5: day **never invokes a shell**. A command probe's argv is split on
  whitespace and executed directly, the way `KanClient::run` and `Git::run`
  already spawn. There is no `sh -c` path, so shell metacharacters in a claim
  cannot become operators in a subprocess.
- REQ-6: All command execution lives behind a single module, and no other
  module spawns a process for a probe — the property that makes REQ-5
  greppable rather than merely intended, mirroring how `src/git.rs` confines
  git.
- REQ-7: A command probe is bounded by a timeout, default 120 seconds and
  overridable with `--timeout`. On expiry the child is killed and the probe is
  reported as timed out, distinct from both satisfied and unsatisfied.
- REQ-8: The MCP `assess_telos` tool never executes a command probe. There is
  no argument an MCP caller can pass that sets `--run`.
- REQ-9: Two tiers with different powers, following `src/docs.rs`. The
  **material** tier reports probe results and alone determines the exit code. The
  **record** tier reports what the log says — the claims on the telos subject,
  whether any is an assessment, and what any bridge targeting this telos
  computes — and only ever prompts.
- REQ-10: The report visibly distinguishes **materially witnessed** from
  **asserted in prose**. A witness evidenced only by a narrative claim is
  reported as such and never counted as material.
- REQ-11: A telos declaring no witnesses, or a witness with no declared probe,
  is named and reported as not mechanically assessable — the move
  `bridge::Report::render` already makes — rather than being guessed at or
  silently passed.
- REQ-12: `day assess telos` writes nothing to kan or the working tree. It
  prints a runnable `kan result --subject telos/<slug> --cites <cid>` for the
  reader, keeping performing an assessment separate from recording one.
- REQ-13: `day telos tension a b <why>` emits a `kan relate telos/a
  in-tension-with telos/b` edge **and** the prose claim carrying the reason,
  because a kan relation has no narrative body. The verb's argument surface
  does not change.
- REQ-14: Tensions already recorded as prose are **not rewritten**. Edges are
  back-filled alongside them, citing the prose claims they correspond to, and
  day reads both forms.
- REQ-15: `docs/CONVENTIONS.md` documents the `schema/witness` convention and
  the `day-witness` block; `CLAUDE.md` records the shell as day's third
  substrate with the guardrails that bound it.
- REQ-16: The vocabulary gains **two** assessment atoms, closing the gap where
  `day assess docs` shipped without any atom describing it. `atom/assess-docs`
  takes `published-artifact` and composes from `atom/release`;
  `atom/assess-telos` takes `code-change` and composes from
  `atom/generative-build`. Both produce `assessment`.

## Acceptance Criteria

- [ ] AC-1: Given a telos whose newest claim carries a `day-telos` block with
      two witnesses, `day assess telos <slug>` names both and reports a status
      for each. (REQ-1)
- [ ] AC-2: With no `schema/witness` declared, the command explains and prints a
      runnable command recording the starter, and records nothing. Changing that
      claim to map a witness to a different probe changes what is checked with no
      code or config file edited. (REQ-2)
- [ ] AC-3: A `path` probe matching a tracked file is satisfied and one matching
      nothing is not; likewise a `tag` probe against a repository with and
      without a matching tag. (REQ-3)
- [ ] AC-4: Without `--run`, a command probe is reported as not run and the
      stub it names is never executed — asserted by the stub writing a sentinel
      file that must not exist afterward. With `--run`, the same probe executes
      and its exit status determines the result. (REQ-4)
- [ ] AC-5: A probe whose argv contains shell metacharacters (`;`, `|`, `&&`,
      backticks) executes none of them: with `--run`, the metacharacters are
      passed as literal arguments and no second process runs. (REQ-5)
- [ ] AC-6: A test greps the source tree and fails if `Command::new` appears
      outside `src/git.rs`, `src/kan_client.rs`, and `src/probe.rs`, or if
      `src/probe.rs` invokes `sh`, `bash`, or `-c` — the whitelist style
      `tests/assess.rs` already uses for git. (REQ-6)
- [ ] AC-7: A command probe that sleeps past `--timeout` is killed and reported
      as timed out, and the assessment returns rather than hanging. (REQ-7)
- [ ] AC-8: An MCP `tools/list` includes `assess_telos`; calling it returns the
      same text as the CLI verb run *without* `--run` for the same repository
      state, and a command probe's stub sentinel does not exist afterward.
      (REQ-8)
- [ ] AC-9: An unsatisfied material probe exits non-zero; a run where only
      record-tier prompts and not-run probes remain exits zero. (REQ-9)
- [ ] AC-10: A telos with a witness evidenced only by a narrative claim on its
      subject reports that witness as asserted rather than material, and the
      material tier does not count it as satisfied. (REQ-10)
- [ ] AC-11: A telos with no witnesses, and a witness absent from the declared
      probe map, are each named in the output as not mechanically assessable.
      (REQ-11)
- [ ] AC-12: Running `day assess telos` against a repository leaves the kan log
      and the working tree byte-identical, asserted by comparing `kan status`
      and `git status --porcelain` before and after; the printed `kan result`
      command is present in the output. (REQ-12)
- [ ] AC-13: `day telos tension a b "why"` produces both an `in-tension-with`
      relation between the two subjects and a claim containing the reason text,
      with the same arguments the verb accepted before this change. (REQ-13)
- [ ] AC-14: After back-filling, the prose tension claims on
      `telos/legible-process` and `telos/no-store-of-its-own` are still live
      and unmodified, with their original CIDs resolvable. (REQ-14)
- [ ] AC-15: `docs/CONVENTIONS.md` contains the `schema/witness` prefix usage
      and the `day-witness` fence token, checked against the code's own
      constants, and `CLAUDE.md` names the shell as a substrate. (REQ-15)
- [ ] AC-16: After declaring both atoms, `day doctor` reports seven atoms and
      `composition: ok`, and `day next release` names `assess-docs` while
      `day next generative-build` names `assess-telos`. (REQ-16)

## Architecture

**`src/probe.rs` (new)** is the third substrate, confined the way
`src/git.rs` confines the second. It holds the `Probe` enum (`Path`, `Tag`,
`Command`), a `Verdict` (`Satisfied`, `Unsatisfied`, `NotRun`, `TimedOut`), and
the execution guardrails. Command execution splits the argv on whitespace and
calls `std::process::Command::new(argv[0]).args(&argv[1..])` — the same shape
`KanClient::run` uses at `src/kan_client.rs` and `Git::run` at `src/git.rs`.
There is deliberately no `sh -c`, which is what makes AC-5 hold by construction
rather than by escaping. The timeout is a spawn plus a `try_wait` poll to a
deadline, killing the child on expiry, because `std::process` has no built-in
deadline and pulling tokio into a synchronous path for this would be a larger
change than the feature warrants.

**`src/telos.rs` (new)** mirrors `src/docs.rs` closely, because it is the same
shape of thing: a `WitnessSchema` loaded through the existing
`atoms::newest_fenced`, a material tier, a record tier, and a `Report::render`.
Keeping the two tiers as separate types is what makes the exit code depend only
on the material one, exactly as `docs::Report::is_clean` does.

**`src/git.rs`** gains `tags_matching(pattern)` and `tracked_files(pathspec)`.
`latest_version_tag` becomes a caller of the former rather than a second
implementation. Both are `git tag --list` and `git ls-files`, so the read-only
whitelist in `tests/assess.rs` extends by one entry and its guarantee is
unchanged.

**`src/kan_client.rs`** gains `relate(a, kind, b, cites)`. The existing
`append` builds `<verb> <text> --subject <s>`, and `kan relate <A> <KIND> <B>`
does not fit that shape — it takes two positional subjects and a kind, with no
text. This is a second write path through kan's public CLI, not a widening of
what day may do: kan still signs and owns the log format, and there is still no
destroy path to reach.

**`src/cli/mod.rs`** gains `AssessAction::Telos { slug, all, run, timeout }`
beside the existing `Docs` leaf, and `TelosAction::Tension`'s handler emits the
edge before the prose claim so the claim can cite it.

**`src/mcp.rs`** gains one `assess_telos` tool dispatching to the same function
the CLI calls, with `run` hard-wired false. The equivalence assertion in
`tests/mcp_server.rs` already covers every listed tool — a prior session's
finding recorded on the `assess-docs` subject — so this is picked up
automatically rather than needing a bespoke test.

**Nothing here writes an assessment.** `day assess telos` reads kan, reads git,
optionally runs declared probes, and prints. Recording is a separate act, for
the reason `.design/assess-docs.md` gives: conflating "I checked" with "I
recorded that I checked" lets the tool manufacture its own evidence.

## Resolved Questions

- **Command probes are in, and the shell becomes day's third substrate.**
  Chosen over a reads-only `path` + `tag` set, because "builds, tests, diffs"
  is what `docs/CONVENTIONS.md` already names as material evidence and a
  witness like `passing-tests` is not expressible without execution. The cost
  is real and is recorded in `CLAUDE.md` rather than left to be discovered.
- **Recorded-discharge claims were rejected as the evidence mechanism.** Having
  a witness discharged by appending a claim that names an artifact would need
  no new substrate at all, but the evidence would then be day's own prose —
  which `docs/CONVENTIONS.md` already says is worth much less than a claim
  citing an artifact. It would let the tool satisfy its own assessment by
  writing a sentence.
- **No shell, ever.** The argv is split and executed directly rather than
  passed to `sh -c`, so metacharacters arriving from a claim are inert. This
  costs pipelines and redirection in probe definitions, which is an acceptable
  loss for a check whose whole value is being hard to game.
- **`--run` is opt-in per invocation, and unavailable over MCP.** Execution is
  authorized by a human at a terminal each time, never by an agent calling a
  read-shaped tool.
- **`path` probes go through `git ls-files` rather than a new glob
  dependency.** It adds no crate, reuses a substrate day already reads under an
  existing whitelist, and tracked-in-git is *stronger* evidence than a file
  existing on disk — a build output or a stray local file should not witness a
  telos.
- **Not-run is not failure.** A command probe skipped for want of `--run` is
  absence of evidence, not negative evidence, and exits zero; treating it as a
  failure would make every default invocation look broken.
- **`day telos tension` emits the edge and keeps the prose claim.** kan
  relations carry no narrative body — verified against this repo's log — so the
  edge alone would discard the reason, which is the part a reader needs.
- **Existing prose tensions are not rewritten.** They are real claims. Edges
  are back-filled alongside them and day reads both forms.
- **Two assessment atoms, not one.** `atom/assess-docs` takes
  `published-artifact` at the release boundary; `atom/assess-telos` takes
  `code-change` and does not require a release to run. Collapsing them into a
  single `atom/assess` would have to claim a shared interface they do not
  have — assessing docs against a shipped artifact and assessing a telos
  against material evidence are different process units, and the vocabulary
  should say so rather than average them.
- **The tension edges are back-filled by hand**, in four `kan relate`
  invocations, not by a migration verb. `docs/CONVENTIONS.md` already holds
  that the conventions are the contract and a hand-written claim following them
  is exactly as valid, so a migration needs no surface — and a verb correct
  exactly once is a poor addition to a CLI whose smallness is a stated telos.

## Out of Scope

- **Drift detection** — teloi that shifted without the shift being recorded,
  unstated teloi inferred from what work optimized, silent trade-offs between
  teloi in tension. It is v0.4's remaining item and wants the queryable tension
  edge this milestone lands, which is an argument for doing it next rather than
  now.
- **Cross-frame assessment.** Realizability and assessment alike remain
  frame-internal, and the report says so rather than letting a single-frame
  result read as settled. Frames are v0.5.
- **Tracking whether a bridge's steps happened.** Unchanged from
  `docs/CONVENTIONS.md`: day checks whether an arrangement could reach a telos
  and whether witnesses exist, not how far along anything is.
- **Probe kinds beyond the three.** No HTTP probe, no registry probe, no
  structured output parsing. Exit status and existence are enough to learn
  whether this shape works.
- **Writing assessments into kan**, and any change to kan itself.
