use std::collections::{BTreeMap, BTreeSet};
use std::ffi::OsString;
use std::fmt;
use std::path::{Path, PathBuf};
use std::process::Command;

use serde::de::{self, DeserializeSeed, MapAccess, SeqAccess, Visitor};
use serde_json::{json, Map, Value};
use sha2::{Digest, Sha256};

use crate::outcome::{CouldNotCheck, Finding, Outcome};

pub fn run(root: &Path, args: &[OsString]) -> Outcome<()> {
    let (path, self_test) = match parse_args(args) {
        Ok(options) => options,
        Err(detail) => return Outcome::CouldNotCheck(CouldNotCheck::new(detail)),
    };
    let root = effective_root(root);
    let path = path.unwrap_or_else(|| PathBuf::from("rfcs/vectors/1-process-model.json"));
    let path = if path.is_absolute() {
        path
    } else {
        root.join(path)
    };
    let source = match std::fs::read_to_string(&path) {
        Ok(source) => source,
        Err(error) => return failed(format!("{}: {error}", path.display())),
    };
    let data = match parse_ijson(&source) {
        Ok(data) => data,
        Err(error) => return failed(error.to_string()),
    };
    if let Err(detail) = validate_all(&data, &root) {
        return failed(detail);
    }
    if self_test {
        if let Err(detail) = run_self_test(&data, &root) {
            return failed(detail);
        }
    }
    println!("RFC 1 vectors: valid");
    Outcome::Passed(())
}

fn parse_ijson(source: &str) -> Result<Value, serde_json::Error> {
    let mut deserializer = serde_json::Deserializer::from_str(source);
    let value = IJsonSeed.deserialize(&mut deserializer)?;
    deserializer.end()?;
    Ok(value)
}

struct IJsonSeed;

impl<'de> DeserializeSeed<'de> for IJsonSeed {
    type Value = Value;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_any(IJsonVisitor)
    }
}

struct IJsonVisitor;

impl<'de> Visitor<'de> for IJsonVisitor {
    type Value = Value;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("I-JSON without duplicate property names or unsafe integers")
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E> {
        Ok(Value::Null)
    }

    fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E> {
        Ok(Value::Bool(value))
    }

    fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        const MAX_JCS_INTEGER: i64 = 9_007_199_254_740_992;
        if !(-MAX_JCS_INTEGER..=MAX_JCS_INTEGER).contains(&value) {
            return Err(E::custom("integer is outside the I-JSON safe range"));
        }
        Ok(Value::Number(value.into()))
    }

    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        const MAX_JCS_INTEGER: u64 = 9_007_199_254_740_992;
        if value > MAX_JCS_INTEGER {
            return Err(E::custom("integer is outside the I-JSON safe range"));
        }
        Ok(Value::Number(value.into()))
    }

    fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        serde_json::Number::from_f64(value)
            .map(Value::Number)
            .ok_or_else(|| E::custom("number is not finite IEEE-754"))
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        require_ijson_string::<E>(value)?;
        Ok(Value::String(value.to_owned()))
    }

    fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        require_ijson_string::<E>(&value)?;
        Ok(Value::String(value))
    }

    fn visit_seq<A>(self, mut sequence: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut values = Vec::new();
        while let Some(value) = sequence.next_element_seed(IJsonSeed)? {
            values.push(value);
        }
        Ok(Value::Array(values))
    }

    fn visit_map<A>(self, mut object: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut values = Map::new();
        while let Some(key) = object.next_key::<String>()? {
            require_ijson_string::<A::Error>(&key)?;
            if values.contains_key(&key) {
                return Err(de::Error::custom(format!(
                    "duplicate JSON property name: {key}"
                )));
            }
            values.insert(key, object.next_value_seed(IJsonSeed)?);
        }
        Ok(Value::Object(values))
    }
}

fn require_ijson_string<E: de::Error>(value: &str) -> Result<(), E> {
    let contains_noncharacter = value.chars().any(|character| {
        let codepoint = character as u32;
        (0xfdd0..=0xfdef).contains(&codepoint) || codepoint & 0xffff >= 0xfffe
    });
    if contains_noncharacter {
        Err(E::custom("string contains an I-JSON noncharacter"))
    } else {
        Ok(())
    }
}

fn parse_args(args: &[OsString]) -> Result<(Option<PathBuf>, bool), String> {
    let mut path = None;
    let mut self_test = false;
    for argument in args {
        if argument == "--self-test" {
            self_test = true;
        } else if path.is_none() {
            path = Some(PathBuf::from(argument));
        } else {
            return Err("usage: xtask validate vectors [PATH] [--self-test]".into());
        }
    }
    Ok((path, self_test))
}

