#!/bin/sh
# Compatibility surface; validation policy lives in xtask.
set -eu

repo_root=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
echo "deprecated: use 'cargo run -p xtask -- evidence behaviour-diff'" >&2
cd "$repo_root"
exec cargo run --quiet --manifest-path "$repo_root/Cargo.toml" -p xtask -- \
  evidence behaviour-diff "$@"
