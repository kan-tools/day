use std::collections::{BTreeMap, BTreeSet};
use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};

use serde_json::Value;

use crate::capability::process::{Process, ProcessRequest};
use crate::outcome::{CouldNotCheck, Finding, Outcome};

use super::publication::{self, PublicationKind};

const RFC_SECTIONS: [&str; 16] = [
    "Summary",
    "Motivation",
    "Terminology",
    "Denotational target",
    "Operational profile v1",
    "Approximation map",
    "Canonicalization and equivalence",
    "Resolution or processing algorithm",
    "Authority and trust model",
    "Security considerations",
    "Compatibility",
    "Alternatives considered",
    "Reference test vectors",
    "Unresolved questions",
    "Deferred questions",
    "Implementation status",
];
const ADR_SECTIONS: [&str; 7] = [
    "Context",
    "Decision",
    "Rationale",
    "Consequences",
    "Evidence",
    "Alternatives considered",
    "Supersession",
];
const RFC_FIELDS: [&str; 11] = [
    "Status",
    "Authors",
    "Created",
    "Discussion",
    "Review-started-at",
    "Review-period-ends",
    "Review-override",
    "Supersedes",
    "Superseded-by",
    "Profile-relationship",
    "Implementation",
];
const ADR_FIELDS: [&str; 6] = [
    "Status",
    "Date",
    "Authors",
    "Supersedes",
    "Superseded-by",
    "Related-RFC",
];

#[derive(Debug)]
struct CheckError {
    detail: String,
    unavailable: bool,
}

pub fn run(root: &Path, process: &dyn Process, self_test: bool) -> Outcome<()> {
    let root = effective_root(root);
    let baseline = std::env::var_os("DAY_RFC_BASE_REGISTRY").map(PathBuf::from);
    let publication_skip =
        std::env::var_os("DAY_RFC_PUBLICATION_SKIP").is_some_and(|value| value == "1");
    match check(&root, process, publication_skip, baseline.as_deref()) {
        Err(error) => reported(error),
        Ok(_) if self_test => match run_self_tests(&root, process, publication_skip) {
            Ok(()) => Outcome::Passed(()),
            Err(error) => reported(error),
        },
        Ok((rfcs, adrs)) => {
            println!("RFC/ADR check: {rfcs} RFC(s), {adrs} ADR(s), templates, allocation registry, and indexes valid");
            Outcome::Passed(())
        }
    }
}

