set positional-arguments

# List supported repository tasks.
default:
    @just --list

# Fast local validation.
check:
    cargo run -p xtask -- validate profile quick

# Full pull-request validation.
ci:
    cargo run -p xtask -- validate profile ci

# Compile every workspace target.
build:
    cargo build --workspace --all-targets

# Run every test target without hiding later target results.
test:
    cargo test --workspace --no-fail-fast

# Treat every Clippy warning as a failure.
lint:
    cargo clippy --workspace --all-targets -- -D warnings

# Check Rust formatting.
format-check:
    cargo fmt --all -- --check

# Release validation profile (release cutting remains separate).
release-check:
    cargo run -p xtask -- validate profile release

# Verify the committed v0.13 instance equals the independently typed contract.
verify-v013-contract:
    cargo run -p xtask -- release verify-v013

# Grade a raw v0.13 /askme evidence bundle.
grade-v013-askme bundle:
    cargo run -p xtask -- release grade-askme-v013 {{bundle}}

# Grade a v0.13 fresh-session reconstruction bundle and its removal controls.
grade-v013-reconstruction bundle:
    cargo run -p xtask -- release grade-reconstruction-v013 {{bundle}}

# RFC and ADR validation.
check-rfc *args:
    cargo run -p xtask -- validate rfc {{args}}

# Revert demonstration or verification; pass the ordinary harness arguments.
demonstrate *args:
    cargo run -p xtask -- evidence revert {{args}}

# Run one mutation; pass path, anchor, replacement, and display name.
mutate *args:
    cargo run -p xtask -- evidence mutate {{args}}

# Compare current day behaviour with a base revision over the fixture corpus.
behaviour-diff *args:
    cargo run -p xtask -- evidence behaviour-diff {{args}}

# Account for every commit in a range under the demonstration rule.
census-demonstrations *args:
    cargo run -p xtask -- census demonstrations {{args}}

# Account for every review finding on a kan subject.
census-findings subject *args:
    cargo run -p xtask -- census findings {{subject}} {{args}}

# List the exact checks in a validation profile.
list-profile profile="ci":
    cargo run -p xtask -- validate profile {{profile}} --list
