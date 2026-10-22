use fairspec_metadata::Column;
use polars::prelude::*;
use serde_json::{Value, json};

use crate::actions::column::denarrow::denarrow_column;
use crate::helpers::value::get_json_value;
use crate::models::column::{ColumnMapping, PolarsColumn};

fn get_row(cell: &str, target: Value) -> Value {
    let categorical = DataType::from_categories(Categories::global());
    let series = Series::new("name".into(), [cell])
        .cast(&categorical)
        .unwrap();
    let mapping = ColumnMapping {
        source: PolarsColumn {
            name: "name".to_string(),
            r#type: categorical,
            time_zone: None,
        },
        target: serde_json::from_value::<Column>(target).unwrap(),
    };
    let table = DataFrame::new(1, vec![series.into()]).unwrap().lazy();
    let frame = table
        .select([denarrow_column(&mapping, col("name"))])
        .collect()
        .unwrap();
    let column = &frame.columns()[0];
    json!({column.name().as_str(): get_json_value(&column.get(0).unwrap())})
}

#[test]
fn test_string_categorical() {
    let target = json!({
        "name": "name",
        "type": "categorical",
        "property": {
            "type": "string",
            "format": "categorical",
            "categories": ["red", "green", "blue"],
        },
    });
    for (value, expected) in [("red", "red"), ("green", "green")] {
        assert_eq!(
            get_row(value, target.clone()),
            json!({"name": expected}),
            "{value}"
        );
    }
}

#[test]
fn test_integer_categorical() {
    let target = json!({
        "name": "name",
        "type": "categorical",
        "property": {
            "type": "integer",
            "format": "categorical",
            "categories": [{"value": 1, "label": "Low"}, {"value": 2, "label": "High"}],
        },
    });
    for (value, expected) in [("Low", 1), ("High", 2)] {
        assert_eq!(
            get_row(value, target.clone()),
            json!({"name": expected}),
            "{value}"
        );
    }
}
