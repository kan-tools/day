#!/usr/bin/env bash
# One cell of the kan<->day compatibility matrix: this day commit against one
# released kan.
#
# Emits a single token on stdout. Three of the four are the cell's committed
# outcome; the fourth is the refusal to record one.
#
#   ok                     every conformance test passed against this kan   (exit 0)
#   incompatible           at least one conformance test failed             (exit 0)
#   unbuildable            this kan tag does not build with the current
#                          toolchain                                        (exit 0)
#   could-not-run          the measurement did not happen. NOT a fact about
#                          the pairing, and never a table row               (exit 2)
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
    echo "unbuildable"
    exit 0
fi

# A directory containing only this kan, prepended to PATH. Prepending the
# binary's own directory would be wrong when it sits beside other tools.
shim="$(mktemp -d)"
ln -sf "$(cd "$(dirname "$kan_bin")" && pwd)/$(basename "$kan_bin")" "$shim/kan"

# Unset so a stray value in the environment cannot redirect a test that is
# supposed to be talking to the real thing.
unset DAY_KAN_BIN || true

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
if ! PATH="$shim:$PATH" cargo test --quiet --no-run --test kan_conformance \
        >/tmp/kan-compat.log 2>&1; then
    if grep -q "could not compile" /tmp/kan-compat.log; then
        echo "unbuildable"
        exit 0
    fi
    echo "could-not-run" >&2
    echo "cargo did not get as far as building the suite — this says NOTHING" >&2
    echo "about the pairing. See /tmp/kan-compat.log." >&2
    echo "could-not-run"
    exit 2
fi

# `|| status=$?` rather than a bare call: `set -e` would take a failing suite —
# the `incompatible` outcome this exists to report — as a reason to abort.
status=0
PATH="$shim:$PATH" cargo test --quiet --test kan_conformance -- \
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
