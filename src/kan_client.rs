//! day's only integration with kan is kan's public CLI (ADR-18: the
//! companion tool consumes kan via its CLI/MCP, it does not link kan as a
//! library or touch its data model). Every read here is a subprocess call to
//! the `kan` binary, parsed from its stdout — the same shape kan's own
//! `GitAncestry` provider uses for git.
//!
//! Nothing in this module appends, retracts, or rejects a claim. day records
//! claims by *instructing* an agent to call kan's write verbs (the commands
//! do this); the binary itself only ever runs kan's read verbs, so there is
//! no path by which day can alter or destroy a subject.
//!
//! One honest caveat: kan initializes its own workspace (`.kan/`) on first
//! use, so running a kan read verb in a repo kan has never seen creates an
//! empty log there. That is kan's behavior, not day's, and it touches no
//! claims — but it does mean "day never causes a write to disk" would be
//! too strong a claim to make.

use std::path::PathBuf;
use std::process::Command;

/// Overrides the `kan` binary day shells out to. Exists so tests can point
/// at a stub emitting canned `kan` output instead of requiring a real kan
/// install in CI.
pub const KAN_BIN_ENV: &str = "DAY_KAN_BIN";

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("kan is not reachable (tried to run `{bin}`): {source}\nInstall it with `cargo install kan`, or set {KAN_BIN_ENV} to its path.")]
    NotReachable {
        bin: String,
        #[source]
        source: std::io::Error,
    },
    /// The same failure, when `DAY_KAN_BIN` chose the path — which needs a
    /// **different remedy**, and saying so is the whole point of the split.
    ///
    /// `NotReachable`'s text ends "or set `DAY_KAN_BIN` to its path", which is
    /// useless advice when that variable is already set and is what selected the
    /// missing file. It also opens with "Install it with `cargo install kan`",
    /// pointing at a missing kan install that is not the problem.
    ///
    /// day#178 is what that cost. `scripts/behaviour-diff.py` writes its own kan
    /// **stub** into a work directory and points `DAY_KAN_BIN` at it; when the
    /// stub went missing, CI reported "kan is not reachable … Install it with
    /// `cargo install kan`" — in a job that had already asserted `kan --version`
    /// runs. The message sent every reader to kan's installation, and the file
    /// was one the harness itself had written seconds earlier.
    ///
    /// **The words `cargo install kan` are deliberately absent from this text**,
    /// and that is not stylistic. The first draft read "`cargo install kan` will
    /// not fix it" — which contains the exact substring
    /// `ac2_doctor_exits_non_zero_...` greps for, so that test went on passing
    /// against a message stating the command would NOT help. A phrase-presence
    /// assertion cannot tell a recommendation from its negation, which
    /// `CLAUDE.md` records as a defect class ("a classifier keyed on the absence
    /// of a phrase"). Keeping the string out of this variant is what lets the two
    /// cases be told apart by a grep at all.
    #[error(
        "kan is not reachable (tried to run `{bin}`): {source}\n\n\
         {KAN_BIN_ENV} is set and selected that path, so this is NOT a missing kan \
         install and installing one will not help. Either the path is wrong, the \
         file is not executable, or something removed it. Unset {KAN_BIN_ENV} to \
         fall back to the `kan` on PATH."
    )]
    OverrideNotReachable {
        bin: String,
        #[source]
        source: std::io::Error,
    },
    #[error(
        "could not read `{args}` output from kan: {detail}\n\nThis usually means kan's \
         --json shape changed. day pins to a shape version rather than parsing rendered \
         output, so this is an error instead of a silently empty result."
    )]
    Shape { args: String, detail: String },
    #[error(
        "this kan cannot serve `kan show --all --json`, which day reads the log with.\n\n\
         day requires kan >= {oldest}. Upgrade kan, or point {KAN_BIN_ENV} at a newer one.\n\n\
         day does not fall back to reading one subject at a time — that path is gone, and \
         reporting an empty log would be worse than saying this."
    )]
    TooOldForBulkRead { oldest: String },
    #[error(
        "kan lists `{subject}` but did not return it in the bulk read, so day \
         cannot tell whether it is unreadable or was dropped.\n\nThis is \
         reported rather than treated as absent: concluding `{subject}` has \
         nothing would be an absence day never verified."
    )]
    Unaccounted { subject: String },
    #[error(
        "`{subject}` has {count} claim(s) this view's trust base does not admit, \
         so day is reading a partial history of it.\n\nday resolves \
         newest-wins, so a withheld newer claim would silently promote a \
         superseded one to current — reporting on it would assert a currency \
         day cannot establish.\n\nWiden the view — `--trust me`, or \
         `--trust <did>` for the author who recorded it."
    )]
    PartiallyWithheld { subject: String, count: u64 },
    #[error(
        "`{subject}` is not in this view, and {count} claim(s) elsewhere in the \
         log are withheld from it — so day cannot tell whether it is undeclared \
         or simply not admitted by this trust base.\n\nday will not offer to \
         declare it: appending under an unadmitted key adds a second, competing \
         claim and forks the vocabulary silently.\n\nWiden the view to check \
         whether it is already declared — `--trust me`, or `--trust <did>` for \
         the author who would have recorded it."
    )]
    AbsentUnderNarrowedTrust { subject: String, count: u64 },
    #[error("`{bin} {args}` failed ({status}){stderr}")]
    Failed {
        bin: String,
        args: String,
        status: String,
        stderr: String,
    },
}

