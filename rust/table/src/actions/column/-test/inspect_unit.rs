use chrono::{NaiveDate, NaiveTime};
use fairspec_metadata::{ColumnError, ColumnMissingError, TableError, TableSchema, get_columns};
use indexmap::IndexMap;
use polars::prelude::*;
use serde_json::{Value, json};

use crate::actions::column::inspect::inspect_columns;
use crate::actions::table::_shared::helpers::merge_missing_values;
use crate::helpers::schema::get_polars_schema;
use crate::models::column::ColumnMapping;

/// The column half of Python's `inspect_table`, which builds the mappings this way.
fn inspect_table(mut table: LazyFrame, table_schema: Value) -> Vec<Value> {
    let polars_schema = get_polars_schema(&table.collect_schema().unwrap());
    let descriptor = table_schema.as_object().unwrap();
    let typed_schema: TableSchema = serde_json::from_value(table_schema.clone()).unwrap();
    let columns = get_columns(descriptor).unwrap();

    let mut missing_errors: IndexMap<String, Vec<TableError>> = IndexMap::new();
    let mut mappings = Vec::new();
    for column in &columns {
        let Some(polars_column) = polars_schema
            .columns
            .iter()
            .find(|polars_column| polars_column.name == column.name())
        else {
            let is_required = typed_schema.all_required.unwrap_or(false)
                || typed_schema
                    .required
                    .as_ref()
                    .is_some_and(|required| required.iter().any(|name| name == column.name()));
            let errors = if is_required {
                vec![TableError::Column(ColumnError::Missing(
                    ColumnMissingError {
                        resource_name: None,
                        column_name: column.name().to_string(),
                    },
                ))]
            } else {
                Vec::new()
            };
            missing_errors.insert(column.name().to_string(), errors);
            continue;
        };
        mappings.push(ColumnMapping {
            source: polars_column.clone(),
            target: merge_missing_values(column, &typed_schema),
        });
    }

    let cell_errors = inspect_columns(&mappings, table, 1000, &[]).unwrap();
    let mut errors: Vec<Value> = Vec::new();
    for column in &columns {
        let column_errors = match missing_errors.get(column.name()) {
            Some(column_errors) if !column_errors.is_empty() => column_errors,
            _ => match cell_errors.get(column.name()) {
                Some(column_errors) => column_errors,
                None => continue,
            },
        };
        errors.extend(
            column_errors
                .iter()
                .map(|error| serde_json::to_value(error).unwrap()),
        );
    }
    errors.sort_by_key(|error| !error["type"].as_str().unwrap().starts_with("column/"));
    errors
}

fn get_cell_errors(errors: &[Value]) -> Vec<(Value, Value)> {
    errors
        .iter()
        .filter(|error| error["type"] == "cell/type")
        .map(|error| (error["cell"].clone(), error["rowNumber"].clone()))
        .collect()
}

fn create_datetime_table(time_zone: Option<&str>) -> LazyFrame {
    let value = NaiveDate::from_ymd_opt(2024, 1, 15)
        .unwrap()
        .and_hms_opt(10, 30, 0)
        .unwrap();
    let time_zone = TimeZone::opt_try_new(time_zone).unwrap();
    df!("at" => [value])
        .unwrap()
        .lazy()
        .with_column(col("at").cast(DataType::Datetime(TimeUnit::Microseconds, time_zone)))
}

#[test]
fn test_should_report_error_when_column_names_dont_match() {
    let table = df!("actual_id" => [1i64, 2, 3]).unwrap().lazy();

    let errors = inspect_table(
        table,
        json!({"allRequired": true, "properties": {"id": {"type": "number"}}}),
    );

    assert_eq!(
        errors,
        vec![json!({"type": "column/missing", "columnName": "id"})]
    );
}

#[test]
fn test_should_not_error_when_column_names_match() {
    let table = df!("id" => [1i64, 2, 3]).unwrap().lazy();
    assert!(inspect_table(table, json!({"properties": {"id": {"type": "number"}}})).is_empty());
}

#[test]
fn test_should_be_case_sensitive_when_comparing_column_names() {
    let table = df!("ID" => [1i64, 2, 3]).unwrap().lazy();

    let errors = inspect_table(
        table,
        json!({"allRequired": true, "properties": {"id": {"type": "number"}}}),
    );

    assert_eq!(
        errors,
        vec![json!({"type": "column/missing", "columnName": "id"})]
    );
}

#[test]
fn test_should_report_error_when_column_types_dont_match() {
    let table = df!("id" => [true, false, true]).unwrap().lazy();

    let errors = inspect_table(table, json!({"properties": {"id": {"type": "integer"}}}));

    assert_eq!(
        errors,
        vec![json!({
            "type": "column/type",
            "columnName": "id",
            "expectedColumnType": "integer",
            "actualColumnType": "boolean",
        })]
    );
}

