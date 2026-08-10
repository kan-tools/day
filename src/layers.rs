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
/// **There is no `Default` variant, and that is not an oversight.** REQ-12's
/// first layer is day's shipped default, which a *config struct* has per field —
/// but the witness map has no fixed key set, so "no claim anywhere" means the
/// key does not exist rather than that it falls back to something. A variant
/// nothing can produce would be dead, and `pub` is exactly what stops the
/// compiler from saying so: `Compat::is_notable` and `BlockSchemas::extract`
/// both shipped that way, both `pub`, both called only by their own tests, with
/// clippy silent for both. It arrives with the first loader that has defaults
/// to fall back to.
///
/// fallback-untested: this block describes a variant that does not exist, not a
/// degrade path — there is no state in which this code falls back, so there is
/// nothing for a premise to assert. The real fallback in this module is the
/// legacy-block layer, which is marked and registered at its own site below.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Layer {
    /// A whole-block claim on the parent subject. Every key it sets shares this
    /// CID, which is precisely why per-key provenance needs per-key subjects.
    LegacyBlock(String),
    /// The key's own subject.
    Key(String),
}

/// An assembled value, with where each key came from.
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
}

/// The parent subject a per-key subject hangs under.
fn parent_of(slug: &str) -> String {
    format!("{SCHEMA_PREFIX}{slug}")
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
    let mut schema = match newest_fenced::<WitnessSchema>(client, &parent)? {
        Some((cid, schema)) => {
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
        None => WitnessSchema::default(),
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
        schema.probes.remove(key);
        schema.records.remove(key);
        schema.unsupported.remove(key);
        insert_entry(
            key.to_string(),
            entry.0,
            &mut schema.probes,
            &mut schema.records,
            &mut schema.unsupported,
        );
        provenance.insert(key.to_string(), Layer::Key(cid));
    }

    Ok(Effective {
        value: schema,
        provenance,
        withheld: client.claims_withheld_from_view(),
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
fn newest_entry(
    client: &KanClient,
    subject: &str,
) -> Result<Option<(String, WitnessEntry)>, Error> {
    for claim in client.show(subject)?.iter().rev() {
        let Some(text) = claim.text.as_deref() else {
            continue;
        };
        match extract_fenced::<WitnessEntry>(text) {
            Some(Ok(entry)) => return Ok(Some((claim.cid.clone(), entry))),
            Some(Err(source)) => {
                return Ok(Some((
                    claim.cid.clone(),
                    // Handed on as a value the entry parser will reject, so the
                    // key lands in `unsupported` with a reason rather than
                    // vanishing. The reason names the block error, which is what
                    // a reader needs to fix the claim.
                    WitnessEntry(serde_json::Value::String(source.to_string())),
                )));
            }
            None => continue,
        }
    }
    Ok(None)
}
