use serde::{Deserialize, Serialize};

use super::common::{ContentTypeGeneral, NumberType, RelatedIdentifierType, RelationType};
use super::contributor::Contributors;
use super::creator::Creators;
use super::publication_year::PublicationYear;
use super::title::Titles;

pub type RelatedItems = Vec<RelatedItem>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RelatedItem {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub related_item_identifier: Option<RelatedItemIdentifier>,
    pub related_item_type: ContentTypeGeneral,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub creators: Option<Creators>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contributors: Option<Contributors>,
    pub titles: Titles,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publication_year: Option<PublicationYear>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub volume: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issue: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub first_page: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_page: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub edition: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publisher: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub number: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub number_type: Option<NumberType>,
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RelatedItemIdentifier {
    pub related_item_identifier: String,
    pub related_item_identifier_type: RelatedIdentifierType,
}