fn check(
    root: &Path,
    process: &dyn Process,
    publication_skip: bool,
    baseline_registry: Option<&Path>,
) -> Result<(usize, usize), CheckError> {
    for path in [
        "rfcs/README.md",
        "rfcs/template.md",
        "rfcs/numbers.tsv",
        "adrs/README.md",
        "adrs/template.md",
    ] {
        require(root.join(path).is_file(), format!("missing {path}"))?;
    }
    check_rfc_shape(root, process, "rfcs/template.md")?;
    check_adr_shape(root, "adrs/template.md")?;
    for script in [
        "scripts/check-rfc1-vectors.py",
        "scripts/check-rfc0-publication.py",
        "scripts/check-rfc1-denotational-publication.py",
        "scripts/check-rfc1-formal-obligations.py",
        "scripts/check-rfc-review.py",
    ] {
        require(
            is_executable(&root.join(script)),
            format!("{script} is not executable"),
        )?;
    }
    require(
        std::fs::metadata(root.join("rfcs/maintainers.txt"))
            .map(|meta| meta.len() > 0)
            .unwrap_or(false),
        "rfcs/maintainers.txt is absent or empty",
    )?;

    if !publication_skip {
        require_passed(
            publication::run(PublicationKind::Rfc0, root, process, &[]),
            "RFC 0 publication could not be verified",
        )?;
        require_passed(
            publication::run(PublicationKind::Denotational, root, process, &[]),
            "RFC 1 denotational publication could not be verified",
        )?;
    }
    let vector_source = read(root, "rfcs/vectors/1-process-model.json")?;
    let vectors: Value =
        serde_json::from_str(&vector_source).map_err(|error| finding(error.to_string()))?;
    super::vectors::validate(&vectors).map_err(finding)?;
    println!("RFC 1 vectors: valid");

    let rfc_files = numbered_files(root.join("rfcs"), true)?;
    let adr_files = numbered_files(root.join("adrs"), false)?;
    for file in rfc_files.iter().chain(&adr_files) {
        require(
            !read_path(file)?
                .lines()
                .any(|line| line.starts_with("- Kan-claim:")),
            "normative RFC bytes contain a claim-CID backlink",
        )?;
    }
    let registry = parse_registry(&read(root, "rfcs/numbers.tsv")?)?;
    let rfc_index = read(root, "rfcs/README.md")?;
    let mut seen = BTreeSet::new();
    for file in &rfc_files {
        let base = file.file_name().unwrap().to_string_lossy();
        let number = file_number(&base, true)?;
        require(
            seen.insert(number),
            format!("duplicate RFC number {number}"),
        )?;
        let relative = format!("rfcs/{base}");
        check_rfc_shape(root, process, &relative)?;
        let source = read_path(file)?;
        let (heading_number, heading) = heading(&source, "RFC")?;
        require(
            heading_number == number,
            format!("{relative} heading number differs from filename"),
        )?;
        let status = field_value(&source, "Status").unwrap_or_default();
        let expected = format!("- [RFC {number}: {heading}]({base}) — {status}");
        require(
            rfc_index.lines().filter(|line| *line == expected).count() == 1,
            format!("{relative} index row is missing or disagrees with title/status"),
        )?;
        require(
            registry.get(&number) == Some(&(base.to_string(), heading.to_owned())),
            format!("{relative} disagrees with rfcs/numbers.tsv allocation"),
        )?;
    }
    require(
        registry.len() == rfc_files.len(),
        "rfcs/numbers.tsv contains stale or missing allocations",
    )?;
    let baseline = baseline_contents(root, process, baseline_registry)?;
    for (number, (file, title)) in parse_registry(&baseline)? {
        require(
            registry.get(&number) == Some(&(file.clone(), title)),
            format!("historical RFC allocation changed: {number} -> {file}"),
        )?;
    }
    require(
        index_rows(&rfc_index, "RFC") == rfc_files.len(),
        "rfcs/README.md contains stale or missing RFC rows",
    )?;

    let adr_index = read(root, "adrs/README.md")?;
    let mut adr_seen = BTreeSet::new();
    for file in &adr_files {
        let base = file.file_name().unwrap().to_string_lossy();
        let number = file_number(&base, false)?;
        require(
            adr_seen.insert(number),
            format!("duplicate ADR number {number}"),
        )?;
        let relative = format!("adrs/{base}");
        check_adr_shape(root, &relative)?;
        let source = read_path(file)?;
        let (heading_number, title) = heading(&source, "ADR")?;
        require(
            heading_number == number,
            format!("{relative} heading number differs from filename"),
        )?;
        let status = field_value(&source, "Status").unwrap_or_default();
        let expected = format!("- [ADR {number}: {title}]({base}) — {status}");
        require(
            adr_index.lines().filter(|line| *line == expected).count() == 1,
            format!("{relative} index row is missing or disagrees with title/status"),
        )?;
    }
    require(
        index_rows(&adr_index, "ADR") == adr_files.len(),
        "adrs/README.md contains stale or missing ADR rows",
    )?;

    require(
        root.join("rfcs/1/denotational-semantics.md").is_file(),
        "missing RFC 1 denotational companion source",
    )?;
    require(
        root.join("rfcs/1/denotational-semantics.html").is_file(),
        "missing RFC 1 denotational companion HTML",
    )?;
    let rfc1 = read(root, "rfcs/1-frame-indexed-process-model.md")?;
    let html = read(root, "rfcs/1/denotational-semantics.html")?;
    require(
        rfc1.contains("[`rfcs/1/denotational-semantics.md`](1/denotational-semantics.md)"),
        "RFC 1 does not incorporate its denotational companion",
    )?;
    require(
        html.contains("Canonical source: <a href=\"denotational-semantics.md\">"),
        "denotational HTML does not link its canonical source",
    )?;
    require(
        html.contains("mathjax@3/es5/tex-svg.js"),
        "denotational HTML lacks MathJax rendering",
    )?;
    let rendered = command(
        root,
        process,
        "python3",
        ["scripts/render-denotational-semantics.py", "--check"],
    )
    .map_err(|error| {
        if error.unavailable {
            error
        } else {
            finding("denotational HTML is not the current rendering of its source")
        }
    })?;
    print!("{rendered}");
    let companion = read(root, "rfcs/1/denotational-semantics.md")?;
    super::formal::validate(&rfc1, &companion).map_err(finding)?;
    println!("RFC 1 formal vocabulary and unresolved obligations: valid");
    Ok((rfc_files.len(), adr_files.len()))
}

