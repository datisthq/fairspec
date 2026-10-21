use fairspec_metadata::{Descriptor, Error, Result};
use serde::Deserialize;
use serde::de::DeserializeOwned;
use serde_json::Value;

use crate::helpers::http::{HttpRequest, send_http_request};

const BOUNDARY: &str = "----FormBoundary7MA4YWxkTrZu0gW";

pub fn make_ckan_api_request<M: DeserializeOwned>(
    ckan_url: &str,
    action: &str,
    payload: &Descriptor,
    upload: Option<(&str, &[u8])>,
    api_key: Option<&str>,
) -> Result<M> {
    let url = format!("{}/api/3/action/{action}", get_url_origin(ckan_url)?);

    let mut headers = Vec::new();
    if let Some(api_key) = api_key.filter(|api_key| !api_key.is_empty()) {
        headers.push(("Authorization".to_string(), api_key.to_string()));
    }

    let body = match upload {
        Some((file_name, file_data)) => {
            headers.push((
                "Content-Type".to_string(),
                format!("multipart/form-data; boundary={BOUNDARY}"),
            ));
            create_multipart_body(payload, file_name, file_data)
        }
        None => {
            headers.push(("Content-Type".to_string(), "application/json".to_string()));
            serde_json::to_vec(payload).map_err(|error| Error::Parse(error.to_string()))?
        }
    };

    let response = send_http_request(&HttpRequest {
        method: "POST".to_string(),
        url: url.clone(),
        headers,
        body: Some(body),
    })?;
    if !(200..300).contains(&response.status) {
        return Err(Error::Http(format!(
            "HTTP Error {}: {url}",
            response.status
        )));
    }

    let envelope: CkanResponse = parse_response(action, &response.body)?;

    if !envelope.success {
        let message = envelope
            .error
            .as_ref()
            .and_then(|error| error.get("message"))
            .and_then(Value::as_str)
            .filter(|message| !message.is_empty())
            .map(String::from)
            .unwrap_or_else(|| serde_json::to_string(&envelope.error).unwrap_or_default());
        return Err(Error::Http(format!("CKAN API error: {message}")));
    }

    let result = serde_json::to_vec(&envelope.result).unwrap_or_default();
    parse_response(action, &result)
}

/// The scheme and authority of a URL, like Python's `{scheme}://{netloc}`.
pub(crate) fn get_url_origin(url: &str) -> Result<String> {
    let parsed = url::Url::parse(url).map_err(|error| Error::Parse(format!("{url}: {error}")))?;
    Ok(parsed[..url::Position::BeforePath].to_string())
}

#[derive(Deserialize)]
struct CkanResponse {
    success: bool,
    #[serde(default)]
    result: Value,
    #[serde(default)]
    error: Option<Descriptor>,
}

fn parse_response<M: DeserializeOwned>(action: &str, body: &[u8]) -> Result<M> {
    let deserializer = &mut serde_json::Deserializer::from_slice(body);
    serde_path_to_error::deserialize(deserializer).map_err(|error| {
        Error::Parse(format!(
            "CKAN API returned an unexpected response from {action}:\n{error}"
        ))
    })
}

fn create_multipart_body(payload: &Descriptor, file_name: &str, file_data: &[u8]) -> Vec<u8> {
    let mut body = Vec::new();
    for (key, value) in payload {
        let value = match value {
            Value::String(value) => value.clone(),
            value => value.to_string(),
        };
        body.extend(format!("--{BOUNDARY}\r\n").as_bytes());
        body.extend(format!("Content-Disposition: form-data; name=\"{key}\"\r\n\r\n").as_bytes());
        body.extend(format!("{value}\r\n").as_bytes());
    }
    body.extend(format!("--{BOUNDARY}\r\n").as_bytes());
    body.extend(
        format!("Content-Disposition: form-data; name=\"upload\"; filename=\"{file_name}\"\r\n")
            .as_bytes(),
    );
    body.extend(b"Content-Type: application/octet-stream\r\n\r\n");
    body.extend(file_data);
    body.extend(format!("\r\n--{BOUNDARY}--\r\n").as_bytes());
    body
}

/// Python's `model_dump(by_alias=True, exclude_none=True)` for a model that is a JSON object.
pub(crate) fn dump_descriptor(model: &impl serde::Serialize) -> Result<Descriptor> {
    match serde_json::to_value(model) {
        Ok(Value::Object(descriptor)) => Ok(descriptor),
        Ok(_) => Err(Error::Parse("Model is not an object".to_string())),
        Err(error) => Err(Error::Parse(error.to_string())),
    }
}