pub fn validate(data: &Value) -> Result<(), String> {
    require(
        data.get("version").and_then(Value::as_i64) == Some(3),
        "unsupported vector version",
    )?;
    let composition = object(data.get("composition"), "composition is absent")?;
    require(
        string(composition.get("convention")) == Some("right-to-left"),
        "composition convention changed",
    )?;
    let atom_values = array(composition.get("atoms"), "composition atoms are absent")?;
    let mut atoms = BTreeMap::new();
    for atom in atom_values {
        let name = required_string(atom.get("name"), "atom name is absent")?;
        atoms.insert(name.to_owned(), atom);
    }
    let order = string_array(composition.get("bridge_order"), "bridge order is absent")?;
    require(
        !order.is_empty()
            && order.iter().cloned().collect::<BTreeSet<_>>() == atoms.keys().cloned().collect(),
        "bridge order and atom census disagree",
    )?;
    for pair in order.windows(2) {
        let left = object(Some(atoms[pair[0].as_str()]), "atom is malformed")?;
        let right = object(Some(atoms[pair[1].as_str()]), "atom is malformed")?;
        require(
            left.get("target") == right.get("source"),
            format!("bridge boundary mismatch: {} -> {}", pair[0], pair[1]),
        )?;
    }
    let first = object(Some(atoms[order[0].as_str()]), "first atom is malformed")?;
    let last = object(
        Some(atoms[order.last().unwrap().as_str()]),
        "last atom is malformed",
    )?;
    let source = first.get("source").cloned().unwrap_or(Value::Null);
    let target = last.get("target").cloned().unwrap_or(Value::Null);
    let expected_boundary = json!({"source": source, "target": target});
    require(
        composition.get("expected_composite") == Some(&expected_boundary),
        "declared composite boundary is wrong",
    )?;
    require(
        nested(composition, "present", "context") == expected_boundary.get("source"),
        "present predicate has wrong context",
    )?;
    require(
        nested(composition, "target", "context") == expected_boundary.get("target"),
        "target telos has wrong context",
    )?;
    require(
        nested(composition, "identity", "source") == nested(composition, "identity", "target")
            && nested(composition, "identity", "source") == expected_boundary.get("source"),
        "identity process is not an identity at the source",
    )?;

    let cells = array(composition.get("local_cells"), "local cells are absent")?;
    require(
        cells.len() == order.len(),
        "one local cell is required per atom",
    )?;
    for (index, (cell, atom_name)) in cells.iter().zip(&order).enumerate() {
        let cell = object(Some(cell), "local cell is malformed")?;
        require(
            string(cell.get("atom")) == Some(atom_name),
            format!("local cell {index} names the wrong atom"),
        )?;
        require(
            cell.get("target_context")
                == object(Some(atoms[atom_name.as_str()]), "atom is malformed")?.get("target"),
            format!("local cell {index} has the wrong boundary"),
        )?;
        if index > 0 {
            require(
                cell.get("source_predicate")
                    == object(Some(&cells[index - 1]), "local cell is malformed")?
                        .get("target_predicate"),
                format!("local cells do not paste at {index}"),
            )?;
        }
    }
    require(
        object(Some(&cells[0]), "local cell is malformed")?.get("source_predicate")
            == nested(composition, "present", "name"),
        "first local cell does not start at the present predicate",
    )?;
    require(
        object(Some(&cells[cells.len() - 1]), "local cell is malformed")?.get("target_predicate")
            == nested(composition, "target", "name"),
        "last local cell does not end at the target telos",
    )?;
    require(
        cells
            .iter()
            .all(|cell| cell.get("exists") == Some(&Value::Bool(true))),
        "global realization names a nonexistent local cell",
    )?;
    let expression = order.iter().rev().cloned().collect::<Vec<_>>().join(" ⊙ ");
    let realization = format!(
        "{} ⇒ {} ⊙ ({expression})",
        required_string(
            nested(composition, "present", "name"),
            "present name is absent"
        )?,
        required_string(
            nested(composition, "target", "name"),
            "target name is absent"
        )?
    );
    require(
        string(composition.get("expected_realization")) == Some(realization.as_str()),
        "realization expression is stale or wrongly ordered",
    )?;
    let no_realization = object(
        composition.get("typeable_without_realization"),
        "no-realization vector is absent",
    )?;
    let counterfactual = array(
        no_realization.get("local_cell_exists"),
        "no-realization local-cell vector is malformed",
    )?;
    require(
        no_realization.get("bridge_order") == composition.get("bridge_order")
            && counterfactual.len() == cells.len(),
        "no-realization vector is malformed",
    )?;
    require(
        counterfactual.iter().all(Value::is_boolean)
            && !counterfactual
                .iter()
                .all(|value| value == &Value::Bool(true))
            && string(no_realization.get("expected")) == Some("no-realization"),
        "typeable/no-realization distinction changed",
    )?;
    require(
        composition.get("reversed_realization")
            == Some(&json!({"expression":"P0 ⇒ (A2 ⊙ A1) ⊙ T","well_typed":false})),
        "reversed realization must be rejected",
    )?;

    validate_witnesses(data)?;
    validate_certificate_profile(data)?;
    validate_frame_reads(data)?;
    validate_relationship_examples(data)?;
    validate_migrations(data)
}

fn validate_all(data: &Value, root: &Path) -> Result<(), String> {
    validate(data)?;
    validate_resolved_procedure(data, root)
}

fn validate_certificate_profile(data: &Value) -> Result<(), String> {
    let profile = object(
        data.get("certificate_profile"),
        "certificate profile is absent",
    )?;
    let declaration = object(
        profile.get("declaration"),
        "certificate declaration is absent",
    )?;
    require(
        declaration.get("_version") == Some(&json!(3))
            && string(declaration.get("relationship")) == Some("sufficient"),
        "unsupported certificate declaration",
    )?;
    let declaration_bytes = serde_json_canonicalizer::to_vec(declaration)
        .map_err(|error| format!("could not canonicalize declaration: {error}"))?;
    let declaration_digest = format!("{:x}", Sha256::digest(&declaration_bytes));
    let cases = array(profile.get("cases"), "certificate cases are absent")?;
    let by_id = cases_by_id(cases, "certificate case ID is absent")?;
    require(
        by_id.keys().copied().collect::<BTreeSet<_>>()
            == BTreeSet::from(["certified", "coordinate-mismatch"]),
        "certificate case census changed",
    )?;
    for case in cases {
        validate_certificate_case(
            declaration,
            object(Some(case), "certificate case malformed")?,
            &declaration_digest,
        )?;
    }
    Ok(())
}