#[test]
fn test_should_not_error_when_column_types_match() {
    let table = df!("id" => [1i64, 2, 3]).unwrap().lazy();
    assert!(inspect_table(table, json!({"properties": {"id": {"type": "number"}}})).is_empty());
}

#[test]
fn test_should_match_a_zoned_datetime_to_date_time_only() {
    let date_time = json!({"type": "string", "format": "date-time"});
    let local_date_time = json!({"type": "string", "format": "local-date-time"});

    let table = create_datetime_table(Some("Europe/Rome"));
    assert!(inspect_table(table.clone(), json!({"properties": {"at": date_time}})).is_empty());
    let errors = inspect_table(table, json!({"properties": {"at": local_date_time}}));

    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0]["type"], "column/type");
    assert_eq!(errors[0]["expectedColumnType"], "local-date-time");
    assert_eq!(errors[0]["actualColumnType"], "date-time");
}

#[test]
fn test_should_match_a_naive_datetime_to_local_date_time_only() {
    let date_time = json!({"type": "string", "format": "date-time"});
    let local_date_time = json!({"type": "string", "format": "local-date-time"});

    let table = create_datetime_table(None);
    assert!(
        inspect_table(
            table.clone(),
            json!({"properties": {"at": local_date_time}})
        )
        .is_empty()
    );
    let errors = inspect_table(table, json!({"properties": {"at": date_time}}));

    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0]["type"], "column/type");
    assert_eq!(errors[0]["expectedColumnType"], "date-time");
    assert_eq!(errors[0]["actualColumnType"], "local-date-time");
}

#[test]
fn test_should_match_a_time_to_time_and_local_time() {
    let table = df!("at" => [NaiveTime::from_hms_opt(10, 30, 0).unwrap()])
        .unwrap()
        .lazy();

    for format in ["time", "local-time"] {
        let table_schema = json!({"properties": {"at": {"type": "string", "format": format}}});
        assert!(
            inspect_table(table.clone(), table_schema).is_empty(),
            "{format}"
        );
    }
}

#[test]
fn test_should_not_crash_when_data_has_column_named_number() {
    let table = df!("name" => ["Alice", "Bob", "Charlie"], "number" => [1i64, 2, 3])
        .unwrap()
        .lazy();
    let table_schema = json!({
        "properties": {"name": {"type": "string"}, "number": {"type": "integer"}},
    });

    assert!(inspect_table(table, table_schema).is_empty());
}

#[test]
fn test_should_report_cell_errors_with_correct_row_numbers_for_column_named_number() {
    let table = df!("name" => ["Alice", "Bob", "Charlie"], "number" => ["1", "bad", "3"])
        .unwrap()
        .lazy();
    let table_schema = json!({
        "properties": {"name": {"type": "string"}, "number": {"type": "integer"}},
    });

    let errors = inspect_table(table, table_schema);

    assert_eq!(
        errors,
        vec![json!({
            "type": "cell/type",
            "columnName": "number",
            "rowNumber": 2,
            "cell": "bad",
            "columnType": "integer",
        })]
    );
}

#[test]
fn test_should_validate_string_to_integer_conversion_errors() {
    let table = df!("id" => ["1", "bad", "3", "4x"]).unwrap().lazy();

    let errors = inspect_table(table, json!({"properties": {"id": {"type": "integer"}}}));

    assert_eq!(
        errors,
        vec![
            json!({
                "type": "cell/type",
                "columnName": "id",
                "rowNumber": 2,
                "cell": "bad",
                "columnType": "integer",
            }),
            json!({
                "type": "cell/type",
                "columnName": "id",
                "rowNumber": 4,
                "cell": "4x",
                "columnType": "integer",
            }),
        ]
    );
}

#[test]
fn test_should_validate_string_to_number_conversion_errors() {
    let table = df!("price" => ["10.5", "twenty", "30.75", "$40"])
        .unwrap()
        .lazy();

    let errors = inspect_table(table, json!({"properties": {"price": {"type": "number"}}}));

    assert_eq!(errors.len(), 2);
    assert_eq!(errors[0]["columnType"], "number");
    assert_eq!(
        get_cell_errors(&errors),
        vec![(json!("twenty"), json!(2)), (json!("$40"), json!(4))]
    );
}

#[test]
fn test_should_validate_string_to_boolean_conversion_errors() {
    let table = df!("active" => ["true", "yes", "false", "0", "1"])
        .unwrap()
        .lazy();

    let errors = inspect_table(
        table,
        json!({"properties": {"active": {"type": "boolean"}}}),
    );

    assert_eq!(
        errors,
        vec![json!({
            "type": "cell/type",
            "columnName": "active",
            "rowNumber": 2,
            "cell": "yes",
            "columnType": "boolean",
        })]
    );
}

