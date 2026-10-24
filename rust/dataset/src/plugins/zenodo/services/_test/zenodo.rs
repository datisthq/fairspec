use fairspec_metadata::Result;
use serde_json::{Value, json};

use crate::helpers::test_server::serve;
use crate::plugins::zenodo::models::deposition::ZenodoDeposition;
use crate::plugins::zenodo::services::zenodo::{TEST_BASE_URL, make_zenodo_api_request};

fn request_deposition(body: Value) -> Result<ZenodoDeposition> {
    let (base_url, _server) = serve(vec![(200, body.to_string().into_bytes())]);
    TEST_BASE_URL.set(Some(base_url));
    make_zenodo_api_request("/deposit/depositions/1", None, None, None, None, false)
}

#[test]
fn test_returns_the_response_parsed_by_the_model() {
    let body = json!({"id": 1, "links": {"html": "https://zenodo.org/deposit/1"}, "x": 1});
    let deposition = request_deposition(body).unwrap();

    assert_eq!(deposition.id, 1);
    assert_eq!(deposition.links.html, "https://zenodo.org/deposit/1");
}

#[test]
fn test_rejects_a_response_that_does_not_match_the_model() {
    let error = request_deposition(json!({"id": 1, "links": {}}))
        .unwrap_err()
        .to_string();

    assert!(
        error.starts_with("Zenodo API returned an unexpected response from /deposit/depositions/1")
    );
    assert!(error.contains("links") && error.contains("html"), "{error}");
}
