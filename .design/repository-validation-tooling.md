# Feature: Repository validation as a first-class development tool

## Summary

Replace the flat collection of validation scripts with a private Rust workspace
tool, exposed to maintainers and CI through a small `justfile`. The tool is not
part of day's public CLI: it is repository machinery for proving properties of
day itself. It gives checks one typed outcome model, one command-running
boundary, one Git/GitHub boundary, and one discoverable command tree.

The first migration covers validation and evidence machinery: RFC validation,
publication checks, formal vectors, behavior comparison, mutation,
revert-demonstration verification, and demonstration/finding censuses. Existing
documented script paths remain thin compatibility shims. Two GitHub issues make
their removal explicit: one sunsets the general legacy entry points and one
retires RFC0's normative `scripts/check-rfcs-adrs.sh` path through the RFC
process. Release cutting, migration/compatibility cells, corpus capture, and
rendering remain outside the first migration.

## Motivation

`scripts/` currently describes neither ownership nor authority. It contains
validators, test harnesses, renderers, release automation, compatibility cells,
and evidence-capture helpers in one namespace. CI and Rust integration tests
address those files directly, so filenames have accidentally become APIs.

The implementation also repeats the hardest semantics. Several programs parse
trailers, invoke Git or Cargo, classify subprocess failures, and decide whether
an unavailable dependency means failure or “could not check.” The RFC checker
is a shell orchestrator over Python programs and contains its own mutation
self-tests. The revert demonstration and census paths maintain parallel trailer
grammars. This is precisely where a validator can become vacuous while still
printing success.

Rearranging the directory would make the inventory prettier but preserve these
failure modes. The useful unit of cleanup is therefore a first-class development
tool with shared types and explicit boundaries, plus a deliberately small
human-facing task surface.

## Requirements

- REQ-1: Add a private, non-published Rust workspace binary named `xtask`. It
  must not add a verb, flag, or dependency to the shipped `day` binary.
- REQ-2: Add a root `justfile` as the supported human and CI entry point. `just`
  recipes are thin aliases only: they contain no validation policy and delegate
  to `cargo run -p xtask -- ...`.
- REQ-3: The command tree groups work by meaning rather than implementation:
  `validate`, `evidence`, and `census`. The first migration supplies at least
  `validate rfc`, `validate rfc --self-test`, `validate publication`,
  `validate vectors`, `evidence behaviour-diff`, `evidence mutate`,
  `evidence revert`, `evidence revert --verify`, `census demonstrations`, and
  `census findings`.
- REQ-4: Every check returns a shared typed result that distinguishes `passed`,
  `finding`, and `could-not-check`. A command may refine these into domain
  outcomes such as `VACUOUS` or `BASELINE-RED`, but it may not collapse a
  could-not-check into success or a finding.
- REQ-5: Shared adapters own process execution, repository inspection, and
  GitHub reads. Domain validators receive those capabilities explicitly rather
  than spawning commands ad hoc. Tests can replace each adapter with a fixture
  implementation and assert which authority was exercised.
- REQ-6: Validation profiles are explicit compositions of named checks. The
  initial profiles are `quick`, `ci`, and `release`; listing a profile prints
  its ordered members, and execution reports every member and its result.
  Profiles never discover checks from a directory scan.
- REQ-7: The CI workflow invokes supported `just` recipes rather than files
  under `scripts/`. The recipes used in CI are also runnable locally and print
  the delegated `xtask` command.
- REQ-8: Existing migrated script entry points remain executable compatibility
  shims during the transition. Each shim delegates without reimplementing
  policy, prints a deprecation notice to stderr, preserves arguments and exit
  behavior, and is covered by a parity test against the corresponding `xtask`
  command.
- REQ-9: `scripts/check-rfcs-adrs.sh` remains as the RFC0 compatibility surface
  until a successor RFC change removes the path-level requirement. Its body is
  only a delegation to the new tool; RFC0's promised invocation and `--self-test`
  behavior remain valid.
- REQ-10: Open and link two GitHub sunset issues before implementation is
  accepted: one enumerates all migrated legacy script paths and their removal
  criterion; the other tracks replacing RFC0's named script path with a stable
  validator contract through the RFC process. Neither issue may use a date
  alone as its removal criterion.
- REQ-11: The migration preserves the current hermetic and adversarial
  demonstrations. A check is not considered migrated until its existing tests,
  mutations, and revert demonstrations exercise the new implementation rather
  than merely exercising its shim.
