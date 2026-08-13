//! The layers a configuration value is assembled from, in one place.
//!
//! `.design/vocabulary-packs.md` REQ-11, REQ-12, REQ-17 and REQ-21, built for
//! `schema/witness` first. Today a whole block lives in one claim, so a claim
//! setting one entry resets the others and an eighteen-entry map has to be
//! restated to add one — twelve times so far on `schema/witness` alone. Making
//! a key its own subject turns newest-wins *per subject* into per-key
//! resolution with no change to the fold.
//!
//! **Three layers, one rule** (REQ-12): `Default`, overlaid by a legacy
//! whole-block claim on the parent subject if one exists, overlaid by the
//! newest live claim on the key's own subject. A project that adopts nothing
//! sees today's behaviour byte-for-byte, which is what "no migration" has to
//! mean.
//!
//! **What this deliberately does not claim** (REQ-20): a key inherited from the
//! legacy block cannot be granularly retracted, because the claim carrying it
//! also carries every other key. Granular retraction is available only for keys
//! a per-key claim set. Every adopting project passes through that hybrid
//! state, and an earlier draft of the design claimed the capability uniformly.

use std::collections::BTreeMap;

use crate::atoms::{extract_fenced, newest_fenced, Error};
use crate::kan_client::KanClient;
use crate::schema::SCHEMA_PREFIX;
use crate::telos::{insert_entry, WitnessEntry, WitnessSchema, WITNESS_SLUG};

/// Which layer a key's effective value came from.
///
/// Carries the CID for the two layers that have one, because that is what
/// `.design/day-config.md` reports per key and what makes the value traceable
/// to the claim that set it.
/// **`Default` arrives here with [`config`], exactly as this comment said it
/// would.** It was deliberately absent while `witness` was the only assembler:
/// the witness map has no fixed key set, so "no claim anywhere" means the key
/// does not exist rather than that it falls back to something, and a variant
/// nothing can produce is dead code that `pub` hides from the compiler —
/// `Compat::is_notable` and `BlockSchemas::extract` both shipped that way. A
/// config struct is the first shape with a per-field default to fall back to,
/// so the variant is now producible and produced.
///
/// fallback: config-shipped-default
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Layer {
    /// day's shipped default: no claim anywhere set this key.
    ///
    /// Only a config struct can reach this — a map or list key that nothing
    /// declared is absent rather than defaulted.
    Default,
    /// A whole-block claim on the parent subject. Every key it sets shares this
    /// CID, which is precisely why per-key provenance needs per-key subjects.
    LegacyBlock(String),
    /// The key's own subject.
    Key(String),
}

/// An assembled value, with where each key came from.
#[derive(Debug)]
pub struct Effective<T> {
    pub value: T,
    /// Per key, the layer that decided it.
    pub provenance: BTreeMap<String, Layer>,
    /// Claims withheld from this view, log-wide.
    ///
    /// Carried because this function **enumerates** subjects rather than asking
    /// for one by name, and [`KanClient::claims_withheld_from_view`] states that
    /// obligation directly: kan gives no per-subject evidence for a subject it
    /// withheld entirely, so an enumerating reader can never be told by
    /// `show` that a key's subject is missing rather than hidden. Reported
    /// rather than acted on — a caller that renders per-key provenance must be
    /// able to say "and there may be keys this view cannot see".
    pub withheld: u64,
    /// Whether **any** claim contributed a layer, as opposed to any key having
    /// resolved to a value.
    ///
    /// These come apart on an empty declaration. A legacy block of `{}` is a
    /// claim that declares no keys: provenance is empty and the project has
    /// nonetheless declared a witness schema. Deciding "is anything declared"
    /// from `provenance.is_empty()` turned that into `NotDeclared`, where the
    /// whole-block loader returned an empty schema — a behaviour change REQ-12
    /// forbids in the words "byte-identical to today's behaviour". Found by
    /// running it, not by reading it.
    pub declared: bool,
}

/// The parent subject a per-key subject hangs under.
fn parent_of(slug: &str) -> String {
    format!("{SCHEMA_PREFIX}{slug}")
}