/// The `--json` shape version day understands.
///
/// kan documents the shape as **versioned and additive-only**, and the
/// rendered form as free to change. day reads the structured form for
/// exactly that reason: it parsed the rendered form once, kan changed it,
/// and day read a full log as empty while reporting success.
///
/// Checked rather than assumed. A shape day does not know is an error with a
/// message, never a silently empty read — that failure mode is the one this
/// migration exists to end.
const SHAPE_VERSION: u32 = 1;

/// One live claim, from `kan show --json`.
///
/// Unknown fields are ignored by construction, which is what makes kan's
/// additive-only promise usable: a field added upstream cannot break day.
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
pub struct Claim {
    pub cid: String,
    pub kind: String,
    /// The claim's narrative text, when its body carries one (`Status`
    /// claims and relations do not).
    #[serde(default)]
    pub text: Option<String>,
    /// The declared subject title, present only on `Subject` claims. A
    /// subject's name is an rkey, not a label; this is what it's called.
    #[serde(default)]
    pub title: Option<String>,
    /// The signing DID. Exposed by `--json` and not by the rendered form;
    /// day#25's locally-signed injection scoping has no other way to tell
    /// whose claim it is reading.
    #[serde(default)]
    pub author: Option<String>,
    /// When kan recorded this claim, as **microseconds since the epoch** — the
    /// integer `kan show --json` actually emits (verified against the real
    /// binary via `tests/kan_conformance.rs`, which caught this typed as a
    /// string). The only field that orders claims *across* subjects: `show`
    /// returns each subject oldest-first, but "the last recorded assessment"
    /// among several `atom/*` subjects needs a global order, and larger is
    /// newer. Additive and optional, so a kan that omits it degrades to no
    /// ordering rather than failing.
    ///
    /// fallback: kan-omits-recorded-at
    #[serde(default)]
    pub recorded_at: Option<i64>,
}

/// `kan show --all --json` (kan#123, ADR-71).
///
/// Each entry is a **full** `ShowJson` — repeated `trust` field and all —
/// which ADR-71 chose deliberately so day could reuse the parser it already
/// has for a single subject rather than write a second one. Taking that deal
/// is the point: [`Claim`] below is unchanged.
#[derive(Debug, serde::Deserialize)]
struct ShowAllEnvelope {
    v: u32,
    #[serde(default)]
    subjects: Vec<ShowAllEntry>,
    /// Live claims withheld from this view's trust base, across the whole log.
    ///
    /// **This is the only signal that exists for a subject kan withheld
    /// entirely**, measured against kan 0.11.0-beta.1 rather than assumed:
    ///
    /// ```text
    /// $ KAN_IDENTITY_FILE=…/roles.d/agent-s1 kan show --all --json --trust me
    /// {"v":1,"trust":{"base":"Solo",…},"excluded_by_trust":1,"subjects":[]}
    /// ```
    ///
    /// The subject is **omitted** — from `show --all --json` and from
    /// `status --json` both — so there is no entry to carry a count and nothing
    /// names which subject went missing. A first attempt at day#120 keyed on an
    /// entry with `claims: []` and a non-zero count, which kan never emits;
    /// `tests/kan_conformance.rs` now pins both shapes against the real binary,
    /// because a stub validates day against day's own idea of kan's CLI.
    #[serde(default)]
    excluded_by_trust: u64,
}

#[derive(Debug, serde::Deserialize)]
struct ShowAllEntry {
    subject: String,
    #[serde(default)]
    claims: Vec<Claim>,
    /// Claims withheld from this view **on this subject** (day#120).
    ///
    /// Non-zero here means a **partial** view: kan returned the entry, some
    /// claims are visible, and at least one is not. Measured, not assumed:
    ///
    /// ```text
    /// subject: shared | visible claims: 1 | entry excluded_by_trust: 1
    /// ```
    ///
    /// That is the shape day has to care about most, because **day resolves
    /// newest-wins**: `newest_fenced` walks claims in reverse and takes the
    /// first that parses. A withheld *newest* claim therefore silently promotes
    /// a superseded declaration to current, and day validates against it at
    /// exit 0. A cold review demonstrated exactly that — a schema whose newer
    /// revision added a required section passed a document missing it.
    ///
    /// fallback: kan-omits-excluded-by-trust
    #[serde(default)]
    excluded_by_trust: u64,
}

#[derive(Debug, serde::Deserialize)]
struct SubjectsEnvelope {
    v: u32,
    #[serde(default)]
    subjects: Vec<SubjectEntry>,
    /// `status --json` carries the log-wide withheld count too, and day needs
    /// it from HERE as well as from the bulk read.
    ///
    /// `subjects()` does not go through `ensure_log`, so a reader that only
    /// enumerates — `render_teloi`, `atoms::load` — would otherwise ask for the
    /// count before anything had populated it and be told zero. That is how the
    /// first attempt at this fix failed to fire: the guard was right, the value
    /// it consulted was not yet set, and the behaviour-diff corpus caught it
    /// because the fixture's output did not change.
    #[serde(default)]
    excluded_by_trust: u64,
}

