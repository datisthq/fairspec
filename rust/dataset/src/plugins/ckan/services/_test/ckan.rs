use fairspec_metadata::{Descriptor, Result};
use serde_json::{Value, json};

use crate::helpers::test_server::serve;
use crate::plugins::ckan::models::schema::CkanSchema;
use crate::plugins::ckan::services::ckan::make_ckan_api_request;

fn request_schema(body: Value, action: &str) -> Result<CkanSchema> {
    let (base_url, _server) = serve(vec![(200, body.to_string().into_bytes())]);
    let payload: Descriptor = serde_json::from_value(json!({"resource_id": "1"})).unwrap();
    make_ckan_api_request(
        &format!("{base_url}/dataset/test"),
        action,
        &payload,
        None,
        None,
    )
}

#[test]
fn test_returns_the_result_parsed_by_the_model() {
    let result = json!({"fields": [{"id": "name", "type": "text"}], "records": []});
    let schema = request_schema(
        json!({"success": true, "result": result}),
        "datastore_search",
    )
    .unwrap();

    assert_eq!(schema.fields.len(), 1);
    assert_eq!(schema.fields[0].id, "name");
    assert_eq!(schema.fields[0].r#type, "text");
}

#[test]
fn test_rejects_a_result_that_does_not_match_the_model() {
    let body = json!({"success": true, "result": {"fields": [{"id": "name"}]}});
    let error = request_schema(body, "datastore_search")
        .unwrap_err()
        .to_string();

    assert!(error.starts_with("CKAN API returned an unexpected response from datastore_search"));
    assert!(
        error.contains("fields[0]") && error.contains("type"),
        "{error}"
    );
}

#[test]
fn test_reports_the_error_of_an_unsuccessful_response() {
    let error = json!({"message": "Not found", "__type": "Not Found Error"});
    let body = json!({"success": false, "error": error});
    let error = request_schema(body, "package_show")
        .unwrap_err()
        .to_string();

    assert!(error.contains("CKAN API error: Not found"), "{error}");
}
