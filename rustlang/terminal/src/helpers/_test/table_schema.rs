use indexmap::IndexMap;
use serde_json::json;

use crate::helpers::table_schema::create_infer_table_schema_options;
use crate::params::file_dialect::StringList;
use crate::params::table_schema::{
    ColumnTypes, TableSchemaParams, parse_column_properties, parse_column_types,
};

#[test]
fn test_should_leave_unset_options_out() {
    let options = create_infer_table_schema_options(&TableSchemaParams::default());
    assert_eq!(serde_json::to_value(options).unwrap(), json!({}));
}

#[test]
fn test_should_pass_option_values() {
    let params = TableSchemaParams {
        sample_rows: Some(10),
        keep_strings: true,
        column_types: Some(ColumnTypes(IndexMap::from([
            ("id".to_string(), "integer".to_string()),
            ("name".to_string(), "string".to_string()),
        ]))),
        missing_values: Some(StringList(vec!["NA".to_string(), "-".to_string()])),
        array_type: Some("list".to_string()),
        list_item_type: Some("integer".to_string()),
        ..TableSchemaParams::default()
    };
    let options = create_infer_table_schema_options(&params);
    let mut expected = json!({
        "sampleRows": 10,
        "keepStrings": true,
        "columnTypes": {"id": "integer", "name": "string"},
        "missingValues": ["NA", "-"],
        "arrayType": "list",
        "listItemType": "integer",
    });
    let mut actual = serde_json::to_value(options).unwrap();
    expected.as_object_mut().unwrap().sort_keys();
    actual.as_object_mut().unwrap().sort_keys();
    assert_eq!(actual, expected);
}

#[test]
fn test_should_parse_name_type_pairs() {
    let types = parse_column_types("id:integer,name:string").unwrap();
    assert_eq!(
        serde_json::to_value(types.0).unwrap(),
        json!({"id": "integer", "name": "string"})
    );
}

#[test]
fn test_should_parse_json_object() {
    let properties =
        parse_column_properties(r#"{"id": {"type": "string"}, "name": {"title": "Name"}}"#)
            .unwrap();
    assert_eq!(
        serde_json::to_value(properties.0).unwrap(),
        json!({"id": {"type": "string"}, "name": {"title": "Name"}})
    );
}

#[test]
fn test_should_reject_json_that_is_not_an_object() {
    let error = parse_column_properties("[1, 2]").unwrap_err();
    assert!(error.contains("must be a JSON object"));
}
