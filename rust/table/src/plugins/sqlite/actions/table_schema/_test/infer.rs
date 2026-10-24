use fairspec_dataset::get_temp_file_path;
use fairspec_metadata::Resource;
use serde_json::json;

use crate::plugins::sqlite::actions::table_schema::infer::infer_table_schema_from_sqlite;

fn create_resource(path: Option<&str>) -> Resource {
    let mut resource = json!({"fileDialect": {"format": "sqlite", "tableName": "fairspec"}});
    if let Some(path) = path {
        resource["data"] = json!(path);
    }
    serde_json::from_value(resource).unwrap()
}

#[test]
fn test_should_infer_schema() {
    // The table save_sqlite_table writes for string, integer and number columns
    let path = get_temp_file_path(false, None, None).unwrap();
    let connection = rusqlite::Connection::open(&path).unwrap();
    connection
        .execute_batch(
            "CREATE TABLE \"fairspec\" (\"string\" text, \"integer\" integer, \"number\" real);\
             INSERT INTO \"fairspec\" VALUES ('string', 1, 1.1);",
        )
        .unwrap();
    drop(connection);

    let schema = infer_table_schema_from_sqlite(&create_resource(Some(&path))).unwrap();

    assert_eq!(schema.required, Some(vec![]));
    assert_eq!(schema.primary_key, None);
    assert_eq!(
        serde_json::to_value(schema.properties).unwrap(),
        json!({
            "string": {"type": ["string", "null"]},
            "integer": {"type": ["integer", "null"]},
            "number": {"type": ["number", "null"]},
        })
    );
}

#[test]
fn test_should_raise_error_when_resource_path_is_not_defined() {
    let error = infer_table_schema_from_sqlite(&create_resource(None)).unwrap_err();
    assert!(error.to_string().contains("Database is not defined"));
}
