use fairspec_metadata::DecimalColumn;
use polars::prelude::*;
use serde_json::{Value, json};

use crate::actions::column::types::decimal::{parse_decimal_column, stringify_decimal_column};
use crate::helpers::value::get_json_value;

fn create_column(property: Value) -> DecimalColumn {
    serde_json::from_value(json!({"name": "name", "property": property})).unwrap()
}

/// Python compares `Decimal`s by value, so trailing zeros of the scale are dropped
fn get_decimal_row(table: LazyFrame, expr: Expr) -> Value {
    let frame = table.select([expr]).collect().unwrap();
    let column = &frame.columns()[0];
    assert!(matches!(column.dtype(), DataType::Decimal(_, _)));
    let text = column.cast(&DataType::String).unwrap();
    let value = match text.get(0).unwrap().get_str() {
        Some(text) if text.contains('.') => {
            Value::from(text.trim_end_matches('0').trim_end_matches('.'))
        }
        Some(text) => Value::from(text),
        None => Value::Null,
    };
    json!({column.name().as_str(): value})
}

fn assert_parse(property: Value, cases: &[(&str, Option<&str>)]) {
    let column = create_column(property);
    for (cell, expected) in cases {
        let table = df!("name" => [*cell]).unwrap().lazy();
        let row = get_decimal_row(table, parse_decimal_column(&column, col("name")));
        assert_eq!(row, json!({"name": expected}), "{cell}");
    }
}

#[test]
fn test_parse_default() {
    assert_parse(
        json!({"type": "string", "format": "decimal"}),
        &[
            ("1", Some("1")),
            ("2", Some("2")),
            ("1000", Some("1000")),
            ("1.5", Some("1.5")),
            ("4.14159", Some("4.14159")),
            ("-42", Some("-42")),
            ("-3.14", Some("-3.14")),
            ("", None),
            ("bad", None),
            ("text", None),
        ],
    );
}

#[test]
fn test_parse_group_char_comma() {
    assert_parse(
        json!({"type": "string", "format": "decimal", "groupChar": ","}),
        &[
            ("1", Some("1")),
            ("1,000", Some("1000")),
            ("1,000,000", Some("1000000")),
            ("1,234.56", Some("1234.56")),
        ],
    );
}

#[test]
fn test_parse_decimal_char_comma() {
    assert_parse(
        json!({"type": "string", "format": "decimal", "decimalChar": ","}),
        &[("1,5", Some("1.5")), ("3,14", Some("3.14"))],
    );
}

#[test]
fn test_parse_group_char_dot_decimal_char_comma() {
    assert_parse(
        json!({"type": "string", "format": "decimal", "groupChar": ".", "decimalChar": ","}),
        &[("1.234,56", Some("1234.56")), ("1.000,00", Some("1000"))],
    );
}

#[test]
fn test_parse_with_text() {
    assert_parse(
        json!({"type": "string", "format": "decimal", "withText": true}),
        &[
            ("$1.5", Some("1.5")),
            ("1.5%", Some("1.5")),
            ("\u{20ac}1000", Some("1000")),
            ("1000\u{20ac}", Some("1000")),
        ],
    );
}

#[test]
fn test_parse_with_text_and_group_char() {
    assert_parse(
        json!({"type": "string", "format": "decimal", "withText": true, "groupChar": ","}),
        &[("$1,000.00", Some("1000")), ("1,234.56$", Some("1234.56"))],
    );
}

#[test]
fn test_parse_with_text_group_char_dot_decimal_char_comma() {
    assert_parse(
        json!({
            "type": "string",
            "format": "decimal",
            "withText": true,
            "groupChar": ".",
            "decimalChar": ",",
        }),
        &[
            ("\u{20ac} 1.000,00", Some("1000")),
            ("1.000,00 \u{20ac}", Some("1000")),
            ("1.234,56 \u{20ac}", Some("1234.56")),
        ],
    );
}

#[test]
fn test_stringify_default() {
    let column = create_column(json!({"type": "string", "format": "decimal"}));
    let cases = [
        ("1.0", "1.0"),
        ("2.0", "2.0"),
        ("1000.0", "1000.0"),
        ("3.14", "3.14"),
        ("42.5", "42.5"),
        ("-1.0", "-1.0"),
        ("-100.5", "-100.5"),
        ("0.0", "0.0"),
    ];
    for (value, expected) in cases {
        // A one-value Python series infers its scale from the value's digits
        let scale = value.split('.').nth(1).map_or(0, str::len);
        let series = Series::new("name".into(), [value])
            .cast(&DataType::Decimal(38, scale))
            .unwrap();
        let table = DataFrame::new(1, vec![series.into()]).unwrap().lazy();
        let frame = table
            .select([stringify_decimal_column(&column, col("name"))])
            .collect()
            .unwrap();
        let column = &frame.columns()[0];
        let row = json!({column.name().as_str(): get_json_value(&column.get(0).unwrap())});
        assert_eq!(row, json!({"name": expected}), "{value}");
    }
}
