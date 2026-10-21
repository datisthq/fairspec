use serde_json::json;

use crate::actions::file_dialect::support::get_supported_file_dialect;
use crate::models::file_dialect::file_dialect::FileDialect;
use crate::models::resource::Resource;

fn create_resource(descriptor: serde_json::Value) -> Resource {
    serde_json::from_value(descriptor).unwrap()
}

#[test]
fn test_infers_a_supported_format_from_the_path() {
    let resource = create_resource(json!({"data": "table.csv"}));
    let dialect = get_supported_file_dialect(&resource, &["csv", "tsv"]).unwrap();
    assert!(matches!(dialect, Some(FileDialect::Csv(_))));
}

#[test]
fn test_prefers_the_declared_dialect() {
    let resource = create_resource(json!({"data": "table.txt", "fileDialect": {"format": "tsv"}}));
    let dialect = get_supported_file_dialect(&resource, &["tsv"]).unwrap();
    assert!(matches!(dialect, Some(FileDialect::Tsv(_))));
}

#[test]
fn test_returns_none_for_an_unsupported_format() {
    let resource = create_resource(json!({"data": "table.xlsx"}));
    assert_eq!(
        get_supported_file_dialect(&resource, &["csv"]).unwrap(),
        None
    );
}

#[test]
fn test_returns_none_for_inline_data() {
    let resource = create_resource(json!({"data": [{"id": 1}], "fileDialect": {"format": "csv"}}));
    assert_eq!(
        get_supported_file_dialect(&resource, &["csv"]).unwrap(),
        None
    );
}
