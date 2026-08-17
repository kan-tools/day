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
    require(relationship in {"sufficient", "necessary", "exact", "unspecified"}, f"unknown relationship: {relationship}")
    components = case.get("components", [])
    require(components, "witness case has no components")
    require(len({component.get("name") for component in components}) == len(components), "component names are absent or duplicated")
    outcomes = [component.get("outcome") for component in components]
    require(all(outcome in {"material", "missing", "vacuous", "timeout", "error", "not-run"} for outcome in outcomes), "unknown component outcome")
    coordinates = [component.get("coordinate") for component in components]
    require(all(isinstance(value, str) and value for value in coordinates), "every component must supply its shared coordinate")
    if relationship in {"necessary", "exact"}:
        return "unsupported"
    if relationship == "unspecified":
        return "component-report"
    if any(outcome in {"timeout", "error", "not-run"} for outcome in outcomes):
        return "uncheckable"
    if not all(outcome == "material" for outcome in outcomes):
        return "not-certified"
    return "certified" if len(set(coordinates)) == 1 else "not-certified"


def migration_outcome(case):
    require(case.get("comparison") in {"invertible", "lax", "absent"}, "unknown migration comparison")
    require(case.get("procedure") in {"supported", "unsupported"}, "unknown migration procedure status")
    require(case.get("coordinates") in {"preserved", "forgotten"}, "unknown migration coordinate status")
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
    require(local_cells[0].get("source_predicate") == composition["present"]["name"], "first local cell does not start at the present predicate")
    require(local_cells[-1].get("target_predicate") == composition["target"]["name"], "last local cell does not end at the target telos")
    require(all(cell.get("exists") is True for cell in local_cells), "global realization names a nonexistent local cell")
    composite_expression = " ⊙ ".join(reversed(order))
    derived_realization = f"{composition['present']['name']} ⇒ {composition['target']['name']} ⊙ ({composite_expression})"
    require(composition.get("expected_realization") == derived_realization, "realization expression is stale or wrongly ordered")
    no_realization = composition.get("typeable_without_realization", {})
    counterfactual = no_realization.get("local_cell_exists")
    require(no_realization.get("bridge_order") == order and isinstance(counterfactual, list) and len(counterfactual) == len(local_cells), "no-realization vector is malformed")
    require(all(isinstance(value, bool) for value in counterfactual) and not all(counterfactual) and no_realization.get("expected") == "no-realization", "typeable/no-realization distinction changed")
    require(composition.get("reversed_realization") == {
        "expression": "P0 ⇒ (A2 ⊙ A1) ⊙ T", "well_typed": False
    }, "reversed realization must be rejected")

    witness_cases = data.get("witness_cases", [])
    witness = {case["id"]: case for case in witness_cases}
    required_witness = {
        "artifact-two-evidence", "shared-evidence-reuse", "independent-components",
        "coordinate-mismatch", "missing-sufficient", "unavailable-sufficient",
        "legacy-flat", "necessary-unsupported", "exact-unsupported",
    }
    require(set(witness) == required_witness, "witness vector census changed")
    require(len(witness_cases) == len(witness), "witness case IDs are duplicated")
    for case_id in ("artifact-two-evidence", "shared-evidence-reuse"):
        require(set(witness[case_id]) == {"id", "evidence_cids", "artifact", "independent_observations"}, f"evidence case shape changed: {case_id}")
        require(isinstance(witness[case_id].get("artifact"), str) and witness[case_id]["artifact"], f"artifact address absent: {case_id}")
        require(isinstance(witness[case_id].get("evidence_cids"), list) and all(isinstance(cid, str) and cid for cid in witness[case_id]["evidence_cids"]), f"evidence CIDs malformed: {case_id}")
        require(len(witness[case_id]["evidence_cids"]) == 2, f"evidence case must have two uses: {case_id}")
        observed = len(set(witness[case_id]["evidence_cids"]))
        require(witness[case_id]["independent_observations"] == observed, f"wrong evidence independence: {case_id}")
    require(witness["artifact-two-evidence"]["artifact"] == witness["shared-evidence-reuse"]["artifact"], "artifact identity differs between reuse examples")
    require(witness["artifact-two-evidence"]["independent_observations"] == 2, "distinct evidence case is not independent")
    require(witness["shared-evidence-reuse"]["independent_observations"] == 1, "shared evidence case is not reuse")
    for case_id in required_witness - {"artifact-two-evidence", "shared-evidence-reuse"}:
        require(set(witness[case_id]) == {"id", "relationship", "components", "expected"}, f"witness case shape changed: {case_id}")
        for component in witness[case_id].get("components", []):
            require(set(component) == {"name", "outcome", "coordinate"}, f"component shape changed: {case_id}")
            require(isinstance(component.get("name"), str) and component["name"], f"component name absent: {case_id}")
        require(witness[case_id]["expected"] == witness_outcome(witness[case_id]), f"wrong witness result: {case_id}")

    migration = {case["id"]: case for case in data.get("migration_cases", [])}
    expected_migration = {
        "invertible-reindexing": "equivalent", "unsupported-procedure": "lossy",
        "lax-comparison": "lossy", "successful-gluing": "equivalent",
        "forgotten-coordinate": "lossy", "incomparable-frames": "incomparable",
    }
    require(set(migration) == set(expected_migration), "migration vector census changed")
    require(len(data.get("migration_cases", [])) == len(migration), "migration case IDs are duplicated")
    expected_details = {
        "invertible-reindexing": ({"telos", "evidence", "procedure", "assessment", "witness", "atom", "bridge", "realization"}, set()),
        "unsupported-procedure": ({"telos", "evidence", "artifact-coordinate"}, {"procedure", "assessment-outcome", "certificate"}),
        "lax-comparison": ({"evidence", "individual-components"}, {"monoidal-equivalence", "assembled-witness"}),
        "successful-gluing": ({"shared-coordinate", "component-assessments", "assembled-witness"}, set()),
        "forgotten-coordinate": ({"component-assessments"}, {"shared-coordinate", "gluing-proof", "assembled-witness"}),
        "incomparable-frames": (set(), {"all-cross-frame-judgments"}),
    }
    for case_id, expected in expected_migration.items():
        require(set(migration[case_id]) == {"id", "comparison", "procedure", "coordinates", "transported", "lost", "expected"}, f"migration case shape changed: {case_id}")
        require(migration[case_id]["expected"] == expected, f"wrong migration fixture: {case_id}")
        require(migration[case_id]["expected"] == migration_outcome(migration[case_id]), f"wrong migration result: {case_id}")
        require(isinstance(migration[case_id].get("transported"), list) and isinstance(migration[case_id].get("lost"), list), f"migration detail absent: {case_id}")
        migration_vocabulary = {"telos", "evidence", "procedure", "assessment", "witness", "atom", "bridge", "realization", "artifact-coordinate", "assessment-outcome", "certificate", "individual-components", "monoidal-equivalence", "assembled-witness", "shared-coordinate", "component-assessments", "gluing-proof", "all-cross-frame-judgments"}
        transported = migration[case_id]["transported"]
        lost = migration[case_id]["lost"]
        require(all(isinstance(item, str) and item in migration_vocabulary for item in transported + lost), f"unknown migration detail: {case_id}")
        require(len(set(transported)) == len(transported) and len(set(lost)) == len(lost) and not set(transported) & set(lost), f"migration detail overlaps or duplicates: {case_id}")
        require((set(transported), set(lost)) == expected_details[case_id], f"migration transport/loss semantics changed: {case_id}")
        if expected == "equivalent":
            require(transported and not lost, f"equivalent migration lacks transport or reports loss: {case_id}")
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
        ("stale-realization", lambda d: d["composition"]["atoms"][0].update(name="BROKEN")),
        ("invented-outcome", lambda d: next(c for c in d["witness_cases"] if c["id"] == "missing-sufficient")["components"][0].update(outcome="invented")),
        ("invented-migration", lambda d: next(c for c in d["migration_cases"] if c["id"] == "unsupported-procedure").update(comparison="invented")),
        ("invented-legacy-outcome", lambda d: next(c for c in d["witness_cases"] if c["id"] == "legacy-flat")["components"][0].update(outcome="invented")),
        ("malformed-evidence-cids", lambda d: next(c for c in d["witness_cases"] if c["id"] == "artifact-two-evidence").update(evidence_cids="cid-a")),
        ("duplicate-witness-id", lambda d: d["witness_cases"].append(copy.deepcopy(d["witness_cases"][0]))),
        ("invented-migration-detail", lambda d: next(c for c in d["migration_cases"] if c["id"] == "lax-comparison")["lost"].append("invented")),
        ("empty-equivalent-transport", lambda d: next(c for c in d["migration_cases"] if c["id"] == "invertible-reindexing").update(transported=[])),
        ("empty-evidence-set", lambda d: next(c for c in d["witness_cases"] if c["id"] == "artifact-two-evidence").update(evidence_cids=[])),
        ("reuse-became-independent", lambda d: next(c for c in d["witness_cases"] if c["id"] == "shared-evidence-reuse").update(evidence_cids=["cid-a", "cid-b"], independent_observations=2)),
        ("invented-evidence-expected", lambda d: next(c for c in d["witness_cases"] if c["id"] == "artifact-two-evidence").update(expected="invented")),
        ("absent-legacy-name", lambda d: next(c for c in d["witness_cases"] if c["id"] == "legacy-flat")["components"][0].update(name=None)),
        ("duplicate-migration-id", lambda d: d["migration_cases"].append(copy.deepcopy(d["migration_cases"][0]))),
        ("reversed-migration-detail", lambda d: next(c for c in d["migration_cases"] if c["id"] == "forgotten-coordinate").update(transported=["shared-coordinate", "gluing-proof", "assembled-witness"], lost=["component-assessments"])),
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