fn validate_certificate_case(
    declaration: &Map<String, Value>,
    case: &Map<String, Value>,
    expected_declaration_digest: &str,
) -> Result<(), String> {
    let certificate = object(case.get("certificate"), "certificate is absent")?;
    let subject = required_string(declaration.get("subject"), "declaration subject is absent")?;
    require(
        nested(certificate, "scope", "subject").and_then(Value::as_str) == Some(subject)
            && nested(certificate, "witness_system", "subject").and_then(Value::as_str)
                == Some(subject),
        "certificate is not bound to its declaration",
    )?;
    let declaration_digest = required_string(
        nested(certificate, "witness_system", "declaration_sha256"),
        "declaration digest is absent",
    )?;
    require(
        declaration_digest == expected_declaration_digest,
        format!(
            "certificate declaration digest does not match canonical declaration bytes: expected {expected_declaration_digest}, got {declaration_digest}"
        ),
    )?;
    let procedure = object(
        declaration.get("procedure_spec"),
        "procedure specification is absent",
    )?;
    validate_artifact_address(procedure, "procedure specification", true)?;
    require(
        certificate.get("procedure_spec") == declaration.get("procedure_spec"),
        "certificate procedure specification does not match its declaration",
    )?;
    let declared_components = array(declaration.get("components"), "declared components absent")?;
    let assessed_components = array(certificate.get("components"), "component outcomes absent")?;
    require(
        !declared_components.is_empty() && declared_components.len() == assessed_components.len(),
        "declared and assessed component census disagree",
    )?;
    let evidence = array(
        certificate.get("evidence"),
        "certificate evidence is absent",
    )?;
    let evidence_cids = evidence
        .iter()
        .map(|item| required_string(item.get("cid"), "evidence CID is absent"))
        .collect::<Result<BTreeSet<_>, _>>()?;
    require(
        evidence_cids.len() == evidence.len(),
        "evidence CIDs are duplicated",
    )?;
    for item in evidence {
        validate_artifact_address(
            object(item.get("artifact"), "evidence artifact is absent")?,
            "evidence artifact",
            false,
        )?;
    }
    let assessed = assessed_components
        .iter()
        .map(|item| {
            let component = object(Some(item), "component assessment is malformed")?;
            let name = required_string(component.get("name"), "component name is absent")?;
            let outcome = required_string(component.get("outcome"), "component outcome is absent")?;
            require(
                [
                    "material", "missing", "vacuous", "timeout", "error", "not-run",
                ]
                .contains(&outcome),
                "unknown component outcome",
            )?;
            let bindings = string_array(
                component.get("evidence_cids"),
                "component evidence bindings are absent",
            )?;
            require(
                !bindings.is_empty()
                    && bindings
                        .iter()
                        .all(|cid| evidence_cids.contains(cid.as_str())),
                "component cites unbound evidence",
            )?;
            Ok((name, component))
        })
        .collect::<Result<BTreeMap<_, _>, String>>()?;
    require(
        assessed.len() == assessed_components.len(),
        "component assessments are duplicated",
    )?;
    for item in declared_components {
        let declared = object(Some(item), "declared component is malformed")?;
        let name = required_string(declared.get("name"), "declared component name is absent")?;
        let assessed = assessed
            .get(name)
            .ok_or("undeclared or missing component outcome")?;
        let coordinates = object(
            assessed.get("coordinates"),
            "component coordinates are absent",
        )?;
        for coordinate in string_array(
            declared.get("coordinates"),
            "declared component coordinates are absent",
        )? {
            required_string(
                coordinates.get(&coordinate),
                format!("component coordinate is absent: {name}/{coordinate}"),
            )?;
        }
    }
    let assembly = object(
        declaration.get("assembly"),
        "assembly declaration is absent",
    )?;
    require(
        string(assembly.get("kind")) == Some("all"),
        "unsupported assembly kind",
    )?;
    let mut coherent = true;
    for constraint in array(
        assembly.get("shared_coordinates"),
        "shared-coordinate declaration is absent",
    )? {
        let constraint = object(
            Some(constraint),
            "shared-coordinate declaration is malformed",
        )?;
        let coordinate = required_string(constraint.get("name"), "coordinate name is absent")?;
        let names = string_array(
            constraint.get("components"),
            "coordinate components are absent",
        )?;
        require(!names.is_empty(), "coordinate component set is empty")?;
        let values = names
            .iter()
            .map(|name| {
                assessed
                    .get(name.as_str())
                    .and_then(|item| item.get("coordinates"))
                    .and_then(|map| map.get(coordinate))
                    .and_then(Value::as_str)
                    .filter(|value| !value.is_empty())
                    .ok_or_else(|| format!("shared coordinate is unbound: {name}/{coordinate}"))
            })
            .collect::<Result<BTreeSet<_>, _>>()?;
        coherent &= values.len() == 1;
    }
    let derived = if assessed.values().any(|item| {
        matches!(
            string(item.get("outcome")),
            Some("timeout" | "error" | "not-run")
        )
    }) {
        "uncheckable"
    } else if coherent
        && assessed
            .values()
            .all(|item| string(item.get("outcome")) == Some("material"))
    {
        "certified"
    } else {
        "not-certified"
    };
    require(
        string(certificate.get("outcome")) == Some(derived)
            && string(case.get("expected")) == Some(derived),
        "certificate outcome is not derived from component assessments",
    )
}

fn validate_artifact_address(
    address: &Map<String, Value>,
    label: &str,
    require_version: bool,
) -> Result<(), String> {
    for field in ["repository", "path"] {
        required_string(address.get(field), format!("{label} {field} is absent"))?;
    }
    if require_version {
        required_string(address.get("version"), format!("{label} version is absent"))?;
    }
    let commit = required_string(address.get("commit"), format!("{label} commit is absent"))?;
    let digest = required_string(address.get("sha256"), format!("{label} digest is absent"))?;
    require(
        commit.len() == 40 && commit.bytes().all(|byte| byte.is_ascii_hexdigit()),
        format!("{label} commit is not a full object ID"),
    )?;
    require(
        digest.len() == 64
            && digest
                .bytes()
                .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte)),
        format!("{label} digest is not lowercase SHA-256"),
    )
}

