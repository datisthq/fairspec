use serde::{Deserialize, Serialize};

use crate::models::foreign_key::ForeignKey;

define_literal!(ForeignKeyErrorType, ForeignKey, "foreignKey");

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ForeignKeyError {
    pub r#type: ForeignKeyErrorType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,
    pub foreign_key: ForeignKey,
    pub cells: Vec<String>,
}