#[derive(Debug, serde::Deserialize)]
struct SubjectEntry {
    subject: String,
}

pub struct KanClient {
    bin: String,
    cwd: PathBuf,
    /// The whole log, read once per invocation and served to every `show`.
    ///
    /// **Not a store, on the same terms as [`crate::probe::ClaimLog`]:** it
    /// lives for one invocation and dies with it, so `telos/no-store-of-its-own`
    /// is untouched. What it removes is the duplication that survived the bulk
    /// read — eight independent readers each ran their own `subjects()` +
    /// `show()` loop, so `session-start` read `atom/*` three times over and paid
    /// 48 process startups beyond the one bulk call it had already made.
    ///
    /// `RefCell` rather than `OnceCell` because it must be **invalidated on
    /// write**: `record.rs` appends and then reads back, and a memo that
    /// outlived an append would hand a caller the log as it was before its own
    /// claim. `KanClient` is constructed per MCP call and never shared across
    /// threads, so interior mutability without a lock is sound.
    log: std::cell::RefCell<Option<Vec<(String, Claim)>>>,
    /// `kan status --json`, memoized for the same invocation. Kept as its own
    /// read rather than derived from `log`: kan lists a subject whether or not
    /// it has live claims, so deriving the set from claims would silently drop
    /// any subject that has none.
    subject_memo: std::cell::RefCell<Option<Vec<String>>>,
    /// Whether reachability has been established. `--help` costs a process like
    /// everything else, and the answer cannot change mid-invocation.
    probed: std::cell::Cell<bool>,
    /// Subjects kan listed that the bulk read did not return. Computed once,
    /// at the read, so every consumer of the log inherits it.
    unaccounted: std::cell::RefCell<Vec<String>>,
    /// Subjects that came back with no claims *because* claims were withheld
    /// from this view's trust base, and how many (day#120). Computed at the
    /// read, beside `unaccounted`, for the same reason: a guarantee wired at a
    /// call site is the defect this repo keeps rediscovering, and there are
    /// three loaders that would each have to remember to ask.
    trust_withheld: std::cell::RefCell<Vec<(String, u64)>>,
    /// Claims withheld from this view across the WHOLE log (day#120). The only
    /// evidence that a subject missing from this read might be withheld rather
    /// than undeclared, because kan omits such a subject entirely.
    trust_withheld_total: std::cell::Cell<u64>,
    /// Subjects the bulk read returned an ENTRY for, whether or not that entry
    /// carried visible claims. What `unaccounted` is computed against.
    returned_subjects: std::cell::RefCell<Vec<String>>,
    /// Whether `bin` came from `DAY_KAN_BIN` rather than the default. Decides
    /// which unreachable-message a reader gets, because the two have different
    /// remedies and the wrong one costs real time (day#178).
    bin_from_env: bool,
}

impl KanClient {
    pub fn new(cwd: impl Into<PathBuf>) -> Self {
        // Provenance is recorded, not re-derived at the point of failure: reading
        // the environment again when the error is built would report whatever the
        // variable says *then*, which in a test that sets and clears it is a
        // different fact from the one that chose this binary.
        let (bin, bin_from_env) = match std::env::var(KAN_BIN_ENV) {
            Ok(v) => (v, true),
            Err(_) => ("kan".to_string(), false),
        };
        Self {
            bin,
            bin_from_env,
            cwd: cwd.into(),
            log: std::cell::RefCell::new(None),
            subject_memo: std::cell::RefCell::new(None),
            probed: std::cell::Cell::new(false),
            unaccounted: std::cell::RefCell::new(Vec::new()),
            trust_withheld: std::cell::RefCell::new(Vec::new()),
            trust_withheld_total: std::cell::Cell::new(0),
            returned_subjects: std::cell::RefCell::new(Vec::new()),
        }
    }

    pub fn with_bin(cwd: impl Into<PathBuf>, bin: impl Into<String>) -> Self {
        Self {
            bin: bin.into(),
            // Chosen by the caller, not by the environment — so the remedy the
            // override message names would be wrong here.
            bin_from_env: false,
            cwd: cwd.into(),
            log: std::cell::RefCell::new(None),
            subject_memo: std::cell::RefCell::new(None),
            probed: std::cell::Cell::new(false),
            unaccounted: std::cell::RefCell::new(Vec::new()),
            trust_withheld: std::cell::RefCell::new(Vec::new()),
            trust_withheld_total: std::cell::Cell::new(0),
            returned_subjects: std::cell::RefCell::new(Vec::new()),
        }
    }

