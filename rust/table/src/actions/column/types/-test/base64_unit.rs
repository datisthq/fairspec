use fairspec_metadata::Base64Column;
use polars::prelude::*;
use serde_json::{Value, json};

use crate::actions::column::types::base64::parse_base64_column;
use crate::helpers::value::get_json_value;

fn get_row(cell: &str) -> Value {
    let column: Base64Column = serde_json::from_value(
        json!({"name": "name", "property": {"type": "string", "format": "base64"}}),
    )
    .unwrap();
    let table = df!("name" => [cell]).unwrap().lazy();
    let frame = table
        .select([parse_base64_column(&column, col("name"))])
        .collect()
        .unwrap();
    let column = &frame.columns()[0];
    json!({column.name().as_str(): get_json_value(&column.get(0).unwrap())})
}

#[test]
fn test_parse() {
    let cases = [
        ("SGVsbG8gV29ybGQ=", Some("SGVsbG8gV29ybGQ=")),
        ("YWJjZGVm", Some("YWJjZGVm")),
        ("YWJjZA", None),
        ("-_-_", None),
        ("!!!invalid!!!", None),
        ("not base64", None),
    ];
    for (cell, expected) in cases {
        assert_eq!(get_row(cell), json!({"name": expected}), "{cell}");
    }
}
