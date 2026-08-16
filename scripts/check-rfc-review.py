#!/usr/bin/env python3
"""Verify an Accepted RFC's GitHub review clock and any unanimous override."""

import datetime
import json
import pathlib
import re
import subprocess
import sys


def fail(message):
    print(f"RFC REVIEW CHECK FAILED: {message}", file=sys.stderr)
    raise SystemExit(1)


if len(sys.argv) != 6:
    fail("usage: check-rfc-review.py FILE PR_URL START END OVERRIDE")
file, url, start_text, end_text, override = sys.argv[1:]
match = re.fullmatch(r"https://github\.com/kan-tools/day/pull/(\d+)", url)
if not match:
    fail("Discussion is not a day pull-request address")

def timestamp(value):
    try:
        return datetime.datetime.fromisoformat(value.replace("Z", "+00:00"))
    except ValueError:
        fail(f"invalid RFC3339 timestamp: {value}")

start, end = timestamp(start_text), timestamp(end_text)
if override == "None" and end - start < datetime.timedelta(hours=72):
    fail("fewer than 72 review hours elapsed")
try:
    pr = json.loads(subprocess.run(
        ["gh", "pr", "view", url, "--json", "files,commits"],
        check=True, capture_output=True, text=True,
    ).stdout)
except (subprocess.CalledProcessError, json.JSONDecodeError):
    fail("Discussion pull request is not readable")
if file not in {entry["path"] for entry in pr.get("files", [])}:
    fail("Discussion pull request does not contain the RFC file")
commits = pr.get("commits", [])
if not commits:
    fail("Discussion pull request has no commits")
latest = max(commits, key=lambda entry: timestamp(entry["committedDate"]))
latest_time = timestamp(latest["committedDate"])
if start < latest_time:
    fail("review clock starts before the proposal's latest commit")
if end > datetime.datetime.now(datetime.timezone.utc):
    fail("review period has not ended")

if override == "None":
    pass
else:
    override_match = re.fullmatch(r"unanimous:(https://github\.com/kan-tools/day/pull/(\d+))@([0-9a-f]{40})", override)
    if not override_match or override_match.group(1) != url or override_match.group(3) != latest["oid"]:
        fail("override does not name this PR and its latest commit")
    repo_root = pathlib.Path(__file__).resolve().parent.parent
    maintainers = {
        line.strip() for line in (repo_root / "rfcs/maintainers.txt").read_text().splitlines()
        if line.strip() and not line.startswith("#")
    }
    if not maintainers:
        fail("maintainer registry is empty")
    try:
        reactions = json.loads(subprocess.run(
            ["gh", "api", f"repos/kan-tools/day/issues/{override_match.group(2)}/reactions", "--paginate"],
            check=True, capture_output=True, text=True,
        ).stdout)
    except (subprocess.CalledProcessError, json.JSONDecodeError):
        fail("override reactions are not readable")
    rockets = {
        reaction["user"]["login"] for reaction in reactions
        if reaction.get("content") == "rocket" and timestamp(reaction["created_at"]) >= latest_time
    }
    missing = maintainers - rockets
    if missing:
        fail("override lacks post-commit rockets from: " + ", ".join(sorted(missing)))

print(f"RFC review: {file} has verifiable review evidence")
