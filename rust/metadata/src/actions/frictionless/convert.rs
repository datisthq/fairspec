use serde_json::Value;

use super::inspect::inspect_frictionless_model;
use super::mapping::{FrictionlessMapping, FrictionlessRecorder};
use crate::actions::dataset::from_frictionless::convert_dataset_from_frictionless;
use crate::actions::file_dialect::from_frictionless::convert_file_dialect_from_frictionless;
use crate::actions::table_schema::from_frictionless::convert_table_schema_from_frictionless;
use crate::models::descriptor::Descriptor;
use crate::models::error::error::FairspecError;
use crate::models::frictionless::dialect::FRICTIONLESS_DIALECT;
use crate::models::frictionless::package::FRICTIONLESS_PACKAGE;
use crate::models::frictionless::schema::FRICTIONLESS_SCHEMA;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrictionlessKind {
    Dataset,
    TableSchema,
    FileDialect,
}

const FRICTIONLESS_RESOURCE_KEYS: [&str; 3] = ["path", "schema", "dialect"];

const FRICTIONLESS_DIALECT_KEYS: [&str; 18] = [
    "header",
    "headerRows",
    "headerJoin",
    "commentRows",
    "commentChar",
    "delimiter",
    "lineTerminator",
    "quoteChar",
    "doubleQuote",
    "escapeChar",
    "nullSequence",
    "skipInitialSpace",
    "property",
    "itemType",
    "itemKeys",
    "sheetNumber",
    "sheetName",
    "table",
];

/// Whether a descriptor is the Frictionless counterpart of a Dataset, Table Schema or
/// File Dialect, judged by its content; a Fairspec `$schema` always wins
pub fn get_is_frictionless_descriptor(descriptor: &Descriptor, kind: FrictionlessKind) -> bool {
    let schema = descriptor
        .get("$schema")
        .and_then(Value::as_str)
        .unwrap_or_default();
    if schema.starts_with("https://fairspec.org/") {
        return false;
    }

    match kind {
        FrictionlessKind::TableSchema => {
            descriptor.get("fields").is_some_and(Value::is_array)
                && !descriptor.contains_key("properties")
        }
        FrictionlessKind::FileDialect => {
            !descriptor.contains_key("format")
                && FRICTIONLESS_DIALECT_KEYS
                    .iter()
                    .any(|key| descriptor.contains_key(*key))
        }
        FrictionlessKind::Dataset => {
            if descriptor.get("profile").is_some_and(Value::is_string) {
                return true;
            }
            if schema.contains("datapackage.org") || schema.contains("frictionlessdata.io") {
                return true;
            }
            let resources = descriptor.get("resources").and_then(Value::as_array);
            resources.into_iter().flatten().any(|resource| {
                resource.as_object().is_some_and(|resource| {
                    FRICTIONLESS_RESOURCE_KEYS
                        .iter()
                        .any(|key| resource.contains_key(*key))
                })
            })
        }
    }
}

/// Convert a Frictionless descriptor to its Fairspec counterpart, with the mapping that
/// points errors about the result back at the original
/// A descriptor the Frictionless model cannot read comes back as metadata errors instead
pub fn convert_descriptor_from_frictionless(
    descriptor: &Descriptor,
    kind: FrictionlessKind,
    root_json_pointer: Option<&str>,
) -> (Option<Descriptor>, FrictionlessMapping, Vec<FairspecError>) {
    let mut mapping = FrictionlessMapping::default();
    let root = root_json_pointer.unwrap_or_default();

    let shape = match kind {
        FrictionlessKind::Dataset => &FRICTIONLESS_PACKAGE,
        FrictionlessKind::TableSchema => &FRICTIONLESS_SCHEMA,
        FrictionlessKind::FileDialect => &FRICTIONLESS_DIALECT,
    };
    let (parsed, errors) = inspect_frictionless_model(descriptor, shape, root_json_pointer);
    let Some(parsed) = parsed else {
        return (None, mapping, errors);
    };

    let converted = match kind {
        FrictionlessKind::Dataset => Some(convert_dataset_from_frictionless(
            &parsed,
            Some(&mut mapping),
        )),
        FrictionlessKind::TableSchema => {
            let mut record = FrictionlessRecorder::new(&mut mapping.pointers, root, root);
            Some(convert_table_schema_from_frictionless(
                &parsed,
                Some(&mut record),
            ))
        }
        FrictionlessKind::FileDialect => {
            let mut record = FrictionlessRecorder::new(&mut mapping.pointers, root, root);
            convert_file_dialect_from_frictionless(Some(&parsed), None, Some(&mut record))
        }
    };
    (converted, mapping, Vec::new())
}

/// Convert a descriptor when it is Frictionless, else pass it through unchanged
pub(crate) fn prepare_frictionless_descriptor(
    descriptor: Descriptor,
    kind: FrictionlessKind,
    root_json_pointer: Option<&str>,
) -> (
    Option<Descriptor>,
    Option<FrictionlessMapping>,
    Vec<FairspecError>,
) {
    if !get_is_frictionless_descriptor(&descriptor, kind) {
        return (Some(descriptor), None, Vec::new());
    }
    let (descriptor, mapping, errors) =
        convert_descriptor_from_frictionless(&descriptor, kind, root_json_pointer);
    (descriptor, Some(mapping), errors)
}