- REQ-12: Release cutting, compatibility/migration cell execution, corpus and
  footer capture, foreign-contribution helpers, and denotational-semantics
  rendering remain under their present paths in this change. The new tool may
  expose no placeholder commands for them.
- REQ-13: Contributor documentation names `just --list` as the discovery
  surface, explains the three result classes, distinguishes repository tooling
  from the public `day` CLI, and documents how to add a named check and include
  it in a profile.
- REQ-14: Python and shell dependencies used only by migrated implementations
  are removed after parity is demonstrated. Compatibility shims must use only
  portable shell and must not depend on Python.

## Acceptance Criteria

- [ ] AC-1: Installing/building `day` without the workspace development member
      produces the same public binary command tree as before. (REQ-1)
- [ ] AC-2: `just --list` names the supported validation recipes, and inspecting
      every recipe shows only delegation or composition of other recipes—no
      grep, Git, GitHub, parsing, or policy logic. (REQ-2, REQ-13)
- [ ] AC-3: A command-tree snapshot covers every command named by REQ-3 and
      rejects an unknown domain or check at non-zero exit. (REQ-3)
- [ ] AC-4: Fixture tests force one pass, one substantive finding, and one
      unavailable-tool condition through the same runner and observe three
      distinct typed results and exit/report behavior. (REQ-4)
- [ ] AC-5: A validator fixture that was not granted GitHub authority cannot
      perform a GitHub read; a publication validator granted that authority
      records the exact read. The corresponding assertion holds for process
      and Git access. (REQ-5)
- [ ] AC-6: `xtask validate profile --list quick|ci|release` emits a checked-in,
      ordered inventory, and a test fails when a profile references an unknown
      check or when CI invokes an unlisted profile. (REQ-6)
- [ ] AC-7: A source check fails if a workflow invokes a migrated file below
      `scripts/`; the checked-in workflows pass and invoke `just` recipes that
      run locally. (REQ-7)
- [ ] AC-8: For every migrated legacy entry point, fixture-driven parity tests
      compare stdout payload, stderr classification, and exit status with the
      corresponding `xtask` command. The shim itself contains no policy branch
      beyond locating and executing the tool. (REQ-8)
- [ ] AC-9: Both `scripts/check-rfcs-adrs.sh` and
      `scripts/check-rfcs-adrs.sh --self-test` pass by delegation, while RFC0's
      publication and exact-artifact checks remain green. (REQ-9)
- [ ] AC-10: The design/result claim links two open GitHub issues. One contains
      the exhaustive migrated-path inventory plus evidence-based removal gates;
      the other names the RFC0 amendment and forbids removing the shim before
      that amendment is accepted. (REQ-10)
- [ ] AC-11: Each migrated validator has at least one negative fixture or
      mutation that fails when its substantive decision is inverted. Existing
      `Demonstrated-by` claims re-derive against the new implementation. (REQ-11)
- [ ] AC-12: A command-tree snapshot and changed-path check show no new `xtask`
      command and no implementation movement for any excluded operational or
      rendering tool named in REQ-12. (REQ-12)
- [ ] AC-13: Contributor documentation lets a fresh clone identify the local CI
      command and the distinction between `finding` and `could-not-check`
      without reading workflow YAML or Rust source. (REQ-13)
- [ ] AC-14: The migrated paths run in an environment without Python installed;
      no migrated implementation imports or invokes Python, and all remaining
      Python files correspond to the explicitly excluded scope. (REQ-14)

## Architecture

Add `xtask/` as a private workspace member with `src/main.rs` limited to CLI
parsing, report rendering, and exit-code selection. `xtask/src/lib.rs` exposes
the runner to integration tests. The internal modules are organized around
contracts rather than old filenames:

```text
xtask/
  src/
    command.rs          # typed command tree
    outcome.rs          # Passed / Finding / CouldNotCheck
    capability/
      process.rs
      repository.rs
      github.rs
    validate/
      mod.rs             # named checks and profiles
      rfc.rs
      publication.rs
      formal.rs
    evidence/
      behaviour.rs
      mutation.rs
      revert.rs
    census/
      demonstrations.rs
      findings.rs
  tests/
    command_tree.rs
    outcome_contract.rs
    shim_parity.rs
    profile_completeness.rs
```

`Outcome<T>` is not a boolean. `Passed(T)`, `Finding(F)`, and
`CouldNotCheck(C)` remain distinct until the outermost renderer maps them to
machine-readable output and exit codes. Domain enums such as the revert
harness's seven outcomes map into that common classification without losing
their names.

