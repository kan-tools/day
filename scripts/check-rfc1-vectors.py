#!/usr/bin/env python3
"""Validate the executable conformance vectors for RFC 1."""

import copy
import json
import pathlib
import sys


class InvalidVector(ValueError):
    pass


def require(condition, message):
    if not condition:
        raise InvalidVector(message)


def witness_outcome(case):
    relationship = case.get("relationship")
    if relationship in {"necessary", "exact"}:
        return "unsupported"
    if relationship == "unspecified":
        return "component-report"
    require(relationship == "sufficient", f"unknown relationship: {relationship}")
    outcomes = case.get("component_outcomes", [])
    require(outcomes, "sufficient case has no components")
    if any(outcome in {"timeout", "error", "not-run"} for outcome in outcomes):
        return "uncheckable"
    if not all(outcome == "material" for outcome in outcomes):
        return "not-certified"
    coordinates = case.get("coordinates", [])
    return "certified" if coordinates and len(set(coordinates)) == 1 else "not-certified"


def migration_outcome(case):
    if case.get("comparison") == "absent":
        return "incomparable"
    if (case.get("comparison") == "invertible"
            and case.get("procedure") == "supported"
            and case.get("coordinates") == "preserved"):
        return "equivalent"
    return "lossy"


def validate(data):
    require(data.get("version") == 1, "unsupported vector version")
    composition = data.get("composition", {})
    require(composition.get("convention") == "right-to-left", "composition convention changed")
    require(composition.get("composite") == "A2 ⊙ A1:X0->X2", "bridge composition is ill-typed")
    require(composition.get("realization") == "P0 ⇒ T ⊙ (A2 ⊙ A1)", "realization order is wrong")
    require(composition.get("reversed_realization") == {
        "expression": "P0 ⇒ (A2 ⊙ A1) ⊙ T", "well_typed": False
    }, "reversed realization must be rejected")

    witness = {case["id"]: case for case in data.get("witness_cases", [])}
    required_witness = {
        "artifact-two-evidence", "shared-evidence-reuse", "independent-components",
        "coordinate-mismatch", "missing-sufficient", "unavailable-sufficient",
        "legacy-flat", "necessary-unsupported", "exact-unsupported",
    }
    require(set(witness) == required_witness, "witness vector census changed")
    for case_id in ("artifact-two-evidence", "shared-evidence-reuse"):
        observed = len(set(witness[case_id]["evidence_cids"]))
        require(witness[case_id]["independent_observations"] == observed, f"wrong evidence independence: {case_id}")
    for case_id in required_witness - {"artifact-two-evidence", "shared-evidence-reuse"}:
        require(witness[case_id]["expected"] == witness_outcome(witness[case_id]), f"wrong witness result: {case_id}")

    migration = {case["id"]: case for case in data.get("migration_cases", [])}
    expected_migration = {
        "invertible-reindexing": "equivalent", "unsupported-procedure": "lossy",
        "lax-comparison": "lossy", "successful-gluing": "equivalent",
        "forgotten-coordinate": "lossy", "incomparable-frames": "incomparable",
    }
    require(set(migration) == set(expected_migration), "migration vector census changed")
    for case_id, expected in expected_migration.items():
        require(migration[case_id]["expected"] == expected, f"wrong migration fixture: {case_id}")
        require(migration[case_id]["expected"] == migration_outcome(migration[case_id]), f"wrong migration result: {case_id}")


def self_test(data):
    mutations = [
        ("composition-order", lambda d: d["composition"].update(realization="P0 ⇒ (A2 ⊙ A1) ⊙ T")),
        ("coherence", lambda d: next(c for c in d["witness_cases"] if c["id"] == "coordinate-mismatch").update(expected="certified")),
        ("claim-reuse", lambda d: next(c for c in d["witness_cases"] if c["id"] == "shared-evidence-reuse").update(independent_observations=2)),
        ("legacy-strengthening", lambda d: next(c for c in d["witness_cases"] if c["id"] == "legacy-flat").update(expected="certified")),
        ("lossy-equivalence", lambda d: next(c for c in d["migration_cases"] if c["id"] == "forgotten-coordinate").update(expected="equivalent")),
    ]
    for name, mutate in mutations:
        candidate = copy.deepcopy(data)
        mutate(candidate)
        try:
            validate(candidate)
        except InvalidVector:
            continue
        raise InvalidVector(f"self-test accepted mutation: {name}")


def main():
    path = pathlib.Path(sys.argv[1] if len(sys.argv) > 1 and sys.argv[1] != "--self-test" else "rfcs/vectors/1-process-model.json")
    data = json.loads(path.read_text())
    validate(data)
    if "--self-test" in sys.argv[1:]:
        self_test(data)
    print("RFC 1 vectors: valid")


if __name__ == "__main__":
    try:
        main()
    except (InvalidVector, KeyError, TypeError, json.JSONDecodeError) as error:
        print(f"RFC 1 VECTOR CHECK FAILED: {error}", file=sys.stderr)
        raise SystemExit(1)
