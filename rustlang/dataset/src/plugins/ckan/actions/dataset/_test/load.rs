use crate::helpers::test_cassette::serve_cassette;
use crate::plugins::ckan::actions::dataset::load::load_dataset_from_ckan;

const CASSETTE: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/plugins/ckan/actions/dataset/_test/fixtures/generated/TestLoadDatasetFromCkan.test_loads_a_dataset.yaml"
);

#[test]
fn test_loads_a_dataset() {
    let base_url = serve_cassette(CASSETTE);

    let dataset = load_dataset_from_ckan(
        &format!(
            "{base_url}/dataset/referendum-on-the-thirty-seventh-amendment-of-the-constitution-bill-2018"
        ),
        None,
    )
    .unwrap();

    assert!(!dataset["titles"].as_array().unwrap().is_empty());
    let resources = dataset["resources"].as_array().unwrap();
    assert!(!resources.is_empty());
    assert!(
        resources
            .iter()
            .any(|resource| resource.get("tableSchema").is_some())
    );
}