fn validate_resolved_procedure(data: &Value, root: &Path) -> Result<(), String> {
    let declaration = object(
        data.pointer("/certificate_profile/declaration"),
        "certificate declaration is absent",
    )?;
    let address = object(
        declaration.get("procedure_spec"),
        "procedure specification is absent",
    )?;
    let repository = required_string(address.get("repository"), "procedure repository absent")?;
    let remote = Command::new("git")
        .args(["config", "--get", "remote.origin.url"])
        .current_dir(root)
        .output()
        .map_err(|error| format!("could not resolve procedure repository: {error}"))?;
    require(
        remote.status.success(),
        "could not resolve procedure repository",
    )?;
    require(
        normalized_repository(std::str::from_utf8(&remote.stdout).unwrap_or_default())
            == normalized_repository(repository),
        "procedure repository does not match the current repository",
    )?;
    let commit = required_string(address.get("commit"), "procedure commit absent")?;
    let path = required_string(address.get("path"), "procedure path absent")?;
    let object = format!("{commit}:{path}");
    let resolved = Command::new("git")
        .args(["show", &object])
        .current_dir(root)
        .output()
        .map_err(|error| format!("could not resolve procedure bytes: {error}"))?;
    require(
        resolved.status.success(),
        "procedure artifact does not resolve at its declared commit and path",
    )?;
    let actual_digest = format!("{:x}", Sha256::digest(&resolved.stdout));
    require(
        string(address.get("sha256")) == Some(actual_digest.as_str()),
        "resolved procedure digest does not match its address",
    )?;
    let procedure: Value = serde_json::from_slice(&resolved.stdout)
        .map_err(|error| format!("resolved procedure is not JSON: {error}"))?;
    require(
        procedure.get("version") == address.get("version")
            && procedure.get("relationship") == declaration.get("relationship"),
        "resolved procedure version or relationship disagrees with the declaration",
    )?;
    let procedure_components = string_array(
        procedure.get("components"),
        "resolved procedure components are absent",
    )?;
    let declared_components = array(declaration.get("components"), "declared components absent")?
        .iter()
        .map(|component| {
            required_string(component.get("name"), "declared component absent").map(str::to_owned)
        })
        .collect::<Result<Vec<_>, _>>()?;
    require(
        procedure_components == declared_components
            && procedure.get("assembly") == declaration.get("assembly"),
        "resolved procedure components or assembly disagree with the declaration",
    )
}

fn normalized_repository(value: &str) -> &str {
    value.trim().trim_end_matches('/').trim_end_matches(".git")
}

fn validate_frame_reads(data: &Value) -> Result<(), String> {
    let cases = array(data.get("frame_read_cases"), "frame read cases are absent")?;
    let by_id = cases_by_id(cases, "frame read case ID is absent")?;
    require(
        by_id.keys().copied().collect::<BTreeSet<_>>()
            == BTreeSet::from([
                "same-provenance-stored",
                "different-provenance-stored",
                "same-provenance-fresh",
            ]),
        "frame read case census changed",
    )?;
    for value in cases {
        let case = object(Some(value), "frame read case is malformed")?;
        let operation = required_string(case.get("operation"), "frame operation is absent")?;
        let derived = match operation {
            "read-stored-certificate" => {
                if case.get("certificate_provenance") == case.get("current") {
                    "historical-only"
                } else {
                    "provenance-mismatch"
                }
            }
            "execute-declared-procedure" => {
                require(
                    case.get("certificate_provenance").is_none(),
                    "fresh execution cannot be asserted by stored certificate bytes",
                )?;
                "certified"
            }
            _ => return Err("unknown frame operation".into()),
        };
        require(
            string(case.get("expected")) == Some(derived),
            "stored verdict was transported into the current frame",
        )?;
    }
    Ok(())
}

fn validate_relationship_examples(data: &Value) -> Result<(), String> {
    let cases = array(
        data.get("relationship_examples"),
        "relationship examples are absent",
    )?;
    let by_id = cases_by_id(cases, "relationship example ID is absent")?;
    require(
        by_id.keys().copied().collect::<BTreeSet<_>>()
            == BTreeSet::from(["sufficient-only", "necessary-only", "exact"]),
        "relationship example census changed",
    )?;
    for value in cases {
        let case = object(Some(value), "relationship example is malformed")?;
        require(
            string(case.get("category")) == Some("finite-poset"),
            "relationship example is not direction-discriminating",
        )?;
        let assembly = case
            .get("assembly_rank")
            .and_then(Value::as_i64)
            .ok_or("assembly rank absent")?;
        let observation = case
            .get("observation_rank")
            .and_then(Value::as_i64)
            .ok_or("observation rank absent")?;
        let forward = assembly <= observation;
        let reverse = observation <= assembly;
        let relationship = if forward && reverse {
            "exact"
        } else if forward {
            "sufficient"
        } else if reverse {
            "necessary"
        } else {
            "incomparable"
        };
        require(
            case.get("forward") == Some(&Value::Bool(forward))
                && case.get("reverse") == Some(&Value::Bool(reverse))
                && string(case.get("relationship")) == Some(relationship),
            "relationship direction does not follow the finite poset",
        )?;
    }
    Ok(())
}

