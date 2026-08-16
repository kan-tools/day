#!/usr/bin/env python3
"""Resolve RFC 0's signed publication fixture from a fresh local clone."""

import copy
import hashlib
import json
import os
import pathlib
import subprocess
import sys
import tempfile


class InvalidPublication(ValueError):
    pass


def require(condition, message):
    if not condition:
        raise InvalidPublication(message)


def validate(vector, checkout, claims):
    require(vector.get("version") == 2, "publication vector version changed")
    require(vector.get("claim_location") == "external", "claim is not external")
    require(vector.get("normative_cid_embedding") == "forbidden", "CID embedding is not forbidden")
    artifact_path = vector["artifact_path"]
    artifact_commit = vector["artifact_commit"]
    claim = next((item for item in claims if item.get("cid") == vector["claim_cid"]), None)
    require(claim is not None, "published claim CID does not resolve")
    require(claim.get("kind") == "Decision" and claim.get("subject") == vector["subject"], "claim kind or subject is wrong")
    expected_file = f'FileAt("{artifact_path}", "{artifact_commit}")'
    require(expected_file in claim.get("artifacts", []), "claim does not carry the exact commit/path address")
    committed = subprocess.run(["git", "show", f"{artifact_commit}:{artifact_path}"], cwd=checkout, check=True, capture_output=True).stdout
    require(hashlib.sha256(committed).hexdigest() == vector["artifact_sha256"], "addressed artifact bytes changed")
    require(b"Kan-claim:" not in committed, "normative RFC bytes contain their own claim CID")
    statuses = [item for item in claims if item.get("kind") == "Status" and vector["claim_cid"] in item.get("cites", [])]
    require(any(item.get("status") == "Closed" for item in statuses), "fixture claim has no closed status")


def main():
    root = pathlib.Path(__file__).resolve().parent.parent
    vector = json.loads((root / "rfcs/vectors/0-publication.json").read_text())
    with tempfile.TemporaryDirectory(prefix="day-rfc0-fresh-clone-") as temp:
        checkout = pathlib.Path(temp) / "clone"
        subprocess.run(["git", "clone", "--quiet", "--no-local", str(root), str(checkout)], check=True)
        projection = checkout / vector["projection_path"]
        require(projection.is_dir() and any(projection.glob("*.md")), "published claim projection is absent from fresh clone")
        identity = root / ".kan/identity"
        require(identity.is_file(), "the fixture author's signing identity is unavailable")
        clean_env = os.environ.copy()
        clean_env["KAN_IDENTITY_FILE"] = str(identity)
        subprocess.run(["kan", "restore"], cwd=checkout, env=clean_env, check=True, capture_output=True)
        output = subprocess.run(["kan", "show", vector["subject"], "--json"], cwd=checkout, env=clean_env, check=True, capture_output=True, text=True)
        claims = json.loads(output.stdout)["claims"]
        validate(vector, checkout, claims)
        if "--self-test" in sys.argv[1:]:
            mutations = {"cid": ("claim_cid", "bafywrong"), "commit": ("artifact_commit", "0" * 40), "path": ("artifact_path", "rfcs/fixtures/wrong.md"), "bytes": ("artifact_sha256", "0" * 64)}
            for name, (field, value) in mutations.items():
                candidate = copy.deepcopy(vector)
                candidate[field] = value
                try:
                    validate(candidate, checkout, claims)
                except (InvalidPublication, subprocess.CalledProcessError):
                    continue
                raise InvalidPublication(f"self-test accepted {name} mutation")
    print(f"RFC 0 publication: fresh clone resolved {vector['claim_cid']}")


if __name__ == "__main__":
    try:
        main()
    except (InvalidPublication, KeyError, json.JSONDecodeError, subprocess.CalledProcessError) as error:
        print(f"RFC 0 PUBLICATION CHECK FAILED: {error}", file=sys.stderr)
        raise SystemExit(1)
