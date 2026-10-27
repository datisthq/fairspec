use fairspec_metadata::Resource;
use serde_json::json;

use crate::plugins::sqlite::actions::table::load::load_sqlite_table;

#[test]
fn test_should_raise_error_when_resource_path_is_not_defined() {
    let resource: Resource = serde_json::from_value(
        json!({"fileDialect": {"format": "sqlite", "tableName": "fairspec"}}),
    )
    .unwrap();
    let Err(error) = load_sqlite_table(&resource, &Default::default()) else {
        panic!("expected an error");
    };
    assert!(error.to_string().contains("Resource path is not defined"));
}