fn validate_witnesses(data: &Value) -> Result<(), String> {
    let cases = array(data.get("witness_cases"), "witness cases are absent")?;
    let witness = cases_by_id(cases, "witness case ID is absent")?;
    let required = BTreeSet::from([
        "artifact-two-evidence",
        "shared-evidence-reuse",
        "independent-components",
        "missing-sufficient",
        "unavailable-sufficient",
        "legacy-flat",
        "necessary-unsupported",
        "exact-unsupported",
    ]);
    require(
        witness.keys().copied().collect::<BTreeSet<_>>() == required,
        "witness vector census changed",
    )?;
    require(
        cases.len() == witness.len(),
        "witness case IDs are duplicated",
    )?;
    for id in ["artifact-two-evidence", "shared-evidence-reuse"] {
        let case = object(Some(witness[id]), "evidence case is malformed")?;
        require(
            keys(case)
                == BTreeSet::from([
                    "id",
                    "evidence_cids",
                    "artifact",
                    "independent_observations",
                ]),
            format!("evidence case shape changed: {id}"),
        )?;
        require(
            !required_string(case.get("artifact"), "artifact address absent")?.is_empty(),
            format!("artifact address absent: {id}"),
        )?;
        let cids = string_array(
            case.get("evidence_cids"),
            format!("evidence CIDs malformed: {id}"),
        )?;
        require(
            cids.len() == 2,
            format!("evidence case must have two uses: {id}"),
        )?;
        let observed = cids.iter().collect::<BTreeSet<_>>().len() as i64;
        require(
            case.get("independent_observations").and_then(Value::as_i64) == Some(observed),
            format!("wrong evidence independence: {id}"),
        )?;
    }
    let first = object(
        Some(witness["artifact-two-evidence"]),
        "evidence case is malformed",
    )?;
    let reused = object(
        Some(witness["shared-evidence-reuse"]),
        "evidence case is malformed",
    )?;
    require(
        first.get("artifact") == reused.get("artifact"),
        "artifact identity differs between reuse examples",
    )?;
    require(
        first
            .get("independent_observations")
            .and_then(Value::as_i64)
            == Some(2),
        "distinct evidence case is not independent",
    )?;
    require(
        reused
            .get("independent_observations")
            .and_then(Value::as_i64)
            == Some(1),
        "shared evidence case is not reuse",
    )?;

    for (id, value) in &witness {
        if ["artifact-two-evidence", "shared-evidence-reuse"].contains(id) {
            continue;
        }
        let case = object(Some(value), "witness case is malformed")?;
        require(
            keys(case) == BTreeSet::from(["id", "relationship", "components", "expected"]),
            format!("witness case shape changed: {id}"),
        )?;
        for component in array(case.get("components"), "witness case has no components")? {
            let component = object(Some(component), "component is malformed")?;
            require(
                keys(component) == BTreeSet::from(["name", "outcome", "coordinate"]),
                format!("component shape changed: {id}"),
            )?;
            required_string(
                component.get("name"),
                format!("component name absent: {id}"),
            )?;
        }
        require(
            string(case.get("expected")) == Some(witness_outcome(case)?.as_str()),
            format!("wrong witness result: {id}"),
        )?;
    }
    Ok(())
}

fn witness_outcome(case: &Map<String, Value>) -> Result<String, String> {
    let relationship = required_string(case.get("relationship"), "unknown relationship: None")?;
    require(
        ["sufficient", "necessary", "exact", "unspecified"].contains(&relationship),
        format!("unknown relationship: {relationship}"),
    )?;
    let components = array(case.get("components"), "witness case has no components")?;
    require(!components.is_empty(), "witness case has no components")?;
    let names = components
        .iter()
        .map(|item| item.get("name").and_then(Value::as_str))
        .collect::<Vec<_>>();
    require(
        names.iter().all(Option::is_some)
            && names.iter().collect::<BTreeSet<_>>().len() == names.len(),
        "component names are absent or duplicated",
    )?;
    let outcomes = components
        .iter()
        .map(|item| item.get("outcome").and_then(Value::as_str))
        .collect::<Vec<_>>();
    require(
        outcomes.iter().all(|outcome| {
            outcome.is_some_and(|value| {
                [
                    "material", "missing", "vacuous", "timeout", "error", "not-run",
                ]
                .contains(&value)
            })
        }),
        "unknown component outcome",
    )?;
    let coordinates = components
        .iter()
        .map(|item| item.get("coordinate").and_then(Value::as_str))
        .collect::<Vec<_>>();
    require(
        coordinates
            .iter()
            .all(|value| value.is_some_and(|value| !value.is_empty())),
        "every component must supply its shared coordinate",
    )?;
    if ["necessary", "exact"].contains(&relationship) {
        return Ok("unsupported".into());
    }
    if relationship == "unspecified" {
        return Ok("component-report".into());
    }
    if outcomes
        .iter()
        .any(|value| value.is_some_and(|value| ["timeout", "error", "not-run"].contains(&value)))
    {
        return Ok("uncheckable".into());
    }
    if !outcomes.iter().all(|value| *value == Some("material")) {
        return Ok("not-certified".into());
    }
    Ok(if coordinates.iter().collect::<BTreeSet<_>>().len() == 1 {
        "certified"
    } else {
        "not-certified"
    }
    .into())
}

