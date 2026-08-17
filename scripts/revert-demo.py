#!/bin/sh
set -eu

repo_root=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
echo "deprecated: use 'just demonstrate' or 'cargo run -p xtask -- evidence revert'" >&2
exec cargo run --quiet --manifest-path "$repo_root/Cargo.toml" -p xtask -- \
  evidence revert "$@"
