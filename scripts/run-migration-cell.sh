#!/usr/bin/env bash
# One cell of the migration matrix: hand a historical `day` binary the block
# shapes this commit writes, and print a single stable token naming what it did.
#
# The probe is `day doctor` over a `day-atom` block. Both are deliberate: they
# are the one verb and the one block type *every* released version has, so the
# matrix spans day's whole release history rather than only v0.6 onward. `day
# status` would silently narrow the matrix to the versions that happen to have
# it, which is the kind of coverage gap this whole milestone is about.
#
# Prints exactly one of:
#   refused-honestly   the reader said it could not read the block
#   silently-widened   the reader loaded it as though the unreadable part were
#                      absent, and certified the result
#   errored            the reader failed in some other way
#
# Usage: scripts/run-migration-cell.sh /path/to/day
set -uo pipefail

BIN="${1:?usage: run-migration-cell.sh /path/to/day}"
# Resolved before the `cd` below, because a relative path stops meaning what the
# caller meant the moment this script changes directory. CI happens to pass an
# absolute path, which is what makes this worth fixing rather than leaving: it
# would work there and mislead anyone running it by hand.
case "$BIN" in
  /*) ;;
  *) BIN="$PWD/$BIN" ;;
esac
# A binary that is missing or not executable is a **harness** fault, not one of
# the outcomes this script classifies. Reporting it as `errored` would put a
# broken invocation into the matrix as though it were a fact about a released
# version.
if [ ! -x "$BIN" ]; then
  echo "run-migration-cell.sh: $BIN is not an executable file" >&2
  exit 64
fi
REPO="$(git rev-parse --show-toplevel)"
BLOCKS="$REPO/tests/fixtures/migration-blocks.json"
WORK="$(mktemp -d)"
trap 'rm -rf "$WORK"' EXIT

mkdir -p "$WORK/data"

# Serve each block from the fixture on its own subject, in kan's --json shape.
python3 - "$BLOCKS" "$WORK/data" <<'PY'
import json, sys, pathlib
blocks = json.loads(pathlib.Path(sys.argv[1]).read_text())
data = pathlib.Path(sys.argv[2])
subjects = []
for entry in blocks:
    subject, fence, body = entry["subject"], entry["fence"], entry["body"]
    text = "%s\n\n```%s\n%s\n```\n" % (
        entry.get("note", "A captured declaration."),
        fence,
        json.dumps(body),
    )
    doc = {
        "v": 1, "subject": subject, "subjects": [subject],
        "claims": [{
            "cid": "bafyreimigration" + subject.replace("/", ""),
            "kind": "Observation", "subject": subject,
            "author": "did:key:zMigrationStub", "text": text,
        }],
        "inbound": [],
    }
    (data / ("show-%s.json" % subject.replace("/", "_"))).write_text(json.dumps(doc))
    subjects.append(subject)
(data / "status.json").write_text(json.dumps({
    "v": 1,
    "subjects": [{"subject": s, "subjects": [s], "state": "Unclassified"} for s in subjects],
}))
PY

cat > "$WORK/kan-stub.sh" <<'STUB'
#!/bin/sh
D="$(dirname "$0")/data"
case "$1" in
  --help) echo "kan (migration stub)"; exit 0 ;;
  identity) echo "did:key:zMigrationStub"; exit 0 ;;
  status) cat "$D/status.json"; exit 0 ;;
  issues) cat "$D/status.json"; exit 0 ;;
  show)
    f="$D/show-$(printf '%s' "$2" | tr '/' '_').json"
    if [ -f "$f" ]; then cat "$f"
    else printf '{"v":1,"subject":"%s","subjects":[],"claims":[],"inbound":[]}\n' "$2"; fi
    exit 0 ;;
  *) exit 0 ;;
esac
STUB
chmod +x "$WORK/kan-stub.sh"

cd "$WORK"
git init -q . && git commit -q --allow-empty -m init

out="$(DAY_KAN_BIN="$WORK/kan-stub.sh" "$BIN" doctor 2>&1)"
code=$?

# Classification, in precedence order. "Refused" is checked first because a
# reader that refuses also exits non-zero, so exit code alone cannot tell a
# refusal from an unrelated failure.
if printf '%s' "$out" | grep -qiE "could not be read|upgrade day|unknown field|_version"; then
  echo "refused-honestly"
elif printf '%s' "$out" | grep -q "composition: ok"; then
  # The reader accepted a declaration it could only partly read, and certified
  # the result. This is day#78's shape, and for every version released before
  # `v0.7.0-beta.2` it is the expected outcome rather than a surprise.
  echo "silently-widened"
elif [ "$code" -ne 0 ]; then
  echo "errored"
else
  echo "errored"
fi
