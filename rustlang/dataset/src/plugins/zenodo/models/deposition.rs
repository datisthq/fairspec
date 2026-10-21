use serde::{Deserialize, Serialize};

use super::link::ZenodoLink;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ZenodoDeposition {
    pub id: i64,
    pub links: ZenodoLink,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub submitted: Option<bool>,
}
