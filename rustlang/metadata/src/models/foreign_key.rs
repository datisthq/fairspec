use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct ForeignKey {
    pub columns: Vec<String>,
    pub reference: ForeignKeyReference,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct ForeignKeyReference {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    pub columns: Vec<String>,
}
