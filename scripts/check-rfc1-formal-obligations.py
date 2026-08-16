#!/usr/bin/env python3
"""Check RFC 1's incorporated formal vocabulary and open-obligation census."""

from __future__ import annotations

import os
import re
import sys
from pathlib import Path


ROOT = Path(os.environ.get("DAY_RFC_ROOT", Path(__file__).resolve().parents[1]))
RFC = ROOT / "rfcs/1-frame-indexed-process-model.md"
COMPANION = ROOT / "rfcs/1/denotational-semantics.md"

REQUIRED_CHOICES = (
    "Epistemic site and telos-relative topology",
    "Realization prestack, descent, and model structure",
    "Obstruction coefficients and cohomology theory",
    "Effective realization fragment and provability ledger",
)


class InvalidFormalAccount(ValueError):
    pass


def require(condition: bool, message: str) -> None:
    if not condition:
        raise InvalidFormalAccount(message)


def validate(rfc: str, companion: str) -> None:
    combined = rfc + "\n" + companion
    require(
        "W_T:\\mathcal I_T\\to" in rfc
        and "W_T:\\mathcal I_T\\longrightarrow" in companion,
        "witness diagrams must use the declared indexing category \\mathcal I_T",
    )
    require(
        not re.search(r"W_T\s*:\s*J_T", combined),
        "J_T cannot be both a witness indexing category and a Grothendieck topology",
    )
    require(
        "A Grothendieck topology $J_T$ on $\\mathcal C^T_{A,f_0}$" in companion,
        "the telos-relative Grothendieck topology J_T is absent or renamed inconsistently",
    )
    for choice in REQUIRED_CHOICES:
        require(
            f"| {choice} |" in rfc,
            f"RFC 1 unresolved-question table lacks: {choice}",
        )


def self_test(rfc: str, companion: str) -> None:
    mutations = (
        (
            "witness-topology-collision",
            rfc.replace("W_T:\\mathcal I_T\\to", "W_T:J_T\\to", 1),
            companion,
        ),
        *(
            (
                f"missing-{choice.lower().replace(' ', '-')}",
                rfc.replace(f"| {choice} |", "| Removed choice |", 1),
                companion,
            )
            for choice in REQUIRED_CHOICES
        ),
    )
    for name, candidate_rfc, candidate_companion in mutations:
        try:
            validate(candidate_rfc, candidate_companion)
        except InvalidFormalAccount:
            print(f"RFC 1 formal-obligation self-test: {name} mutation rejected")
            continue
        raise InvalidFormalAccount(f"self-test accepted mutation: {name}")


def main() -> None:
    rfc = RFC.read_text(encoding="utf-8")
    companion = COMPANION.read_text(encoding="utf-8")
    validate(rfc, companion)
    if "--self-test" in sys.argv[1:]:
        self_test(rfc, companion)
    print("RFC 1 formal vocabulary and unresolved obligations: valid")


if __name__ == "__main__":
    try:
        main()
    except (OSError, InvalidFormalAccount) as error:
        print(f"RFC 1 FORMAL OBLIGATION CHECK FAILED: {error}", file=sys.stderr)
        raise SystemExit(1)
