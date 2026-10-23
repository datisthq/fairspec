use std::sync::RwLock;

use crate::models::exception::{Error, Result};
use crate::models::host::HostPolicy;

pub const HOST_POLICY_ENV: &str = "FAIRSPEC_HOST_POLICY";

static HOST_POLICY: RwLock<Option<HostPolicy>> = RwLock::new(None);

/// Returns the policy remote fetches follow: the one set, then FAIRSPEC_HOST_POLICY, then `Any`.
pub fn get_host_policy() -> Result<HostPolicy> {
    let policy = *HOST_POLICY
        .read()
        .unwrap_or_else(|error| error.into_inner());
    resolve_host_policy(policy, std::env::var(HOST_POLICY_ENV).ok().as_deref())
}

/// Sets the policy remote fetches follow; `None` falls back to FAIRSPEC_HOST_POLICY.
pub fn set_host_policy(policy: Option<HostPolicy>) {
    *HOST_POLICY
        .write()
        .unwrap_or_else(|error| error.into_inner()) = policy;
}

pub(crate) fn resolve_host_policy(
    policy: Option<HostPolicy>,
    env: Option<&str>,
) -> Result<HostPolicy> {
    if let Some(policy) = policy {
        return Ok(policy);
    }
    match env {
        None | Some("") | Some("any") => Ok(HostPolicy::Any),
        Some("public") => Ok(HostPolicy::Public),
        Some(value) => Err(Error::Load(format!(
            "Invalid {HOST_POLICY_ENV}: {value} (expected \"any\" or \"public\")"
        ))),
    }
}
