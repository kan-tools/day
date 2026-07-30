#!/usr/bin/env bash
# Captures, for every released tag, the fenced blocks that version actually
# wrote — by building that tag and running its own `day` binary against a stub
# kan, then reading the blocks back out of what it appended.
#
# Run by hand, not in CI. The output is committed, and `tests/block_corpus.rs`
# consumes it hermetically on every push. Building seven tags takes minutes and
# needs the network for each tag's own dependency set, which is exactly what a
# per-push job must not do.
#
# **Why generated rather than written.** The forward guarantee is "this build
# reads every block shape any released version wrote." A hand-written corpus
# cannot support that claim: it records what a maintainer *believes* an old
# version wrote. Driving each tag's own binary records what it *did* write, so a
# shape nobody ever emitted cannot get into the corpus and pass for history.
#
# **Known coverage gap, stated rather than left to be discovered.** The stub
# below is write-only: it records appends and serves nothing back. So any verb
# that reads its own writes fails, and the blocks those verbs would have written
# are missing from the corpus. In practice that means `day-bridge` (declaring a
# bridge reads the atoms its plan names) and `day-witness` (the starter is
# printed by a path that first reads the telos). What *is* captured is
# `day-atom`, `day-telos`, `day-schema` and `day-docs`.
#
# Closing the gap means giving the stub read-back, the way `tests/common`'s does.
# Tracked rather than done here, because a partial corpus that says which parts
# are partial is honest, and a complete-looking one that quietly omits two block
# types is the failure this whole milestone is about.
#
# Usage: scripts/capture-block-corpus.sh [output-dir]
set -euo pipefail

OUT="${1:-tests/fixtures/block-corpus}"
REPO="$(git rev-parse --show-toplevel)"
WORK="$(mktemp -d)"
trap 'rm -rf "$WORK"; git -C "$REPO" worktree prune' EXIT

mkdir -p "$REPO/$OUT"

# A kan stub that records every append and serves nothing back. Deliberately
# minimal: the point is to capture what day *wrote*, so the read side only has
# to be well-formed enough not to abort the verb.
make_stub() {
  local dir="$1"
  mkdir -p "$dir/data"
  cat > "$dir/kan-stub.sh" <<'STUB'
#!/bin/sh
D="$(dirname "$0")/data"
case "$1" in
  --help) echo "kan (corpus stub)"; exit 0 ;;
  identity) echo "did:key:zCorpusStub"; exit 0 ;;
  status) printf '{"v":1,"subjects":[]}\n'; exit 0 ;;
  show) printf '{"v":1,"subject":"%s","subjects":[],"claims":[],"inbound":[]}\n' "$2"; exit 0 ;;
  issues) printf '{"v":1,"subjects":[]}\n'; exit 0 ;;
  observe|plan|decide|result|resolve|relate)
    printf '%s\n<<<REC>>>\n' "$*" >> "$D/appends.log"
    printf 'bafyreicorpusstub\n'; exit 0 ;;
  *) exit 0 ;;
esac
STUB
  chmod +x "$dir/kan-stub.sh"
}

# Every fenced day-* block in the append log, one JSON body per line, tagged
# with its fence. `python3` rather than sed: a block body is multi-line and
# newline-delimited extraction is how this would silently truncate.
extract_blocks() {
  python3 - "$1" <<'PY'
import json, re, sys, pathlib
log = pathlib.Path(sys.argv[1])
if not log.exists():
    sys.exit(0)
text = log.read_text()
seen = set()
for fence, body in re.findall(r"```(day-[a-z]+)\n(.*?)\n```", text, re.S):
    body = body.strip()
    try:
        parsed = json.loads(body)
    except json.JSONDecodeError:
        continue
    canonical = json.dumps(parsed, sort_keys=True, separators=(",", ":"))
    if (fence, canonical) in seen:
        continue
    seen.add((fence, canonical))
    print(json.dumps({"fence": fence, "body": parsed}, sort_keys=True))
PY
}

for tag in $(git -C "$REPO" tag --list 'v*' --sort=creatordate); do
  echo "==> $tag"
  tree="$WORK/$tag"
  git -C "$REPO" worktree add --detach -q "$tree" "$tag" 2>/dev/null || {
    echo "    could not check out $tag, skipping"; continue; }

  if ! (cd "$tree" && cargo build --quiet --release 2>/dev/null); then
    echo "    does not build with the current toolchain, skipping"
    git -C "$REPO" worktree remove --force "$tree" 2>/dev/null || true
    continue
  fi

  bin="$tree/target/release/day"
  run="$WORK/run-$tag"
  make_stub "$run"
  ( cd "$run" && git init -q . && git commit -q --allow-empty -m init )

  # Verbs that write a fenced block. Not every tag has every verb, so each is
  # attempted and a failure is fine — the corpus records what this version
  # could write, which is the thing being captured.
  export DAY_KAN_BIN="$run/kan-stub.sh"
  ( cd "$run" && "$bin" atom declare corpus-atom --in intent --out design-doc --next build ) >/dev/null 2>&1 || true
  ( cd "$run" && "$bin" atom declare corpus-done --in a --out b --done published-artifact ) >/dev/null 2>&1 || true
  ( cd "$run" && "$bin" telos declare corpus-telos "A captured telos." --witness published-artifact ) >/dev/null 2>&1 || true
  ( cd "$run" && "$bin" telos declare corpus-scoped "A scoped telos." --witness published-artifact --scope 'published-artifact=v9*' ) >/dev/null 2>&1 || true
  ( cd "$run" && "$bin" bridge declare corpus-bridge --telos corpus-telos --have intent --plan "design > build" ) >/dev/null 2>&1 || true
  ( cd "$run" && "$bin" telos tension corpus-a corpus-b "A captured tension." ) >/dev/null 2>&1 || true
  unset DAY_KAN_BIN

  # The starters are the other shapes a version wrote: day prints them for a
  # project to record, so they are declarations that version authored even
  # though no verb appends them.
  ( cd "$run" && DAY_KAN_BIN="$run/kan-stub.sh" "$bin" design check /dev/null ) >> "$run/data/appends.log" 2>&1 || true
  ( cd "$run" && DAY_KAN_BIN="$run/kan-stub.sh" "$bin" assess docs ) >> "$run/data/appends.log" 2>&1 || true
  ( cd "$run" && DAY_KAN_BIN="$run/kan-stub.sh" "$bin" assess telos corpus-telos ) >> "$run/data/appends.log" 2>&1 || true

  out="$REPO/$OUT/$tag.jsonl"
  extract_blocks "$run/data/appends.log" | sort > "$out"
  # `wc -l`, not `grep -c ""` — grep exits 1 on an empty file, so the `|| echo 0`
  # fallback appended a second count and produced "0\n0", which then failed an
  # integer comparison. A version that captured nothing must be removed rather
  # than committed as an empty fixture claiming to be that version's shapes.
  count=$(wc -l < "$out" | tr -d ' ')
  echo "    captured $count distinct block(s) -> $OUT/$tag.jsonl"
  if [ "$count" -eq 0 ]; then
    echo "    (removing empty fixture: this version wrote no capturable block)"
    rm -f "$out"
  fi

  git -C "$REPO" worktree remove --force "$tree" 2>/dev/null || true
done

echo
echo "Corpus written to $OUT. Commit it; tests/block_corpus.rs reads it on every push."