fn check_rfc_shape(root: &Path, process: &dyn Process, path: &str) -> Result<(), CheckError> {
    let source = read(root, path)?;
    require_fields(&source, path, &RFC_FIELDS)?;
    require_sections(&source, path, &RFC_SECTIONS)?;
    let status = field_value(&source, "Status").unwrap();
    require(
        [
            "Draft",
            "Review",
            "Accepted",
            "Implemented",
            "Rejected",
            "Withdrawn",
            "Superseded",
        ]
        .contains(&status),
        format!("{path} has unrecognized status: {status}"),
    )?;
    let relationship = field_value(&source, "Profile-relationship").unwrap();
    require(
        ["not-applicable", "approximation"].contains(&relationship),
        format!("{path} has unrecognized Profile-relationship: {relationship}"),
    )?;
    if ["Accepted", "Implemented", "Superseded"].contains(&status) {
        let discussion = field_value(&source, "Discussion").unwrap();
        let start = field_value(&source, "Review-started-at").unwrap();
        let end = field_value(&source, "Review-period-ends").unwrap();
        let over = field_value(&source, "Review-override").unwrap();
        super::review::check_shape(discussion, start, end, over).map_err(|error| {
            let (detail, unavailable) = error.into_parts();
            CheckError {
                detail: format!("{path} is {status} but {detail}"),
                unavailable,
            }
        })?;
        super::review::check(root, process, path, discussion, start, end, over).map_err(
            |error| {
                let (detail, unavailable) = error.into_parts();
                CheckError {
                    detail,
                    unavailable,
                }
            },
        )?;
    }
    Ok(())
}
fn check_adr_shape(root: &Path, path: &str) -> Result<(), CheckError> {
    let source = read(root, path)?;
    require_fields(&source, path, &ADR_FIELDS)?;
    require_sections(&source, path, &ADR_SECTIONS)?;
    let status = field_value(&source, "Status").unwrap();
    require(
        [
            "Proposed",
            "Accepted",
            "Rejected",
            "Deprecated",
            "Superseded",
        ]
        .contains(&status),
        format!("{path} has unrecognized status: {status}"),
    )
}

fn require_fields(source: &str, path: &str, fields: &[&str]) -> Result<(), CheckError> {
    for field in fields {
        require(
            field_value(source, field).is_some_and(|v| !v.is_empty()),
            format!("{path} lacks metadata: {field}"),
        )?;
    }
    Ok(())
}
fn require_sections(source: &str, path: &str, sections: &[&str]) -> Result<(), CheckError> {
    for section in sections {
        require(
            source.lines().any(|line| line == format!("## {section}")),
            format!("{path} lacks section: {section}"),
        )?;
    }
    Ok(())
}
fn field_value<'a>(source: &'a str, field: &str) -> Option<&'a str> {
    let prefix = format!("- {field}: ");
    source.lines().find_map(|line| line.strip_prefix(&prefix))
}
fn numbered_files(dir: PathBuf, allow_zero: bool) -> Result<Vec<PathBuf>, CheckError> {
    let mut files = std::fs::read_dir(&dir)
        .map_err(|error| unavailable(error.to_string()))?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| path.extension() == Some(OsStr::new("md")))
        .filter(|path| {
            path.file_name().is_some_and(|name| {
                name.to_string_lossy()
                    .chars()
                    .next()
                    .is_some_and(|c| c.is_ascii_digit())
            })
        })
        .collect::<Vec<_>>();
    files.sort();
    for path in &files {
        file_number(&path.file_name().unwrap().to_string_lossy(), allow_zero)?;
    }
    Ok(files)
}
fn file_number(name: &str, allow_zero: bool) -> Result<u32, CheckError> {
    let token = name.split('-').next().unwrap_or("");
    let number = token
        .parse::<u32>()
        .map_err(|_| finding(format!("{name} does not use shortest decimal numbering")))?;
    require(
        (allow_zero || number > 0) && number.to_string() == token,
        format!(
            "{name} does not use {}shortest decimal numbering",
            if allow_zero { "" } else { "positive " }
        ),
    )?;
    Ok(number)
}
fn heading<'a>(source: &'a str, kind: &str) -> Result<(u32, &'a str), CheckError> {
    let line = source.lines().next().unwrap_or("");
    let rest = line
        .strip_prefix(&format!("# {kind} "))
        .ok_or_else(|| finding(format!("{kind} heading is malformed")))?;
    let (number, title) = rest
        .split_once(": ")
        .ok_or_else(|| finding(format!("{kind} heading is malformed")))?;
    Ok((
        number
            .parse()
            .map_err(|_| finding(format!("{kind} heading is malformed")))?,
        title,
    ))
}
fn parse_registry(source: &str) -> Result<BTreeMap<u32, (String, String)>, CheckError> {
    let mut rows = BTreeMap::new();
    for line in source
        .lines()
        .skip(1)
        .filter(|line| !line.trim().is_empty())
    {
        let parts = line.split('\t').collect::<Vec<_>>();
        require(parts.len() == 3, "rfcs/numbers.tsv row is malformed")?;
        let number = parts[0]
            .parse()
            .map_err(|_| finding("rfcs/numbers.tsv number is malformed"))?;
        rows.insert(number, (parts[1].into(), parts[2].into()));
    }
    Ok(rows)
}
fn index_rows(source: &str, kind: &str) -> usize {
    source
        .lines()
        .filter(|line| {
            line.starts_with(&format!("- [{kind} "))
                && line.contains("](")
                && line.contains(".md) — ")
        })
        .count()
}

