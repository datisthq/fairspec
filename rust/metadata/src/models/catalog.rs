use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

pub type Catalog = Vec<CatalogDataset>;

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct CatalogDataset {
    #[serde(flatten, default, skip_serializing_if = "Map::is_empty")]
    pub extra: Map<String, Value>,
    pub loc: String,
    pub upd: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub del: Option<bool>,
}
