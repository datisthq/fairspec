use std::fs;

use super::parse::parse_descriptor;
use crate::actions::path::general::{get_file_protocol, get_is_remote_path};
use crate::actions::remote::agent::{create_http_agent, create_http_error};
use crate::models::descriptor::Descriptor;
use crate::models::exception::{Error, Result};
use crate::models::path::PathOr;

pub fn load_descriptor(path: &str, only_remote: bool) -> Result<Descriptor> {
    let is_remote = get_is_remote_path(path);

    if !is_remote && only_remote {
        return Err(Error::Load(
            "Cannot load descriptor for security reasons".to_string(),
        ));
    }

    if is_remote {
        return load_remote_descriptor(path);
    }
    load_local_descriptor(path)
}

pub(crate) fn load_descriptor_source(source: impl Into<PathOr<Descriptor>>) -> Result<Descriptor> {
    match source.into() {
        PathOr::Path(path) => load_descriptor(&path, false),
        PathOr::Value(descriptor) => Ok(descriptor),
    }
}

/// Fetches a remote text under the host policy; unit tests reach only the loopback host.
pub(crate) fn fetch_remote_text(url: &str) -> Result<String> {
    check_network_access(url)?;
    let mut response = create_http_agent(ureq::Agent::config_builder())?
        .get(url)
        .header("User-Agent", crate::settings::USER_AGENT)
        .call()
        .map_err(|error| create_http_error(url, error))?;
    response
        .body_mut()
        .read_to_string()
        .map_err(|error| Error::Http(format!("{url}: {error}")))
}

/// Sends a HEAD request and returns the URL reached after redirects.
pub(crate) fn resolve_remote_url(url: &str) -> Result<String> {
    use ureq::ResponseExt;

    check_network_access(url)?;
    let response = create_http_agent(ureq::Agent::config_builder())?
        .head(url)
        .header("User-Agent", crate::settings::USER_AGENT)
        .call()
        .map_err(|error| create_http_error(url, error))?;
    Ok(response.get_uri().to_string())
}

fn load_local_descriptor(path: &str) -> Result<Descriptor> {
    // The io error alone does not name the file, which Python's message does
    let text = fs::read_to_string(path)
        .map_err(|error| std::io::Error::new(error.kind(), format!("{error}: '{path}'")))?;
    parse_descriptor(&text)
}

fn load_remote_descriptor(path: &str) -> Result<Descriptor> {
    let protocol = get_file_protocol(path);
    if protocol != "http" && protocol != "https" {
        return Err(Error::Load(format!(
            "Unsupported remote protocol: {protocol}"
        )));
    }
    parse_descriptor(&fetch_remote_text(path)?)
}

#[cfg(not(test))]
fn check_network_access(_url: &str) -> Result<()> {
    Ok(())
}

#[cfg(test)]
fn check_network_access(url: &str) -> Result<()> {
    let host = url::Url::parse(url)
        .ok()
        .and_then(|url| url.host_str().map(String::from));
    match host.as_deref() {
        Some("127.0.0.1") => Ok(()),
        _ => Err(Error::Http(format!(
            "Network access is disabled in tests: {url}"
        ))),
    }
}