fn baseline_contents(
    root: &Path,
    process: &dyn Process,
    explicit: Option<&Path>,
) -> Result<String, CheckError> {
    if let Some(path) = explicit {
        return read_path(path);
    }
    let exists = command_status(
        root,
        process,
        "git",
        ["cat-file", "-e", "main:rfcs/numbers.tsv"],
    )?;
    if exists == 0 {
        return command(root, process, "git", ["show", "main:rfcs/numbers.tsv"]);
    }
    let log = command(
        root,
        process,
        "git",
        [
            "log",
            "--diff-filter=A",
            "--format=%H",
            "--",
            "rfcs/numbers.tsv",
        ],
    )?;
    if let Some(commit) = log.lines().last().filter(|line| !line.is_empty()) {
        command(
            root,
            process,
            "git",
            ["show", &format!("{commit}:rfcs/numbers.tsv")],
        )
    } else {
        Ok(String::new())
    }
}
fn command<I, S>(
    cwd: &Path,
    process: &dyn Process,
    program: &str,
    args: I,
) -> Result<String, CheckError>
where
    I: IntoIterator<Item = S>,
    S: Into<OsString>,
{
    let request = ProcessRequest::new(program, args, cwd);
    let output = process.run(&request).map_err(unavailable)?;
    if output.status == 0 {
        Ok(output.stdout)
    } else {
        Err(finding(format!(
            "{}: {}",
            request.display(),
            output.stderr.trim()
        )))
    }
}
fn command_status<I, S>(
    cwd: &Path,
    process: &dyn Process,
    program: &str,
    args: I,
) -> Result<i32, CheckError>
where
    I: IntoIterator<Item = S>,
    S: Into<OsString>,
{
    let request = ProcessRequest::new(program, args, cwd);
    process
        .run(&request)
        .map(|out| out.status)
        .map_err(unavailable)
}
fn require_passed(outcome: Outcome<()>, detail: &str) -> Result<(), CheckError> {
    match outcome {
        Outcome::Passed(()) => Ok(()),
        Outcome::Finding(f) => Err(finding(if f.detail.is_empty() {
            detail
        } else {
            &f.detail
        })),
        Outcome::CouldNotCheck(e) => Err(unavailable(if e.detail.is_empty() {
            detail
        } else {
            &e.detail
        })),
    }
}
fn read(root: &Path, path: &str) -> Result<String, CheckError> {
    read_path(&root.join(path))
}
fn read_path(path: &Path) -> Result<String, CheckError> {
    std::fs::read_to_string(path)
        .map_err(|error| unavailable(format!("{}: {error}", path.display())))
}
fn require(condition: bool, detail: impl Into<String>) -> Result<(), CheckError> {
    condition.then_some(()).ok_or_else(|| finding(detail))
}
fn finding(detail: impl Into<String>) -> CheckError {
    CheckError {
        detail: detail.into(),
        unavailable: false,
    }
}
fn unavailable(detail: impl Into<String>) -> CheckError {
    CheckError {
        detail: detail.into(),
        unavailable: true,
    }
}
fn reported(error: CheckError) -> Outcome<()> {
    eprintln!("RFC/ADR CHECK FAILED: {}", error.detail);
    if error.unavailable {
        Outcome::CouldNotCheck(CouldNotCheck::reported(error.detail, 2))
    } else {
        Outcome::Finding(Finding::reported(error.detail))
    }
}
fn effective_root(root: &Path) -> PathBuf {
    std::env::var_os("DAY_RFC_ROOT")
        .map(PathBuf::from)
        .unwrap_or_else(|| root.to_path_buf())
}
#[cfg(unix)]
fn is_executable(path: &Path) -> bool {
    use std::os::unix::fs::PermissionsExt;
    std::fs::metadata(path)
        .is_ok_and(|meta| meta.is_file() && meta.permissions().mode() & 0o111 != 0)
}
#[cfg(not(unix))]
fn is_executable(path: &Path) -> bool {
    path.is_file()
}

