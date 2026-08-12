#!/usr/bin/env python3
"""Account for every review finding on a kan subject.

**Why this exists.** Review findings are recorded as claims, and their
disposition was written as prose, from memory, once per round. That cannot fail
to omit a member: round 2 found three severity-1 defects on `harness-footer`,
the disposition claim written before round 3 opened "SEVERITY 1 — BOTH FIXED"
and named two, and the third — the ahead/behind exchange — was reported to the
operator as closed while surviving the full suite. Round 3, in a different
harness and a different model, found it again.

`CLAUDE.md` already has the rule this violates: *a list that can be derived must
be derived, and a count and a list are different guarantees*. The repo derives
its atom directory, its block corpus and its witness map. It had never derived
the one list whose omissions carry ACROSS rounds, which is the only list where
an omission survives the session that made it.

So this is `demonstration-census.py`'s shape pointed at findings. Every finding
lands in exactly one bucket:

  fixed        a disposition claim says so
  accepted     a disposition claim says so, WITH a reason
  open         a disposition claim says so — deliberately still open
  unaccounted  no disposition claim names it. THE FAILURE, and the only verdict.

Only `unaccounted` is a verdict; the rest are a census. Whether a disposition is
TRUE is a judgement a script cannot make — the same boundary the demonstration
census draws, and for the same reason.

## The conventions it reads

A FINDING is a claim whose text contains `FINDING` before its first colon —
which is how they have been written on this repo's subjects already, so the
convention is descriptive rather than newly imposed.

A DISPOSITION is a line, in any later claim on the same subject:

    Disposition: <cid> fixed|accepted|open <reason>

The cid is the finding's own CID, so a disposition cannot drift onto the wrong
finding by paraphrase. `accepted` and `open` require a reason; `fixed` does not,
because the fix is evidenced by the code and its test rather than by prose.

## Exit codes are the contract

  0  every finding accounted for
  1  at least one finding unaccounted — the finding
  2  could not check (kan unreadable, subject absent, malformed read)
  3  the subject carries no findings at all

Four rather than two, because a caller that cannot tell "kan was unreadable"
from "a finding is unaccounted" reports could-not-check as a defect, which is
the mistake `demonstration-census.py` shipped once and fixed.

Usage:  scripts/finding-census.py <subject> [--json]
"""
from __future__ import annotations

import json
import re
import subprocess
import sys

OK = 0
UNACCOUNTED = 1
COULD_NOT_CHECK = 2
NO_FINDINGS = 3

# A finding announces itself before its first colon. Anchored to the start of a
# line so a claim *quoting* a finding does not become one — the `starts_with`
# lesson day#70 records, where `contains` matched the very decision that defined
# its own marker.
FINDING_RE = re.compile(r"^[^\n:]*\bFINDING\b[^\n:]*:", re.MULTILINE)

DISPOSITION_RE = re.compile(
    r"^Disposition:\s+([0-9a-z]{20,})\s+(fixed|accepted|open)\b[ \t]*(.*)$",
    re.MULTILINE,
)

REASON_REQUIRED = {"accepted", "open"}


class CouldNotCheck(Exception):
    """The census could not be taken. Never reported as a finding."""


def read_subject(subject: str) -> list[dict]:
    """Claims on one subject, through the bulk verb.

    `kan show --all --json`, not `kan show <subject>`: the latter is O(n^2) in
    commit-anchored claims and was measured at 141 s where the bulk read takes
    72 ms (kan#181). A read that fails raises rather than returning empty — an
    unreadable log and a subject with no findings are different states and must
    not share an exit code.
    """
    try:
        out = subprocess.run(
            ["kan", "show", "--all", "--json"],
            capture_output=True,
            text=True,
        )
    except OSError as e:
        raise CouldNotCheck(f"kan is not runnable: {e}") from e
    if out.returncode != 0:
        raise CouldNotCheck(f"kan show --all --json exited {out.returncode}: {out.stderr.strip()[:200]}")
    try:
        envelope = json.loads(out.stdout)
    except json.JSONDecodeError as e:
        raise CouldNotCheck(f"kan's output did not parse as JSON: {e}") from e

    for entry in envelope.get("subjects", []):
        if entry.get("subject") == subject:
            return entry.get("claims", [])
    raise CouldNotCheck(f"no subject `{subject}` in the log")


def census(claims: list[dict]) -> tuple[list[dict], list[str]]:
    """(findings with their disposition, malformed-disposition complaints)."""
    findings: list[dict] = []
    dispositions: dict[str, tuple[str, str]] = {}
    complaints: list[str] = []

    for claim in claims:
        text = claim.get("text") or ""
        if FINDING_RE.search(text):
            findings.append(
                {
                    "cid": claim.get("cid", ""),
                    # The first line is the finding's own summary of itself.
                    "summary": text.strip().splitlines()[0][:120],
                    "status": None,
                    "reason": "",
                }
            )
        for cid, status, reason in DISPOSITION_RE.findall(text):
            reason = reason.strip()
            if status in REASON_REQUIRED and not reason:
                # Loud, not silent: an `accepted` with no reason is exactly the
                # thing this census exists to make visible, so refusing to
                # record it is better than recording it as accounted.
                complaints.append(
                    f"disposition `{status}` for {cid[:12]} states no reason; "
                    f"{'/'.join(sorted(REASON_REQUIRED))} require one"
                )
                continue
            dispositions[cid] = (status, reason)

    for finding in findings:
        if finding["cid"] in dispositions:
            finding["status"], finding["reason"] = dispositions[finding["cid"]]

    return findings, complaints


def main() -> int:
    args = [a for a in sys.argv[1:] if not a.startswith("--")]
    as_json = "--json" in sys.argv[1:]
    if len(args) != 1:
        print(__doc__.strip().splitlines()[-1])
        return COULD_NOT_CHECK

    try:
        claims = read_subject(args[0])
    except CouldNotCheck as e:
        print(f"COULD-NOT-CHECK: {e}")
        return COULD_NOT_CHECK

    findings, complaints = census(claims)
    if not findings:
        print(f"NO-FINDINGS: `{args[0]}` carries no claim announcing a finding.")
        return NO_FINDINGS

    unaccounted = [f for f in findings if f["status"] is None]

    if as_json:
        print(json.dumps({"findings": findings, "complaints": complaints}, indent=2))
    else:
        buckets: dict[str, int] = {}
        for f in findings:
            buckets[f["status"] or "unaccounted"] = buckets.get(f["status"] or "unaccounted", 0) + 1
        print(f"| bucket | count |\n| --- | --- |")
        for name in ("fixed", "accepted", "open", "unaccounted"):
            print(f"| {name} | {buckets.get(name, 0)} |")
        print(f"| **total** | **{len(findings)}** |")
        for f in findings:
            mark = f["status"] or "UNACCOUNTED"
            reason = f" — {f['reason']}" if f["reason"] else ""
            print(f"\n  {mark:11} {f['cid'][:12]}  {f['summary']}{reason}")

    for complaint in complaints:
        print(f"\nMALFORMED: {complaint}")

    if unaccounted or complaints:
        print(
            f"\nUNACCOUNTED: {len(unaccounted)} finding(s) carry no disposition. "
            f"Record one as `Disposition: <cid> fixed|accepted|open <reason>` in a "
            f"claim on this subject. A finding nobody disposed of is a finding that "
            f"stops existing between rounds — which is how a severity-1 was reported "
            f"closed while surviving the suite."
        )
        return UNACCOUNTED
    return OK


if __name__ == "__main__":
    sys.exit(main())
