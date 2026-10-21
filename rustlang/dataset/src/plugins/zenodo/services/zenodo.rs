use fairspec_metadata::{Descriptor, Error, Result};
use serde::de::DeserializeOwned;

use crate::helpers::http::{HttpRequest, send_http_request};

const BOUNDARY: &str = "----FormBoundary7MA4YWxkTrZu0gW";

pub fn make_zenodo_api_request<M: DeserializeOwned>(
    endpoint: &str,
    method: Option<&str>,
    payload: Option<&Descriptor>,
    upload: Option<(&str, &[u8])>,
    api_key: Option<&str>,
    sandbox: bool,
) -> Result<M> {
    let mut url = format!("{}{endpoint}", get_base_url(sandbox));

    if let Some(api_key) = api_key.filter(|api_key| !api_key.is_empty()) {
        let separator = if url.contains('?') { "&" } else { "?" };
        url = format!("{url}{separator}access_token={api_key}");
    }

    let mut headers = Vec::new();
    let mut body = None;

    if let Some((file_name, file_data)) = upload {
        headers.push((
            "Content-Type".to_string(),
            format!("multipart/form-data; boundary={BOUNDARY}"),
        ));
        let mut data = Vec::new();
        data.extend(format!("--{BOUNDARY}\r\n").as_bytes());
        data.extend(
            format!("Content-Disposition: form-data; name=\"file\"; filename=\"{file_name}\"\r\n")
                .as_bytes(),
        );
        data.extend(b"Content-Type: application/octet-stream\r\n\r\n");
        data.extend(file_data);
        data.extend(format!("\r\n--{BOUNDARY}--\r\n").as_bytes());
        body = Some(data);
    } else if let Some(payload) = payload {
        headers.push(("Content-Type".to_string(), "application/json".to_string()));
        body = Some(serde_json::to_vec(payload).map_err(|error| Error::Parse(error.to_string()))?);
    }

    let response = send_http_request(&HttpRequest {
        method: method.unwrap_or("GET").to_string(),
        url,
        headers,
        body,
    })?;
    if !(200..300).contains(&response.status) {
        return Err(Error::Http(format!(
            "HTTP Error {}: {endpoint}",
            response.status
        )));
    }

    let body: &[u8] = if response.body.is_empty() {
        b"{}"
    } else {
        &response.body
    };
    let deserializer = &mut serde_json::Deserializer::from_slice(body);
    serde_path_to_error::deserialize(deserializer).map_err(|error| {
        Error::Parse(format!(
            "Zenodo API returned an unexpected response from {endpoint}:\n{error}"
        ))
    })
}

#[cfg(not(test))]
fn get_base_url(sandbox: bool) -> String {
    create_base_url(sandbox)
}

#[cfg(test)]
fn get_base_url(sandbox: bool) -> String {
    TEST_BASE_URL
        .with_borrow(Clone::clone)
        .unwrap_or_else(|| create_base_url(sandbox))
}

fn create_base_url(sandbox: bool) -> String {
    match sandbox {
        true => "https://sandbox.zenodo.org/api".to_string(),
        false => "https://zenodo.org/api".to_string(),
    }
}

#[cfg(test)]
thread_local! {
    /// Points requests at a local test server instead of zenodo.org.
    pub(crate) static TEST_BASE_URL: std::cell::RefCell<Option<String>> =
        const { std::cell::RefCell::new(None) };
}
