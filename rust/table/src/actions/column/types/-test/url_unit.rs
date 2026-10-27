use fairspec_metadata::UrlColumn;
use polars::prelude::*;
use serde_json::{Value, json};

use crate::actions::column::types::url::parse_url_column;
use crate::helpers::value::get_json_value;

fn get_row(cell: &str) -> Value {
    let column: UrlColumn = serde_json::from_value(
        json!({"name": "name", "property": {"type": "string", "format": "url"}}),
    )
    .unwrap();
    let table = df!("name" => [cell]).unwrap().lazy();
    let frame = table
        .select([parse_url_column(&column, col("name"))])
        .collect()
        .unwrap();
    let column = &frame.columns()[0];
    json!({column.name().as_str(): get_json_value(&column.get(0).unwrap())})
}

#[test]
fn test_parse() {
    let cases = [
        ("https://example.com", Some("https://example.com")),
        ("http://example.com", Some("http://example.com")),
        (
            "https://example.com/path?query=1",
            Some("https://example.com/path?query=1"),
        ),
        ("HTTPS://example.com", Some("HTTPS://example.com")),
        ("http://localhost", Some("http://localhost")),
        ("http:///path", None),
        ("https://exämple.com", None),
        ("https://example.com/a b", None),
        ("", None),
        ("example.com", None),
        ("ftp://example.com", None),
        ("not a url", None),
    ];
    for (cell, expected) in cases {
        assert_eq!(get_row(cell), json!({"name": expected}), "{cell}");
    }
}
