#!/usr/bin/env python3
"""Resolve RFC 1's denotational companion claim from a fresh local clone."""

from __future__ import annotations

import copy
import hashlib
import json
import pathlib
import subprocess
import sys
import tempfile


class InvalidPublication(ValueError):
    pass


def require(condition, message):
    if not condition:
        raise InvalidPublication(message)


def canonical_origin(checkout):
    origin = subprocess.run(
        ["git", "remote", "get-url", "origin"],
        cwd=checkout,
        check=True,
        capture_output=True,
        text=True,
    ).stdout.strip()
    seen = set()
    while not origin.startswith(("https://", "ssh://", "git@")):
        candidate = pathlib.Path(origin)
        if not candidate.is_absolute():
            candidate = (pathlib.Path(checkout) / candidate).resolve()
        require(candidate not in seen and candidate.is_dir(), "repository origin chain is invalid")
        seen.add(candidate)
        origin = subprocess.run(
            ["git", "remote", "get-url", "origin"],
            cwd=candidate,
            check=True,
            capture_output=True,
            text=True,
        ).stdout.strip()
    return origin


def require_projection(checkout, vector):
    projection = pathlib.Path(checkout) / vector["projection_path"]
    require(
        projection.is_dir() and any(projection.glob("*.md")),
        "published companion projection is absent from fresh clone",
    )


def validate(vector, checkout, claims, repository_origin):
    require(vector.get("version") == 1, "publication vector version changed")
    require(vector.get("claim_location") == "external", "claim is not external")
    require(vector.get("normative_cid_embedding") == "forbidden", "CID embedding is not forbidden")
    require(vector.get("repository") == repository_origin, "fixture repository origin is wrong")
    artifact_path = vector["artifact_path"]
    artifact_commit = vector["artifact_commit"]
    claim = next((item for item in claims if item.get("cid") == vector["claim_cid"]), None)
    require(claim is not None, "published companion claim CID does not resolve")
    require(
        claim.get("kind") == "Decision" and claim.get("subject") == vector["subject"],
        "companion claim kind or subject is wrong",
    )
    expected_file = f'FileAt("{artifact_path}", "{artifact_commit}")'
    require(expected_file in claim.get("artifacts", []), "claim does not carry the exact commit/path address")
    committed = subprocess.run(
        ["git", "show", f"{artifact_commit}:{artifact_path}"],
        cwd=checkout,
        check=True,
        capture_output=True,
    ).stdout
    require(hashlib.sha256(committed).hexdigest() == vector["artifact_sha256"], "addressed companion bytes changed")
    require(b"Kan-claim:" not in committed, "normative companion bytes contain their own claim CID")
    require(
        any(
            item.get("kind") == "Publication"
            and f'Commit("{artifact_commit}")' in item.get("artifacts", [])
            for item in claims
        ),
        "companion has no Publication claim at the addressed commit",
    )


def main():
    root = pathlib.Path(__file__).resolve().parent.parent
    vector = json.loads((root / "rfcs/vectors/1-denotational-publication.json").read_text())
    with tempfile.TemporaryDirectory(prefix="day-rfc1-denotational-fresh-clone-") as temp:
        checkout = pathlib.Path(temp) / "clone"
        origin = canonical_origin(root)
        subprocess.run(["git", "clone", "--quiet", "--no-local", str(root), str(checkout)], check=True)
        require_projection(checkout, vector)
        output = subprocess.run(
            ["kan", "show", vector["subject"], "--json", "--trust", vector["author"]],
            cwd=checkout,
            check=True,
            capture_output=True,
            text=True,
        )
        claims = json.loads(output.stdout)["claims"]
        validate(vector, checkout, claims, origin)
        if "--self-test" in sys.argv[1:]:
            mutations = {
                "cid": ("claim_cid", "bafywrong"),
                "commit": ("artifact_commit", "0" * 40),
                "path": ("artifact_path", "rfcs/1/wrong.md"),
                "bytes": ("artifact_sha256", "0" * 64),
                "repository": ("repository", "https://example.com/wrong.git"),
            }
            for name, (field, value) in mutations.items():
                candidate = copy.deepcopy(vector)
                candidate[field] = value
                try:
                    validate(candidate, checkout, claims, origin)
                except (InvalidPublication, subprocess.CalledProcessError):
                    continue
                raise InvalidPublication(f"self-test accepted {name} mutation")
            try:
                validate(vector, checkout, [c for c in claims if c.get("kind") != "Publication"], origin)
            except InvalidPublication:
                pass
            else:
                raise InvalidPublication("self-test accepted missing Publication claim")
            projection = checkout / vector["projection_path"]
            hidden = projection.with_name(projection.name + ".hidden")
            projection.rename(hidden)
            try:
                require_projection(checkout, vector)
            except InvalidPublication:
                pass
            else:
                raise InvalidPublication("self-test accepted missing companion projection")
            finally:
                hidden.rename(projection)
    print(f"RFC 1 denotational publication: fresh clone resolved {vector['claim_cid']}")


if __name__ == "__main__":
    try:
        main()
    except (InvalidPublication, KeyError, json.JSONDecodeError, subprocess.CalledProcessError) as error:
        print(f"RFC 1 DENOTATIONAL PUBLICATION CHECK FAILED: {error}", file=sys.stderr)
        raise SystemExit(1)