    fn run(&self, args: &[&str]) -> Result<String, Error> {
        let output = Command::new(&self.bin)
            .args(args)
            .current_dir(&self.cwd)
            .output()
            .map_err(|source| {
                let bin = self.bin.clone();
                if self.bin_from_env {
                    Error::OverrideNotReachable { bin, source }
                } else {
                    Error::NotReachable { bin, source }
                }
            })?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
            return Err(Error::Failed {
                bin: self.bin.clone(),
                args: args.join(" "),
                status: output.status.to_string(),
                stderr: if stderr.is_empty() {
                    stderr
                } else {
                    format!(": {stderr}")
                },
            });
        }
        Ok(String::from_utf8_lossy(&output.stdout).into_owned())
    }

    /// Cheapest possible reachability check — `kan --help` touches no
    /// workspace state, so it distinguishes "kan isn't installed" from
    /// "kan is installed but this isn't a kan repo".
    pub fn probe(&self) -> Result<(), Error> {
        if self.probed.get() {
            return Ok(());
        }
        self.run(&["--help"])?;
        self.probed.set(true);
        Ok(())
    }

    /// Loads the whole log once; every `show` is served from it.
    fn ensure_log(&self) -> Result<(), Error> {
        if self.log.borrow().is_some() {
            return Ok(());
        }
        // Cross-checked HERE, not by one caller. The first version of this was
        // computed in `status::compute`, so the hook channels were protected
        // and `assess telos` was not — it reported `[MISSING]` for evidence it
        // had never received. A guarantee that holds only where someone
        // remembered to ask for it is the defect this repo keeps finding, so
        // the check belongs at the read.
        //
        // Costs at most one `status --json`, memoized, and that is the price of
        // the invariant: day went from 167 invocations to 6, and spending one
        // of them on not lying is the easiest trade in the change.
        //
        // **ORDER IS LOAD-BEARING: the subject list is taken FIRST.** kan's log
        // is shared, and another agent may append while day is reading. Taking
        // the list *after* the bulk read would show a subject created in
        // between as listed-but-not-returned — day would call a healthy kan
        // incomplete and refuse to answer. Taken first, that subject appears in
        // the bulk read and not in the list, which is a surplus and harmless:
        // day simply holds a claim it did not expect. The check must only ever
        // fire on evidence that went MISSING, never on evidence that arrived.
        let listed = self.subjects()?;
        let all = self.read_all()?;
        // From the ENTRIES `read_all` recorded, not from the claims — see the
        // note there. A subject kan returned as an empty entry was returned.
        let returned: std::collections::BTreeSet<String> =
            self.returned_subjects.borrow().iter().cloned().collect();
        let missing: Vec<String> = listed
            .iter()
            .filter(|s| !returned.contains(s.as_str()))
            .cloned()
            .collect();
        *self.unaccounted.borrow_mut() = missing;
        *self.log.borrow_mut() = Some(all);
        Ok(())
    }

    /// Drops the memo. Called after every write, because a caller that appends
    /// and then reads must see its own claim.
    fn invalidate(&self) {
        *self.log.borrow_mut() = None;
        *self.subject_memo.borrow_mut() = None;
        self.unaccounted.borrow_mut().clear();
        // Cleared with the rest, and not as an afterthought: this set is
        // recomputed by `read_all`, so leaving it behind would make a stale
        // "day cannot read this" outlive the read it was derived from — and it
        // is now a hard error, so the stale version refuses rather than
        // degrades.
        self.trust_withheld.borrow_mut().clear();
        self.trust_withheld_total.set(0);
        self.returned_subjects.borrow_mut().clear();
    }

    /// kan's version, via `kan --version`, or `None` when it cannot be
    /// determined.
    ///
    /// Returns `None` rather than an error for the same reason [`identity`]
    /// does: a caller deciding whether to warn needs a value it can branch on,
    /// and "I could not tell" must read as *unknown*, never as *incompatible*.
    /// A day that treats an unparseable version string as a mismatch is a day
    /// that cries wolf against every kan whose output format shifts.
    ///
    /// [`identity`]: Self::identity
    pub fn version(&self) -> Option<crate::compat::Version> {
        crate::compat::Version::parse(self.run(&["--version"]).ok()?.trim())
    }

    /// Every subject's live claims, from **one** invocation
    /// (`kan show --all --json`, kan#123 / ADR-71).
    ///
    /// This exists because the cost of reading day's log is entirely fixed
    /// per-process startup — an empty log costs the same as a full one, and
    /// `kan identity did`, which reads no log at all, costs the same again. So
    /// no optimisation inside a read helps and only the invocation count does:
    /// day paid ~48ms × 98 calls at session start to answer questions that one
    /// call answers.
    ///
    /// **Requires kan >= 0.9.1.** An older kan rejects `--all` and this returns
    /// the error rather than an empty log — a partial read reported as a whole
    /// one is the failure `src/probe.rs` and `telos/honest-reads` both forbid.
    /// `src/compat.rs` states the floor and `day doctor` tells the user to
    /// upgrade.
    pub fn show_all(&self) -> Result<Vec<(String, Claim)>, Error> {
        self.ensure_log()?;
        Ok(self
            .log
            .borrow()
            .as_ref()
            .expect("ensure_log populated it")
            .clone())
    }

    /// A hash of the whole log, for answering "has anything been recorded since
    /// last time?" without recomputing a position (day#111).
    ///
    /// Served from the same memo as every other read, so on a turn that goes on
    /// to recompute, this costs **no extra kan invocation** — the fingerprint
    /// read and the compute share one. It is only a real read on turns where
    /// nothing changed, which is the case that has to stay cheap.
    ///
    /// Hashes the `(subject, cid)` pairs rather than the claim bodies: a CID is
    /// content-addressed, so it moves if and only if something was appended or
    /// retracted, and skipping the text keeps this proportional to the number of
    /// claims instead of the size of the log.
    ///
    /// **Not** built on `kan status`. That output does not change when a claim
    /// is appended to an existing subject whose state is unchanged — measured —
    /// so a fingerprint over it would miss exactly the appends that move a
    /// position: an observation, a result, a verdict on a subject already open.
    pub fn log_fingerprint(&self) -> Result<String, Error> {
        let claims = self.show_all()?;
        // FNV rather than `DefaultHasher`, which is documented-unstable across
        // Rust releases: this value is rendered into `.day/` and compared by a
        // later run, so a toolchain upgrade would make every log look changed.
        // `src/record.rs` holds the algorithm and the full rationale. NUL
        // separators, because neither a subject nor a CID can contain one, so
        // two different pair lists cannot feed identical bytes.
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&(claims.len() as u64).to_le_bytes());
        for (subject, claim) in &claims {
            bytes.extend_from_slice(subject.as_bytes());
            bytes.push(0);
            bytes.extend_from_slice(claim.cid.as_bytes());
            bytes.push(0);
        }
        Ok(format!("{:016x}", crate::record::fnv1a(&bytes)))
    }

    /// The actual `kan show --all --json` process. Everything else is served
    /// from the memo.
    fn read_all(&self) -> Result<Vec<(String, Claim)>, Error> {
        let args = ["show", "--all", "--json"];
        // A kan predating the flag fails with clap's "unexpected argument",
        // which tells a user nothing about what to do. day knows the floor, so
        // it says it (REQ-2).
        //
        // The cause is decided by **asking kan its version**, not by matching
        // its error prose. The first version keyed on `stderr.contains("--all")`
        // — a classifier reading another program's wording, which `CLAUDE.md`
        // warns against, and which would have silently stopped classifying the
        // day clap rephrased its message. Costs one extra process only on the
        // failure path, where day is about to abort anyway.
        let out = self.run(&args).map_err(|e| {
            let too_old = self.version().is_some_and(|v| {
                crate::compat::classify(Some(&v)) == crate::compat::Compat::TooOld
            });
            if too_old {
                Error::TooOldForBulkRead {
                    oldest: crate::compat::OLDEST_SUPPORTED.to_string(),
                }
            } else {
                e
            }
        })?;
        let envelope: ShowAllEnvelope = parse(&out, &args)?;
        check_shape(envelope.v, &args)?;

        // day#120, keyed on the two shapes kan actually emits — measured
        // against 0.11.0-beta.1 and pinned in `tests/kan_conformance.rs`, after
        // a first attempt keyed on a third shape that does not occur.
        //
        // PARTIAL: the entry is present, some claims visible, a count on the
        // entry. This is the dangerous one, because newest-wins means a
        // withheld newest claim promotes a superseded one.
        *self.trust_withheld.borrow_mut() = envelope
            .subjects
            .iter()
            .filter(|e| e.excluded_by_trust > 0)
            .map(|e| (e.subject.clone(), e.excluded_by_trust))
            .collect();

        // ABSENT: kan omits a fully-withheld subject from `show --all --json`
        // AND from `status --json`, so nothing names it and the accounting
        // guard cannot see it either. The envelope total is the only evidence
        // that any absence in this read is unreliable.
        self.trust_withheld_total.set(envelope.excluded_by_trust);

        // **The subjects kan RETURNED, taken from the entries rather than from
        // the claims.** The accounting guard's own words are "kan lists
        // `{subject}` but did not return it in the bulk read" — and an entry
        // carrying zero visible claims *was* returned. Deriving the set from
        // flattened claims made every claimless subject look dropped, so a
        // subject whose claims kan withheld by trust reported as "day cannot
        // tell whether it is unreadable or was dropped" when day could now tell
        // exactly, and would say so one branch further down.
        //
        // This narrows what `Unaccounted` fires on: an entry that is present
        // and empty is no longer it. That is the invariant stated correctly
        // rather than a weakening — the case it stops covering is precisely the
        // case the new error covers better.
        *self.returned_subjects.borrow_mut() = envelope
            .subjects
            .iter()
            .map(|e| e.subject.clone())
            .collect();

        Ok(envelope
            .subjects
            .into_iter()
            .flat_map(|entry| {
                let subject = entry.subject;
                entry.claims.into_iter().map(move |c| (subject.clone(), c))
            })
            .collect())
    }

    /// Subjects kan lists but the bulk read did not return.
    ///
    /// **This recovers what the whole-log read cost day.** Reading one subject
    /// at a time, a subject kan could not serve produced an error naming it.
    /// One bulk read cannot: a subject missing from the payload is
    /// indistinguishable from a subject that simply has nothing, and day would
    /// report an absence it never verified — the exact failure
    /// `telos/honest-reads` forbids.
    ///
    /// Cross-checking the two sets closes it, and closes it *wider* than the
    /// per-subject loop ever did: that could only catch a failure kan
    /// **reported**, while this also catches one it silently omitted.
    ///
    /// A subject can never legitimately be missing. A subject exists by virtue
    /// of having claims, and retracting the last one appends a `Retraction`,
    /// which is itself a claim — verified against a real kan, where a subject
    /// whose only claim was retracted still comes back with one.
    ///
    /// **Costs no extra invocation.** It compares two reads day has already
    /// made and returns empty unless both are in hand, so it can never turn
    /// into a third call.
    pub fn unaccounted_subjects(&self) -> Vec<String> {
        self.unaccounted.borrow().clone()
    }

    /// This workspace's identity, via `kan identity did`.
    ///
    /// `did` is the public identifier and is explicitly safe to share.
    /// **Never `kan identity phrase`**, which prints the recovery phrase for
    /// the signing key.
    ///
    /// Returns `None` rather than an error on any failure, deliberately.
    /// kan's identity access can block on a macOS keychain prompt that never
    /// arrives non-interactively — kan's own `src/sign.rs` documents this,
    /// and it silently emptied day's reads once already. A caller deciding
    /// whether to trust a claim needs a value it can branch on, not an error
    /// that aborts a hook, because the right response to "identity unknown"
    /// is to project nothing and say so.
    pub fn identity(&self) -> Option<String> {
        let did = self.run(&["identity", "did"]).ok()?.trim().to_string();
        (!did.is_empty()).then_some(did)
    }

    /// Every subject in the log, via `kan status --json`.
    pub fn subjects(&self) -> Result<Vec<String>, Error> {
        if let Some(memo) = self.subject_memo.borrow().as_ref() {
            return Ok(memo.clone());
        }
        let names = self.subject_names(&["status", "--json"])?;
        *self.subject_memo.borrow_mut() = Some(names.clone());
        Ok(names)
    }

    /// Subjects that are not yet resolved, via `kan issues --json`.
    pub fn issues(&self) -> Result<Vec<String>, Error> {
        self.subject_names(&["issues", "--json"])
    }

    fn subject_names(&self, args: &[&str]) -> Result<Vec<String>, Error> {
        let out = self.run(args)?;
        let envelope: SubjectsEnvelope = parse(&out, args)?;
        check_shape(envelope.v, args)?;
        // **Recorded here as well as from the bulk read**, because the readers
        // that need it most never take the other path. `subjects()` does not go
        // through `ensure_log`, so `render_teloi` and `atoms::load` — which
        // enumerate rather than look up — asked for the count before anything
        // set it and were told zero. The guard was correct and inert; the
        // behaviour-diff corpus caught it, because the fixture's output did not
        // change when it was supposed to.
        //
        // `max`, not assignment: the two reads report the same log-wide number,
        // and taking the larger makes the result independent of which happened
        // first rather than dependent on an ordering nobody would think to
        // preserve.
        let seen = self
            .trust_withheld_total
            .get()
            .max(envelope.excluded_by_trust);
        self.trust_withheld_total.set(seen);
        Ok(envelope.subjects.into_iter().map(|s| s.subject).collect())
    }

    /// A subject's live claims, via `kan show <subject> --json`.
    /// One subject's live claims, served from the whole-log read.
    ///
    /// This used to be its own `kan show <subject> --json` process. It is not,
    /// since day#71: a kan read costs fixed process startup, so N targeted reads
    /// are strictly worse than one bulk read plus a filter, and day made 39 of
    /// them per session start for subjects the bulk read already held.
    pub fn show(&self, subject: &str) -> Result<Vec<Claim>, Error> {
        self.ensure_log()?;
        // Restores, exactly, what reading one subject at a time gave for free:
        // a subject day could not obtain is an error naming it, never an empty
        // result. Without this, `assess docs` reads a dropped subject as
        // "nobody wrote it down".
        if self.unaccounted.borrow().iter().any(|s| s == subject) {
            return Err(Error::Unaccounted {
                subject: subject.to_string(),
            });
        }
        // day#120, and the same argument as the guard above: a result day did
        // not establish is a false certification. Three loaders turn what they
        // get here into "no schema is declared" plus a runnable starter, so the
        // distinction lives here rather than in each of them — fixing the one
        // in the traceback and leaving the other two is the call-site shape
        // day#101 is about.
        //
        // Two cases, because kan emits two shapes and the first version of this
        // handled a third that does not exist.
        let claims: Vec<Claim> = {
            let log = self.log.borrow();
            let all = log.as_ref().expect("ensure_log populated it");
            all.iter()
                .filter(|(s, _)| s == subject)
                .map(|(_, c)| c.clone())
                .collect()
        };

        // PARTIAL: kan returned this subject and withheld some of its claims.
        // Refused even though claims are visible, because day takes the NEWEST
        // parseable claim and the withheld one may be newer — answering from
        // what is left promotes a superseded declaration to current.
        if let Some((_, count)) = self
            .trust_withheld
            .borrow()
            .iter()
            .find(|(s, _)| s == subject)
        {
            return Err(Error::PartiallyWithheld {
                subject: subject.to_string(),
                count: *count,
            });
        }

        // **The absent case is NOT decided here**, and a first version deciding
        // it here was a cold review's MAJOR-4.
        //
        // kan omits a fully-withheld subject from both reads, so day has no
        // per-subject evidence — only the envelope total, which is LOG-WIDE.
        // Erroring on it per subject meant one withheld claim anywhere made
        // every never-declared subject a hard error: `assess docs` exited 2
        // over `schema/docs` because two claims on an unrelated subject were
        // withheld, and the printed remedy ("re-run where the count is zero")
        // is unreachable, since a collaborator's claim in a committed
        // `.claims/` is permanent.
        //
        // Worse, it was the wrong way round. day refused hardest where its
        // evidence was weakest (a log-wide count) and stayed silent where it
        // was strongest (an enumerated read that knows a subject vanished).
        //
        // A log-wide fact is now reported log-wide — see
        // [`Self::claims_withheld_from_view`], which the enumerating renders
        // surface — and the per-subject error survives only in
        // `atoms::newest_fenced`, where a loader is about to conclude "nothing
        // is declared here" and print a runnable starter. That conclusion is
        // the day#120 harm; an ordinary empty read is not.
        Ok(claims)
    }

    /// Claims withheld from this view across the whole log (day#120).
    ///
    /// **Log-wide, and it must be reported that way.** kan gives no per-subject
    /// evidence for a subject it withheld entirely, so any absence in this read
    /// is unreliable and day cannot say which one. Every surface that
    /// ENUMERATES subjects — rather than asking for one by name — has to carry
    /// this, because those readers never call [`Self::show`] for a subject that
    /// is not there and so can never be told by it.
    pub fn claims_withheld_from_view(&self) -> u64 {
        self.trust_withheld_total.get()
    }

    /// Appends a narrative claim through kan's own write verb and returns
    /// the CID kan prints.
    ///
    /// This is the v0.2 invariant, stated precisely (`docs/ROADMAP.md`): day
    /// writes, but only ever by invoking kan's public verbs. kan signs,
    /// content-addresses, and owns the log format; day never touches storage
    /// and still has no destroy path, because kan exposes none to reach.
    ///
    /// Chaining is the point. day assembles `--cites` from CIDs it captured
    /// itself, which makes the "pass a file path to `--cites`" class of error
    /// unreachable rather than merely documented against — that bug existed
    /// in the prose instructions this replaces.
    pub fn append(&self, write: Write<'_>) -> Result<String, Error> {
        let mut args: Vec<&str> = vec![write.verb, write.text];
        args.push("--subject");
        args.push(write.subject);
        for cid in write.cites {
            args.push("--cites");
            args.push(cid);
        }
        if let Some((title, kind)) = write.declaration {
            args.extend_from_slice(&["--title", title, "--kind", kind]);
        }
        let cid = self.run(&args)?.trim().to_string();
        // A caller that appends and then reads must see its own claim.
        self.invalidate();
        Ok(cid)
    }

    /// Asserts a domain-semantic edge between two subjects, via
    /// `kan relate <A> <KIND> <B>`.
    ///
    /// Deliberately **not** routed through [`Self::append`]. That method
    /// builds `<verb> <text> --subject <s>`, and `kan relate` takes its two
    /// subjects positionally with no text at all — the same argument-shape
    /// asymmetry that put a command which does not run into
    /// `docs/CONVENTIONS.md` for several releases (day#27, kan#78). A verb
    /// with a different shape gets its own method;
    /// `tests/kan_conformance.rs` enforces that.
    ///
    /// A relation carries no narrative body, so whatever *reason* the edge
    /// has must live in a claim the edge cites. Callers pass that CID here.
    pub fn relate(&self, a: &str, kind: &str, b: &str, cites: &[String]) -> Result<String, Error> {
        let mut args: Vec<&str> = vec!["relate", a, kind, b];
        for cid in cites {
            args.push("--cites");
            args.push(cid);
        }
        let cid = self.run(&args)?.trim().to_string();
        // A caller that appends and then reads must see its own claim.
        self.invalidate();
        Ok(cid)
    }
}

