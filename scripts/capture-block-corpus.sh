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
# The stub **serves back what it was given** (day#87). It has to: a verb that
# reads its own writes -- `bridge declare` resolves the atoms its plan names,
# `telos tension` reads both subjects -- fails against a write-only stub, and the
# blocks it would have written never reach the corpus. That was the original
# coverage gap, and it left `day-bridge` and `day-witness` out precisely because
# those are the block types whose readers changed most recently (day#34's scope,
# day#70's ClaimShape), so the uncaptured half was the half with the most history.
#
# Usage: scripts/capture-block-corpus.sh [--current <tag>] [output-dir]
#
#   --current <tag>   capture only what the CURRENT tree writes, as <tag>.
#                     Run by scripts/cut-release.sh before tagging; the
#                     all-tags sweep stays the by-hand regeneration path.
set -euo pipefail

CURRENT_TAG=
if [ "${1:-}" = "--current" ]; then
  CURRENT_TAG="${2:?--current needs a tag}"
  shift 2
fi
OUT="${1:-tests/fixtures/block-corpus}"
REPO="$(git rev-parse --show-toplevel)"
WORK="$(mktemp -d)"
trap 'rm -rf "$WORK"; git -C "$REPO" worktree prune' EXIT

mkdir -p "$REPO/$OUT"

# A kan stub that records every append AND serves it back, so a verb which reads
# its own writes works. The read-back is what makes `day-bridge` and
# `day-witness` capturable at all (day#87); the same mechanism exists in
# `tests/common` for the same reason.
make_stub() {
  local dir="$1"
  mkdir -p "$dir/data"
  cat > "$dir/data/append.py" <<'APPEND'
import json, os, sys
data, subj, cid, text = sys.argv[1:5]
path = os.path.join(data, "show-%s.json" % subj.replace("/", "_"))
if os.path.exists(path):
    doc = json.load(open(path))
else:
    doc = {"v": 1, "subject": subj, "subjects": [subj], "claims": [], "inbound": []}
doc["claims"].append({
    "cid": cid, "subject": subj, "author": "did:key:zCorpusStub",
    "kind": "Observation", "text": text,
})
json.dump(doc, open(path, "w"))

status_path = os.path.join(data, "status.json")
status = json.load(open(status_path)) if os.path.exists(status_path) else {"v": 1, "subjects": []}
if not any(s["subject"] == subj for s in status["subjects"]):
    status["subjects"].append({"subject": subj, "subjects": [subj], "state": "Unclassified"})
    json.dump(status, open(status_path, "w"))
APPEND
  # `kan show --all --json`, the bulk read day makes since v0.8 (day#71).
  # Without it the stub answered `show --all` as though `--all` were a subject
  # named "--all", so every read-before-write verb on a v0.8+ day met an empty
  # log and was refused — and `day-bridge`, `day-witness` and `day-tension`
  # silently vanished from the corpus for every tag from v0.8.0-beta.1 on.
  # That is the generator's documented failure mode ("less output, checked by
  # nothing") recurring in the tool built after it was documented; the
  # monotone-coverage test in tests/block_corpus.rs is the check that now
  # fails on it. Shape mirrors tests/common's STUB_SHOW_ALL_PY.
  cat > "$dir/data/show_all.py" <<'SHOWALL'
import json, sys, pathlib
data = pathlib.Path(sys.argv[1])
entries = []
for f in sorted(data.glob("show-*.json")):
    entry = json.loads(f.read_text())
    entry.setdefault("inbound", [])
    entry.setdefault("trust", {"base": "Solo", "authors": []})
    entry.setdefault("excluded_by_trust", 0)
    entries.append(entry)
print(json.dumps({
    "v": 1,
    "trust": {"base": "Solo", "authors": []},
    "excluded_by_trust": 0,
    "subjects": entries,
}))
SHOWALL
  printf '{"v":1,"subjects":[]}\n' > "$dir/data/status.json"
  cat > "$dir/kan-stub.sh" <<'STUB'
#!/bin/sh
D="$(dirname "$0")/data"
case "$1" in
  --help) echo "kan (corpus stub)"; exit 0 ;;
  identity) echo "did:key:zCorpusStub"; exit 0 ;;
  status|issues) cat "$D/status.json"; exit 0 ;;
  show)
    if [ "$2" = "--all" ]; then python3 "$D/show_all.py" "$D"; exit 0; fi
    f="$D/show-$(printf '%s' "$2" | tr '/' '_').json"
    if [ -f "$f" ]; then cat "$f"
    else printf '{"v":1,"subject":"%s","subjects":[],"claims":[],"inbound":[]}\n' "$2"; fi
    exit 0 ;;
  observe|plan|decide|result|resolve)
    n=$(cat "$D/n" 2>/dev/null || echo 0); n=$((n + 1)); printf '%s' "$n" > "$D/n"
    printf '%s\n<<<REC>>>\n' "$*" >> "$D/appends.log"
    cid=$(printf 'bafyreicorpus%08d' "$n")
    shift; text="$1"; subj="general"
    while [ $# -gt 0 ]; do
      if [ "$1" = "--subject" ]; then subj="$2"; fi
      shift
    done
    python3 "$D/append.py" "$D" "$subj" "$cid" "$text"
    printf '%s\n' "$cid"; exit 0 ;;
  relate)
    n=$(cat "$D/n" 2>/dev/null || echo 0); n=$((n + 1)); printf '%s' "$n" > "$D/n"
    printf '%s\n<<<REC>>>\n' "$*" >> "$D/appends.log"
    printf 'bafyreicorpusrel\n'; exit 0 ;;
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

