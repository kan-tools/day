# Release: v0.12.1-beta.1 — the audit closes at the mechanism

## Summary

`v0.12.1-beta.1` is the bugfix release produced by the full-repository audit of
`v0.12.0-beta.4`. It closes every material audit finding, absorbs adjacent
honesty and harness defects that share the same mechanisms, completes the
previously designed three-state read and `day config` work, and makes the
release record reproduce what was actually verified. The release is not ready
because the ordinary suite is green; it is ready only when the hostile cases
that produced the findings are green and the audit and fix verdicts are
traceable in kan.

This serves `telos/honest-reads`, `telos/legible-configuration`,
`telos/legible-process`, `telos/surface-honesty`, and
`telos/no-store-of-its-own`. It accepts the existing tension with
`telos/affordance-not-enforcement`: display and hook paths remain advisory and
exit zero, while verification and release machinery refuse when they cannot
establish an answer.

## Requirements

- REQ-1: In a Git worktree whose main checkout contains a kan workspace but the
  current checkout does not, day must never report that no teloi, atoms, or
  project practice are recorded. Every ambient and manual surface that would
  make that inference instead reports that it is reading a different or
  uninitialised kan workspace, with the main checkout path named.

- REQ-2: `v0.12.1-beta.1` does not make day redirect kan reads or writes to the
  main checkout. Sharing one process log across worktrees is day's workflow
  opinion, but selecting a workspace while preserving the current worktree's
  Git anchor requires a kan-owned public mechanism. Running kan with the main
  checkout as its cwd is forbidden because it can anchor a worktree write to
  the wrong commit. The upstream dependency and the day adoption step are
  recorded explicitly.

- REQ-3: `KanClient::show` and `atoms::newest_fenced` complete the three-state
  migration designed in `.design/read-visibility.md`: present, genuinely
  absent, and unreadable/withheld are distinct values a caller must handle.
  The migration covers every call site and removes hand-taught guards whose
  behavior can drift.

- REQ-4: The log-wide withheld count is not treated as subject-specific
  evidence. Partial, fully withheld, unaccounted, and genuinely absent reads
  retain distinct meanings independent of whether `status --json` or
  `show --all --json` happens first. The real kan shapes on which this depends
  are conformance-tested.

- REQ-5: A positive command witness may use a backward-compatible structured
  form carrying `argv` and `found_nothing_exit`. Exit zero means material,
  exactly the declared non-zero code means missing, and every other non-zero,
  signal termination, spawn failure, wait failure, and timeout means
  could-not-check. Existing string-valued command witnesses retain their
  released interpretation.

- REQ-6: `day config` is implemented in full according to
  `.design/day-config.md`: a read-only rendered and versioned JSON surface that
  reports every schema value day reads, its per-key provenance and layer, and
  an unreadable state distinct from default or absence.

- REQ-7: The configuration inventory is derived from production loaders rather
  than repeated by hand. The provenance-preserving reader is also the reader
  used by those loaders; `day config` must not create a second evaluator whose
  agreement is entrusted only to tests. Config-struct and witness-map shapes
  are supported; list and arbitrary-map merging must either be implemented
  with explicit semantics or remain visibly unsupported rather than omitted.

- REQ-8: The live `telos/legible-configuration` witnesses are revised only
  after REQ-3 and REQ-6 land. They name real test targets that assert the full
  telos—complete inventory, per-key provenance, and three-state reads—and each
  target is shown to fail when its protected behavior is reverted. A missing
  target can never stand in as the intended red state.

- REQ-9: The ETXTBSY retry policy applies at every test-helper exec site that
  creates and immediately runs a stub, not only inside production
  `KanClient`. The release-tag CI reproduction at `tests/fallbacks.rs` cannot
  fail with `ExecutableFileBusy`, and the fix does not hide persistent exec
  failures.

- REQ-10: `scripts/revert-demo.py` classifies compilation from a dedicated
  build result or another positive compiler signal, never by searching
  combined test output for the phrase `could not compile`. A failing test may
  quote that phrase without being reclassified as DID-NOT-COMPILE.

- REQ-11: Windows bootstrap reachability is resolved, not merely measured. The
  plugin either supplies a Windows-native path that emits the same missing
  binary guidance, establishes from authoritative harness behavior that the
  POSIX hook is always dispatched through a compatible shell, or declares the
  platform unsupported visibly. Silent exit-zero with no guidance is not an
  accepted outcome.

