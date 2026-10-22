use fairspec_metadata::Column;
use polars::prelude::*;
use serde_json::{Value, json};

use crate::actions::column::narrow::narrow_column;
use crate::helpers::value::get_json_value;
use crate::models::column::{ColumnMapping, PolarsColumn};

fn create_mapping(series: &Series, target: Value) -> ColumnMapping {
    ColumnMapping {
        source: PolarsColumn {
            name: series.name().to_string(),
            r#type: series.dtype().clone(),
            time_zone: None,
        },
        target: serde_json::from_value::<Column>(target).unwrap(),
    }
}

fn get_rows(series: Series, target: Value) -> Vec<Value> {
    let mapping = create_mapping(&series, target);
    let name = series.name().to_string();
    let table = DataFrame::new(series.len(), vec![series.into()])
        .unwrap()
        .lazy();
    let frame = table
        .select([narrow_column(&mapping, col(name.as_str()))])
        .collect()
        .unwrap();
    let column = &frame.columns()[0];
    (0..frame.height())
        .map(|index| json!({column.name().as_str(): get_json_value(&column.get(index).unwrap())}))
        .collect()
}

fn create_string_categorical() -> Value {
    json!({
        "name": "name",
        "type": "categorical",
        "property": {
            "type": "string",
            "format": "categorical",
            "categories": ["red", "green", "blue"],
        },
    })
}

fn create_integer_categorical() -> Value {
    json!({
        "name": "name",
        "type": "categorical",
        "property": {
            "type": "integer",
            "format": "categorical",
            "categories": [{"value": 1, "label": "Low"}, {"value": 2, "label": "High"}],
        },
    })
}

#[test]
fn test_narrow_float_to_integer() {
    let series = Series::new("id".into(), [1.0, 2.0, 3.0]);
    let target = json!({"name": "id", "type": "integer", "property": {"type": "integer"}});
    assert_eq!(
        get_rows(series, target),
        [json!({"id": 1}), json!({"id": 2}), json!({"id": 3})]
    );
}

#[test]
fn test_non_integer_float_becomes_null() {
    let series = Series::new("id".into(), [1.0, 2.0, 3.5]);
    let target = json!({"name": "id", "type": "integer", "property": {"type": "integer"}});
    assert_eq!(
        get_rows(series, target),
        [json!({"id": 1}), json!({"id": 2}), json!({"id": null})]
    );
}

#[test]
fn test_string_categorical() {
    let cases = [
        ("red", Some("red")),
        ("green", Some("green")),
        ("yellow", None),
    ];
    for (cell, expected) in cases {
        let series = Series::new("name".into(), [cell]);
        let rows = get_rows(series, create_string_categorical());
        assert_eq!(rows, [json!({"name": expected})], "{cell}");
    }
}

#[test]
fn test_integer_categorical() {
    let cases = [(1, Some("Low")), (2, Some("High")), (3, None)];
    for (cell, expected) in cases {
        let series = Series::new("name".into(), [cell as i64]);
        let rows = get_rows(series, create_integer_categorical());
        assert_eq!(rows, [json!({"name": expected})], "{cell}");
    }
}
