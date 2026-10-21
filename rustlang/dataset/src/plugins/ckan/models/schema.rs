use serde::{Deserialize, Serialize};

use super::field::CkanField;

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct CkanSchema {
    pub fields: Vec<CkanField>,
}
