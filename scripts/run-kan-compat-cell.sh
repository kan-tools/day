#!/usr/bin/env bash
# One cell of the kan<->day compatibility matrix: this day commit against one
# released kan.
#
# Emits a single token on stdout. Three of the four are the cell's committed
# outcome; the fourth is the refusal to record one.
#
#   ok                     every conformance test passed against this kan   (exit 0)
#   incompatible           at least one conformance test failed             (exit 0)
#   could-not-run          the measurement did not happen. NOT a fact about
#                          the pairing, and never a table row               (exit 2)
#
# **`unbuildable` is deliberately NOT in this vocabulary**, and removing it is a
# fix rather than a simplification. That outcome means "this kan tag does not
# build", which `.github/workflows/kan-compat.yml` decides with its own `cargo
# install` step BEFORE this script is invoked — by which point the kan binary
# either exists or the workflow has already written the row. Everything this
# script compiles afterwards is DAY'S OWN conformance suite, so a `could not
# compile` here is a fact about day, and publishing it as `unbuildable` is a
# durable claim about the wrong program. Measured: a cargo emitting cargo's
# ordinary "could not compile `day` (test \"kan_conformance\")" produced
# `unbuildable` at exit 0, cached and compared. Found by a cold review, in the
# commit written to stop exactly this class of mislabelling.
#
# `could-not-run` exits non-zero because could-not-check outranks
# checked-and-clean, and a caller that cannot tell the two apart will transcribe
# the first as the second. That is not a hypothetical failure mode here: it is
# what `scripts/mutate.py` and `scripts/demonstration-census.py` were each
# rewritten to stop doing.
#
# The question is deliberately narrow: **does day's argument shape parse, and
# does day read back what it wrote?** That is what `tests/kan_conformance.rs`
# asks, and it is the only suite in the repo that talks to a real kan — every
# other test stubs it through `DAY_KAN_BIN`, which validates day against day's
# own idea of kan's CLI rather than against kan's contract.
#
# `real_kan()` in that suite resolves `kan` through PATH and deliberately
# ignores `DAY_KAN_BIN`, because a stub is exactly what it must not talk to.
# So a cell selects its kan by *prepending to PATH*, and the suite stops
# skipping because kan is now present.
set -euo pipefail

kan_bin="${1:?usage: run-kan-compat-cell.sh <path-to-kan-binary>}"

if [ ! -x "$kan_bin" ]; then
    echo "no executable at $kan_bin — the caller is responsible for building" >&2
    echo "kan; this script cannot tell a tag that will not build from a path" >&2
    echo "that was never written." >&2
    echo "could-not-run"
    exit 2
fi

# **This script does NOT verify that the binary is kan, and that is a decision
# rather than an omission.**
#
# A cold review found that handing it a non-kan executable produced
# `incompatible` at exit 0 — a durable fact about a pairing nobody measured. The
# obvious remedy, a preflight, does not work and the reasons are worth keeping:
#
#   - Identity cannot be read from `--help`. clap derives the usage line from
#     argv[0], so a real kan invoked as `kan-0.12.0` prints
#     `Usage: kan-0.12.0`, and ANY binary placed at a path named `kan` prints
#     `Usage: kan`. The check verifies the caller's filename, not the program.
#   - Identity cannot be read from behaviour either, because for every kan
#     before 0.9.1 "this is not kan" and "this is kan, too old to do anything
#     day needs" are the SAME observation — which is exactly what those nine
#     `incompatible` rows record. A check strong enough to catch an impostor
#     rejects the genuine old versions the table exists to hold.
#   - The tagline changes between releases, so matching it trades a false-fact
#     risk for a guaranteed-red matrix on kan's next reword.
#
# So the guarantee lives where it already holds: `.github/workflows/kan-compat.yml`
# installs kan from a PINNED GIT TAG in this repo's own matrix, which establishes
# provenance before this script is ever invoked. `the_matrix_installs_kan_from_a_pinned_source`
# asserts that, so the guarantee is checked where it is made rather than
# re-derived here from evidence that cannot carry it.
#
# A caller supplying a binary by hand owns its identity. That is a narrower
# promise than "the cell validates its input", and it is the true one.

# A directory containing only this kan, prepended to PATH. Prepending the
# binary's own directory would be wrong when it sits beside other tools.
shim="$(mktemp -d)"
ln -sf "$(cd "$(dirname "$kan_bin")" && pwd)/$(basename "$kan_bin")" "$shim/kan"

# Unset so a stray value in the environment cannot redirect a test that is
# supposed to be talking to the real thing.
unset DAY_KAN_BIN || true