fn validate_migrations(data: &Value) -> Result<(), String> {
    let cases = array(data.get("migration_cases"), "migration cases are absent")?;
    let migration = cases_by_id(cases, "migration case ID is absent")?;
    let expected = BTreeMap::from([
        ("invertible-reindexing", "equivalent"),
        ("unsupported-procedure", "lossy"),
        ("lax-comparison", "lossy"),
        ("successful-gluing", "equivalent"),
        ("forgotten-coordinate", "lossy"),
        ("incomparable-frames", "incomparable"),
    ]);
    require(
        migration.keys().copied().collect::<BTreeSet<_>>() == expected.keys().copied().collect(),
        "migration vector census changed",
    )?;
    require(
        cases.len() == migration.len(),
        "migration case IDs are duplicated",
    )?;
    let details: BTreeMap<&str, (BTreeSet<&str>, BTreeSet<&str>)> = BTreeMap::from([
        (
            "invertible-reindexing",
            (
                BTreeSet::from([
                    "telos",
                    "evidence",
                    "procedure",
                    "assessment",
                    "witness",
                    "atom",
                    "bridge",
                    "realization",
                ]),
                BTreeSet::new(),
            ),
        ),
        (
            "unsupported-procedure",
            (
                BTreeSet::from(["telos", "evidence", "artifact-coordinate"]),
                BTreeSet::from(["procedure", "assessment-outcome", "certificate"]),
            ),
        ),
        (
            "lax-comparison",
            (
                BTreeSet::from(["evidence", "individual-components"]),
                BTreeSet::from(["monoidal-equivalence", "assembled-witness"]),
            ),
        ),
        (
            "successful-gluing",
            (
                BTreeSet::from([
                    "shared-coordinate",
                    "component-assessments",
                    "assembled-witness",
                ]),
                BTreeSet::new(),
            ),
        ),
        (
            "forgotten-coordinate",
            (
                BTreeSet::from(["component-assessments"]),
                BTreeSet::from(["shared-coordinate", "gluing-proof", "assembled-witness"]),
            ),
        ),
        (
            "incomparable-frames",
            (
                BTreeSet::new(),
                BTreeSet::from(["all-cross-frame-judgments"]),
            ),
        ),
    ]);
    let vocabulary = BTreeSet::from([
        "telos",
        "evidence",
        "procedure",
        "assessment",
        "witness",
        "atom",
        "bridge",
        "realization",
        "artifact-coordinate",
        "assessment-outcome",
        "certificate",
        "individual-components",
        "monoidal-equivalence",
        "assembled-witness",
        "shared-coordinate",
        "component-assessments",
        "gluing-proof",
        "all-cross-frame-judgments",
    ]);
    for (id, expected_outcome) in expected {
        let case = object(Some(migration[id]), "migration case is malformed")?;
        require(
            keys(case)
                == BTreeSet::from([
                    "id",
                    "comparison",
                    "procedure",
                    "coordinates",
                    "transported",
                    "lost",
                    "expected",
                ]),
            format!("migration case shape changed: {id}"),
        )?;
        require(
            string(case.get("expected")) == Some(expected_outcome),
            format!("wrong migration fixture: {id}"),
        )?;
        require(
            string(case.get("expected")) == Some(migration_outcome(case)?.as_str()),
            format!("wrong migration result: {id}"),
        )?;
        let transported = string_array(
            case.get("transported"),
            format!("migration detail absent: {id}"),
        )?;
        let lost = string_array(case.get("lost"), format!("migration detail absent: {id}"))?;
        require(
            transported
                .iter()
                .chain(&lost)
                .all(|item| vocabulary.contains(item.as_str())),
            format!("unknown migration detail: {id}"),
        )?;
        let transported_set = transported
            .iter()
            .map(String::as_str)
            .collect::<BTreeSet<_>>();
        let lost_set = lost.iter().map(String::as_str).collect::<BTreeSet<_>>();
        require(
            transported_set.len() == transported.len()
                && lost_set.len() == lost.len()
                && transported_set.is_disjoint(&lost_set),
            format!("migration detail overlaps or duplicates: {id}"),
        )?;
        require(
            (transported_set, lost_set) == details[id],
            format!("migration transport/loss semantics changed: {id}"),
        )?;
        if expected_outcome == "equivalent" {
            require(
                !transported.is_empty() && lost.is_empty(),
                format!("equivalent migration lacks transport or reports loss: {id}"),
            )?;
        } else {
            require(
                !lost.is_empty(),
                format!("non-equivalent migration does not expose loss: {id}"),
            )?;
        }
    }
    Ok(())
}

fn migration_outcome(case: &Map<String, Value>) -> Result<String, String> {
    let comparison = required_string(case.get("comparison"), "unknown migration comparison")?;
    let procedure = required_string(case.get("procedure"), "unknown migration procedure status")?;
    let coordinates = required_string(
        case.get("coordinates"),
        "unknown migration coordinate status",
    )?;
    require(
        ["invertible", "lax", "absent"].contains(&comparison),
        "unknown migration comparison",
    )?;
    require(
        ["supported", "unsupported"].contains(&procedure),
        "unknown migration procedure status",
    )?;
    require(
        ["preserved", "forgotten"].contains(&coordinates),
        "unknown migration coordinate status",
    )?;
    Ok(if comparison == "absent" {
        "incomparable"
    } else if comparison == "invertible" && procedure == "supported" && coordinates == "preserved" {
        "equivalent"
    } else {
        "lossy"
    }
    .into())
}

fn run_self_test(data: &Value, root: &Path) -> Result<(), String> {
    const NAMES: [&str; 36] = [
        "composition-boundary",
        "coherence",
        "claim-reuse",
        "legacy-strengthening",
        "lossy-equivalence",
        "missing-component-coordinate",
        "stale-realization",
        "invented-outcome",
        "invented-migration",
        "invented-legacy-outcome",
        "malformed-evidence-cids",
        "duplicate-witness-id",
        "invented-migration-detail",
        "empty-equivalent-transport",
        "empty-evidence-set",
        "reuse-became-independent",
        "invented-evidence-expected",
        "absent-legacy-name",
        "duplicate-migration-id",
        "reversed-migration-detail",
        "missing-certificate-outcome",
        "undeclared-certificate-component",
        "unbound-component-evidence",
        "missing-shared-coordinate",
        "post-result-assembly-change",
        "procedure-path-mismatch",
        "procedure-repository-mismatch",
        "procedure-commit-mismatch",
        "procedure-digest-mismatch",
        "procedure-version-mismatch",
        "stale-declaration-digest",
        "unresolved-procedure",
        "stored-verdict-transport",
        "hidden-provenance-mismatch",
        "set-relationship-example",
        "reversed-poset-direction",
    ];
    for name in NAMES {
        let mut candidate = data.clone();
        mutate(&mut candidate, name)?;
        if validate_all(&candidate, root).is_ok() {
            return Err(format!("self-test accepted mutation: {name}"));
        }
    }
    Ok(())
}

