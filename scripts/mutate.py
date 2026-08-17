#!/bin/sh
set -eu

repo_root=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
echo "deprecated: use 'cargo run -p xtask -- evidence mutate'" >&2
exec cargo run --quiet --manifest-path "$repo_root/Cargo.toml" -p xtask -- \
  evidence mutate "$@"