/// One append, as arguments rather than a long parameter list — the write
/// verbs differ only in which kan verb they invoke.
pub struct Write<'a> {
    pub verb: &'a str,
    pub text: &'a str,
    pub subject: &'a str,
    pub cites: &'a [String],
    /// `--title` and `--kind`, together or not at all. One field rather than
    /// two `Option`s because the half-set state used to be representable and
    /// was silently dropped: a title passed without a kind emitted neither
    /// flag, guarded only by clap's `requires` at one call surface — the
    /// call-site shape day#101 names, on the exact defect `CLAUDE.md` records
    /// (`--title` silently discarding a title). A pair cannot be half-set.
    pub declaration: Option<(&'a str, &'a str)>,
}

impl<'a> Write<'a> {
    pub fn new(verb: &'a str, subject: &'a str, text: &'a str) -> Self {
        Self {
            verb,
            text,
            subject,
            cites: &[],
            declaration: None,
        }
    }

    pub fn cites(mut self, cites: &'a [String]) -> Self {
        self.cites = cites;
        self
    }

    pub fn declaring(mut self, title: &'a str, kind: &'a str) -> Self {
        self.declaration = Some((title, kind));
        self
    }
}

/// Deserializes a `--json` envelope, naming the command when it fails.
///
/// A parse failure here is loud on purpose. The whole point of migrating off
/// the rendered form is that a shape day cannot read must never look like an
/// empty log.
fn parse<T: serde::de::DeserializeOwned>(out: &str, args: &[&str]) -> Result<T, Error> {
    serde_json::from_str(out).map_err(|source| Error::Shape {
        args: args.join(" "),
        detail: source.to_string(),
    })
}

