use std::io::Read;

use fairspec_metadata::{Error, Result};
use ureq::ResponseExt;

const USER_AGENT: &str = "fairspec-rust";

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct HttpRequest {
    pub method: String,
    pub url: String,
    pub headers: Vec<(String, String)>,
    pub body: Option<Vec<u8>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct HttpResponse {
    pub status: u16,
    pub url: String,
    pub body: Vec<u8>,
}

/// Sends a request with the `fairspec-rust` user agent; non-2xx statuses are returned, not raised.
pub fn send_http_request(request: &HttpRequest) -> Result<HttpResponse> {
    check_network_access(&request.url)?;
    let mut builder = ureq::http::Request::builder()
        .method(request.method.as_str())
        .uri(request.url.as_str())
        .header("User-Agent", USER_AGENT);
    for (name, value) in &request.headers {
        builder = builder.header(name.as_str(), value.as_str());
    }
    let http_request = builder
        .body(request.body.clone().unwrap_or_default())
        .map_err(|error| create_error(&request.url, error))?;

    let agent: ureq::Agent = ureq::Agent::config_builder()
        .http_status_as_error(false)
        .build()
        .into();
    let response = agent
        .run(http_request)
        .map_err(|error| create_error(&request.url, error))?;
    let status = response.status().as_u16();
    let url = response.get_uri().to_string();
    let mut body = Vec::new();
    response
        .into_body()
        .into_reader()
        .read_to_end(&mut body)
        .map_err(|error| create_error(&request.url, error))?;
    Ok(HttpResponse { status, url, body })
}

/// Streams a remote file, reading at most `max_bytes` when given.
pub fn load_remote_stream(url: &str, max_bytes: Option<usize>) -> Result<Box<dyn Read + Send>> {
    check_network_access(url)?;
    let response = ureq::get(url)
        .header("User-Agent", USER_AGENT)
        .call()
        .map_err(|error| create_error(url, error))?;
    let reader = response.into_body().into_reader();
    Ok(match max_bytes {
        Some(max_bytes) => Box::new(reader.take(max_bytes as u64)),
        None => Box::new(reader),
    })
}

fn create_error(url: &str, error: impl std::fmt::Display) -> Error {
    Error::Http(format!("{url}: {error}"))
}

#[cfg(not(test))]
fn check_network_access(_url: &str) -> Result<()> {
    Ok(())
}

// Unit tests reach only the loopback host, so they never touch the network
#[cfg(test)]
fn check_network_access(url: &str) -> Result<()> {
    let host = url
        .split_once("://")
        .map(|(_, rest)| rest.split(['/', '?', '#']).next().unwrap_or_default())
        .and_then(|authority| authority.rsplit('@').next())
        .map(|host| host.split(':').next().unwrap_or_default());
    match host {
        Some("127.0.0.1") => Ok(()),
        _ => Err(Error::Http(format!(
            "Network access is disabled in tests: {url}"
        ))),
    }
}
