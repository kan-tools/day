# Contributing to day

day is early and pre-1.0. Everything published so far is a prerelease, the
conventions are v0, and the parts of `docs/TELOS.md` that are not implemented
are not a backlog to burn down.

That shapes what is useful to send.

## The most useful thing you can send is a field report

**Every defect found in day so far came from running it, not from testing
it.** Not most — every one. A green suite has been necessary here and has
never been sufficient, because tests assert day's *output* while the defects
have been in what that output means, or in whether anything receives it.

So a report from using day on a real project is worth more than a patch, and
the record says so:

| report | what it moved |
|---|---|
| [#97](https://github.com/kan-tools/day/issues/97), [#98](https://github.com/kan-tools/day/issues/98) | the *v0.9 — position honesty* milestone |
| [#95](https://github.com/kan-tools/day/issues/95) | the *v0.10 — the graph tells the truth* milestone |
| [#74](https://github.com/kan-tools/day/issues/74), [#76](https://github.com/kan-tools/day/issues/76), [#77](https://github.com/kan-tools/day/issues/77) | the *v0.7.0-beta.3 — the vocabulary substrate* milestone |
| [#156](https://github.com/kan-tools/day/issues/156) | `/plugin install` never having worked from a URL |

Open one with the **Field report** template and the `field-report` label. What
makes a report actionable, in rough order of value:

- **What you ran and what it printed.** Paste it. A verdict day printed is
  more useful than a description of the verdict.
- **`day --version` and `kan --version`.** The pair matters — day declares a
  supported kan range, and `day doctor` prints it.
- **What you expected instead, and why.** Often the defect is that a report
  claimed more precision than it had, which only you can see.
- **Whether day was *wrong* or merely *unhelpful*.** Both are worth filing;
  they get triaged differently.

You do not need to diagnose it. Several of the reports above were right about
the symptom and wrong about the cause, and were still the reason the milestone
happened.

## Behaviour changes start as an issue, not as a PR

If you are about to change what day *does* — a new verb, a new probe kind, a
change to what a check reports — please open an issue first and let it get a
design pass. An unsolicited PR that changes behaviour will most likely be
asked to go back through that, which wastes your time.

The design pass is a real step, not a formality: it lands in
`.design/<slug>.md`, is recorded into kan, and is validated by `day design
check` against the schema this project declared. `docs/CONVENTIONS.md` is
authoritative for what day actually reads and writes.

Things that do **not** need this: typo and documentation fixes, a test that
pins existing behaviour, a fix for an already-filed bug where the fix is
obvious from the issue.

## When a design becomes an RFC or ADR

A validated `.design/` document is working material. A proposal additionally
needs an RFC when it changes a public process primitive, durable convention,
compatibility promise, governance or trust rule, execution-authority boundary,
or architecture spanning independently changeable components. Copy
`rfcs/template.md`; [RFC 0](rfcs/0-rfc-and-adr-process.md) defines numbering,
review, acceptance, publication, and supersession.

An ADR records a decision actually taken. It is appropriate for lasting local
implementation rationale that does not need a public RFC. An accepted RFC is
already the governing decision and should not receive a duplicate ADR; an ADR
is required when implementation materially departs from that RFC.

The merged RFC or ADR file is the normative content. Its published kan claim
provides durable identity and an exact repository artifact address, not a
second mutable copy of the document.

## The two properties a change must not break

These are day's own teloi, recorded in its log as `telos/no-store-of-its-own`
and `telos/affordance-not-enforcement`, and both have tests behind them.

**day stores nothing of its own.** Every durable thing day knows is an
ordinary kan claim, read back through kan's public CLI — no config store, no
sidecar database, no state file. If a change seems to need day-owned
persistent state, that is a signal the feature is wrong or belongs in kan.

There is one carve-out, the `.day/` render cache, and it is narrow on
purpose: nothing durable lives in it, it is strictly derived from kan and git,
it regenerates, and it is **never read to decide anything — only to display**.
Exactly one module touches it and `tests/plugin.rs` greps the rest of `src/`
to prove it. If day ever reads the cache to decide something, the carve-out
has been abused.

**Advisory, never blocking.** day's hooks inject context. They do not gate,
deny, or reject an agent's action, and a test enforces that the shipped hook
config contains no blocking construct. This is a direct lesson from
`crosslink`, whose blocking hooks caused the friction that motivated splitting
day out of kan — a tool that can block is a tool people route around.

Two more that are narrower but equally enforced:

- **day cannot alter or destroy a subject.** It only ever appends, through
  kan's public CLI, and never retracts or rejects. It must never write kan's
  files directly or bypass its signing.
- **Three substrates, three spawn sites.** kan (`src/kan_client.rs`); git,
  read-only (`src/git.rs`, which has no method that stages, commits, tags,
  checks out or pushes, and is grepped for those by `tests/assess.rs`); and
  project-declared commands (`src/probe.rs`, bounded by four rules — no shell
  ever, `--run` opt-in, never reachable over MCP, and a timeout that kills).
  Do not add a fourth spawn site.

## Where a feature belongs: day or kan

The boundary is kan's ADR-18, read from this side.

**kan owns it** if it needs a new or existing `ClaimBody` / `ClaimKind` /
`Anchor` / `RelationKind` variant, or if it is a pure read or fold over the
claim graph. **day owns it** if it is buildable as a calling convention over
kan's existing primitives — process, orchestration, multi-turn interaction.

So if your day feature would require changing kan's data model, that is a kan
design question first. File it against
[kan-tools/kan](https://github.com/kan-tools/kan), or file it here with the
`upstream-kan` label if the coordination between the two is the point.

## Two house rules that are easy to miss

**A fix that answers a review finding ships with a test that fails without
it, in the same commit.** Not a test that passes, and not a demonstration in
a terminal — a test that goes red when the fix is reverted. This is checked
rather than promised: `scripts/revert-demo.py` produces a `Demonstrated-by:`
trailer, and `.github/workflows/revert-demo.yml` re-derives every trailer on
the branch by reverting the commit again and confirming the named test
actually fails.

```bash
python3 scripts/revert-demo.py --tests plugin::some_test
```

Qualify the test target (`plugin::some_test`, not `some_test`) — the
unqualified form builds every integration target three times.

`VACUOUS` is a finding, not a nuisance. It means the fix was taken away and
the test written to close the finding passed anyway; the commit is not ready.

**Do not add a check that cannot fail.** A check that has silently stopped
detecting things is worse than no check, because it converts an unchecked area
into one that merely *looks* checked. Before you trust a new check,
demonstrate it going red. This repo has filed the same shape repeatedly: a
mutation harness that reported `SURVIVED` for a mutation that did not compile,
a `pub fn` whose only callers were its own tests, a workflow whose commit
range was always empty so it was permanently green for having found nothing.

Related, and worth knowing before you reach for a loop: `scripts/` already
has the tools. `mutate.py`, `revert-demo.py`, `demonstration-census.py` and
`capture-block-corpus.sh` each report could-not-check as distinct from
checked-and-clean, which hand-rolled versions of them have repeatedly failed
to do.

## Building and testing

```bash
cargo build --workspace --all-targets
cargo test --workspace
cargo test --test behaviour_diff -- --ignored --test-threads=1
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --all -- --check
```

The MSRV is declared in `Cargo.toml` as `rust-version` and is checked by CI's
`msrv` job, which reads the declared value rather than repeating it.

No test requires a real `kan` install — `DAY_KAN_BIN` stubs it. The one
deliberate exception is `tests/kan_conformance.rs`, which talks to the real
binary and **skips when kan is absent**; CI installs a pinned kan so it does
not skip there. If you are changing anything about how day invokes kan, that
is the file that checks it against kan's actual contract rather than against
day's idea of it.

## Issues, labels and milestones

Labels already in use, so please do not invent parallel ones:

| label | for |
|---|---|
| `bug` / `enhancement` / `documentation` | the usual |
| `field-report` | reported from using day on another project |
| `design-question` | needs a decision or a design pass, not a patch |
| `process` | about day's own working process or discipline, not its code |
| `tooling` | scripts, CI workflows, test harnesses |
| `upstream-kan` | depends on or coordinates with kan-tools/kan |
| `v1.0-bar` | bears on the v1.0 bar |

Milestones are named for what the release was *about* rather than for a
version alone (*v0.9 — position honesty*, *v0.11 — verification that can
fail*). Please leave milestone assignment to a maintainer.

## Pull requests

One PR per milestone, off `main`, merged with `--merge` rather than squashed
so a milestone's internal commits stay visible — `scripts/demonstration-census.py`
walks that history, and squashing would leave it nothing to account for.

The PR template carries the checklist. Say what you ran, and paste what it
printed.

## Code of conduct

By participating you agree to the [Code of Conduct](CODE_OF_CONDUCT.md).

## Licence

day is MIT. Contributions are accepted under the same licence.