Adapters form the authority boundary. A validator receives traits for the
operations it needs, and the production implementations call Git, GitHub, or
child processes. Fixtures record attempted calls. This does not pretend that
all validators are hermetic; it makes non-hermetic requirements visible and
testable.

Profiles are checked-in Rust data or a checked-in declarative manifest parsed
into the same typed registry. The implementation should prefer Rust data unless
maintainers demonstrate a need to edit profiles without compiling. In either
case, startup validation rejects duplicate names, unknown members, and cycles.
`quick` is the inexpensive local subset, `ci` is the required pull-request
surface, and `release` is a superset of `ci`; the exact membership is recorded
during implementation after timing the current checks.

The `justfile` is intentionally boring. It provides memorable recipes such as
`just check`, `just check-rfc`, `just demonstrate`, and `just ci`, each of which
delegates to a named `xtask` command or profile. CI uses those same recipes.

Compatibility shims are leaves, never orchestration nodes. They locate the
repository, print a deprecation notice, and `exec` the equivalent `cargo run -p
xtask -- ...` invocation. This keeps accepted documentation and RFC0 true while
making the new implementation the only source of policy.

## Migration Sequence

1. Land the `xtask` skeleton, outcome contract, capability traits, profile
   registry, `justfile`, and contract tests without moving validators.
2. Migrate shared grammars and census logic first, especially the duplicated
   `Demonstrated-by` parsing, so subsequent harnesses consume one parser.
3. Migrate behavior comparison, mutation, and revert verification with their
   negative demonstrations intact.
4. Migrate RFC validation, publication checks, formal obligations, and vectors;
   retain `check-rfcs-adrs.sh` as the RFC0 shim.
5. Switch CI and contributor documentation to `just`; add source checks against
   new direct workflow dependencies on migrated scripts.
6. Remove implementation-only Python/shell files, retaining shims; open and link
   both sunset issues with their evidence gates.
7. In a later change, amend RFC0 to specify the validator contract, remove its
   shim only after acceptance, and retire the remaining shims when their issue's
   usage and parity gates are satisfied.

## Resolved Questions

- **Use a private Rust tool, not the public CLI.** Repository validation is how
  day proves itself, not vocabulary day transports to projects. Adding it to
  `day` would widen a deliberately small product surface and confuse these two
  authorities.
- **Use `just` for accessibility, not implementation.** A future maintainer and
  CI should invoke the same memorable commands, while validation semantics stay
  typed and testable in Rust.
- **Call the private crate `xtask`.** `xtask` is the established Rust convention
  for repository-development tasks, while `just` remains the supported
  human-facing surface. The conventional internal name helps Rust contributors
  recognize the crate without making them learn or type it for routine work.
- **Preserve legacy paths through shims.** Existing documentation, tests, and
  RFC0 made filenames interfaces. Abrupt removal would make accepted claims
  false; permanent duplication would preserve the architectural problem.
- **Sunset work is explicit.** One GitHub issue governs ordinary compatibility
  shims. RFC0's path gets a separate issue because its removal requires a change
  in normative authority, not merely proof that callers migrated.
- **Migrate validation before operations.** Release, compatibility cells,
  capture utilities, and rendering have different side effects and operational
  audiences. Pulling them into the first abstraction would design the command
  tree before those requirements are studied.
- **Profiles enumerate rather than discover.** A filesystem scan can report
  green after a check disappears. Named checked-in membership makes a shrinking
  validation surface observable.
- **Keep domain outcomes while sharing the top-level classification.** The
  revert harness needs `VACUOUS`, `BASELINE-RED`, and its other precise states;
  every validator also needs the common distinction between a finding and an
  inability to check.

## Deferred Questions

- Whether operational tools eventually join `xtask`, become a separate
  release-operations binary, or remain standalone is deferred until their
  authority and rollback requirements are designed.
- Whether denotational-semantics rendering belongs in a documentation tool or
  in `xtask` is deferred; it is generation, not validation merely because a
  validator currently calls it.
- Exact `quick`, `ci`, and `release` membership is deferred to an implementation
  inventory with measured runtime, but the completeness and ordering laws are
  fixed here.
- Stable machine-readable output is required internally by the typed outcome
  model; whether it becomes a supported external JSON contract is deferred.

## Non-Goals

- Adding repository-maintenance commands to the public `day` CLI.
- Rewriting release cutting, compatibility or migration cells, evidence capture,
  foreign-contribution helpers, or document rendering in the first migration.
- Removing a compatibility shim merely because a date has elapsed.
- Making every validator hermetic; external authority must instead be explicit
  and inability to exercise it must remain a could-not-check.
- Creating a general workflow engine or plugin system for validation.
