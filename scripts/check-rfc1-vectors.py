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
    components = case.get("components", [])
    require(components, "sufficient case has no components")
    require(len({component.get("name") for component in components}) == len(components), "component names are absent or duplicated")
    outcomes = [component.get("outcome") for component in components]
    if any(outcome in {"timeout", "error", "not-run"} for outcome in outcomes):
        return "uncheckable"
    if not all(outcome == "material" for outcome in outcomes):
        return "not-certified"
    coordinates = [component.get("coordinate") for component in components]
    require(all(isinstance(value, str) and value for value in coordinates), "every component must supply its shared coordinate")
    return "certified" if len(set(coordinates)) == 1 else "not-certified"


def migration_outcome(case):
    if case.get("comparison") == "absent":
        return "incomparable"
    if (case.get("comparison") == "invertible"
            and case.get("procedure") == "supported"
            and case.get("coordinates") == "preserved"):
        return "equivalent"
    return "lossy"


def validate(data):
    require(data.get("version") == 2, "unsupported vector version")
    composition = data.get("composition", {})
    require(composition.get("convention") == "right-to-left", "composition convention changed")
    atoms = {atom["name"]: atom for atom in composition.get("atoms", [])}
    order = composition.get("bridge_order", [])
    require(order and set(order) == set(atoms), "bridge order and atom census disagree")
    for left, right in zip(order, order[1:]):
        require(atoms[left]["target"] == atoms[right]["source"], f"bridge boundary mismatch: {left} -> {right}")
    expected_boundary = {"source": atoms[order[0]]["source"], "target": atoms[order[-1]]["target"]}
    require(composition.get("expected_composite") == expected_boundary, "declared composite boundary is wrong")
    require(composition.get("present", {}).get("context") == expected_boundary["source"], "present predicate has wrong context")
    require(composition.get("target", {}).get("context") == expected_boundary["target"], "target telos has wrong context")
    identity = composition.get("identity", {})
    require(identity.get("source") == identity.get("target") == expected_boundary["source"], "identity process is not an identity at the source")
    local_cells = composition.get("local_cells", [])
    require(len(local_cells) == len(order), "one local cell is required per atom")
    for index, (cell, atom_name) in enumerate(zip(local_cells, order)):
        require(cell.get("atom") == atom_name, f"local cell {index} names the wrong atom")
        require(cell.get("target_context") == atoms[atom_name]["target"], f"local cell {index} has the wrong boundary")
        if index:
            require(cell.get("source_predicate") == local_cells[index - 1].get("target_predicate"), f"local cells do not paste at {index}")
    require(composition.get("expected_realization") == "P0 ⇒ T ⊙ (A2 ⊙ A1)", "realization order is wrong")
    no_realization = composition.get("typeable_without_realization", {})
    missing = no_realization.get("missing_local_cell")
    require(no_realization.get("bridge_order") == order and isinstance(missing, int) and 0 <= missing < len(local_cells), "no-realization vector is malformed")
    require(local_cells[missing].get("exists") is True and no_realization.get("expected") == "no-realization", "typeable/no-realization distinction changed")
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
        require(isinstance(migration[case_id].get("transported"), list) and isinstance(migration[case_id].get("lost"), list), f"migration detail absent: {case_id}")
        if expected == "equivalent":
            require(not migration[case_id]["lost"], f"equivalent migration reports loss: {case_id}")
        else:
            require(migration[case_id]["lost"], f"non-equivalent migration does not expose loss: {case_id}")


def self_test(data):
    mutations = [
        ("composition-boundary", lambda d: d["composition"]["atoms"][1].update(source="WRONG")),
        ("coherence", lambda d: next(c for c in d["witness_cases"] if c["id"] == "coordinate-mismatch").update(expected="certified")),
        ("claim-reuse", lambda d: next(c for c in d["witness_cases"] if c["id"] == "shared-evidence-reuse").update(independent_observations=2)),
        ("legacy-strengthening", lambda d: next(c for c in d["witness_cases"] if c["id"] == "legacy-flat").update(expected="certified")),
        ("lossy-equivalence", lambda d: next(c for c in d["migration_cases"] if c["id"] == "forgotten-coordinate").update(expected="equivalent")),
        ("missing-component-coordinate", lambda d: next(c for c in d["witness_cases"] if c["id"] == "independent-components")["components"][1].pop("coordinate")),
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
