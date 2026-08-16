#!/usr/bin/env python3
"""Check RFC 0's non-recursive external-claim publication contract."""

import hashlib
import json
import pathlib
import shutil
import subprocess
import sys
import tempfile


def fail(message):
    print(f"RFC 0 PUBLICATION CHECK FAILED: {message}", file=sys.stderr)
    raise SystemExit(1)


root = pathlib.Path(__file__).resolve().parent.parent
vector = json.loads((root / "rfcs/vectors/0-publication.json").read_text())
if vector != {
    "version": 1,
    "subject": "rfc/0",
    "artifact_path": "rfcs/0-rfc-and-adr-process.md",
    "projection_path": ".claims/by-subject/rfc/0",
    "claim_location": "external",
    "normative_cid_embedding": "forbidden",
}:
    fail("publication vector changed")

artifact = root / vector["artifact_path"]
before = artifact.read_bytes()
if b"Kan-claim:" in before:
    fail("normative RFC bytes contain a claim-CID backlink")
digest = hashlib.sha256(before).hexdigest()

with tempfile.TemporaryDirectory(prefix="day-rfc0-publication-") as temp:
    clone = pathlib.Path(temp)
    target = clone / vector["artifact_path"]
    target.parent.mkdir(parents=True)
    shutil.copyfile(artifact, target)
    projection = clone / vector["projection_path"]
    projection.parent.mkdir(parents=True)
    projection.write_text(json.dumps({
        "subject": vector["subject"],
        "artifact": {"path": vector["artifact_path"], "sha256": digest},
    }) + "\n")
    if hashlib.sha256(target.read_bytes()).hexdigest() != digest:
        fail("external claim projection changed normative RFC bytes")

if len(sys.argv) == 3 and sys.argv[1] == "--git-ref":
    committed = subprocess.run(
        ["git", "show", f"{sys.argv[2]}:{vector['artifact_path']}"],
        cwd=root, check=True, capture_output=True,
    ).stdout
    if committed != before:
        fail(f"worktree artifact does not resolve to {sys.argv[2]}")
elif len(sys.argv) != 1:
    fail("usage: check-rfc0-publication.py [--git-ref REF]")

print(f"RFC 0 publication: external projection preserves sha256:{digest}")
