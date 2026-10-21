use serde::{Deserialize, Serialize};

pub type AlternateIdentifiers = Vec<AlternateIdentifier>;

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlternateIdentifier {
    pub alternate_identifier: String,
    pub alternate_identifier_type: String,
}
