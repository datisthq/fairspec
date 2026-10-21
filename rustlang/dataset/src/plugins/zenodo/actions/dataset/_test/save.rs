use fairspec_metadata::Dataset;
use serde_json::{Value, json};

use crate::helpers::test_server::serve;
use crate::plugins::zenodo::actions::dataset::save::save_dataset_to_zenodo;
use crate::plugins::zenodo::services::zenodo::TEST_BASE_URL;

const FIXTURES: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/plugins/zenodo/actions/dataset/_test/fixtures"
);

#[test]
fn test_saves_a_dataset() {
    let deposition = read_response("deposition.json");
    let file = read_response("file.json");
    let (base_url, server) = serve(vec![(200, deposition), (200, file.clone()), (200, file)]);
    TEST_BASE_URL.set(Some(format!("{base_url}/api")));

    let result = save_dataset_to_zenodo(&create_dataset(), "test-api-key", true).unwrap();

    assert_eq!(
        Value::Object(result),
        json!({
            "path": "https://sandbox.zenodo.org/records/12345/files/dataset.json",
            "dataset_url": "https://sandbox.zenodo.org/uploads/12345",
        })
    );
    let requests = server.join().unwrap();
    let request_lines: Vec<_> = requests
        .iter()
        .map(|request| request.request_line.trim())
        .collect();
    assert_eq!(
        request_lines,
        vec![
            "POST /api/deposit/depositions?access_token=test-api-key HTTP/1.1",
            "POST /api/deposit/depositions/12345/files?access_token=test-api-key HTTP/1.1",
            "POST /api/deposit/depositions/12345/files?access_token=test-api-key HTTP/1.1",
        ]
    );
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

fn read_response(name: &str) -> Vec<u8> {
    std::fs::read(format!("{FIXTURES}/responses/{name}")).unwrap()
}
