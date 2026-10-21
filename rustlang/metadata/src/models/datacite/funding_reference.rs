use serde::{Deserialize, Serialize};

use super::common::FunderIdentifierType;

pub type FundingReferences = Vec<FundingReference>;

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FundingReference {
    pub funder_name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub funder_identifier: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub funder_identifier_type: Option<FunderIdentifierType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub award_number: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub award_uri: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub award_title: Option<String>,
}