/// Refuses a `--json` shape version day does not know.
fn check_shape(v: u32, args: &[&str]) -> Result<(), Error> {
    if v == SHAPE_VERSION {
        return Ok(());
    }
    Err(Error::Shape {
        args: args.join(" "),
        detail: format!(
            "kan reported --json shape v{v}; day understands v{SHAPE_VERSION}. \
             kan's shape is additive-only, so a higher version is usually readable — \
             but day will not guess, because guessing wrong reads as an empty log."
        ),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The shape day reads, exactly as `kan show --json` emits it.
    const SHOW: &str = r#"{
      "v": 1,
      "subject": "telos/a",
      "claims": [
        {"cid":"bafyreia","kind":"Decision","subject":"telos/a",
         "author":"did:key:zabc","text":"A telos.","cites":[],"artifacts":[]},
        {"cid":"bafyreib","kind":"Subject","subject":"telos/a",
         "author":"did:key:zabc","title":"A"},
        {"cid":"bafyreic","kind":"Relation","subject":"telos/a",
         "author":"did:key:zabc","relation":"InTensionWith","target":"telos/b"}
      ],
      "inbound": []
    }"#;

    /// The bulk envelope, wrapping the single-subject shape above — which is
    /// exactly how ADR-71 defines it, and why day reuses one `Claim` parser.
    ///
    /// Retargeted from `ShowEnvelope` when day#71 made `show --all` the only
    /// read day performs. A test pinning a shape day no longer parses would
    /// have kept passing while saying nothing.
    fn bulk(subject_json: &str) -> String {
        format!(r#"{{"v":1,"subjects":[{subject_json}]}}"#)
    }

    #[test]
    fn claims_come_back_with_the_fields_day_reads() {
        let envelope: ShowAllEnvelope =
            parse(&bulk(SHOW), &["show", "--all"]).expect("should parse");
        assert_eq!(envelope.v, SHAPE_VERSION);
        let claims = &envelope.subjects[0].claims;
        assert_eq!(envelope.subjects[0].subject, "telos/a");
        assert_eq!(claims.len(), 3);

        assert_eq!(claims[0].text.as_deref(), Some("A telos."));
        assert_eq!(claims[0].author.as_deref(), Some("did:key:zabc"));
        // A title rides on the Subject claim, not the narrative one.
        assert_eq!(claims[1].title.as_deref(), Some("A"));
        // Relations carry no narrative body, which is why a tension's reason
        // needs a subject of its own.
        assert_eq!(claims[2].text, None);
        assert_eq!(claims[2].kind, "Relation");
    }

    /// kan's shape is additive-only, so a field day has never heard of must
    /// not break it. This is the property that makes pinning a shape version
    /// safe rather than brittle.
    #[test]
    fn an_unknown_field_is_ignored_rather_than_fatal() {
        let json = r#"{"v":1,"subjects":[{"subject":"telos/a","claims":[
            {"cid":"bafyreia","kind":"Decision","text":"x","invented_later":{"a":1}}
        ],"invented_later":1}]}"#;
        let envelope: ShowAllEnvelope =
            parse(json, &["show", "--all"]).expect("additive change must parse");
        assert_eq!(envelope.subjects[0].claims[0].text.as_deref(), Some("x"));
    }

    /// The failure this whole migration exists to end. day parsed kan's
    /// rendered output, kan changed it, and day reported an empty vocabulary
    /// against seven declared atoms at exit 0. Unreadable output must now be
    /// an error carrying the command that produced it.
    #[test]
    fn output_day_cannot_read_is_an_error_not_an_empty_result() {
        let err = parse::<ShowAllEnvelope>("telos/a (2 live claim(s)):", &["show", "telos/a"])
            .expect_err("rendered output must not parse as a shape");
        let rendered = err.to_string();
        assert!(rendered.contains("show telos/a"), "{rendered}");
        assert!(rendered.contains("silently empty"), "{rendered}");
    }

    /// A shape version day does not know is refused for the same reason.
    /// Additive-only means a higher version is *probably* readable, and
    /// "probably" is what produced a silently empty log once already.
    #[test]
    fn an_unknown_shape_version_is_refused() {
        assert!(check_shape(SHAPE_VERSION, &["status"]).is_ok());
        let err = check_shape(SHAPE_VERSION + 1, &["status"]).expect_err("unknown shape");
        assert!(err.to_string().contains("day understands"), "{err}");
    }

    #[test]
    fn subject_lists_come_back_in_order() {
        let json = r#"{"v":1,"subjects":[
            {"subject":"atom/design","state":"Unclassified"},
            {"subject":"telos/a","state":"Settled","value":"Open"}
        ]}"#;
        let envelope: SubjectsEnvelope = parse(json, &["status"]).expect("should parse");
        let names: Vec<String> = envelope.subjects.into_iter().map(|s| s.subject).collect();
        assert_eq!(names, vec!["atom/design", "telos/a"]);
    }
}