- REQ-12: `.design/*.md` is an actual design corpus. The preregistered review
  exit bar is either moved to a distinct rubric/evaluation directory or
  converted into a schema-valid design document. A sweep validates every
  tracked design and accumulates all failures instead of stopping at the first.

- REQ-13: Every plan-bearing subject that represents shipped work has a
  traceable disposition: an adversarial-review verdict, a recorded exemption
  explaining why it is not an implementation plan, or an explicit open status.
  The fix does not fabricate reviews for historical work; it makes absence
  legible and prevents `atom/adversarial-review` from calling the corpus done.

- REQ-14: The audit of `v0.12.0-beta.4` is recorded in kan as a BLOCK verdict
  citing `bafyreic4wf3xg2pisr23qqn2nqyts2ehhi722ylpfcatnjh6msdjgxzzsy`, with
  each material finding as its own citing claim. Recording first restores or
  adopts the existing workspace identity; it never mints an accidental second
  identity. The `v0.12.1-beta.1` review later cites this design's plan claim.

- REQ-15: Release verification includes build, full tests, clippy with warnings
  denied, format, all-design validation, `day doctor`, `day assess docs`, the
  runnable telos assessments affected here, worktree hostile cases, Windows
  bootstrap coverage, and the release's own migration/block corpus rows. A
  failure in any required job blocks publication rather than being outweighed
  by a successful publish job.

- REQ-16: Release metadata moves coherently to `0.12.1-beta.1`: crate and plugin
  manifests, lockfile, README pins/status, changelog, kan compatibility
  measurement, migration expectations, block corpus, Git tag, GitHub Release,
  crates.io artifact, and release claim. Artifact verification installs the
  published crate into a scratch root and exercises it against this repository.

- REQ-17: The work preserves the repository invariants: hooks remain advisory;
  day writes durable state only through kan; no subject is destroyed; the
  `.day/` cache remains derived display state; git access remains read-only;
  command execution remains no-shell, opt-in, bounded, and unavailable over
  MCP; no citation edge is fabricated or dropped.

- REQ-18: Scope narrowing is recorded per item. An issue may leave this release
  only when its mechanism is genuinely independent, its residual behavior is
  named, and an acceptance criterion proves the released path does not imply it
  was fixed. “Follow-up” is not a disposition for a failing invariant.

## Acceptance Criteria

- [ ] AC-1: (REQ-1, REQ-2) A fixture creates a main checkout with a populated
      `.kan/` and a detached worktree without one. `day hook session-start`,
      `day status`, `day status-line`, and the recomputing user-prompt path never
      contain “No teloi are recorded” or “No process atoms are declared”; they
      name the workspace mismatch and main checkout. A write attempted from the
      worktree is not redirected to the main checkout.

- [ ] AC-2: (REQ-2) A regression test gives the main and worktree checkouts
      different HEAD commits and proves no day write launched from the worktree
      invokes kan with the main checkout as cwd. The design records the upstream
      kan issue and the public workspace-selection contract day needs.

- [ ] AC-3: (REQ-3) Compile-fail fixtures demonstrate that callers of both
      `KanClient::show` and `atoms::newest_fenced` cannot omit the unreadable
      state. A derived call-site census has zero unmigrated consumers.

- [ ] AC-4: (REQ-3, REQ-4) Real-kan conformance cases cover present, absent,
      partial-withheld, fully-withheld, and unaccounted subjects in both read
      orders. Each produces a distinct typed/rendered result and repeated reads
      are order-independent.

- [ ] AC-5: (REQ-5) A structured positive command probe returns MATERIAL for
      exit 0, MISSING only for its declared `found_nothing_exit`, ERROR for a
      second non-zero code and spawn/wait failure, and TIMEOUT for timeout. The
      legacy string form's behavior is pinned byte-for-byte.

- [ ] AC-6: (REQ-5) `scripts/foreign-contribution.sh` is driven through a
      simulated negative result and a simulated infrastructure/rate-limit
      result; only the former renders MISSING. MCP cannot authorize either
      command.