#[test]
fn test_should_validate_string_to_date_conversion_errors() {
    let table = df!("created" => ["2023-01-15", "Jan 15, 2023", "20230115", "not-a-date"])
        .unwrap()
        .lazy();
    let table_schema = json!({"properties": {"created": {"type": "string", "format": "date"}}});

    let errors = inspect_table(table, table_schema);

    assert_eq!(
        get_cell_errors(&errors),
        vec![
            (json!("Jan 15, 2023"), json!(2)),
            (json!("20230115"), json!(3)),
            (json!("not-a-date"), json!(4)),
        ]
    );
}

#[test]
fn test_should_validate_string_to_time_conversion_errors() {
    let table = df!("time" => ["14:30:00Z", "2:30pm", "invalid", "14h30"])
        .unwrap()
        .lazy();
    let table_schema = json!({"properties": {"time": {"type": "string", "format": "time"}}});

    let errors = inspect_table(table, table_schema);

    assert_eq!(
        get_cell_errors(&errors),
        vec![
            (json!("2:30pm"), json!(2)),
            (json!("invalid"), json!(3)),
            (json!("14h30"), json!(4)),
        ]
    );
}

#[test]
fn test_should_validate_string_to_datetime_conversion_errors() {
    let table = df!("timestamp" => [
        "2023-01-15T14:30:00",
        "January 15, 2023 2:30 PM",
        "2023-01-15 14:30",
        "not-a-datetime",
    ])
    .unwrap()
    .lazy();
    let table_schema =
        json!({"properties": {"timestamp": {"type": "string", "format": "date-time"}}});

    let errors = inspect_table(table, table_schema);

    let cells: Vec<Value> = get_cell_errors(&errors)
        .into_iter()
        .map(|(cell, _)| cell)
        .collect();
    assert!(cells.len() >= 2);
    assert!(cells.contains(&json!("January 15, 2023 2:30 PM")));
    assert!(cells.contains(&json!("not-a-datetime")));
}

#[test]
fn test_should_pass_validation_when_all_cells_are_valid() {
    let table = df!("id" => ["1", "2", "3", "4"]).unwrap().lazy();
    assert!(inspect_table(table, json!({"properties": {"id": {"type": "integer"}}})).is_empty());
}

#[test]
fn test_should_validate_with_non_string_source_data() {
    let table = df!("is_active" => [true, false, true, false])
        .unwrap()
        .lazy();
    let table_schema = json!({"properties": {"is_active": {"type": "boolean"}}});

    assert!(inspect_table(table, table_schema).is_empty());
}

#[test]
fn test_should_keep_errors_column_by_column_then_by_row() {
    let table = df!(
        "a" => ["x", "1", "x"],
        "b" => ["P1D", "bad", "bad"],
        "c" => ["x", "x", "3"],
        "d" => [1i64, 2, 3],
    )
    .unwrap()
    .lazy();
    let table_schema = json!({
        "allRequired": true,
        "properties": {
            "a": {"type": "integer"},
            "b": {"type": "string", "format": "duration"},
            "c": {"type": "integer"},
            "d": {"type": "boolean"},
            "e": {"type": "string"},
        },
    });

    let errors = inspect_table(table, table_schema);

    let located: Vec<(Value, Value, Value)> = errors
        .iter()
        .map(|error| {
            (
                error["type"].clone(),
                error["columnName"].clone(),
                error.get("rowNumber").cloned().unwrap_or(Value::Null),
            )
        })
        .collect();
    assert_eq!(
        located,
        vec![
            (json!("column/type"), json!("d"), Value::Null),
            (json!("column/missing"), json!("e"), Value::Null),
            (json!("cell/type"), json!("a"), json!(1)),
            (json!("cell/type"), json!("a"), json!(3)),
            (json!("cell/type"), json!("b"), json!(2)),
            (json!("cell/type"), json!("b"), json!(3)),
            (json!("cell/type"), json!("c"), json!(1)),
            (json!("cell/type"), json!("c"), json!(2)),
        ]
    );
}

// Python's test_should_check_every_column_in_one_collect patches LazyFrame.collect to
// count calls; polars-Rust has no such hook, so it is not ported

fn get_error_types(errors: &[Value]) -> Vec<&str> {
    errors
        .iter()
        .map(|error| error["type"].as_str().unwrap())
        .collect()
}

#[test]
fn test_should_report_every_failing_constraint_in_keyword_order() {
    let table = df!("n" => ["7"]).unwrap().lazy();
    let schema = json!({
        "properties": {"n": {"type": "integer", "multipleOf": 3, "maximum": 5, "enum": [1, 2]}}
    });

    let errors = inspect_table(table, schema);

    assert_eq!(
        get_error_types(&errors),
        ["cell/enum", "cell/maximum", "cell/multipleOf"]
    );
}

#[test]
fn test_should_report_a_type_error_alone() {
    let table = df!("n" => ["x"]).unwrap().lazy();
    let schema = json!({"properties": {"n": {"type": "integer", "minimum": 5, "enum": [1, 2]}}});

    let errors = inspect_table(table, schema);

    assert_eq!(get_error_types(&errors), ["cell/type"]);
}