/// The effective value of a **config struct** subject: `Default` ← legacy whole
/// block ← per-key subjects.
///
/// The second shape to go through this module, after the witness map, and the
/// one REQ-11's own acceptance criterion is written against: claims on
/// `schema/injection/cadence` and `schema/injection/max_practice_items` resolve
/// to *both*, where two claims on the parent subject lose the first.
///
/// **The key vocabulary is derived from `T`, never written out.** Serialising
/// `T::default()` yields exactly the field set serde will accept, so a field
/// added to the struct is declarable the moment it compiles and a hand-kept
/// list cannot drift from it — the defect class this repo has corrected in four
/// separate places.
///
/// **A per-key claim carries the parent's own fence** (REQ-21, which asks for
/// this to be declared rather than implied) holding an object with exactly the
/// one field the subject names. A claim on `schema/injection/cadence` carries a
/// `day-injection` fence whose body is `{"cadence": 25}` — nothing else.
///
/// Same fence as the parent, so a per-key claim inherits the version gate,
/// `deny_unknown_fields`, and the `BlockError` diagnostics rather than growing a
/// second grammar. The field name is checked against the subject's last segment
/// because the two can disagree, and a claim on `.../cadence` that sets
/// `max_practice_items` would otherwise write a key nothing named.
///
/// **An unreadable per-key claim is an error, not a skipped key.** This differs
/// from [`witness`], which demotes a bad entry into `unsupported` because
/// failing a whole schema over one probe took down every hook in a session. A
/// config struct has no such bucket, and the parent-block path for the same type
/// already errors — so silently dropping the key would resolve day's default
/// while a project believes it declared something.
pub fn config<T>(client: &KanClient, slug: &str) -> Result<Effective<T>, Error>
where
    T: serde::Serialize + serde::de::DeserializeOwned + crate::atoms::Versioned + Default,
{
    let parent = parent_of(slug);
    let mut provenance = BTreeMap::new();

    // Layer 1. The field set and the shipped values in one step, from `T`
    // itself.
    let serde_json::Value::Object(mut fields) =
        serde_json::to_value(T::default()).map_err(|source| Error::ConfigShape {
            subject: parent.clone(),
            reason: source.to_string(),
        })?
    else {
        // A config subject whose type is not an object has no keys to resolve
        // per key, and calling it one would report a provenance that cannot
        // exist. `schema/verdicts` is a `Vec` and is deliberately not routed
        // here — RQ-7 records that "per key" is undefined for a list.
        return Err(Error::ConfigShape {
            subject: parent,
            reason: "not an object, so it has no keys to resolve per key".into(),
        });
    };
    for key in fields.keys() {
        provenance.insert(key.clone(), Layer::Default);
    }

    // Layer 2. The guard inside `newest_fenced_declared` stays on for the parent
    // for the same reason it does in `witness`: an absent parent block is what
    // makes day print a `kan observe` starter, and following that starter under
    // a narrowed trust base forks the vocabulary silently.
    //
    // fallback: legacy-config-block
    //
    // A project that never adopts per-key subjects falls back to this layer for
    // every key it set — REQ-12's "no migration", and the mode day's own repo is
    // in today.
    let mut declared = false;
    match crate::atoms::newest_fenced_declared::<T>(client, &parent)? {
        crate::kan_client::Read::Present((cid, block, _typed)) => {
            declared = true;
            if let Some(object) = block.as_object() {
                for (key, value) in object {
                    fields.insert(key.clone(), value.clone());
                    provenance.insert(key.clone(), Layer::LegacyBlock(cid.clone()));
                }
            }
        }
        crate::kan_client::Read::Absent => {}
        crate::kan_client::Read::Withheld { count } => {
            return Err(Error::Kan(
                crate::kan_client::Error::AbsentUnderNarrowedTrust {
                    subject: parent,
                    count,
                },
            ))
        }
        crate::kan_client::Read::Indeterminate { log_wide } => {
            return Err(Error::Kan(
                crate::kan_client::Error::AbsentUnderNarrowedTrust {
                    subject: parent,
                    count: log_wide,
                },
            ))
        }
    }

    // Layer 3. Enumerated from the memoised bulk read, so N subjects cost no
    // extra kan invocation.
    let prefix = format!("{parent}/");
    let mut subjects: Vec<String> = client
        .subjects()?
        .into_iter()
        .filter(|s| s.starts_with(&prefix))
        .collect();
    subjects.sort();

    for subject in subjects {
        let Some(key) = subject.strip_prefix(&prefix) else {
            continue;
        };
        // A nested rkey below the key's own subject is not a key of this struct.
        if key.is_empty() || key.contains('/') {
            continue;
        }
        // **REQ-21.** After `kan retract` the subject REMAINS carrying only a
        // `Retraction`, so "subject exists, no block" reads as *this key is
        // absent* — it falls back through the layers below it rather than
        // failing the read. Nothing is swallowed: the `show` error still
        // propagates.
        //
        // fallback: retracted-key-subject
        let Some((cid, block)) = newest_key_block::<T>(client, &subject, key)? else {
            continue;
        };

        let Some(value) = block.as_object().and_then(|o| o.get(key)) else {
            continue;
        };
        fields.insert(key.to_string(), value.clone());
        provenance.insert(key.to_string(), Layer::Key(cid));
        declared = true;
    }

    // One typed parse of the assembled object, then `validate()` — the two
    // halves `parse_block` runs, in the same order, so a value assembled from
    // three layers is refused exactly where one claim carrying it would be.
    //
    // **The second half was missing, and a comment here asserted it was not.**
    // `from_value` alone enforces types and `deny_unknown_fields`; it does not
    // run the structural invariants serde cannot express. `CycleSchema::validate`
    // refuses an empty tag pattern because "the failure would look exactly like
    // working" — so `schema/cycle` carrying `{"tags":""}` was refused while
    // `schema/cycle/tags` carrying the same body was accepted in silence, and
    // cycle semantics feed position inference. Found by a cold review running the
    // built binary against both shapes; every test here passed throughout, because
    // they asserted resolution and none asserted refusal.
    let value: T = serde_json::from_value(serde_json::Value::Object(fields)).map_err(|source| {
        Error::ConfigShape {
            subject: parent.clone(),
            reason: source.to_string(),
        }
    })?;
    value.validate().map_err(|reason| Error::ConfigShape {
        subject: parent.clone(),
        // Named as assembled rather than as a bad claim: no single claim is
        // necessarily wrong here — the layers together produce a value the type
        // refuses, and pointing at one CID would blame a claim that may be fine.
        reason: format!("the value assembled from the declared layers is invalid: {reason}"),
    })?;

    Ok(Effective {
        value,
        provenance,
        withheld: client.claims_withheld_from_view(),
        declared,
    })
}

