#!/usr/bin/env bash
# One cell of the kan<->day compatibility matrix: this day commit against one
# released kan.
#
# Emits a single token on stdout, which is the cell's committed outcome:
#
#   ok                     every conformance test passed against this kan
#   incompatible           at least one conformance test failed
#   unbuildable            this kan tag does not build with the current toolchain
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
if PATH="$shim:$PATH" cargo test --quiet --test kan_conformance -- \
        --skip conformance_kan_78 >/tmp/kan-compat.log 2>&1; then
    echo "ok"
else
    # Distinguish "day failed to compile" from "the pairing does not work" —
    # conflating them would record a toolchain problem as a compatibility fact.
    if grep -q "could not compile" /tmp/kan-compat.log; then
        echo "unbuildable"
    else
        echo "incompatible"
    fi
fi
