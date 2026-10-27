use fairspec_metadata::{TableError, TableSchema};
use polars::prelude::*;
use serde_json::{Value, json};

use crate::actions::table::inspect::inspect_table;

fn create_schema(value: Value) -> TableSchema {
    serde_json::from_value(value).unwrap()
}

fn inspect(table: LazyFrame, schema: &TableSchema, max_errors: Option<usize>) -> Vec<Value> {
    inspect_table(table, Some(schema), None, None, max_errors, None)
        .unwrap()
        .iter()
        .map(|error| serde_json::to_value(error).unwrap())
        .collect()
}

fn get_summary(errors: &[Value]) -> Vec<(String, Value)> {
    errors
        .iter()
        .map(|error| {
            let row_number = error.get("rowNumber").cloned().unwrap_or(Value::Null);
            (error["type"].as_str().unwrap().to_string(), row_number)
        })
        .collect()
}

fn get_of_type<'a>(errors: &'a [Value], r#type: &str) -> Vec<&'a Value> {
    errors
        .iter()
        .filter(|error| error["type"] == r#type)
        .collect()
}

#[test]
fn test_should_not_error_when_all_rows_are_unique_for_primary_key() {
    let table =
        df!("id" => [1i64, 2, 3, 4, 5], "name" => ["Alice", "Bob", "Charlie", "David", "Eve"])
            .unwrap()
            .lazy();
    let schema = create_schema(json!({
        "properties": {"id": {"type": "integer"}, "name": {"type": "string"}},
        "primaryKey": ["id"],
    }));

    assert!(inspect(table, &schema, None).is_empty());
}

#[test]
fn test_should_error_for_duplicate_primary_key_rows() {
    let table =
        df!("id" => [1i64, 2, 3, 2, 5], "name" => ["Alice", "Bob", "Charlie", "Bob2", "Eve"])
            .unwrap()
            .lazy();
    let schema = create_schema(json!({
        "properties": {"id": {"type": "integer"}, "name": {"type": "string"}},
        "primaryKey": ["id"],
    }));

    let errors = inspect(table, &schema, None);
    let pk_errors = get_of_type(&errors, "row/primaryKey");
    assert_eq!(pk_errors.len(), 1);
    assert_eq!(pk_errors[0]["rowNumber"], 4);
    assert_eq!(pk_errors[0]["columnNames"], json!(["id"]));
}

#[test]
fn test_should_not_error_when_all_rows_are_unique_for_unique_key() {
    let table = df!(
        "id" => [1i64, 2, 3, 4, 5],
        "email" => ["a@test.com", "b@test.com", "c@test.com", "d@test.com", "e@test.com"],
    )
    .unwrap()
    .lazy();
    let schema = create_schema(json!({
        "properties": {"id": {"type": "integer"}, "email": {"type": "string"}},
        "uniqueKeys": [["email"]],
    }));

    assert!(inspect(table, &schema, None).is_empty());
}

#[test]
fn test_should_error_for_duplicate_unique_key_rows() {
    let table = df!(
        "id" => [1i64, 2, 3, 4, 5],
        "email" => ["a@test.com", "b@test.com", "a@test.com", "d@test.com", "b@test.com"],
    )
    .unwrap()
    .lazy();
    let schema = create_schema(json!({
        "properties": {"id": {"type": "integer"}, "email": {"type": "string"}},
        "uniqueKeys": [["email"]],
    }));

    let errors = inspect(table, &schema, None);
    let uk_errors = get_of_type(&errors, "row/uniqueKey");
    assert_eq!(uk_errors.len(), 2);
    assert_eq!(uk_errors[0]["rowNumber"], 3);
    assert_eq!(uk_errors[0]["columnNames"], json!(["email"]));
    assert_eq!(uk_errors[1]["rowNumber"], 5);
    assert_eq!(uk_errors[1]["columnNames"], json!(["email"]));
}

#[test]
fn test_should_handle_composite_unique_keys() {
    let table = df!(
        "category" => ["A", "A", "B", "A", "B"],
        "subcategory" => ["X", "Y", "X", "X", "Y"],
        "value" => [1i64, 2, 3, 4, 5],
    )
    .unwrap()
    .lazy();
    let schema = create_schema(json!({
        "properties": {
            "category": {"type": "string"},
            "subcategory": {"type": "string"},
            "value": {"type": "integer"},
        },
        "uniqueKeys": [["category", "subcategory"]],
    }));

    let errors = inspect(table, &schema, None);
    let uk_errors = get_of_type(&errors, "row/uniqueKey");
    assert_eq!(uk_errors.len(), 1);
    assert_eq!(uk_errors[0]["rowNumber"], 4);
    assert_eq!(
        uk_errors[0]["columnNames"],
        json!(["category", "subcategory"])
    );
}

