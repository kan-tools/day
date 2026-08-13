//! Read-only inventory of the effective values day consumes from `schema/*`.

use serde::Serialize;

use crate::atoms;
use crate::blocks::{
    BlockSchemas, CycleSchema, InjectionSchema, VerdictVocabulary, BLOCKS_SLUG, CYCLE_SLUG,
    INJECTION_SLUG, VERDICTS_SLUG,
};
use crate::kan_client::{KanClient, Read};
use crate::layers::{Effective, Layer};
use crate::schema::{Schema, DEFAULT_SLUG, SCHEMA_PREFIX};
use crate::telos::WITNESS_SLUG;

pub const SHAPE_VERSION: u32 = 1;

#[derive(Debug, Serialize)]
pub struct Report {
    pub v: u32,
    pub rows: Vec<Row>,
}

#[derive(Debug, Serialize)]
pub struct Row {
    pub subject: String,
    pub key: String,
    pub status: &'static str,
    pub value: serde_json::Value,
    pub layer: String,
    pub provenance: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
}

fn layer(layer: &Layer) -> (String, String) {
    match layer {
        Layer::Default => ("default".into(), "(default)".into()),
        Layer::LegacyBlock(cid) => ("legacy-block".into(), cid.clone()),
        Layer::Key(cid) => ("key".into(), cid.clone()),
    }
}

fn effective<T: Serialize>(subject: &str, value: Result<Effective<T>, atoms::Error>) -> Vec<Row> {
    match value {
        Ok(effective) => {
            let object = serde_json::to_value(&effective.value).unwrap_or(serde_json::Value::Null);
            let mut rows: Vec<Row> = effective
                .provenance
                .iter()
                .map(|(key, source)| {
                    let (layer, provenance) = layer(source);
                    Row {
                        subject: subject.to_string(),
                        key: key.clone(),
                        status: "readable",
                        value: object.get(key).cloned().unwrap_or(serde_json::Value::Null),
                        layer,
                        provenance,
                        detail: (effective.withheld > 0).then(|| {
                            format!("{} claim(s) elsewhere are withheld", effective.withheld)
                        }),
                    }
                })
                .collect();
            if rows.is_empty() {
                rows.push(Row {
                    subject: subject.to_string(),
                    key: "(value)".into(),
                    status: "absent",
                    value: object,
                    layer: "default".into(),
                    provenance: "(default)".into(),
                    detail: None,
                });
            }
            rows
        }
        Err(error) => vec![unreadable(subject, error.to_string())],
    }
}

fn declaration<T>(client: &KanClient, subject: &str, loaded: Result<T, String>) -> Vec<Row>
where
    T: Serialize + serde::de::DeserializeOwned + atoms::Versioned,
{
    // not-per-key: this generic is used only for declaration, map, and list
    // subjects whose effective value is whole-subject newest-wins. Config
    // structs and the witness map are routed through `layers` above.
    let read = atoms::newest_fenced::<T>(client, subject);
    match (read, loaded) {
        (Ok(Read::Present((cid, _))), Ok(value)) => vec![Row {
            subject: subject.into(),
            key: "(value)".into(),
            status: "readable",
            value: serde_json::to_value(value).unwrap_or(serde_json::Value::Null),
            layer: "legacy-block".into(),
            provenance: cid,
            detail: None,
        }],
        (Ok(Read::Absent), Ok(value)) => vec![Row {
            subject: subject.into(),
            key: "(value)".into(),
            status: "absent",
            value: serde_json::to_value(value).unwrap_or(serde_json::Value::Null),
            layer: "default".into(),
            provenance: "(default)".into(),
            detail: None,
        }],
        (Ok(Read::Withheld { count }), Ok(value)) => vec![Row {
            subject: subject.into(),
            key: "(value)".into(),
            status: "readable-with-caveat",
            value: serde_json::to_value(value).unwrap_or(serde_json::Value::Null),
            layer: "default".into(),
            provenance: "(default)".into(),
            detail: Some(format!(
                "production loader supplied this value while {count} subject claim(s) are withheld"
            )),
        }],
        (Ok(Read::Indeterminate { log_wide }), Ok(value)) => vec![Row {
            subject: subject.into(),
            key: "(value)".into(),
            status: "readable-with-caveat",
            value: serde_json::to_value(value).unwrap_or(serde_json::Value::Null),
            layer: "default".into(),
            provenance: "(default)".into(),
            detail: Some(format!(
                "production loader supplied this value while {log_wide} claim(s) are withheld without subject attribution"
            )),
        }],
        (_, Err(error)) => vec![unreadable(subject, error)],
        (Err(error), _) => vec![unreadable(subject, error.to_string())],
    }
}