- [ ] AC-7: (REQ-6, REQ-7) `day config` and `day config --json` report the
      complete derived configuration inventory. Fixtures distinguish default,
      legacy whole-block, per-key, retracted-key, and unreadable provenance,
      including two keys set by different CIDs.

- [ ] AC-8: (REQ-6, REQ-7) Every production schema loader obtains its effective
      value from the provenance-preserving layer used by `day config`. A source
      or type-level guard fails when a second direct evaluator is introduced.
      Counting stubs prove zero kan writes and a before/after tree snapshot
      proves the verb creates no day-owned state.

- [ ] AC-9: (REQ-8) `day assess telos legible-configuration --run` invokes only
      existing targets, exits zero on the completed implementation, and reports
      both witnesses MATERIAL. Reverting the config inventory assertion and the
      three-state read independently makes the corresponding witness MISSING.

- [ ] AC-10: (REQ-9) A stress test repeatedly writes and immediately executes
      the fallback stub through the shared retry helper. A transient ETXTBSY is
      retried within a fixed bound; a persistent ETXTBSY returns a distinct
      could-not-run result. The exact release-tag CI failure no longer recurs.

- [ ] AC-11: (REQ-10) A revert-demo fixture whose ordinary assertion failure
      quotes a source file containing `could not compile` is classified as a
      test failure, while an actual build failure is classified
      DID-NOT-COMPILE. Both cases restore the tree byte-identically.

- [ ] AC-12: (REQ-11) Windows CI drives the actual registered bootstrap command
      in every supported dispatch mode with both binaries absent. Every
      supported mode exits zero and emits the two pinned install commands;
      unsupported mode produces an explicit diagnostic and cannot silently
      succeed with empty output.

- [ ] AC-13: (REQ-12) A command derived from the tracked `.design/*.md` set runs
      `day design check` for every file, reports all failures, and exits zero
      only when all pass. `review-exit-bar.md` no longer enters that set unless
      it satisfies the live schema.

- [ ] AC-14: (REQ-13) A record census lists every plan-bearing subject in
      exactly one of reviewed, exempt-with-reason, or open. The ten subjects
      reported incomplete by the `v0.12.0-beta.4` audit cannot disappear from
      the output merely because the release boundary moves.

- [ ] AC-15: (REQ-14) `kan show` displays the audit BLOCK citing the beta.4
      release claim and separate cited findings for the worktree false absence,
      release-tag CI failure, positive-command ambiguity, broken telos
      witnesses, invalid design-corpus member, and incomplete process record.
      The author DID equals an author already present in the workspace before
      the write.

- [ ] AC-16: (REQ-15) The exact verification commands listed in REQ-15 pass on
      the release commit locally and in required CI. A test intentionally
      failing any one stage prevents the release job from publishing.

- [ ] AC-17: (REQ-16) `day assess docs` reports tag and release claim agreement;
      install-document tests derive the current pins; every released tag has
      migration and block-corpus rows; the installed published binary reports
      `day 0.12.1-beta.1`, composes the live atom vocabulary, and reproduces the
      worktree diagnostic rather than the beta.4 false absence.

- [ ] AC-18: (REQ-17) Existing invariant scans and conformance suites remain
      green, and targeted tests show hooks exit zero, MCP exposes no command-run
      authorization, git's permitted subcommand set contains no mutator, only
      `src/cache.rs` touches `.day/`, and kan writes retain all supplied cites.

- [ ] AC-19: (REQ-18) The final review contains a table for #142, #160, #162,
      #168, #177, #178, the revert-demo defect, and every audit finding, each
      marked fixed, accepted with evidence, or still blocking. No row is
      silently omitted and no accepted row violates an invariant.

## Architecture

### Worktree workspace honesty

`src/git.rs::common_dir` and `src/git.rs::toplevel` already distinguish the
main checkout from a worktree for the footer. Reuse that fact at the
`KanClient` construction boundary in `src/cli/mod.rs` and `src/hooks.rs`: when
the current checkout lacks `.kan/` but the main checkout has it, attach a typed
workspace-mismatch diagnostic to the read context. Consumers render that state
instead of interpreting kan's successful empty log. Do not change
`KanClient::cwd` to the main checkout. The latter would make reads appear fixed
while automatic Git artifacts on writes describe the wrong branch.

