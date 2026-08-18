use std::collections::BTreeSet;
use std::path::Path;

use serde::Deserialize;

use crate::outcome::{Finding, Outcome};

const EXPECTED: &[&str] = &[
    "command:validate/profile",
    "command:validate/rfc",
    "command:validate/publication",
    "command:validate/vectors",
    "command:validate/formal",
    "command:validate/instrumentation",
    "command:validate/review",
    "command:evidence/behaviour-diff",
    "command:evidence/mutate",
    "command:evidence/revert",
    "command:census/demonstrations",
    "command:census/findings",
    "command:release/verify-v013",
    "command:release/verify-plan-v013",
    "command:release/verify-candidate-v013",
    "command:release/verify-publication-v013",
    "workflow:ci",
    "workflow:agent-plugins",
    "workflow:kan-compat",
    "workflow:migration-matrix",
    "workflow:askme-behavioral-trial",
    "workflow:workflow-reconstruction-trial",
    "workflow:release",
    "protocol:askme-v1",
    "protocol:reconstruction-v1",
];

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Inventory {
    schema: u64,
    policy: String,
    default_threat_model: String,
    cost_by_trigger: CostByTrigger,
    owner_by_kind: OwnerByKind,
    lifecycle_rules: LifecycleRules,
    entries: Vec<Entry>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct CostByTrigger {
    local: String,
    ci: String,
    manual: String,
    release: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct OwnerByKind {
    command: String,
    workflow: String,
    protocol: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct LifecycleRules {
    permanent: String,
    release_scoped: String,
    post_merge: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Entry {
    id: String,
    layer: String,
    trigger: String,
    conclusion: String,
    does_not_establish: String,
    lifecycle: String,
    adversarial_attestation: bool,
}

pub fn run(root: &Path, manifest: &Path) -> Outcome<()> {
    match validate(root, manifest) {
        Ok(count) => {
            println!("instrumentation inventory verified: {count} classified surfaces");
            Outcome::Passed(())
        }
        Err(error) => Outcome::Finding(Finding::new(error)),
    }
}

fn validate(root: &Path, manifest: &Path) -> Result<usize, String> {
    let path = root.join(manifest);
    let bytes = std::fs::read(&path)
        .map_err(|error| format!("could not read `{}`: {error}", path.display()))?;
    let inventory: Inventory = serde_json::from_slice(&bytes)
        .map_err(|error| format!("`{}` is malformed: {error}", path.display()))?;
    if inventory.schema != 1
        || inventory.policy != ".design/instrumentation-policy.md"
        || inventory.default_threat_model != "honest-producer-bugs-omissions-and-corruption"
    {
        return Err("instrumentation inventory changed its policy identity or threat model".into());
    }
    let metadata = [
        &inventory.cost_by_trigger.local,
        &inventory.cost_by_trigger.ci,
        &inventory.cost_by_trigger.manual,
        &inventory.cost_by_trigger.release,
        &inventory.owner_by_kind.command,
        &inventory.owner_by_kind.workflow,
        &inventory.owner_by_kind.protocol,
        &inventory.lifecycle_rules.permanent,
        &inventory.lifecycle_rules.release_scoped,
        &inventory.lifecycle_rules.post_merge,
    ];
    if metadata.iter().any(|value| value.trim().is_empty()) {
        return Err(
            "instrumentation inventory has empty cost, owner, or lifecycle metadata".into(),
        );
    }
    let allowed_layers = [
        "deterministic-invariant",
        "repository-integration",
        "external-conformance",
        "observational-trial",
        "release-reconciliation",
    ];
    let allowed_triggers = ["local", "ci", "manual", "release"];
    let allowed_lifecycle = ["permanent", "release-scoped", "post-merge"];
    let mut ids = BTreeSet::new();
    for entry in &inventory.entries {
        if !ids.insert(entry.id.as_str()) {
            return Err(format!("duplicate instrumentation entry `{}`", entry.id));
        }
        if !allowed_layers.contains(&entry.layer.as_str())
            || !allowed_triggers.contains(&entry.trigger.as_str())
            || !allowed_lifecycle.contains(&entry.lifecycle.as_str())
            || entry.conclusion.trim().is_empty()
            || entry.does_not_establish.trim().is_empty()
        {
            return Err(format!(
                "instrumentation entry `{}` is incompletely classified",
                entry.id
            ));
        }
        if entry.adversarial_attestation {
            return Err(format!(
                "instrumentation entry `{}` enables adversarial attestation without a separately approved threat-model contract",
                entry.id
            ));
        }
    }
    let expected: BTreeSet<_> = EXPECTED.iter().copied().collect();
    if ids != expected {
        let missing: Vec<_> = expected.difference(&ids).copied().collect();
        let unknown: Vec<_> = ids.difference(&expected).copied().collect();
        return Err(format!(
            "instrumentation inventory is not exhaustive; missing={missing:?} unknown={unknown:?}"
        ));
    }
    for relative in [
        ".design/instrumentation-policy.md",
        ".github/workflows/ci.yml",
        ".github/workflows/agent-plugins.yml",
        ".github/workflows/kan-compat.yml",
        ".github/workflows/migration-matrix.yml",
        ".github/workflows/askme-behavioral-trial.yml",
        ".github/workflows/workflow-reconstruction-trial.yml",
        ".github/workflows/release.yml",
        ".release/protocols/askme-v1.json",
        ".release/protocols/reconstruction-v1.json",
    ] {
        if !root.join(relative).is_file() {
            return Err(format!(
                "classified instrumentation surface `{relative}` is missing"
            ));
        }
    }
    Ok(inventory.entries.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expected_inventory_ids_are_unique() {
        assert_eq!(
            EXPECTED.iter().copied().collect::<BTreeSet<_>>().len(),
            EXPECTED.len()
        );
    }
}