#[test]
fn test_should_handle_both_primary_key_and_unique_keys() {
    let table = df!(
        "id" => [1i64, 2, 3, 2, 5],
        "email" => ["a@test.com", "b@test.com", "c@test.com", "d@test.com", "a@test.com"],
    )
    .unwrap()
    .lazy();
    let schema = create_schema(json!({
        "properties": {"id": {"type": "integer"}, "email": {"type": "string"}},
        "primaryKey": ["id"],
        "uniqueKeys": [["email"]],
    }));

    let errors = inspect(table, &schema, None);
    assert_eq!(errors.len(), 2);
    let pk_errors = get_of_type(&errors, "row/primaryKey");
    let uk_errors = get_of_type(&errors, "row/uniqueKey");
    assert_eq!(pk_errors.len(), 1);
    assert_eq!(pk_errors[0]["rowNumber"], 4);
    assert_eq!(pk_errors[0]["columnNames"], json!(["id"]));
    assert_eq!(uk_errors.len(), 1);
    assert_eq!(uk_errors[0]["rowNumber"], 5);
    assert_eq!(uk_errors[0]["columnNames"], json!(["email"]));
}

#[test]
fn test_should_handle_null_values_in_unique_keys_correctly() {
    let table = df!(
        "id" => [Some(1i64), Some(2), None, Some(4), None, Some(2)],
        "name" => ["Alice", "Bob", "Charlie", "David", "Eve", "Bob"],
    )
    .unwrap()
    .lazy();
    let schema = create_schema(json!({
        "properties": {"id": {"type": ["integer", "null"]}, "name": {"type": "string"}},
        "uniqueKeys": [["id"], ["id", "name"]],
    }));

    let errors = inspect(table, &schema, None);
    let uk_errors = get_of_type(&errors, "row/uniqueKey");
    assert_eq!(uk_errors.len(), 2);
    assert_eq!(uk_errors[0]["rowNumber"], 6);
    assert_eq!(uk_errors[0]["columnNames"], json!(["id"]));
    assert_eq!(uk_errors[1]["rowNumber"], 6);
    assert_eq!(uk_errors[1]["columnNames"], json!(["id", "name"]));
}

#[test]
fn test_should_skip_unique_keys_with_any_null_part() {
    let table = df!(
        "a" => [Some(1i64), None, None, Some(1), None],
        "b" => [Some("x"), Some("y"), Some("y"), None, None],
    )
    .unwrap()
    .lazy();
    let schema = create_schema(json!({
        "properties": {"a": {"type": ["integer", "null"]}, "b": {"type": ["string", "null"]}},
        "uniqueKeys": [["a", "b"]],
    }));

    assert!(inspect(table, &schema, None).is_empty());
}

#[test]
fn test_should_skip_unique_keys_with_a_missing_value_part() {
    let table = df!("code" => ["NA", "a", "NA"]).unwrap().lazy();
    let schema = create_schema(json!({
        "missingValues": ["NA"],
        "properties": {"code": {"type": ["string", "null"]}},
        "uniqueKeys": [["code"]],
    }));

    assert!(inspect(table, &schema, None).is_empty());
}

#[test]
fn test_should_report_null_primary_key_parts_as_missing_cells() {
    let table = df!(
        "a" => [None, None, Some(1i64)],
        "b" => [None, None, Some("z")],
    )
    .unwrap()
    .lazy();
    let schema = create_schema(json!({
        "properties": {"a": {"type": ["integer", "null"]}, "b": {"type": ["string", "null"]}},
        "primaryKey": ["a", "b"],
    }));

    let errors: Vec<_> = inspect(table, &schema, None)
        .iter()
        .map(|error| {
            (
                error["type"].clone(),
                error["columnName"].clone(),
                error["rowNumber"].clone(),
            )
        })
        .collect();
    assert_eq!(
        errors,
        [("a", 1), ("a", 2), ("b", 1), ("b", 2)].map(|(name, row)| (
            json!("cell/missing"),
            json!(name),
            json!(row)
        ))
    );
}

#[test]
fn test_should_check_a_key_column_named_like_the_probe_count() {
    let table = df!("len" => [1i64, 2, 1]).unwrap().lazy();
    let schema = create_schema(json!({
        "properties": {"len": {"type": "integer"}},
        "primaryKey": ["len"],
    }));

    let errors = inspect(table, &schema, None);
    assert_eq!(
        get_summary(&errors),
        vec![("row/primaryKey".to_string(), json!(3))]
    );
}

#[test]
fn test_should_cap_key_errors_per_check() {
    let table = df!("id" => [1i64; 10], "name" => ["a"; 10]).unwrap().lazy();
    let schema = create_schema(json!({
        "properties": {"id": {"type": "integer"}, "name": {"type": "string"}},
        "primaryKey": ["id"],
    }));

    let errors = inspect(table, &schema, Some(4));
    assert_eq!(
        get_summary(&errors),
        vec![
            ("row/primaryKey".to_string(), json!(2)),
            ("row/primaryKey".to_string(), json!(3)),
        ]
    );
}

