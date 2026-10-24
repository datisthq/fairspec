use fairspec_metadata::EmailColumn;
use polars::prelude::*;
use serde_json::{Value, json};

use crate::actions::column::types::email::parse_email_column;
use crate::helpers::value::get_json_value;

fn get_row(cell: &str) -> Value {
    let column: EmailColumn = serde_json::from_value(
        json!({"name": "name", "property": {"type": "string", "format": "email"}}),
    )
    .unwrap();
    let table = df!("name" => [cell]).unwrap().lazy();
    let frame = table
        .select([parse_email_column(&column, col("name"))])
        .collect()
        .unwrap();
    let column = &frame.columns()[0];
    json!({column.name().as_str(): get_json_value(&column.get(0).unwrap())})
}

#[test]
fn test_parse() {
    let cases = [
        ("user@example.com", Some("user@example.com")),
        ("test.name@domain.org", Some("test.name@domain.org")),
        ("user+tag@example.co.uk", Some("user+tag@example.co.uk")),
        ("", None),
        ("invalid", None),
        ("@example.com", None),
        ("user@", None),
        ("user example.com", None),
    ];
    for (cell, expected) in cases {
        assert_eq!(get_row(cell), json!({"name": expected}), "{cell}");
    }
}