# **`timeout` is GNU coreutils and stock macOS does not ship it.** Bounding a
# hung cargo is worth having and is not worth making the cell unrunnable on a
# developer's machine, so its absence degrades — loudly, on stderr, because a
# silent degrade would remove the guarantee exactly where nobody is looking.
# CI runs on ubuntu, where it is present.
#
# (No `fallback:` marker: tests/fallbacks.rs scans `src/**.rs` only, so writing
# one here would look registered and be read by nothing. The degrade is covered
# by `the_cell_does_not_require_gnu_coreutils` instead.)
if command -v timeout >/dev/null 2>&1; then
    bound="timeout ${CELL_TIMEOUT:-900}"
else
    echo "note: \`timeout\` is not on PATH (GNU coreutils; stock macOS does not" >&2
    echo "ship it), so a hung cargo will NOT be bounded in this run." >&2
    bound=""
fi

# `--skip conformance_kan_78`: that test asserts a property of KAN, not a
# dependency of DAY. day emits `kan result` with its subject positionally and
# never with `--subject`, so a kan predating kan#78 serves day fine. Measured
# as one suite it made day's floor wrong by five releases — every kan through
# v0.7.0 read as `incompatible` when what they lacked was a convenience day
# does not use. The cell's question is "does day work against this kan", and
# only tests of day's own dependencies may answer it.
# Compiling and running are separate questions, and a cell that asks them as one
# answers the wrong one. This used to be a single `cargo test` whose failure was
# sorted by grepping the log for "could not compile" — so exactly one toolchain
# problem was distinguished, and every other one (a bad CARGO_TARGET_DIR, a
# cargo that never started, a killed process) fell through to `incompatible`,
# which is the compatibility fact the comment above says must not be
# manufactured. It is not hypothetical: an empty CARGO_TARGET_DIR in the
# environment produced four `incompatible` cells against a kan day demonstrably
# works with, and only prior knowledge of the answer caught it.
if ! PATH="$shim:$PATH" $bound \
        cargo test --quiet --no-run --test kan_conformance \
        >/tmp/kan-compat.log 2>&1; then
    # Every failure here is could-not-run, including `could not compile`: what
    # failed to compile is DAY, and day failing to build says nothing about the
    # kan tag. See the header.
    echo "day's conformance suite did not build, or cargo never ran. This is a" >&2
    echo "fact about day or about this machine, NOT about the kan tag." >&2
    echo "See /tmp/kan-compat.log." >&2
    echo "could-not-run"
    exit 2
fi

# `|| status=$?` rather than a bare call: `set -e` would take a failing suite —
# the `incompatible` outcome this exists to report — as a reason to abort.
# `timeout` bounds a hung compiler or suite, which otherwise never reaches any
# outcome at all and consumes the job to GitHub's outer limit — a failure class
# the four-state vocabulary named and did not handle. 124 is `timeout`'s own
# exit code, and it lands in the could-not-run branch below because the harness
# rendered no verdict.
status=0
PATH="$shim:$PATH" $bound \
    cargo test --quiet --test kan_conformance -- \
    --skip conformance_kan_78 >/tmp/kan-compat.log 2>&1 || status=$?

# Keyed on the POSITIVE signal — did the harness render a verdict — rather than
# on the absence of a phrase. `CLAUDE.md` records the same defect in the
# mutation harness, which grepped for `FAILED` and so read a build error as a
# survived mutation. An outcome classifier whose good path is "the bad phrase
# was absent" fails toward false confidence every time.
passed=$(awk '/^test result: ok\./ { print $4; exit }' /tmp/kan-compat.log)

if [ "$status" -eq 0 ]; then
    # A suite that ran and asserted nothing is not a working pairing. If
    # `--skip` ever widens to match everything, this is what refuses to call
    # the resulting silence `ok`.
    if [ -n "$passed" ] && [ "$passed" -gt 0 ]; then
        echo "ok"
        exit 0
    fi
    echo "could-not-run" >&2
    echo "the suite reported no passing test — nothing was measured." >&2
    echo "could-not-run"
    exit 2
fi

if grep -q "^test result: FAILED" /tmp/kan-compat.log; then
    echo "incompatible"
    exit 0
fi

# Non-zero, and the harness never reported. Whatever went wrong, it was not the
# pairing.
echo "could-not-run" >&2
echo "the suite exited $status without reporting a result — not a pairing" >&2
echo "outcome. See /tmp/kan-compat.log." >&2
echo "could-not-run"
exit 2
