use serde_json::json;

use crate::helpers::test_cassette::serve_cassette;
use crate::plugins::github::actions::dataset::load::load_dataset_from_github;
use crate::plugins::github::services::github::TEST_BASE_URL;

const CASSETTE: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/plugins/github/actions/dataset/_test/fixtures/generated/TestLoadDatasetFromGithub.test_loads_a_dataset.yaml"
);

#[test]
fn test_loads_a_dataset() {
    TEST_BASE_URL.set(Some(serve_cassette(CASSETTE)));

    let dataset = load_dataset_from_github("https://github.com/roll/data", None).unwrap();

    assert_eq!(dataset["titles"], json!([{"title": "roll/testdata"}]));
    let resources = dataset["resources"].as_array().unwrap();
    assert!(!resources.is_empty());
    for resource in resources {
        let data = resource["data"].as_str().unwrap();
        assert!(data.starts_with("https://raw.githubusercontent.com/roll/testdata/"));
    }
}