fn mutate(data: &mut Value, name: &str) -> Result<(), String> {
    match name {
        "composition-boundary" => data
            .pointer_mut("/composition/atoms/1/source")
            .map(|v| *v = json!("WRONG")),
        "coherence" => {
            let Some(outcome) = data.pointer_mut("/certificate_profile/cases/1/certificate/outcome") else {
                return Err("self-test could not find mismatch certificate outcome".into());
            };
            *outcome = json!("certified");
            data.pointer_mut("/certificate_profile/cases/1/expected")
                .map(|v| *v = json!("certified"))
        }
        "claim-reuse" => field_mut(
            data,
            "witness_cases",
            "shared-evidence-reuse",
            "independent_observations",
        )
        .map(|v| *v = json!(2)),
        "legacy-strengthening" => field_mut(data, "witness_cases", "legacy-flat", "expected")
            .map(|v| *v = json!("certified")),
        "lossy-equivalence" => {
            field_mut(data, "migration_cases", "forgotten-coordinate", "expected")
                .map(|v| *v = json!("equivalent"))
        }
        "missing-component-coordinate" => nested_case_field_mut(
            data,
            "witness_cases",
            "independent-components",
            1,
            "coordinate",
        )
        .map(|v| *v = Value::Null),
        "stale-realization" => data
            .pointer_mut("/composition/atoms/0/name")
            .map(|v| *v = json!("BROKEN")),
        "invented-outcome" => {
            nested_case_field_mut(data, "witness_cases", "missing-sufficient", 0, "outcome")
                .map(|v| *v = json!("invented"))
        }
        "invented-migration" => field_mut(
            data,
            "migration_cases",
            "unsupported-procedure",
            "comparison",
        )
        .map(|v| *v = json!("invented")),
        "invented-legacy-outcome" => {
            nested_case_field_mut(data, "witness_cases", "legacy-flat", 0, "outcome")
                .map(|v| *v = json!("invented"))
        }
        "malformed-evidence-cids" => field_mut(
            data,
            "witness_cases",
            "artifact-two-evidence",
            "evidence_cids",
        )
        .map(|v| *v = json!("cid-a")),
        "duplicate-witness-id" => duplicate_first(data, "witness_cases"),
        "invented-migration-detail" => field_mut(data, "migration_cases", "lax-comparison", "lost")
            .and_then(Value::as_array_mut)
            .map(|v| v.push(json!("invented"))),
        "empty-equivalent-transport" => field_mut(
            data,
            "migration_cases",
            "invertible-reindexing",
            "transported",
        )
        .map(|v| *v = json!([])),
        "empty-evidence-set" => field_mut(
            data,
            "witness_cases",
            "artifact-two-evidence",
            "evidence_cids",
        )
        .map(|v| *v = json!([])),
        "reuse-became-independent" => {
            let Some(value) = field_mut(
                data,
                "witness_cases",
                "shared-evidence-reuse",
                "evidence_cids",
            ) else {
                return Err("self-test could not find reuse evidence CIDs".into());
            };
            *value = json!(["cid-a", "cid-b"]);
            field_mut(
                data,
                "witness_cases",
                "shared-evidence-reuse",
                "independent_observations",
            )
            .map(|v| *v = json!(2))
        }
        "invented-evidence-expected" => {
            case_object_mut(data, "witness_cases", "artifact-two-evidence").map(|case| {
                case.insert("expected".into(), json!("invented"));
            })
        }
        "absent-legacy-name" => {
            nested_case_field_mut(data, "witness_cases", "legacy-flat", 0, "name")
                .map(|v| *v = Value::Null)
        }
        "duplicate-migration-id" => duplicate_first(data, "migration_cases"),
        "reversed-migration-detail" => {
            let Some(value) = field_mut(
                data,
                "migration_cases",
                "forgotten-coordinate",
                "transported",
            ) else {
                return Err("self-test could not find transported details".into());
            };
            *value = json!(["shared-coordinate", "gluing-proof", "assembled-witness"]);
            field_mut(data, "migration_cases", "forgotten-coordinate", "lost")
                .map(|v| *v = json!(["component-assessments"]))
        }
        "missing-certificate-outcome" => data
            .pointer_mut("/certificate_profile/cases/0/certificate/components/0/outcome")
            .map(|v| *v = Value::Null),
        "undeclared-certificate-component" => data
            .pointer_mut("/certificate_profile/cases/0/certificate/components/0/name")
            .map(|v| *v = json!("invented")),
        "unbound-component-evidence" => data
            .pointer_mut("/certificate_profile/cases/0/certificate/components/0/evidence_cids/0")
            .map(|v| *v = json!("cid-absent")),
        "missing-shared-coordinate" => data
            .pointer_mut("/certificate_profile/cases/0/certificate/components/1/coordinates/candidate")
            .map(|v| *v = Value::Null),
        "post-result-assembly-change" => data
            .pointer_mut(
                "/certificate_profile/declaration/assembly/shared_coordinates/0/components/1",
            )
            .map(|v| *v = json!("invented")),
        "procedure-path-mismatch" => data
            .pointer_mut("/certificate_profile/cases/0/certificate/procedure_spec/path")
            .map(|v| *v = json!("procedures/other.json")),
        "procedure-repository-mismatch" => data
            .pointer_mut("/certificate_profile/cases/0/certificate/procedure_spec/repository")
            .map(|v| *v = json!("https://example.invalid/other")),
        "procedure-commit-mismatch" => data
            .pointer_mut("/certificate_profile/cases/0/certificate/procedure_spec/commit")
            .map(|v| *v = json!("2222222222222222222222222222222222222222")),
        "procedure-digest-mismatch" => data
            .pointer_mut("/certificate_profile/cases/0/certificate/procedure_spec/sha256")
            .map(|v| {
                *v = json!("eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee")
            }),
        "procedure-version-mismatch" => data
            .pointer_mut("/certificate_profile/cases/0/certificate/procedure_spec/version")
            .map(|v| *v = json!("2")),
        "stale-declaration-digest" => data
            .pointer_mut("/certificate_profile/cases/0/certificate/witness_system/declaration_sha256")
            .map(|v| *v = json!("eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee")),
        "unresolved-procedure" => {
            let replacement = json!({"repository":"https://github.com/kan-tools/day","commit":"2222222222222222222222222222222222222222","path":"missing-procedure.json","sha256":"eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee","version":"1"});
            let Some(declared) = data.pointer_mut("/certificate_profile/declaration/procedure_spec") else {
                return Err("self-test could not find declared procedure".into());
            };
            *declared = replacement.clone();
            for index in 0..2 {
                let Some(address) = data.pointer_mut(&format!("/certificate_profile/cases/{index}/certificate/procedure_spec")) else {
                    return Err("self-test could not find certificate procedure".into());
                };
                *address = replacement.clone();
            }
            let declaration = data.pointer("/certificate_profile/declaration").ok_or("self-test could not find declaration")?;
            let bytes = serde_json_canonicalizer::to_vec(declaration)
                .map_err(|error| error.to_string())?;
            let digest = format!("{:x}", Sha256::digest(bytes));
            for index in 0..2 {
                let Some(binding) = data.pointer_mut(&format!("/certificate_profile/cases/{index}/certificate/witness_system/declaration_sha256")) else {
                    return Err("self-test could not find declaration binding".into());
                };
                *binding = json!(digest);
            }
            Some(())
        }
        "stored-verdict-transport" => field_mut(
            data,
            "frame_read_cases",
            "same-provenance-stored",
            "expected",
        )
        .map(|v| *v = json!("certified")),
        "hidden-provenance-mismatch" => field_mut(
            data,
            "frame_read_cases",
            "different-provenance-stored",
            "expected",
        )
        .map(|v| *v = json!("historical-only")),
        "set-relationship-example" => {
            field_mut(data, "relationship_examples", "sufficient-only", "category")
                .map(|v| *v = json!("Set"))
        }
        "reversed-poset-direction" => {
            field_mut(data, "relationship_examples", "sufficient-only", "reverse")
                .map(|v| *v = json!(true))
        }
        _ => None,
    }
    .ok_or_else(|| format!("self-test could not construct mutation: {name}"))?;
    Ok(())
}

