use fairspec_metadata::Column;
use polars::prelude::*;
use serde_json::{Value, json};

use crate::actions::column::substitute::substitute_column;
use crate::helpers::value::get_json_value;
use crate::models::column::{ColumnMapping, PolarsColumn};

fn get_row(series: Series, property: Value) -> Value {
    let name = series.name().to_string();
    let mapping = ColumnMapping {
        source: PolarsColumn {
            name: name.clone(),
            r#type: series.dtype().clone(),
            time_zone: None,
        },
        target: serde_json::from_value::<Column>(
            json!({"name": name, "type": property["type"], "property": property}),
        )
        .unwrap(),
    };
    let table = DataFrame::new(1, vec![series.into()]).unwrap().lazy();
    let frame = table
        .select([substitute_column(&mapping, col(name.as_str()))])
        .collect()
        .unwrap();
    let column = &frame.columns()[0];
    json!({column.name().as_str(): get_json_value(&column.get(0).unwrap())})
}

fn assert_string(missing_values: Value, cases: &[(&str, Option<&str>)]) {
    let property = json!({"type": "string", "missingValues": missing_values});
    for (cell, expected) in cases {
        let series = Series::new("name".into(), [*cell]);
        let row = get_row(series, property.clone());
        assert_eq!(row, json!({"name": expected}), "{cell}");
    }
}

fn assert_int64(missing_values: Value, cases: &[(i64, Option<i64>)]) {
    let property = json!({"type": "integer", "missingValues": missing_values});
    for (value, expected) in cases {
        let series = Series::new("value".into(), [*value]);
        let row = get_row(series, property.clone());
        assert_eq!(row, json!({"value": expected}), "{value}");
    }
}

fn assert_float64(missing_values: Value, cases: &[(i64, Option<i64>)]) {
    let property = json!({"type": "integer", "missingValues": missing_values});
    for (value, expected) in cases {
        let series = Series::new("value".into(), [*value as f64]);
        let row = get_row(series, property.clone());
        let expected = expected.map(|expected| expected as f64);
        assert_eq!(row, json!({"value": expected}), "{value}");
    }
}

#[test]
fn test_string_missing_value_x() {
    assert_string(
        json!(["x"]),
        &[
            ("x", None),
            ("-", Some("-")),
            ("", Some("")),
            ("value", Some("value")),
        ],
    );
}

#[test]
fn test_string_missing_value_dash() {
    assert_string(
        json!(["-"]),
        &[
            ("-", None),
            ("", Some("")),
            ("x", Some("x")),
            ("value", Some("value")),
        ],
    );
}

#[test]
fn test_string_missing_value_empty() {
    assert_string(
        json!([""]),
        &[
            ("", None),
            ("-", Some("-")),
            ("x", Some("x")),
            ("value", Some("value")),
        ],
    );
}

#[test]
fn test_string_missing_value_na() {
    assert_string(
        json!(["n/a"]),
        &[
            ("n/a", None),
            ("-", Some("-")),
            ("", Some("")),
            ("value", Some("value")),
        ],
    );
}

#[test]
fn test_string_multiple_missing_values() {
    assert_string(
        json!(["-", "x"]),
        &[
            ("-", None),
            ("x", None),
            ("", Some("")),
            ("value", Some("value")),
        ],
    );
}

#[test]
fn test_integer_missing_value_int64() {
    assert_int64(
        json!([-1]),
        &[(-1, None), (0, Some(0)), (1, Some(1)), (42, Some(42))],
    );
}

#[test]
fn test_integer_missing_value_neg999_int64() {
    assert_int64(
        json!([-999]),
        &[(-999, None), (0, Some(0)), (1, Some(1)), (100, Some(100))],
    );
}

#[test]
fn test_integer_multiple_missing_values_int64() {
    assert_int64(
        json!([-1, -99]),
        &[(-1, None), (-99, None), (0, Some(0)), (42, Some(42))],
    );
}

#[test]
fn test_integer_missing_value_float64() {
    assert_float64(
        json!([-1]),
        &[(-1, None), (0, Some(0)), (1, Some(1)), (42, Some(42))],
    );
}

#[test]
fn test_integer_multiple_missing_values_float64() {
    assert_float64(
        json!([-1, -99]),
        &[(-1, None), (-99, None), (0, Some(0)), (42, Some(42))],
    );
}