The long-term resolution belongs across the boundary: kan exposes an explicit
workspace selector whose log root and Git anchor root are not accidentally
coupled; day adopts it because shared process memory is its declared worktree
policy. Until that contract exists, degraded-and-honest is the complete local
fix.

### One typed read contract

Implement `.design/read-visibility.md` in `src/kan_client.rs`, `src/atoms.rs`,
and all consumers. The read state carries the evidence available at the point
of distinction rather than asking each renderer to reconstruct it from
`excluded_by_trust`. Consolidate #160 and #162 here: one fold establishes
subject visibility, log-wide narrowing remains log-wide, and both bulk/status
envelopes are pinned against real kan.

### Positive command outcomes

Move `found_nothing_exit` to a structured command declaration in
`src/probe.rs`, using an untagged representation so existing JSON remains
valid. Both positive and forbidden command probes consume the same
`CommandOutcome`; only the final interpretation differs. Preserve the
no-shell, explicit authorization, timeout, and MCP restrictions.

### Configuration legibility

Finish `.design/day-config.md` through `src/layers.rs`, the schema loaders, a
new rendering module, `src/cli/mod.rs`, and `src/mcp.rs` only if the machine
read is safe to expose there. The audit rejects the draft's proposed second
reader: production loaders and `day config` must share the same
provenance-carrying fold, with adapters returning only the value to existing
callers. This is the structural closure of the drift risk AC-9 previously left
to pairwise tests.

After it lands, revise the kan claims on
`schema/witness/legible-config` and `schema/witness/three-state-read` to point
at real integration targets. Their current absence was once a deliberate
falsifiability check; after implementation it becomes broken verification and
must not survive the release.

### Harness and release integrity

Extract the bounded ETXTBSY behavior so production and test stub execution use
one helper. Repair `scripts/revert-demo.py` by separating build from test rather
than parsing phrases controlled by tests. Resolve Windows bootstrap at the
registered plugin boundary, not merely in an isolated shell script test.

The former review exit bar now lives at
`docs/audits/harness-footer-review-exit-bar.md`, an audit/rubric location; it is
not padded into a feature design. Add an accumulating design-corpus
check to CI and release verification. Extend the existing census tooling rather
than hand-writing historical dispositions.

### Recording and release sequencing

Before any new kan write, `kan identity did` must resolve to an author already
present in `kan identity authors`; otherwise restore or adopt the existing key
and stop. Record the beta.4 audit first, then this design, then implementation
findings and the cold review. `scripts/cut-release.sh` remains the release
orchestrator, expanded so required CI cannot be red while publication is green.
No tag is moved after consumption; a failed candidate burns the prerelease
number once any external artifact exists.

## Resolved Questions

- RQ-1: day owns the policy that worktrees should share process memory; kan owns
  workspace selection and anchoring. This release detects and reports the
  mismatch but does not redirect kan to the main checkout.

- RQ-2: Positive command probes gain a backward-compatible structured form
  with `found_nothing_exit`; legacy string probes retain released semantics.

- RQ-3: The release implements the full three-state read and `day config`
  designs, then revises the legible-configuration witnesses to assert the
  completed behavior rather than merely renaming them to tests of a subset.

- RQ-4: The release absorbs adjacent #160, #162, #177, #178 and the
  revert-demo classifier defect because they share the audit's honest-read or
  verification mechanisms. Breadth does not waive independent acceptance
  evidence for each item.

- RQ-5: The invalid review exit bar is treated as a corpus-classification bug,
  not padded with empty design sections merely to satisfy the checker.

- RQ-6: The beta.4 audit is recorded with the workspace's existing identity
  before the bugfix implementation record. A missing signing key blocks record
  mutation but does not license minting a replacement identity.

## Out of Scope

- Implementing kan's shared-workspace selector inside day or by reaching into
  kan storage. That requires an upstream kan design and public CLI contract.
- Stable `0.12.1`; this remains a prerelease and preserves the repository's
  explicit-install convention.
- New process vocabulary unrelated to closing the enumerated audit and adjacent
  issues. The existing atom graph may be repaired where its completion census
  is false, but this release does not redesign the full process model.
- General terminal or output redesign beyond the Windows bootstrap reachability
  and truthful diagnostics required here.
- Automatically creating or closing GitHub issues. Issue disposition remains a
  maintainer action after the acceptance evidence is recorded.
