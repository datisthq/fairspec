use fairspec_metadata::Dataset;
use serde_json::{Value, json};

use crate::helpers::test_server::{ReceivedRequest, serve};
use crate::plugins::ckan::actions::dataset::save::save_dataset_to_ckan;

const FIXTURES: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/plugins/ckan/actions/dataset/-test/fixtures"
);

#[test]
fn test_saves_a_dataset() {
    let (result, requests, base_url, resources) = save_dataset();

    assert_eq!(
        Value::Object(result),
        json!({
            "path": resources[1]["url"],
            "dataset_url": format!("{base_url}/dataset/test-dataset"),
        })
    );
    let request_lines: Vec<_> = requests
        .iter()
        .map(|request| request.request_line.trim())
        .collect();
    assert_eq!(
        request_lines,
        vec![
            "POST /api/3/action/package_create HTTP/1.1",
            "POST /api/3/action/resource_create HTTP/1.1",
            "POST /api/3/action/resource_create HTTP/1.1",
        ]
    );
    assert!(get_body(&requests[1]).contains("filename=\"data.csv\""));
    assert!(get_body(&requests[2]).contains("filename=\"dataset.json\""));
}

#[test]
fn test_saved_descriptor_is_found_by_load() {
    let (_, requests, _, _) = save_dataset();
    assert!(get_body(&requests[2]).contains("name=\"name\"\r\n\r\ndataset.json"));
}

fn save_dataset() -> (
    fairspec_metadata::Descriptor,
    Vec<ReceivedRequest>,
    String,
    Vec<Value>,
) {
    let package = read_response("package_create.json");
    let resources = vec![create_resource("resource-1"), create_resource("resource-2")];
    let bodies = [package, resources[0].clone(), resources[1].clone()]
        .into_iter()
        .map(|body| {
            (
                200,
                json!({"success": true, "result": body})
                    .to_string()
                    .into_bytes(),
            )
        })
        .collect();
    let (base_url, server) = serve(bodies);

    let result = save_dataset_to_ckan(
        &create_dataset(),
        "test-api-key",
        &base_url,
        "test-org",
        "test-dataset",
    )
    .unwrap();

    (result, server.join().unwrap(), base_url, resources)
}

fn create_dataset() -> Dataset {
    serde_json::from_value(json!({
        "titles": [{"title": "Test Package"}],
        "resources": [{
            "name": "test_resource",
            "data": format!("{FIXTURES}/data.csv"),
            "fileDialect": {"format": "csv"},
        }],
    }))
    .unwrap()
}

fn create_resource(resource_id: &str) -> Value {
    let mut resource = read_response("resource_create.json");
    resource["id"] = Value::from(resource_id);
    resource["url"] = Value::from(format!(
        "https://ckan.example.com/dataset/test-dataset/resource/{resource_id}/download/data"
    ));
    resource
}

fn read_response(name: &str) -> Value {
    let text = std::fs::read_to_string(format!("{FIXTURES}/responses/{name}")).unwrap();
    serde_json::from_str(&text).unwrap()
}

fn get_body(request: &ReceivedRequest) -> String {
    String::from_utf8_lossy(&request.body).into_owned()
}
