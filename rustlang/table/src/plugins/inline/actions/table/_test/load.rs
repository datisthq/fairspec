use chrono::NaiveDate;
use fairspec_metadata::Resource;
use polars::prelude::*;
use serde_json::{Map, Value, json};

use crate::helpers::value::get_json_value;
use crate::plugins::_shared::collect_records;
use crate::plugins::inline::actions::table::load::load_inline_table;

fn create_resource(value: Value) -> Resource {
    serde_json::from_value(value).unwrap()
}

fn load(value: Value) -> Value {
    collect_records(load_inline_table(&create_resource(value), &Default::default()).unwrap())
}

fn create_properties() -> Value {
    json!({"id": {"type": "integer"}, "name": {"type": "string"}})
}

#[test]
fn test_should_raise_on_no_data() {
    let error = load_inline_table(
        &create_resource(json!({"name": "test"})),
        &Default::default(),
    )
    .err()
    .unwrap();
    assert!(
        error
            .to_string()
            .contains("Resource data is not defined or tabular")
    );
}

#[test]
fn test_should_raise_on_bad_data() {
    let resource = create_resource(json!({"name": "test", "data": "bad"}));
    let error = load_inline_table(&resource, &Default::default())
        .err()
        .unwrap();
    assert!(
        error
            .to_string()
            .contains("Resource data is not defined or tabular")
    );
}

#[test]
fn test_should_read_table_data() {
    assert_eq!(
        load(json!({
            "name": "test",
            "data": [{"id": 1, "name": "english"}, {"id": 2, "name": "中文"}],
        })),
        json!([{"id": 1, "name": "english"}, {"id": 2, "name": "中文"}])
    );
}

#[test]
fn test_should_handle_longer_rows() {
    assert_eq!(
        load(json!({
            "data": [
                {"id": 1, "name": "english"},
                {"id": 2, "name": "中文", "extra": "bad"},
            ],
            "tableSchema": {"properties": create_properties()},
        })),
        json!([{"id": 1, "name": "english"}, {"id": 2, "name": "中文"}])
    );
}

#[test]
fn test_should_handle_shorter_rows() {
    assert_eq!(
        load(json!({
            "name": "test",
            "data": [{"id": 1, "name": "english"}, {"id": 2}],
            "tableSchema": {"properties": create_properties()},
        })),
        json!([{"id": 1, "name": "english"}, {"id": 2, "name": null}])
    );
}

/// JSON data cannot carry Python's native `datetime` cells, so they arrive as ISO text and
/// must come back parsed as the same datetimes.
#[test]
fn test_should_handle_various_data_types() {
    let resource = create_resource(json!({
        "data": [{
            "string": "string",
            "number": 1,
            "boolean": true,
            "date": "2025-01-01T00:00:00",
            "time": "2025-01-01T00:00:00",
            "datetime": "2025-01-01T00:00:00",
        }],
    }));
    let frame = load_inline_table(&resource, &Default::default())
        .unwrap()
        .collect()
        .unwrap();
    let value = NaiveDate::from_ymd_opt(2025, 1, 1)
        .unwrap()
        .and_hms_opt(0, 0, 0)
        .unwrap();
    for name in ["date", "time", "datetime"] {
        let column = frame.column(name).unwrap();
        assert!(matches!(column.dtype(), DataType::Datetime(_, None)));
        let expected = Column::new(name.into(), [value])
            .cast(column.dtype())
            .unwrap();
        assert!(column.equals(&expected));
    }
    let row = frame.get_row(0).unwrap();
    let record: Map<String, Value> = frame
        .get_column_names()
        .iter()
        .zip(row.0.iter())
        .take(3)
        .map(|(name, value)| (name.to_string(), get_json_value(value)))
        .collect();
    assert_eq!(
        Value::Object(record),
        json!({"string": "string", "number": 1, "boolean": true})
    );
}

#[test]
fn test_should_handle_objects_with_shorter_rows() {
    assert_eq!(
        load(json!({
            "data": [{"id": 1, "name": "english"}, {"id": 2, "name": "中文"}, {"id": 3}],
        })),
        json!([
            {"id": 1, "name": "english"},
            {"id": 2, "name": "中文"},
            {"id": 3, "name": null},
        ])
    );
}

#[test]
fn test_should_handle_objects_with_longer_rows() {
    assert_eq!(
        load(json!({
            "data": [
                {"id": 1, "name": "english"},
                {"id": 2, "name": "中文"},
                {"id": 3, "name": "german", "extra": "extra"},
            ],
        })),
        json!([
            {"id": 1, "name": "english", "extra": null},
            {"id": 2, "name": "中文", "extra": null},
            {"id": 3, "name": "german", "extra": "extra"},
        ])
    );
}