fn unreadable(subject: &str, detail: String) -> Row {
    Row {
        subject: subject.into(),
        key: "(value)".into(),
        status: "unreadable",
        value: serde_json::Value::Null,
        layer: "unreadable".into(),
        provenance: "(unreadable)".into(),
        detail: Some(detail),
    }
}

pub fn read(client: &KanClient) -> Report {
    let mut rows = Vec::new();
    rows.extend(effective(
        "schema/injection",
        crate::layers::config::<InjectionSchema>(client, INJECTION_SLUG),
    ));
    rows.extend(effective(
        "schema/cycle",
        crate::layers::config::<CycleSchema>(client, CYCLE_SLUG),
    ));
    rows.extend(match crate::layers::witness(client) {
        Ok(value) => {
            let unsupported = value.value.unsupported.clone();
            let mut rows = effective("schema/witness", Ok(value));
            for row in &mut rows {
                if let Some(reason) = unsupported.get(&row.key) {
                    row.status = "unsupported";
                    row.layer = "unreadable".into();
                    row.value = serde_json::Value::Null;
                    row.detail = Some(reason.clone());
                }
            }
            rows
        }
        Err(error) => vec![unreadable("schema/witness", error.to_string())],
    });
    rows.extend(declaration::<BlockSchemas>(
        client,
        &format!("{SCHEMA_PREFIX}{BLOCKS_SLUG}"),
        BlockSchemas::load(client).map_err(|e| e.to_string()),
    ));
    rows.extend(declaration::<VerdictVocabulary>(
        client,
        &format!("{SCHEMA_PREFIX}{VERDICTS_SLUG}"),
        VerdictVocabulary::load(client).map_err(|e| e.to_string()),
    ));
    rows.extend(declaration::<crate::docs::DocsSchema>(
        client,
        &format!("{SCHEMA_PREFIX}{}", crate::docs::DOCS_SLUG),
        crate::docs::DocsSchema::load(client).map_err(|e| e.to_string()),
    ));
    rows.extend(declaration::<Schema>(
        client,
        &format!("{SCHEMA_PREFIX}{DEFAULT_SLUG}"),
        Schema::load(client, DEFAULT_SLUG).map_err(|e| e.to_string()),
    ));
    debug_assert!(rows
        .iter()
        .any(|r| r.subject == format!("schema/{WITNESS_SLUG}")));
    Report {
        v: SHAPE_VERSION,
        rows,
    }
}

pub fn render(report: &Report) -> String {
    let mut out = String::from("SUBJECT\tKEY\tVALUE\tLAYER\tPROVENANCE\tSTATUS\n");
    for row in &report.rows {
        let value = serde_json::to_string(&row.value).unwrap_or_else(|_| "null".into());
        out.push_str(&format!(
            "{}\t{}\t{}\t{}\t{}\t{}\n",
            row.subject, row.key, value, row.layer, row.provenance, row.status
        ));
        if let Some(detail) = &row.detail {
            out.push_str(&format!("  {}\n", detail.replace('\n', " ")));
        }
    }
    out
}
