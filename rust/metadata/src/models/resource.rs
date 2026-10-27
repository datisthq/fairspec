use serde::de::{DeserializeOwned, Error as _};
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::{Map, Value};

use super::data::Data;
use super::data_schema::DataSchema;
use super::datacite::datacite::Datacite;
use super::descriptor::Descriptor;
use super::file_dialect::file_dialect::FileDialect;
use super::integrity::Integrity;
use super::path::PathOr;
use super::table_schema::TableSchema;
use crate::actions::frictionless::convert::{
    FrictionlessKind, convert_descriptor_from_frictionless, get_is_frictionless_descriptor,
};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Resource {
    #[serde(flatten, default, skip_serializing_if = "Map::is_empty")]
    pub extra: Map<String, Value>,
    #[serde(flatten)]
    pub datacite: Datacite,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<Data>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub textual: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub integrity: Option<Integrity>,
    #[serde(
        default,
        deserialize_with = "deserialize_file_dialect",
        skip_serializing_if = "Option::is_none"
    )]
    pub file_dialect: Option<PathOr<FileDialect>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data_schema: Option<PathOr<DataSchema>>,
    #[serde(
        default,
        deserialize_with = "deserialize_table_schema",
        skip_serializing_if = "Option::is_none"
    )]
    pub table_schema: Option<PathOr<TableSchema>>,
    #[serde(
        rename = "unstable_customMetadata",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub unstable_custom_metadata: Option<Descriptor>,
}

// A typed resource cannot hold a Frictionless schema or dialect, so an inline one is
// converted here, where the other twins convert it when resolving
fn deserialize_table_schema<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<PathOr<TableSchema>>, D::Error> {
    deserialize_frictionless(deserializer, FrictionlessKind::TableSchema)
}

fn deserialize_file_dialect<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<PathOr<FileDialect>>, D::Error> {
    deserialize_frictionless(deserializer, FrictionlessKind::FileDialect)
}

fn deserialize_frictionless<'de, D: Deserializer<'de>, T: DeserializeOwned>(
    deserializer: D,
    kind: FrictionlessKind,
) -> Result<Option<PathOr<T>>, D::Error> {
    let value = match Option::<Value>::deserialize(deserializer)? {
        Some(Value::Object(object)) if get_is_frictionless_descriptor(&object, kind) => {
            let (descriptor, _, errors) = convert_descriptor_from_frictionless(&object, kind, None);
            match descriptor {
                Some(descriptor) => Value::Object(descriptor),
                None => {
                    let message = serde_json::to_string(&errors).unwrap_or_default();
                    return Err(D::Error::custom(format!(
                        "invalid Frictionless descriptor: {message}"
                    )));
                }
            }
        }
        Some(value) => value,
        None => return Ok(None),
    };
    serde_json::from_value(value)
        .map(Some)
        .map_err(D::Error::custom)
}
