#!/bin/sh
set -eu

repo_root=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
manifest=${DAY_XTASK_MANIFEST:-"$repo_root/Cargo.toml"}
exec cargo run --quiet --manifest-path "$manifest" -p xtask -- validate rfc "$@"
