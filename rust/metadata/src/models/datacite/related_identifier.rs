use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use super::common::{ContentTypeGeneral, RelatedIdentifierType, RelationType};

pub type RelatedIdentifiers = Vec<RelatedIdentifier>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RelatedIdentifier {
    #[serde(flatten, default, skip_serializing_if = "Map::is_empty")]
    pub extra: Map<String, Value>,
    pub related_identifier: String,
    pub related_identifier_type: RelatedIdentifierType,
    pub relation_type: RelationType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub related_metadata_scheme: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scheme_uri: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scheme_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource_type_general: Option<ContentTypeGeneral>,
}
