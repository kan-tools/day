use std::collections::BTreeSet;
use std::path::Path;

use clap::{Command, CommandFactory};
use serde::Deserialize;

use crate::command::Cli;
use crate::outcome::{Finding, Outcome};

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
        if !ids.insert(entry.id.clone()) {
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
    for (id, required_layer) in [
        ("command:validate/rfc", "repository-integration"),
        ("command:validate/publication", "external-conformance"),
        ("command:validate/vectors", "deterministic-invariant"),
        ("command:validate/formal", "deterministic-invariant"),
    ] {
        let entry = inventory
            .entries
            .iter()
            .find(|entry| entry.id == id)
            .ok_or_else(|| format!("instrumentation inventory is missing `{id}`"))?;
        if entry.layer != required_layer {
            return Err(format!(
                "instrumentation entry `{id}` misclassifies its checked dependency boundary; expected `{required_layer}`"
            ));
        }
    }
    let expected = discover_surfaces(root)?;
    if ids != expected {
        let missing: Vec<_> = expected.difference(&ids).cloned().collect();
        let unknown: Vec<_> = ids.difference(&expected).cloned().collect();
        return Err(format!(
            "instrumentation inventory is not exhaustive; missing={missing:?} unknown={unknown:?}"
        ));
    }
    if !root.join(".design/instrumentation-policy.md").is_file() {
        return Err("classified instrumentation policy is missing".into());
    }
    Ok(inventory.entries.len())
}

fn discover_surfaces(root: &Path) -> Result<BTreeSet<String>, String> {
    let mut surfaces = BTreeSet::new();
    collect_commands(&Cli::command(), "", &mut surfaces);
    discover_files(
        root,
        ".github/workflows",
        &["yml", "yaml"],
        "workflow",
        &mut surfaces,
    )?;
    discover_files(
        root,
        ".release/protocols",
        &["json"],
        "protocol",
        &mut surfaces,
    )?;
    Ok(surfaces)
}

fn collect_commands(command: &Command, prefix: &str, surfaces: &mut BTreeSet<String>) {
    let children: Vec<_> = command
        .get_subcommands()
        .filter(|child| child.get_name() != "help")
        .collect();
    if children.is_empty() {
        if !prefix.is_empty() {
            surfaces.insert(format!("command:{prefix}"));
        }
        return;
    }
    for child in children {
        let path = if prefix.is_empty() {
            child.get_name().to_owned()
        } else {
            format!("{prefix}/{}", child.get_name())
        };
        collect_commands(child, &path, surfaces);
    }
}

fn discover_files(
    root: &Path,
    directory: &str,
    extensions: &[&str],
    kind: &str,
    surfaces: &mut BTreeSet<String>,
) -> Result<(), String> {
    let directory = root.join(directory);
    let entries = std::fs::read_dir(&directory)
        .map_err(|error| format!("could not enumerate `{}`: {error}", directory.display()))?;
    for entry in entries {
        let path = entry
            .map_err(|error| format!("could not enumerate instrumentation: {error}"))?
            .path();
        let extension = path.extension().and_then(|value| value.to_str());
        if extension.is_some_and(|value| extensions.contains(&value)) {
            let stem = path
                .file_stem()
                .and_then(|value| value.to_str())
                .ok_or_else(|| format!("instrumentation path `{}` is not UTF-8", path.display()))?;
            surfaces.insert(format!("{kind}:{stem}"));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn command_discovery_uses_the_real_clap_tree() {
        let mut commands = BTreeSet::new();
        collect_commands(&Cli::command(), "", &mut commands);
        assert!(commands.contains("command:validate/instrumentation"));
        assert!(commands.contains("command:release/verify-publication-v013"));
        assert!(!commands.iter().any(|command| command.ends_with("/help")));
    }

    #[test]
    fn workflow_discovery_sees_a_file_that_was_not_known_when_compiled() {
        let root = tempfile::tempdir().unwrap();
        let workflows = root.path().join(".github/workflows");
        std::fs::create_dir_all(&workflows).unwrap();
        std::fs::write(workflows.join("future-check.yml"), "name: future\n").unwrap();
        let mut found = BTreeSet::new();
        discover_files(
            root.path(),
            ".github/workflows",
            &["yml", "yaml"],
            "workflow",
            &mut found,
        )
        .unwrap();
        assert_eq!(found, BTreeSet::from(["workflow:future-check".to_owned()]));
    }
}
