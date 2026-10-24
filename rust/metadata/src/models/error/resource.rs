use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ResourceError {
    #[serde(rename = "resource/missing")]
    Missing(ResourceMissingError),
    #[serde(rename = "resource/type")]
    Type(ResourceTypeError),
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceMissingError {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,
    pub referencing_resource_name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceTypeError {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,
    pub expected_resource_type: ExpectedResourceType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub referencing_resource_name: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ExpectedResourceType {
    Data,
    Table,
}