#[test]
fn test_should_find_a_duplicate_key_beside_cell_errors() {
    let table = df!("id" => ["1", "2", "x", "2"]).unwrap().lazy();
    let schema = create_schema(json!({
        "properties": {"id": {"type": "integer"}},
        "primaryKey": ["id"],
    }));

    let errors = inspect(table, &schema, None);
    assert_eq!(
        get_summary(&errors),
        vec![
            ("cell/type".to_string(), json!(3)),
            ("row/primaryKey".to_string(), json!(4)),
        ]
    );
}

#[test]
fn test_should_find_a_duplicate_key_without_cell_checks() {
    let table = df!("name" => ["a", "b", "a"]).unwrap().lazy();
    let schema = create_schema(json!({
        "properties": {"name": {"type": "string"}},
        "uniqueKeys": [["name"]],
    }));

    let errors = inspect(table, &schema, None);
    assert_eq!(
        get_summary(&errors),
        vec![("row/uniqueKey".to_string(), json!(3))]
    );
}

#[test]
fn test_should_find_a_duplicate_key_across_read_batches() {
    let mut ids: Vec<String> = (0..300_000).map(|index| index.to_string()).collect();
    ids.push("0".to_string());
    let table = df!("id" => ids).unwrap().lazy();
    let schema = create_schema(json!({
        "properties": {"id": {"type": "integer"}},
        "primaryKey": ["id"],
    }));

    let errors = inspect(table, &schema, None);
    assert_eq!(
        get_summary(&errors),
        vec![("row/primaryKey".to_string(), json!(300_001))]
    );
}

#[test]
fn test_should_leave_key_errors_out_once_cell_errors_fill_the_report() {
    let table = df!("id" => ["x", "y", "1", "1"]).unwrap().lazy();
    let schema = create_schema(json!({
        "properties": {"id": {"type": "integer"}},
        "primaryKey": ["id"],
    }));

    let errors = inspect(table, &schema, Some(2));
    assert_eq!(
        get_summary(&errors),
        vec![
            ("cell/type".to_string(), json!(1)),
            ("cell/type".to_string(), json!(2)),
        ]
    );
}

#[test]
fn test_should_keep_row_check_order_under_concurrency() {
    let table = df!(
        "id" => [1i64, 1, 2],
        "email" => ["a@a", "a@a", "b@b"],
        "code" => ["x", "x", "y"],
    )
    .unwrap()
    .lazy();
    let schema = create_schema(json!({
        "properties": {
            "id": {"type": "integer"},
            "email": {"type": "string"},
            "code": {"type": "string"},
        },
        "primaryKey": ["id"],
        "uniqueKeys": [["email"], ["code"]],
    }));

    let run = |concurrency| -> Vec<TableError> {
        inspect_table(
            table.clone(),
            Some(&schema),
            None,
            None,
            None,
            Some(concurrency),
        )
        .unwrap()
    };
    let serial = run(1);
    let concurrent = run(4);

    let types: Vec<Value> = serial
        .iter()
        .map(|error| serde_json::to_value(error).unwrap()["type"].clone())
        .collect();
    assert_eq!(
        types,
        vec![
            json!("row/primaryKey"),
            json!("row/uniqueKey"),
            json!("row/uniqueKey")
        ]
    );
    assert_eq!(serial, concurrent);
}

#[test]
fn test_should_compare_numbers_as_values() {
    let table = df!("id" => ["1", "1.0", "2.5"]).unwrap().lazy();
    let schema = create_schema(json!({
        "properties": {"id": {"type": "number"}},
        "primaryKey": ["id"],
    }));

    let errors = inspect(table, &schema, None);

    assert_eq!(
        get_summary(&errors),
        [("row/primaryKey".to_string(), json!(2))]
    );
}

#[test]
fn test_should_find_repeated_keys_that_parse_to_one_value() {
    let folder = tempfile::tempdir().unwrap();
    let path = folder.path().join("table.csv");
    std::fs::write(&path, "flag\nyes\ny\n").unwrap();
    let table = LazyCsvReader::new(PlRefPath::new(path.to_str().unwrap()))
        .with_infer_schema_length(Some(0))
        .finish()
        .unwrap();
    let schema = create_schema(json!({
        "properties": {"flag": {"type": "boolean", "trueValues": ["yes", "y"]}},
        "primaryKey": ["flag"],
    }));

    let errors = inspect(table, &schema, None);

    assert_eq!(
        get_summary(&errors),
        [("row/primaryKey".to_string(), json!(2))]
    );
}
