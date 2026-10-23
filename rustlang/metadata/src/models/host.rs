use serde::{Deserialize, Serialize};

/// Which hosts remote fetches may reach: `any`, or only `public` addresses.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum HostPolicy {
    #[default]
    Any,
    Public,
}
