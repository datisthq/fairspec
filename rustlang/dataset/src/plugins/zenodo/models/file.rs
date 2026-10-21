use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ZenodoFileLinks {
    #[serde(rename = "self")]
    pub self_: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ZenodoFile {
    pub id: String,
    pub key: String,
    pub size: i64,
    pub checksum: String,
    pub links: ZenodoFileLinks,
}