# Drive one built binary through the scenario and write its corpus file.
# Shared by the all-tags loop and `--current`, so the release-time capture
# cannot drift from the historical one.
capture_one() {
  bin="$1"; tag="$2"
  run="$WORK/run-$tag"
  make_stub "$run"
  ( cd "$run" && git init -q . && git commit -q --allow-empty -m init )

  # Verbs that write a fenced block. Not every tag has every verb, so each is
  # attempted and a failure is fine — the corpus records what this version
  # could write, which is the thing being captured.
  export DAY_KAN_BIN="$run/kan-stub.sh"
  # Atom names must be the ones the bridge plan below references, and the
  # tension must name teloi that exist. Getting either wrong is silent: the verb
  # is refused, no block is written, and that block type just quietly does not
  # appear in the corpus -- which is exactly how `day-bridge` and `day-tension`
  # went missing the first time this ran.
  ( cd "$run" && "$bin" atom declare design --in intent --out design-doc --next build ) >/dev/null 2>&1 || true
  ( cd "$run" && "$bin" atom declare build --in design-doc --out code-change --done published-artifact ) >/dev/null 2>&1 || true
  # The shapes newer verbs write, each attempted for the same reason: a tag
  # whose CLI predates the flag refuses it and appends nothing. `--revisits`
  # writes the corpus's only `_version: 2` atom (v0.10+), and the pair is a
  # proper return (corpus-build2 reaches corpus-review through `next`) so the
  # captured block is a well-formed example, not a reported finding.
  ( cd "$run" && "$bin" atom declare corpus-build2 --in design-doc --out code-change --next corpus-review ) >/dev/null 2>&1 || true
  ( cd "$run" && "$bin" atom declare corpus-review --in code-change --out verdict --revisits corpus-build2 ) >/dev/null 2>&1 || true
  ( cd "$run" && "$bin" telos declare corpus-telos "A captured telos." --witness code-change ) >/dev/null 2>&1 || true
  ( cd "$run" && "$bin" telos declare corpus-scoped "A scoped telos." --witness published-artifact --scope 'published-artifact=v9*' ) >/dev/null 2>&1 || true
  # `--witness-any` writes the nested-group witness list (v0.12+).
  ( cd "$run" && "$bin" telos declare corpus-any "Either suffices." --witness-any code-change,verdict ) >/dev/null 2>&1 || true
  ( cd "$run" && "$bin" bridge declare corpus-bridge --telos corpus-telos --have intent --plan "design > build" ) >/dev/null 2>&1 || true
  ( cd "$run" && "$bin" bridge declare corpus-branch --telos corpus-telos --have intent --plan "design > (build | build)" ) >/dev/null 2>&1 || true
  ( cd "$run" && "$bin" telos tension corpus-telos corpus-scoped "A captured tension." ) >/dev/null 2>&1 || true
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
}

if [ "${CURRENT_TAG:-}" != "" ]; then
  # `--current <tag>`: capture what the CURRENT tree writes, under the tag
  # about to be cut. Run by scripts/cut-release.sh before tagging, for the
  # same reason the migration row is measured there: a corpus row added on the
  # tag push cannot fail until the NEXT release, which is how this corpus went
  # stale by eleven releases with every check green (2026-08-10 review,
  # finding 2). The tree at cut time is by construction the release content.
  echo "==> $CURRENT_TAG (current tree)"
  ( cd "$REPO" && cargo build --quiet --release )
  capture_one "$REPO/target/release/day" "$CURRENT_TAG"
else
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

    capture_one "$tree/target/release/day" "$tag"
    git -C "$REPO" worktree remove --force "$tree" 2>/dev/null || true
  done
fi

echo
echo "Corpus written to $OUT. Commit it; tests/block_corpus.rs reads it on every push."