fn field_mut<'a>(
    data: &'a mut Value,
    collection: &str,
    id: &str,
    field: &str,
) -> Option<&'a mut Value> {
    data.get_mut(collection)?
        .as_array_mut()?
        .iter_mut()
        .find(|case| case.get("id").and_then(Value::as_str) == Some(id))?
        .get_mut(field)
}
fn case_object_mut<'a>(
    data: &'a mut Value,
    collection: &str,
    id: &str,
) -> Option<&'a mut Map<String, Value>> {
    data.get_mut(collection)?
        .as_array_mut()?
        .iter_mut()
        .find(|case| case.get("id").and_then(Value::as_str) == Some(id))?
        .as_object_mut()
}
fn nested_case_field_mut<'a>(
    data: &'a mut Value,
    collection: &str,
    id: &str,
    index: usize,
    field: &str,
) -> Option<&'a mut Value> {
    field_mut(data, collection, id, "components")?
        .as_array_mut()?
        .get_mut(index)?
        .get_mut(field)
}
fn duplicate_first(data: &mut Value, collection: &str) -> Option<()> {
    let values = data.get_mut(collection)?.as_array_mut()?;
    let first = values.first()?.clone();
    values.push(first);
    Some(())
}

fn cases_by_id<'a>(
    cases: &'a [Value],
    error: &str,
) -> Result<BTreeMap<&'a str, &'a Value>, String> {
    cases
        .iter()
        .map(|case| required_string(case.get("id"), error).map(|id| (id, case)))
        .collect()
}
fn keys(object: &Map<String, Value>) -> BTreeSet<&str> {
    object.keys().map(String::as_str).collect()
}
fn nested<'a>(object: &'a Map<String, Value>, key: &str, field: &str) -> Option<&'a Value> {
    object.get(key)?.get(field)
}
fn object(value: Option<&Value>, error: impl Into<String>) -> Result<&Map<String, Value>, String> {
    value.and_then(Value::as_object).ok_or_else(|| error.into())
}
fn array(value: Option<&Value>, error: impl Into<String>) -> Result<&Vec<Value>, String> {
    value.and_then(Value::as_array).ok_or_else(|| error.into())
}
fn string(value: Option<&Value>) -> Option<&str> {
    value.and_then(Value::as_str)
}
fn required_string(value: Option<&Value>, error: impl Into<String>) -> Result<&str, String> {
    string(value)
        .filter(|v| !v.is_empty())
        .ok_or_else(|| error.into())
}
fn string_array(value: Option<&Value>, error: impl Into<String>) -> Result<Vec<String>, String> {
    array(value, error)?
        .iter()
        .map(|v| {
            v.as_str()
                .filter(|v| !v.is_empty())
                .map(str::to_owned)
                .ok_or_else(|| "array contains a non-string or empty value".into())
        })
        .collect()
}
fn require(condition: bool, detail: impl Into<String>) -> Result<(), String> {
    condition.then_some(()).ok_or_else(|| detail.into())
}
fn effective_root(root: &Path) -> PathBuf {
    std::env::var_os("DAY_RFC_ROOT")
        .map(PathBuf::from)
        .unwrap_or_else(|| root.to_path_buf())
}
fn failed(detail: impl Into<String>) -> Outcome<()> {
    let detail = detail.into();
    eprintln!("RFC 1 VECTOR CHECK FAILED: {detail}");
    Outcome::Finding(Finding::reported(detail))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn shipped_vectors_and_every_mutation_are_decisive() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
        let data: Value = serde_json::from_str(
            &std::fs::read_to_string(root.join("rfcs/vectors/1-process-model.json")).unwrap(),
        )
        .unwrap();
        validate_all(&data, root).unwrap();
        run_self_test(&data, root).unwrap();
    }
}
