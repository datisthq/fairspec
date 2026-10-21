use serde::{Deserialize, Serialize};

use super::json_schema::JsonSchema;

pub type Profile = JsonSchema;
pub type ProfileRegistry = Vec<ProfileRegistryItem>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProfileRegistryItem {
    pub r#type: ProfileType,
    pub path: String,
    pub version: String,
    pub profile: Profile,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ProfileType {
    Catalog,
    Dataset,
    FileDialect,
    DataSchema,
    TableSchema,
}
