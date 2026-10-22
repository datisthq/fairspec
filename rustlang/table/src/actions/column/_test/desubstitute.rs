use fairspec_metadata::Column;
use polars::prelude::*;
use serde_json::{Value, json};

use crate::actions::column::desubstitute::desubstitute_column;
use crate::helpers::value::get_json_value;
use crate::models::column::{ColumnMapping, DenormalizeColumnOptions, PolarsColumn};

fn get_row(series: Series, property: Value, native_types: Option<&[&str]>) -> Value {
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
    let options = DenormalizeColumnOptions {
        native_types: native_types.map(|types| types.iter().map(|t| t.to_string()).collect()),
    };
    let table = DataFrame::new(1, vec![series.into()]).unwrap().lazy();
    let frame = table
        .select([desubstitute_column(&mapping, col(name.as_str()), &options)])
        .collect()
        .unwrap();
    let column = &frame.columns()[0];
    json!({column.name().as_str(): get_json_value(&column.get(0).unwrap())})
}

fn assert_string(property: Value, cases: &[(Option<&str>, Option<&str>)]) {
    for (value, expected) in cases {
        let series = Series::new("name".into(), [*value]);
        let row = get_row(series, property.clone(), None);
        assert_eq!(row, json!({"name": expected}), "{value:?}");
    }
}

fn assert_integer(missing_values: Value, cases: &[(Option<i64>, i64)]) {
    let property = json!({"type": "integer", "missingValues": missing_values});
    for (value, expected) in cases {
        let series = Series::new("value".into(), [*value]);
        let row = get_row(series, property.clone(), Some(&["integer"]));
        assert_eq!(row, json!({"value": expected}), "{value:?}");
    }
}

fn assert_number(missing_values: Value, cases: &[(Option<f64>, f64)]) {
    let property = json!({"type": "number", "missingValues": missing_values});
    for (value, expected) in cases {
        let series = Series::new("value".into(), [*value]);
        let row = get_row(series, property.clone(), Some(&["number"]));
        assert_eq!(row, json!({"value": expected}), "{value:?}");
    }
}

#[test]
fn test_string_no_missing_values() {
    assert_string(
        json!({"type": "string"}),
        &[
            (None, None),
            (Some("hello"), Some("hello")),
            (Some("value"), Some("value")),
        ],
    );
}

#[test]
fn test_string_empty_missing_values() {
    assert_string(
        json!({"type": "string", "missingValues": []}),
        &[
            (None, None),
            (Some("hello"), Some("hello")),
            (Some("value"), Some("value")),
        ],
    );
}

#[test]
fn test_string_missing_value_dash() {
    assert_string(
        json!({"type": "string", "missingValues": ["-"]}),
        &[
            (None, Some("-")),
            (Some("hello"), Some("hello")),
            (Some("value"), Some("value")),
        ],
    );
}

#[test]
fn test_string_missing_value_x() {
    assert_string(
        json!({"type": "string", "missingValues": ["x"]}),
        &[
            (None, Some("x")),
            (Some("hello"), Some("hello")),
            (Some("value"), Some("value")),
        ],
    );
}

#[test]
fn test_string_missing_value_na() {
    assert_string(
        json!({"type": "string", "missingValues": ["n/a"]}),
        &[
            (None, Some("n/a")),
            (Some("value"), Some("value")),
            (Some("test"), Some("test")),
        ],
    );
}

#[test]
fn test_string_multiple_missing_values_uses_first() {
    assert_string(
        json!({"type": "string", "missingValues": ["-", "n/a", "null"]}),
        &[
            (None, Some("-")),
            (Some("value"), Some("value")),
            (Some("test"), Some("test")),
        ],
    );
}

#[test]
fn test_integer_missing_value_neg1() {
    assert_integer(
        json!([-1]),
        &[(None, -1), (Some(0), 0), (Some(1), 1), (Some(42), 42)],
    );
}

#[test]
fn test_integer_missing_value_neg999() {
    assert_integer(
        json!([-999]),
        &[(None, -999), (Some(0), 0), (Some(1), 1), (Some(100), 100)],
    );
}

#[test]
fn test_integer_multiple_missing_values_uses_first() {
    assert_integer(
        json!([-1, -999, -9999]),
        &[(None, -1), (Some(0), 0), (Some(42), 42)],
    );
}

#[test]
fn test_number_missing_value_float64() {
    assert_number(
        json!([-1]),
        &[
            (None, -1.0),
            (Some(0.0), 0.0),
            (Some(1.0), 1.0),
            (Some(42.0), 42.0),
        ],
    );
}

#[test]
fn test_number_missing_value_neg999_float64() {
    assert_number(
        json!([-999]),
        &[
            (None, -999.0),
            (Some(0.0), 0.0),
            (Some(1.0), 1.0),
            (Some(100.0), 100.0),
        ],
    );
}

#[test]
fn test_number_multiple_missing_values_uses_first_float64() {
    assert_number(
        json!([-1, -999, -9999]),
        &[(None, -1.0), (Some(0.0), 0.0), (Some(42.0), 42.0)],
    );
}
