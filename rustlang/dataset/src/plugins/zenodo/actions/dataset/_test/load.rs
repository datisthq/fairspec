use serde_json::json;

use crate::helpers::test_cassette::serve_cassette;
use crate::plugins::zenodo::actions::dataset::load::load_dataset_from_zenodo;
use crate::plugins::zenodo::services::zenodo::TEST_BASE_URL;

const CASSETTE: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/plugins/zenodo/actions/dataset/_test/fixtures/generated/TestLoadDatasetFromZenodo.test_loads_a_dataset.yaml"
);

#[test]
fn test_loads_a_dataset() {
    TEST_BASE_URL.set(Some(format!("{}/api", serve_cassette(CASSETTE))));

    let dataset = load_dataset_from_zenodo("https://zenodo.org/records/15525711", None).unwrap();

    assert!(!dataset["titles"].as_array().unwrap().is_empty());
    assert!(!dataset["resources"].as_array().unwrap().is_empty());
    assert_eq!(dataset["rightsList"], json!([{"rights": "cc-by-4.0"}]));
    assert_eq!(
        dataset["creators"][0],
        json!({"name": "Sukhija, Vinay Jagdish", "nameType": "Personal"})
    );
}