/// The newest claim on a per-key subject carrying the parent's fence, checked
/// to declare the key its subject names and nothing else.
fn newest_key_block<T>(
    client: &KanClient,
    subject: &str,
    key: &str,
) -> Result<Option<(String, serde_json::Value)>, Error>
where
    T: crate::atoms::Versioned,
{
    let claims = match client.show(subject)? {
        crate::kan_client::Read::Present(claims) => claims,
        crate::kan_client::Read::Absent => return Ok(None),
        crate::kan_client::Read::Withheld { count } => {
            return Err(Error::Kan(
                crate::kan_client::Error::AbsentUnderNarrowedTrust {
                    subject: subject.to_string(),
                    count,
                },
            ))
        }
        crate::kan_client::Read::Indeterminate { log_wide } => {
            return Err(Error::Kan(
                crate::kan_client::Error::AbsentUnderNarrowedTrust {
                    subject: subject.to_string(),
                    count: log_wide,
                },
            ))
        }
    };
    for claim in claims.iter().rev() {
        let Some(text) = claim.text.as_deref() else {
            continue;
        };
        // **AC-30 and AC-31, from the one scanner.** Not finding this type's
        // fence has three causes and only one is an absence, which is the
        // distinction `FenceScan` exists to carry:
        //
        //   Absent        prose, or what `kan retract` leaves behind. Falls
        //                 through to the layer below (AC-30).
        //   Foreign       a closed `day-` block that is not ours. On a per-key
        //                 subject — which exists ONLY to carry this key's
        //                 declaration — that is a misspelling or a block from a
        //                 newer day. Reading it as absence resolved the layer
        //                 below while the project believed it had declared a
        //                 value: `day-injektion` on `schema/injection/cadence`
        //                 resolved `cadence` to 10 from `Layer::Default` (AC-31).
        //   Unterminated  a `day-` fence opened and never closed. Reported by
        //                 `scan_fenced` for every reader in day, not just here.
        //
        // The first fix for this carried its own fence scanner beside
        // `fenced_body`, and the two disagreed about exactly the unterminated
        // case — the cold review's F4. The remedy for a duplicated reader is
        // deleting it, not making the pair agree, so the policy now lives at
        // this call and the parsing lives in one place.
        let body = match crate::atoms::scan_fenced(text, T::FENCE) {
            crate::atoms::FenceScan::Absent => continue,
            crate::atoms::FenceScan::Found(body) => body,
            crate::atoms::FenceScan::Unterminated(info) => {
                return Err(Error::Block {
                    subject: subject.to_string(),
                    cid: claim.cid.clone(),
                    source: crate::atoms::BlockError::Unterminated {
                        fence: crate::atoms::Fence::Owned(info.to_string()),
                    },
                })
            }
            crate::atoms::FenceScan::Foreign(info) => {
                return Err(Error::ConfigShape {
                    subject: subject.to_string(),
                    // Names both, because either could be the mistake: version
                    // skew is fixed by upgrading and a typo by editing, and
                    // telling someone the wrong one is worse than saying nothing
                    // (day#60, `telos/honest-reads`).
                    reason: format!(
                        "declares `{info}` but this key is read from `{}`. A \
                         per-key claim carries the same fence as its parent \
                         subject; if that block came from a newer day, upgrade \
                         day.",
                        T::FENCE
                    ),
                });
            }
        };
        // Version-gated through the same path a whole block takes, so a per-key
        // claim written by a newer day says "upgrade day" rather than reading as
        // a typo — day#60's mistake, and `telos/honest-reads`.
        let object = crate::atoms::version_gate(
            body.trim(),
            crate::atoms::Fence::Borrowed(T::FENCE),
            T::SUPPORTED_VERSION,
        )
        .map_err(|source| Error::Block {
            subject: subject.to_string(),
            cid: claim.cid.clone(),
            source,
        })?;

        let keys: Vec<&String> = match object.as_object() {
            Some(o) => o.keys().collect(),
            None => Vec::new(),
        };
        if keys.len() != 1 || keys[0] != key {
            return Err(Error::ConfigShape {
                subject: subject.to_string(),
                reason: format!(
                    "a per-key claim declares exactly the key its subject names. \
                     This subject names `{key}` and the block declares {}.",
                    if keys.is_empty() {
                        "nothing".to_string()
                    } else {
                        keys.iter()
                            .map(|k| format!("`{k}`"))
                            .collect::<Vec<_>>()
                            .join(", ")
                    }
                ),
            });
        }
        return Ok(Some((claim.cid.clone(), object)));
    }
    Ok(None)
}

