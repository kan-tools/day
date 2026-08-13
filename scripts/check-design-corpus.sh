#!/bin/sh
# Validate every tracked design and report the complete failure set.
set -u

day_bin="${DAY_BIN:-day}"
failures=0
checked=0

if ! files="$(git ls-files '.design/*.md')"; then
  printf '%s\n' 'error: could not enumerate the tracked design corpus' >&2
  exit 2
fi

for file in $files; do
  # `git ls-files` includes staged/working-tree deletions. A file deliberately
  # reclassified out of the corpus is not a corpus member in this tree.
  [ -f "$file" ] || continue
  checked=$((checked + 1))
  if ! "$day_bin" design check "$file"; then
    printf 'DESIGN-CORPUS-FAIL %s\n' "$file" >&2
    failures=$((failures + 1))
  fi
done

printf 'design corpus: %s checked, %s failed\n' "$checked" "$failures"
[ "$checked" -gt 0 ] && [ "$failures" -eq 0 ]
