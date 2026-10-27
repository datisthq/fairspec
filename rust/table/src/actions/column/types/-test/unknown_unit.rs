#![allow(clippy::approx_constant)]

use fairspec_metadata::UnknownColumn;
use polars::prelude::*;
use serde_json::{Value, json};

use crate::actions::column::types::unknown::stringify_unknown_column;
use crate::helpers::value::get_json_value;

fn get_row(series: Series) -> Value {
    let column: UnknownColumn =
        serde_json::from_value(json!({"name": "name", "type": "unknown", "property": {}})).unwrap();
    let table = DataFrame::new(1, vec![series.into()]).unwrap().lazy();
    let frame = table
        .select([stringify_unknown_column(&column, col("name"))])
        .collect()
        .unwrap();
    let column = &frame.columns()[0];
    json!({column.name().as_str(): get_json_value(&column.get(0).unwrap())})
}

#[test]
fn test_stringify() {
    let cases = [
        (Series::new("name".into(), [1.0]), "1.0"),
        (Series::new("name".into(), [3.14]), "3.14"),
        (Series::new("name".into(), [true]), "true"),
        (Series::new("name".into(), [false]), "false"),
        (Series::new("name".into(), ["text"]), "text"),
    ];
    for (series, expected) in cases {
        assert_eq!(get_row(series), json!({"name": expected}), "{expected}");
    }
}
