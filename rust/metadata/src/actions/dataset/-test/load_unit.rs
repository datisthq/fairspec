use crate::actions::dataset::load::load_dataset_descriptor;
use crate::models::error::error::FairspecError;
use crate::models::path::PathOr;

const FIXTURES_DIR: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/actions/dataset/-test/fixtures"
);

#[test]
fn test_loads_dataset_with_table_schema_path() {
    let dataset = load_dataset_descriptor(&format!("{FIXTURES_DIR}/dataset.json")).unwrap();
    let resource = &dataset.resources.unwrap()[0];
    assert_eq!(resource.name.as_deref(), Some("name"));
    assert!(
        matches!(&resource.table_schema, Some(PathOr::Path(path)) if path.ends_with("schema.json"))
    );
}

#[test]
fn test_rejects_invalid_dataset() {
    let error =
        load_dataset_descriptor(&format!("{FIXTURES_DIR}/dataset-invalid.json")).unwrap_err();
    let report = error.report().unwrap();
    assert!(!report.valid);
    let pointers: Vec<_> = report
        .errors
        .iter()
        .map(|error| match error {
            FairspecError::Metadata(error) => error.json_pointer.as_str(),
            _ => "",
        })
        .collect();
    assert_eq!(pointers, vec!["/resources/0/name"]);
}
