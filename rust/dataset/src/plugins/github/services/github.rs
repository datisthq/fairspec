use fairspec_metadata::{Descriptor, Error, Result};
use serde::de::DeserializeOwned;

use crate::helpers::http::{HttpRequest, send_http_request};

pub fn make_github_api_request<M: DeserializeOwned>(
    endpoint: &str,
    method: Option<&str>,
    payload: Option<&Descriptor>,
    api_key: Option<&str>,
) -> Result<M> {
    let url = format!("{}{endpoint}", get_base_url());

    let mut headers = Vec::new();
    if let Some(api_key) = api_key.filter(|api_key| !api_key.is_empty()) {
        headers.push(("Authorization".to_string(), format!("Bearer {api_key}")));
    }

    let mut body = None;
    if let Some(payload) = payload.filter(|payload| !payload.is_empty()) {
        headers.push(("Content-Type".to_string(), "application/json".to_string()));
        body = Some(serde_json::to_vec(payload).map_err(|error| Error::Parse(error.to_string()))?);
    }

    let response = send_http_request(&HttpRequest {
        method: method.unwrap_or("GET").to_string(),
        url: url.clone(),
        headers,
        body,
    })?;
    if !(200..300).contains(&response.status) {
        return Err(Error::Http(format!(
            "HTTP Error {}: {url}",
            response.status
        )));
    }

    let deserializer = &mut serde_json::Deserializer::from_slice(&response.body);
    serde_path_to_error::deserialize(deserializer).map_err(|error| {
        Error::Parse(format!(
            "Github API returned an unexpected response from {endpoint}:\n{error}"
        ))
    })
}

#[cfg(not(test))]
fn get_base_url() -> String {
    BASE_URL.to_string()
}

#[cfg(test)]
fn get_base_url() -> String {
    TEST_BASE_URL
        .with_borrow(Clone::clone)
        .unwrap_or_else(|| BASE_URL.to_string())
}

const BASE_URL: &str = "https://api.github.com";

#[cfg(test)]
thread_local! {
    /// Points requests at a local test server instead of api.github.com.
    pub(crate) static TEST_BASE_URL: std::cell::RefCell<Option<String>> =
        const { std::cell::RefCell::new(None) };
}
