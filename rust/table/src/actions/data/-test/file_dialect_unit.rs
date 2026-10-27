use fairspec_metadata::FileDialect;
use serde_json::{Value, json};

use crate::actions::data::file_dialect::{get_header_labels_from_rows, get_records_from_rows};
use crate::models::data::DataRow;

fn get_records(rows: Value, dialect: Option<Value>) -> Value {
    serde_json::to_value(try_get_records(rows, dialect).unwrap()).unwrap()
}

fn try_get_records(
    rows: Value,
    dialect: Option<Value>,
) -> fairspec_metadata::Result<Vec<crate::models::data::DataRecord>> {
    let rows: Vec<DataRow> = serde_json::from_value(rows).unwrap();
    let dialect: Option<FileDialect> = dialect.map(|mut dialect| {
        dialect["format"] = json!("csv");
        serde_json::from_value(dialect).unwrap()
    });
    get_records_from_rows(&rows, dialect.as_ref())
}

#[test]
fn test_convert_rows_to_records_with_default_header() {
    let rows = json!([
        ["name", "age", "city"],
        ["Alice", 30, "NYC"],
        ["Bob", 25, "LA"]
    ]);
    assert_eq!(
        get_records(rows, None),
        json!([
            {"name": "Alice", "age": 30, "city": "NYC"},
            {"name": "Bob", "age": 25, "city": "LA"},
        ])
    );
}

#[test]
fn test_single_row_with_header() {
    assert_eq!(
        get_records(json!([["name", "age", "city"]]), None),
        json!([])
    );
}

#[test]
fn test_empty_rows() {
    assert!(try_get_records(json!([]), None).is_err());
}

#[test]
fn test_empty_rows_without_header() {
    let dialect = json!({"headerRows": false});
    assert_eq!(get_records(json!([]), Some(dialect)), json!([]));
}

#[test]
fn test_rows_between_header_rows_are_dropped() {
    let rows = json!([["a", "b"], ["skip", "me"], ["c", "d"], [1, 2]]);
    assert_eq!(
        get_records(rows, Some(json!({"headerRows": [1, 3]}))),
        json!([{"a c": 1, "b d": 2}])
    );
}

#[test]
fn test_repeated_header_name() {
    assert!(try_get_records(json!([["a", "a"], [1, 2]]), None).is_err());
}

#[test]
fn test_empty_header_cell_is_an_empty_name() {
    assert_eq!(
        get_records(json!([["a", null], [1, 2]]), None),
        json!([{"a": 1, "": 2}])
    );
}

#[test]
fn test_rows_without_header_when_header_is_false() {
    let rows = json!([["Alice", 30, "NYC"], ["Bob", 25, "LA"]]);
    assert_eq!(
        get_records(rows, Some(json!({"headerRows": false}))),
        json!([
            {"column1": "Alice", "column2": 30, "column3": "NYC"},
            {"column1": "Bob", "column2": 25, "column3": "LA"},
        ])
    );
}

#[test]
fn test_custom_header_rows() {
    let rows = json!([
        ["skip this row"],
        ["name", "age", "city"],
        ["Alice", 30, "NYC"],
        ["Bob", 25, "LA"],
    ]);
    assert_eq!(
        get_records(rows, Some(json!({"headerRows": [2]}))),
        json!([
            {"name": "Alice", "age": 30, "city": "NYC"},
            {"name": "Bob", "age": 25, "city": "LA"},
        ])
    );
}

#[test]
fn test_multiple_header_rows_with_default_join() {
    let rows = json!([
        ["first", "last", "contact"],
        ["name", "name", "email"],
        ["Alice", "Smith", "alice@example.com"],
        ["Bob", "Jones", "bob@example.com"],
    ]);
    assert_eq!(
        get_records(rows, Some(json!({"headerRows": [1, 2]}))),
        json!([
            {"first name": "Alice", "last name": "Smith", "contact email": "alice@example.com"},
            {"first name": "Bob", "last name": "Jones", "contact email": "bob@example.com"},
        ])
    );
}

#[test]
fn test_multiple_header_rows_with_custom_join() {
    let rows = json!([
        ["user", "user", "meta"],
        ["first", "last", "created"],
        ["Alice", "Smith", "2023-01-01"],
        ["Bob", "Jones", "2023-01-02"],
    ]);
    assert_eq!(
        get_records(rows, Some(json!({"headerRows": [1, 2], "headerJoin": "_"}))),
        json!([
            {"user_first": "Alice", "user_last": "Smith", "meta_created": "2023-01-01"},
            {"user_first": "Bob", "user_last": "Jones", "meta_created": "2023-01-02"},
        ])
    );
}

