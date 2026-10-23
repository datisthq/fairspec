use std::fmt;

use ureq::config::ConfigBuilder;
use ureq::http::Uri;
use ureq::typestate::AgentScope;
use ureq::unversioned::resolver::{DefaultResolver, ResolvedSocketAddrs, Resolver};
use ureq::unversioned::transport::{DefaultConnector, NextTimeout};

use crate::actions::host::address::{create_host_message, get_is_public_address};
use crate::actions::host::policy::get_host_policy;
use crate::models::exception::{Error, Result};
use crate::models::host::HostPolicy;

/// Builds a ureq agent under the host policy. Under `Public` its resolver refuses non-public
/// addresses, so every connection, redirects included, reaches only an address it checked.
pub fn create_http_agent(config: ConfigBuilder<AgentScope>) -> Result<ureq::Agent> {
    Ok(create_policy_agent(
        get_host_policy()?,
        config,
        get_is_public_address,
    ))
}

/// Maps a ureq failure to the crate error, keeping a host policy refusal's own message.
pub fn create_http_error(url: &str, error: ureq::Error) -> Error {
    if let ureq::Error::Io(io_error) = &error
        && let Some(refusal) = io_error
            .get_ref()
            .and_then(|inner| inner.downcast_ref::<HostRefusal>())
    {
        return Error::Http(refusal.0.clone());
    }
    Error::Http(format!("{url}: {error}"))
}

pub(crate) fn create_policy_agent(
    policy: HostPolicy,
    config: ConfigBuilder<AgentScope>,
    is_public: fn(&str) -> bool,
) -> ureq::Agent {
    match policy {
        HostPolicy::Any => config.build().into(),
        // A proxy would be the peer the resolver checks, so public fetches go direct
        HostPolicy::Public => ureq::Agent::with_parts(
            config.proxy(None).build(),
            DefaultConnector::new(),
            PublicResolver { is_public },
        ),
    }
}

#[derive(Debug)]
struct PublicResolver {
    is_public: fn(&str) -> bool,
}

impl Resolver for PublicResolver {
    fn resolve(
        &self,
        uri: &Uri,
        config: &ureq::config::Config,
        timeout: NextTimeout,
    ) -> std::result::Result<ResolvedSocketAddrs, ureq::Error> {
        let addresses = DefaultResolver::default().resolve(uri, config, timeout)?;
        let host = uri
            .host()
            .unwrap_or_default()
            .trim_start_matches('[')
            .trim_end_matches(']');
        for address in addresses.iter() {
            let address = address.ip().to_string();
            if !(self.is_public)(&address) {
                let refusal = HostRefusal(create_host_message(host, &address));
                return Err(ureq::Error::Io(std::io::Error::other(refusal)));
            }
        }
        Ok(addresses)
    }
}

#[derive(Debug)]
struct HostRefusal(String);

impl fmt::Display for HostRefusal {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl std::error::Error for HostRefusal {}