/// The effective witness schema: `Default` ← legacy whole block ← per-key
/// subjects.
///
/// **The one assembler** (REQ-17). `WitnessSchema::load` calls this rather than
/// reading the parent subject itself, so there is no second path that resolves
/// a witness differently — seven loaders reimplementing an overlay is the shape
/// day#101 records three instances of.
pub fn witness(client: &KanClient) -> Result<Effective<WitnessSchema>, Error> {
    let parent = parent_of(WITNESS_SLUG);
    let mut provenance = BTreeMap::new();

    // Layer 2. `newest_fenced` keeps its withheld-read guard here: an absent
    // parent block is the case that makes day print a `kan observe` starter,
    // and following that starter under a narrowed trust base is what forks a
    // vocabulary silently.
    //
    // fallback: legacy-witness-block
    //
    // A project that never adopts per-key subjects falls back to this layer for
    // every key, which is REQ-12's "no migration" and is the mode day's own repo
    // is in today — so the mode that needs registering is the *other* one, and
    // the registered test asserts the premise both ways round rather than
    // trusting that this repo exercises either.
    let mut declared = false;
    let mut schema = match newest_fenced::<WitnessSchema>(client, &parent)? {
        crate::kan_client::Read::Present((cid, schema)) => {
            declared = true;
            for key in schema
                .probes
                .keys()
                .chain(schema.records.keys())
                .chain(schema.unsupported.keys())
            {
                provenance.insert(key.clone(), Layer::LegacyBlock(cid.clone()));
            }
            schema
        }
        crate::kan_client::Read::Absent => WitnessSchema::default(),
        crate::kan_client::Read::Withheld { count } => {
            return Err(Error::Kan(
                crate::kan_client::Error::AbsentUnderNarrowedTrust {
                    subject: parent,
                    count,
                },
            ))
        }
        crate::kan_client::Read::Indeterminate { log_wide } => {
            return Err(Error::Kan(
                crate::kan_client::Error::AbsentUnderNarrowedTrust {
                    subject: parent,
                    count: log_wide,
                },
            ))
        }
    };

    // Layer 3. Enumerated from the memoised bulk read, so N subjects cost no
    // extra kan invocation (`src/kan_client.rs`, and REQ-11's own verification).
    let prefix = format!("{parent}/");
    let mut keys: Vec<String> = client
        .subjects()?
        .into_iter()
        .filter(|s| s.starts_with(&prefix))
        .collect();
    keys.sort();

    for subject in keys {
        let Some(key) = subject.strip_prefix(&prefix) else {
            continue;
        };
        // A nested rkey below the key's own subject is not a key of this map.
        // Skipping rather than flattening keeps `schema/witness/a/b` from
        // silently becoming a witness named `a/b`.
        if key.is_empty() || key.contains('/') {
            continue;
        }

        let Some((cid, entry)) = newest_entry(client, &subject)? else {
            // **REQ-21.** After `kan retract` the subject REMAINS, carrying only
            // a `Retraction`, so "subject exists, no block" has to read as *this
            // key is absent* rather than as a read failure. That is also why the
            // guard in `newest_fenced` is not used on this path: its purpose is
            // to stop day offering a starter command for a subject a narrowed
            // view might merely be hiding, and this path offers no starter and
            // draws no conclusion about the parent. Nothing is swallowed — the
            // `show` error still propagates.
            continue;
        };

        // A per-key claim REPLACES the legacy entry for its key, in all three
        // buckets. Clearing first matters: a legacy paired witness overlaid by a
        // material-only key must lose its record half, or the key would keep
        // asking half a question nobody declared.
        declared = true;
        schema.probes.remove(key);
        schema.records.remove(key);
        schema.unsupported.remove(key);
        match entry {
            Ok(entry) => insert_entry(
                key.to_string(),
                entry.0,
                &mut schema.probes,
                &mut schema.records,
                &mut schema.unsupported,
            ),
            // **The block error is reported as itself.** Handing it to the entry
            // parser as a string got its message wrapped in serde's "unknown
            // variant … expected one of `path`, `tag`, …", so a claim written by
            // a NEWER day read as a typo and the reader was pointed at fixing a
            // claim that is fine. day#60 is that exact mistake, and
            // `telos/honest-reads` is the telos it violates: version skew is
            // fixed by upgrading, a malformed block by editing, and telling
            // someone the wrong one is worse than saying nothing.
            Err(source) => {
                schema
                    .unsupported
                    .insert(key.to_string(), source.to_string());
            }
        }
        provenance.insert(key.to_string(), Layer::Key(cid));
    }

    Ok(Effective {
        value: schema,
        provenance,
        withheld: client.claims_withheld_from_view(),
        declared,
    })
}