#[test]
fn test_skip_comment_rows_by_row_number() {
    let rows = json!([
        ["name", "age", "city"],
        ["Alice", 30, "NYC"],
        ["# Comment row", "ignored", "data"],
        ["Bob", 25, "LA"],
    ]);
    assert_eq!(
        get_records(rows, Some(json!({"commentRows": [3]}))),
        json!([
            {"name": "Alice", "age": 30, "city": "NYC"},
            {"name": "Bob", "age": 25, "city": "LA"},
        ])
    );
}

#[test]
fn test_skip_rows_with_comment_character() {
    let rows = json!([
        ["name", "age", "city"],
        ["Alice", 30, "NYC"],
        ["# Comment", "ignored", "data"],
        ["Bob", 25, "LA"],
        ["Regular row", "data", "value"],
    ]);
    assert_eq!(
        get_records(rows, Some(json!({"commentPrefix": "#"}))),
        json!([
            {"name": "Alice", "age": 30, "city": "NYC"},
            {"name": "Bob", "age": 25, "city": "LA"},
            {"name": "Regular row", "age": "data", "city": "value"},
        ])
    );
}

#[test]
fn test_skip_rows_with_multiple_comment_characters() {
    let rows = json!([
        ["name", "age", "city"],
        ["Alice", 30, "NYC"],
        ["# Comment 1", "ignored", "data"],
        ["Bob", 25, "LA"],
        ["## Comment 2", "ignored", "data"],
    ]);
    assert_eq!(
        get_records(rows, Some(json!({"commentPrefix": "#"}))),
        json!([
            {"name": "Alice", "age": 30, "city": "NYC"},
            {"name": "Bob", "age": 25, "city": "LA"},
        ])
    );
}

#[test]
fn test_not_skip_rows_when_first_cell_is_not_string() {
    let rows = json!([
        ["name", "age", "city"],
        ["Alice", 30, "NYC"],
        [123, "data", "test"],
        ["Bob", 25, "LA"],
    ]);
    assert_eq!(
        get_records(rows, Some(json!({"commentPrefix": "#"}))),
        json!([
            {"name": "Alice", "age": 30, "city": "NYC"},
            {"name": 123, "age": "data", "city": "test"},
            {"name": "Bob", "age": 25, "city": "LA"},
        ])
    );
}

#[test]
fn test_rows_with_different_lengths() {
    let rows = json!([
        ["name", "age", "city", "country"],
        ["Alice", 30, "NYC"],
        ["Bob", 25, "LA", "USA"],
        ["Charlie"],
    ]);
    assert_eq!(
        get_records(rows, None),
        json!([
            {"name": "Alice", "age": 30, "city": "NYC", "country": null},
            {"name": "Bob", "age": 25, "city": "LA", "country": "USA"},
            {"name": "Charlie", "age": null, "city": null, "country": null},
        ])
    );
}

#[test]
fn test_null_values() {
    let rows = json!([
        ["name", "age", "city"],
        ["Alice", null, null],
        [null, 25, "LA"]
    ]);
    assert_eq!(
        get_records(rows, None),
        json!([
            {"name": "Alice", "age": null, "city": null},
            {"name": null, "age": 25, "city": "LA"},
        ])
    );
}

#[test]
fn test_boolean_and_number_types() {
    let rows = json!([
        ["name", "active", "count"],
        ["Alice", true, 100],
        ["Bob", false, 0]
    ]);
    assert_eq!(
        get_records(rows, None),
        json!([
            {"name": "Alice", "active": true, "count": 100},
            {"name": "Bob", "active": false, "count": 0},
        ])
    );
}

#[test]
fn test_convert_header_values_to_strings() {
    let rows = json!([[1, 2, 3], ["Alice", 30, "NYC"], ["Bob", 25, "LA"]]);
    assert_eq!(
        get_records(rows, None),
        json!([
            {"1": "Alice", "2": 30, "3": "NYC"},
            {"1": "Bob", "2": 25, "3": "LA"},
        ])
    );
}

#[test]
fn test_empty_header_cells() {
    let rows = json!([
        ["name", "", "city"],
        ["Alice", 30, "NYC"],
        ["Bob", 25, "LA"]
    ]);
    assert_eq!(
        get_records(rows, None),
        json!([
            {"name": "Alice", "": 30, "city": "NYC"},
            {"name": "Bob", "": 25, "city": "LA"},
        ])
    );
}

#[test]
fn test_multi_row_headers_with_empty_cells() {
    let rows = json!([
        ["person", "", "location"],
        ["first", "last", "city"],
        ["Alice", "Smith", "NYC"],
    ]);
    assert_eq!(
        get_records(rows, Some(json!({"headerRows": [1, 2]}))),
        json!([{"person first": "Alice", "person last": "Smith", "location city": "NYC"}])
    );
}

