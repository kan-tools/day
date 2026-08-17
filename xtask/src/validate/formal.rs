use std::ffi::OsString;
use std::path::{Path, PathBuf};

use crate::outcome::{CouldNotCheck, Finding, Outcome};

const REQUIRED_CHOICES: [&str; 4] = [
    "Epistemic site and telos-relative topology",
    "Realization prestack, descent, and model structure",
    "Obstruction coefficients and cohomology theory",
    "Effective realization fragment and provability ledger",
];

pub fn run(root: &Path, args: &[OsString]) -> Outcome<()> {
    let self_test = match args {
        [] => false,
        [flag] if flag == "--self-test" => true,
        _ => {
            return Outcome::CouldNotCheck(CouldNotCheck::new(
                "usage: xtask validate formal [--self-test]",
            ))
        }
    };
    let root = effective_root(root);
    let rfc = match read(&root.join("rfcs/1-frame-indexed-process-model.md")) {
        Ok(source) => source,
        Err(detail) => return failed(detail),
    };
    let companion = match read(&root.join("rfcs/1/denotational-semantics.md")) {
        Ok(source) => source,
        Err(detail) => return failed(detail),
    };
    if let Err(detail) = validate(&rfc, &companion) {
        return failed(detail);
    }
    if self_test {
        if let Err(detail) = run_self_test(&rfc, &companion) {
            return failed(detail);
        }
    }
    println!("RFC 1 formal vocabulary and unresolved obligations: valid");
    Outcome::Passed(())
}

pub fn validate(rfc: &str, companion: &str) -> Result<(), String> {
    require(
        rfc.contains(r"W_T:\mathcal I_T\to")
            && companion.contains(r"W_T:\mathcal I_T\longrightarrow"),
        r"witness diagrams must use the declared indexing category \mathcal I_T",
    )?;
    require(
        !contains_witness_topology_collision(rfc)
            && !contains_witness_topology_collision(companion),
        "J_T cannot be both a witness indexing category and a Grothendieck topology",
    )?;
    require(
        companion.contains(r"A Grothendieck topology $J_T$ on $\mathcal C^T_{A,f_0}$"),
        "the telos-relative Grothendieck topology J_T is absent or renamed inconsistently",
    )?;
    for choice in REQUIRED_CHOICES {
        require(
            rfc.contains(&format!("| {choice} |")),
            format!("RFC 1 unresolved-question table lacks: {choice}"),
        )?;
    }
    require(
        companion.contains(r"{\scriptstyle 1_{X_0}}\downarrow")
            && companion.contains(r"{\scriptstyle 1_X}\downarrow")
            && companion.contains(r"\downarrow{\scriptstyle 1_I}"),
        "realization, identity, and pasting diagrams must be globular cells",
    )?;
    let compact = companion
        .chars()
        .filter(|character| !character.is_whitespace())
        .collect::<String>();
    for predicate in ["P_0", "P_1", "P_2", "T"] {
        require(
            !compact.contains(&format!(r"{{\scriptstyle{predicate}}}\Downarrow"))
                && !compact.contains(&format!(r"\Downarrow{{\scriptstyle{predicate}}}")),
            format!("horizontal predicate {predicate} appears as a vertical boundary"),
        )?;
    }
    require(
        companion.contains(r"\underset{\eta_2\ast A_1}{\Rightarrow}")
            && companion.contains(r"P_2\odot A_2\odot A_1\;}{\rightsquigarrow}"),
        "two-atom pasting must whisker into a globular vertical composite",
    )?;
    require(
        rfc.contains("finite poset")
            && companion.contains(r"finite poset $0<1$")
            && companion.contains("the reverse does not"),
        "witness relationship examples must discriminate arrow direction",
    )?;
    Ok(())
}

fn run_self_test(rfc: &str, companion: &str) -> Result<(), String> {
    let collision = rfc.replacen(r"W_T:\mathcal I_T\to", r"W_T:J_T\to", 1);
    reject_mutation("witness-topology-collision", &collision, companion)?;
    for choice in REQUIRED_CHOICES {
        let candidate = rfc.replacen(&format!("| {choice} |"), "| Removed choice |", 1);
        let name = format!("missing-{}", choice.to_lowercase().replace(' ', "-"));
        reject_mutation(&name, &candidate, companion)?;
    }
    let ill_typed = companion.replacen(
        r"{\scriptstyle 1_{X_0}}\downarrow",
        r"{\scriptstyle P_0}\Downarrow",
        1,
    );
    reject_mutation("ill-typed-equipment-square", rfc, &ill_typed)?;
    let ill_typed_pasting = companion.replacen(
        r"{\scriptstyle 1_{X_0}}\downarrow & \underset{\eta_1}{\Rightarrow}",
        r"{\scriptstyle P_0}\Downarrow & \underset{\eta_1}{\Rightarrow}",
        1,
    );
    reject_mutation("ill-typed-pasting-square", rfc, &ill_typed_pasting)?;
    let set_example = companion.replacen(r"finite poset $0<1$", r"category $\mathbf{Set}$", 1);
    reject_mutation("non-discriminating-set-example", rfc, &set_example)?;
    Ok(())
}

fn reject_mutation(name: &str, rfc: &str, companion: &str) -> Result<(), String> {
    if validate(rfc, companion).is_err() {
        println!("RFC 1 formal-obligation self-test: {name} mutation rejected");
        Ok(())
    } else {
        Err(format!("self-test accepted mutation: {name}"))
    }
}

fn contains_witness_topology_collision(source: &str) -> bool {
    source
        .chars()
        .filter(|character| !character.is_whitespace())
        .collect::<String>()
        .contains("W_T:J_T")
}

fn require(condition: bool, detail: impl Into<String>) -> Result<(), String> {
    condition.then_some(()).ok_or_else(|| detail.into())
}

fn read(path: &Path) -> Result<String, String> {
    std::fs::read_to_string(path).map_err(|error| format!("{}: {error}", path.display()))
}

fn effective_root(root: &Path) -> PathBuf {
    std::env::var_os("DAY_RFC_ROOT")
        .map(PathBuf::from)
        .unwrap_or_else(|| root.to_path_buf())
}

fn failed(detail: impl Into<String>) -> Outcome<()> {
    let detail = detail.into();
    eprintln!("RFC 1 FORMAL OBLIGATION CHECK FAILED: {detail}");
    Outcome::Finding(Finding::reported(detail))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_formal_self_test_mutation_is_rejected() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
        let rfc = read(&root.join("rfcs/1-frame-indexed-process-model.md")).unwrap();
        let companion = read(&root.join("rfcs/1/denotational-semantics.md")).unwrap();
        validate(&rfc, &companion).unwrap();
        run_self_test(&rfc, &companion).unwrap();
    }
}
