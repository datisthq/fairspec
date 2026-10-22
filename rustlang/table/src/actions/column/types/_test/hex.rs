use fairspec_metadata::HexColumn;
use polars::prelude::*;
use serde_json::{Value, json};

use crate::actions::column::types::hex::parse_hex_column;
use crate::helpers::value::get_json_value;

fn get_row(cell: &str) -> Value {
    let column: HexColumn = serde_json::from_value(
        json!({"name": "name", "property": {"type": "string", "format": "hex"}}),
    )
    .unwrap();
    let table = df!("name" => [cell]).unwrap().lazy();
    let frame = table
        .select([parse_hex_column(&column, col("name"))])
        .collect()
        .unwrap();
    let column = &frame.columns()[0];
    json!({column.name().as_str(): get_json_value(&column.get(0).unwrap())})
}

#[test]
fn test_parse() {
    let cases = [
        ("0123456789abcdef", Some("0123456789abcdef")),
        ("ABCDEF", Some("ABCDEF")),
        ("deadbeef", Some("deadbeef")),
        ("ghijkl", None),
        ("0x1234", None),
        ("hello world", None),
    ];
    for (cell, expected) in cases {
        assert_eq!(get_row(cell), json!({"name": expected}), "{cell}");
    }
}