fn run_self_tests(
    root: &Path,
    process: &dyn Process,
    publication_skip: bool,
) -> Result<(), CheckError> {
    let fixture = tempfile::Builder::new()
        .prefix("day-rfc-check.")
        .tempdir()
        .map_err(|error| unavailable(error.to_string()))?;
    for label in [
        "rfc-template-section",
        "adr-template-section",
        "adr-metadata",
        "stale-index",
        "status-mismatch",
        "heading-number",
        "allocation-reuse",
        "accepted-metadata",
        "historical-renumber",
        "forged-review",
        "short-review",
        "profile-relationship",
        "coherence-vector",
        "recursive-publication",
        "denotational-source",
        "denotational-math",
        "denotational-freshness",
    ] {
        reset_fixture(root, fixture.path())?;
        mutate_fixture(fixture.path(), label)?;
        let baseline = fixture.path().join("base-numbers.tsv");
        match check(fixture.path(), process, true, Some(&baseline)) {
            Ok(_) => return Err(finding(format!("self-test accepted {label} mutation"))),
            Err(error) => {
                let expected = expected_error(label);
                require(
                    error.detail.contains(expected),
                    format!(
                        "self-test {label} failed for the wrong reason: {}",
                        error.detail
                    ),
                )?;
                println!("RFC/ADR self-test: {label} mutation rejected");
            }
        }
    }
    if !publication_skip {
        require_passed(
            publication::run(
                PublicationKind::Rfc0,
                root,
                process,
                &["--self-test".into()],
            ),
            "RFC 0 publication self-test failed",
        )?;
        require_passed(
            publication::run(
                PublicationKind::Denotational,
                root,
                process,
                &["--self-test".into()],
            ),
            "RFC 1 publication self-test failed",
        )?;
    }
    let self_test = [OsString::from("--self-test")];
    require_passed(
        super::vectors::run(root, &self_test),
        "RFC 1 vector self-test failed",
    )?;
    require_passed(
        super::formal::run(root, &self_test),
        "RFC 1 formal-obligation self-test failed",
    )?;
    Ok(())
}
fn reset_fixture(root: &Path, fixture: &Path) -> Result<(), CheckError> {
    for name in ["rfcs", "adrs", "scripts"] {
        let target = fixture.join(name);
        if target.exists() {
            std::fs::remove_dir_all(&target).map_err(|error| unavailable(error.to_string()))?;
        }
        copy_dir(&root.join(name), &target)?;
    }
    std::fs::copy(
        root.join("rfcs/numbers.tsv"),
        fixture.join("base-numbers.tsv"),
    )
    .map_err(|error| unavailable(error.to_string()))?;
    Ok(())
}
fn copy_dir(source: &Path, target: &Path) -> Result<(), CheckError> {
    std::fs::create_dir_all(target).map_err(|error| unavailable(error.to_string()))?;
    for entry in std::fs::read_dir(source).map_err(|error| unavailable(error.to_string()))? {
        let entry = entry.map_err(|error| unavailable(error.to_string()))?;
        let to = target.join(entry.file_name());
        if entry.path().is_dir() {
            copy_dir(&entry.path(), &to)?;
        } else {
            let metadata = entry
                .metadata()
                .map_err(|error| unavailable(error.to_string()))?;
            std::fs::copy(entry.path(), &to).map_err(|error| unavailable(error.to_string()))?;
            std::fs::set_permissions(&to, metadata.permissions())
                .map_err(|error| unavailable(error.to_string()))?;
        }
    }
    Ok(())
}
fn replace(path: &Path, from: &str, to: &str) -> Result<(), CheckError> {
    let source = read_path(path)?;
    require(
        source.contains(from),
        format!(
            "self-test could not construct mutation in {}",
            path.display()
        ),
    )?;
    std::fs::write(path, source.replacen(from, to, 1))
        .map_err(|error| unavailable(error.to_string()))
}
fn mutate_fixture(root: &Path, label: &str) -> Result<(), CheckError> {
    match label {
        "rfc-template-section" => replace(
            &root.join("rfcs/template.md"),
            "## Security considerations",
            "## Security notes",
        ),
        "adr-template-section" => {
            replace(&root.join("adrs/template.md"), "## Evidence", "## Material")
        }
        "adr-metadata" => {
            std::fs::copy(root.join("adrs/template.md"), root.join("adrs/1-test.md"))
                .map_err(|e| unavailable(e.to_string()))?;
            replace(
                &root.join("adrs/1-test.md"),
                "# ADR N: Title",
                "# ADR 1: Test",
            )?;
            replace(
                &root.join("adrs/1-test.md"),
                "- Authors: Name or identity\n",
                "",
            )?;
            append(
                &root.join("adrs/README.md"),
                "\n- [ADR 1: Test](1-test.md) — Proposed\n",
            )
        }
        "stale-index" => append(
            &root.join("rfcs/README.md"),
            "\n- [RFC 99: Stale](99-stale.md) — Draft\n",
        ),
        "status-mismatch" => mutate_index_status(&root.join("rfcs/README.md")),
        "heading-number" => replace(
            &root.join("rfcs/1-frame-indexed-process-model.md"),
            "# RFC 1: Frame-indexed",
            "# RFC 2: Frame-indexed",
        ),
        "allocation-reuse" => replace(
            &root.join("rfcs/numbers.tsv"),
            "Frame-indexed process model",
            "Unrelated replacement",
        ),
        "accepted-metadata" => replace(
            &root.join("rfcs/0-rfc-and-adr-process.md"),
            "- Discussion: https://github.com/kan-tools/day/pull/220",
            "- Discussion: Not opened",
        ),
        "historical-renumber" => {
            std::fs::rename(
                root.join("rfcs/1-frame-indexed-process-model.md"),
                root.join("rfcs/2-frame-indexed-process-model.md"),
            )
            .map_err(|e| unavailable(e.to_string()))?;
            replace(
                &root.join("rfcs/2-frame-indexed-process-model.md"),
                "# RFC 1:",
                "# RFC 2:",
            )?;
            replace(
                &root.join("rfcs/README.md"),
                "RFC 1: Frame-indexed process model](1-frame-indexed-process-model.md)",
                "RFC 2: Frame-indexed process model](2-frame-indexed-process-model.md)",
            )?;
            replace(
                &root.join("rfcs/numbers.tsv"),
                "1\t1-frame-indexed-process-model.md",
                "2\t2-frame-indexed-process-model.md",
            )
        }
        "forged-review" => {
            replace(
                &root.join("rfcs/0-rfc-and-adr-process.md"),
                "- Discussion: https://github.com/kan-tools/day/pull/220",
                "- Discussion: x",
            )?;
            replace(
                &root.join("rfcs/0-rfc-and-adr-process.md"),
                "- Review-period-ends: 2026-08-17T00:22:14Z",
                "- Review-period-ends: not-a-date",
            )?;
            replace(&root.join("rfcs/0-rfc-and-adr-process.md"),"- Review-override: unanimous:https://github.com/kan-tools/day/pull/220@de4becf5ae056bc422e510954d6e6f337ce66736","- Review-override: forged")
        }
        "short-review" => {
            replace(
                &root.join("rfcs/0-rfc-and-adr-process.md"),
                "- Discussion: https://github.com/kan-tools/day/pull/220",
                "- Discussion: https://github.com/kan-tools/day/pull/7",
            )?;
            replace(
                &root.join("rfcs/0-rfc-and-adr-process.md"),
                "- Review-started-at: 2026-08-16T21:28:40Z",
                "- Review-started-at: 2026-08-01T00:00:00Z",
            )?;
            replace(
                &root.join("rfcs/0-rfc-and-adr-process.md"),
                "- Review-period-ends: 2026-08-17T00:22:14Z",
                "- Review-period-ends: 2026-08-01T01:00:00Z",
            )?;
            replace_override_none(&root.join("rfcs/0-rfc-and-adr-process.md"))
        }
        "profile-relationship" => replace(
            &root.join("rfcs/1-frame-indexed-process-model.md"),
            "- Profile-relationship: approximation",
            "- Profile-relationship: full-implementation",
        ),
        "coherence-vector" => replace(
            &root.join("rfcs/vectors/1-process-model.json"),
            "\"outcome\": \"not-certified\",\n          \"limitations\": [\"shared coordinate candidate does not match\"]",
            "\"outcome\": \"certified\",\n          \"limitations\": [\"shared coordinate candidate does not match\"]",
        ),
        "recursive-publication" => replace(
            &root.join("rfcs/0-rfc-and-adr-process.md"),
            "- Authors:",
            "- Kan-claim: bafyrecursive\n- Authors:",
        ),
        "denotational-source" => {
            std::fs::remove_file(root.join("rfcs/1/denotational-semantics.md"))
                .map_err(|e| unavailable(e.to_string()))
        }
        "denotational-math" => replace(
            &root.join("rfcs/1/denotational-semantics.html"),
            "mathjax@3/es5/tex-svg.js",
            "math-disabled.js",
        ),
        "denotational-freshness" => append(
            &root.join("rfcs/1/denotational-semantics.md"),
            "\nA stale-render mutation.\n",
        ),
        _ => Err(finding("unknown RFC self-test mutation")),
    }
}
fn replace_override_none(path: &Path) -> Result<(), CheckError> {
    let source = read_path(path)?;
    let line = source
        .lines()
        .find(|line| line.starts_with("- Review-override: "))
        .ok_or_else(|| finding("review override absent"))?
        .to_owned();
    replace(path, &line, "- Review-override: None")
}
fn mutate_index_status(path: &Path) -> Result<(), CheckError> {
    let source = read_path(path)?;
    let line = source
        .lines()
        .find(|line| {
            line.starts_with("- [RFC 1:") && line.contains("](1-frame-indexed-process-model.md) — ")
        })
        .ok_or_else(|| {
            finding(format!(
                "self-test could not find RFC 1 index row in {}",
                path.display()
            ))
        })?;
    let (prefix, current) = line
        .rsplit_once(" — ")
        .ok_or_else(|| finding("RFC 1 index row has no status separator"))?;
    let mismatched = if current == "Accepted" {
        "Draft"
    } else {
        "Accepted"
    };
    replace(path, line, &format!("{prefix} — {mismatched}"))
}
fn append(path: &Path, text: &str) -> Result<(), CheckError> {
    use std::io::Write;
    let mut file = std::fs::OpenOptions::new()
        .append(true)
        .open(path)
        .map_err(|e| unavailable(e.to_string()))?;
    file.write_all(text.as_bytes())
        .map_err(|e| unavailable(e.to_string()))
}
fn expected_error(label: &str) -> &str {
    match label {
        "rfc-template-section" => "lacks section: Security considerations",
        "adr-template-section" => "lacks section: Evidence",
        "adr-metadata" => "lacks metadata: Authors",
        "stale-index" => "contains stale or missing RFC rows",
        "status-mismatch" => "index row is missing or disagrees with title/status",
        "heading-number" => "heading number differs from filename",
        "allocation-reuse" => "disagrees with rfcs/numbers.tsv allocation",
        "accepted-metadata" | "forged-review" => "Discussion is not a day pull-request address",
        "historical-renumber" => "historical RFC allocation changed",
        "short-review" => "fewer than 72 review hours elapsed",
        "profile-relationship" => "unrecognized Profile-relationship",
        "coherence-vector" => "certificate outcome is not derived",
        "recursive-publication" => "normative RFC bytes contain a claim-CID backlink",
        "denotational-source" => "missing RFC 1 denotational companion source",
        "denotational-math" => "denotational HTML lacks MathJax rendering",
        "denotational-freshness" => "denotational HTML is not the current rendering",
        _ => "",
    }
}
