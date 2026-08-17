#!/bin/sh
set -eu

repo_root=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
echo "deprecated: use 'cargo run -p xtask -- census findings' or a just recipe" >&2
exec cargo run --quiet --manifest-path "$repo_root/Cargo.toml" -p xtask -- \
  census findings "$@"
