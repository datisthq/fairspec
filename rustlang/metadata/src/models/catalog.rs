use serde::{Deserialize, Serialize};

pub type Catalog = Vec<CatalogDataset>;

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct CatalogDataset {
    pub loc: String,
    pub upd: String,
}
