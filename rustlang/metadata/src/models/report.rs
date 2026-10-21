use serde::{Deserialize, Serialize};

use super::error::FairspecError;

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Report {
    pub valid: bool,
    pub errors: Vec<FairspecError>,
}