/// The newest claim on `subject` carrying a `day-witness` block, as a raw entry
/// value.
///
/// [`newest_fenced`] without its absent-under-narrowed-trust guard, for the
/// reason given at the call site. A malformed block **demotes that one key**
/// rather than failing the read, matching what the map deserializer does with a
/// probe kind it cannot read: `WitnessSchema`'s own doc records that failing the
/// whole schema over one entry took down every hook and status line in a
/// session.
#[allow(clippy::type_complexity)]
fn newest_entry(
    client: &KanClient,
    subject: &str,
) -> Result<Option<(String, Result<WitnessEntry, crate::atoms::BlockError>)>, Error> {
    let claims = match client.show(subject)? {
        crate::kan_client::Read::Present(claims) => claims,
        crate::kan_client::Read::Absent => return Ok(None),
        crate::kan_client::Read::Withheld { count } => {
            return Err(Error::Kan(
                crate::kan_client::Error::AbsentUnderNarrowedTrust {
                    subject: subject.to_string(),
                    count,
                },
            ))
        }
        crate::kan_client::Read::Indeterminate { log_wide } => {
            return Err(Error::Kan(
                crate::kan_client::Error::AbsentUnderNarrowedTrust {
                    subject: subject.to_string(),
                    count: log_wide,
                },
            ))
        }
    };
    for claim in claims.iter().rev() {
        let Some(text) = claim.text.as_deref() else {
            continue;
        };
        // The block error is carried out **as an error**, not flattened into a
        // value. Flattening it into a `Value::String` for the entry parser to
        // reject is what buried a version-skew message inside serde's "unknown
        // variant … expected one of `path`, `tag`, …", so a claim written by a
        // newer day read as a typo. The caller reports the error's own text.
        match extract_fenced::<WitnessEntry>(text) {
            Some(result) => return Ok(Some((claim.cid.clone(), result))),
            None => continue,
        }
    }
    Ok(None)
}