#[test]
fn test_combination_of_header_rows_and_comment_rows() {
    let rows = json!([
        ["skip row 1"],
        ["name", "age", "city"],
        ["# Comment", "data", "data"],
        ["Alice", 30, "NYC"],
        ["Bob", 25, "LA"],
    ]);
    assert_eq!(
        get_records(rows, Some(json!({"headerRows": [2], "commentRows": [3]}))),
        json!([
            {"name": "Alice", "age": 30, "city": "NYC"},
            {"name": "Bob", "age": 25, "city": "LA"},
        ])
    );
}

#[test]
fn test_combination_of_comment_rows_and_comment_prefix() {
    let rows = json!([
        ["name", "age", "city"],
        ["# Inline comment", "data", "data"],
        ["Alice", 30, "NYC"],
        ["Comment by row number", "data", "data"],
        ["Bob", 25, "LA"],
    ]);
    assert_eq!(
        get_records(
            rows,
            Some(json!({"commentRows": [4], "commentPrefix": "#"}))
        ),
        json!([
            {"name": "Alice", "age": 30, "city": "NYC"},
            {"name": "Bob", "age": 25, "city": "LA"},
        ])
    );
}

#[test]
fn test_generate_column_names_based_on_longest_row_when_no_header() {
    let rows = json!([
        ["Alice", 30],
        ["Bob", 25, "LA", "USA"],
        ["Charlie", 35, "SF"]
    ]);
    assert_eq!(
        get_records(rows, Some(json!({"headerRows": false}))),
        json!([
            {"column1": "Alice", "column2": 30, "column3": null, "column4": null},
            {"column1": "Bob", "column2": 25, "column3": "LA", "column4": "USA"},
            {"column1": "Charlie", "column2": 35, "column3": "SF", "column4": null},
        ])
    );
}

#[test]
fn test_use_column_names_when_provided_with_header_rows_false() {
    let rows = json!([["Alice", 30, "NYC"], ["Bob", 25, "LA"]]);
    let dialect = json!({"headerRows": false, "columnNames": ["name", "age", "city"]});
    assert_eq!(
        get_records(rows, Some(dialect)),
        json!([
            {"name": "Alice", "age": 30, "city": "NYC"},
            {"name": "Bob", "age": 25, "city": "LA"},
        ])
    );
}

#[test]
fn test_use_column_names_even_when_file_has_headers() {
    let rows = json!([
        ["firstName", "years", "location"],
        ["Alice", 30, "NYC"],
        ["Bob", 25, "LA"]
    ]);
    assert_eq!(
        get_records(rows, Some(json!({"columnNames": ["name", "age", "city"]}))),
        json!([
            {"name": "Alice", "age": 30, "city": "NYC"},
            {"name": "Bob", "age": 25, "city": "LA"},
        ])
    );
}

#[test]
fn test_column_names_with_rows_longer_than_column_names_array() {
    let rows = json!([["Alice", 30, "NYC", "USA"], ["Bob", 25, "LA", "USA"]]);
    let dialect = json!({"headerRows": false, "columnNames": ["name", "age"]});
    assert!(try_get_records(rows, Some(dialect)).is_err());
}

#[test]
fn test_column_names_with_rows_shorter_than_column_names_array() {
    let rows = json!([["Alice", 30], ["Bob"]]);
    let dialect = json!({"headerRows": false, "columnNames": ["name", "age", "city", "country"]});
    assert_eq!(
        get_records(rows, Some(dialect)),
        json!([
            {"name": "Alice", "age": 30, "city": null, "country": null},
            {"name": "Bob", "age": null, "city": null, "country": null},
        ])
    );
}

#[test]
fn test_row_longer_than_the_header_is_a_long_row_error() {
    let rows = json!([["id", "name"], [1, "apple", "extra"]]);
    let error = try_get_records(rows, None).unwrap_err();
    assert!(crate::helpers::file_dialect::get_is_long_row_error(&error));
}

#[test]
fn test_header_labels_are_the_own_header_whatever_column_names_says() {
    let rows: Vec<DataRow> = serde_json::from_value(json!([["a", "b"], [1, 2]])).unwrap();
    let dialect: FileDialect =
        serde_json::from_value(json!({"format": "csv", "columnNames": ["id", "name"]})).unwrap();
    let labels = get_header_labels_from_rows(&rows, Some(&dialect)).unwrap();
    assert_eq!(labels, Some(vec!["a".to_string(), "b".to_string()]));
}

#[test]
fn test_header_labels_are_none_when_the_rows_end_before_the_header_rows() {
    assert_eq!(
        get_header_labels_from_rows(&[], None).unwrap(),
        Some(vec![])
    );
}
