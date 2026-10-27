#![allow(clippy::approx_constant)]

use fairspec_metadata::NumberColumn;
use polars::prelude::*;
use serde_json::{Value, json};

use crate::actions::column::types::number::{parse_number_column, stringify_number_column};
use crate::helpers::value::get_json_value;

fn create_column(property: Value) -> NumberColumn {
    serde_json::from_value(json!({"name": "name", "property": property})).unwrap()
}

fn get_row(table: LazyFrame, expr: Expr) -> Value {
    let frame = table.select([expr]).collect().unwrap();
    let column = &frame.columns()[0];
    json!({column.name().as_str(): get_json_value(&column.get(0).unwrap())})
}

fn assert_parse(property: Value, cases: &[(&str, Option<f64>)]) {
    let column = create_column(property);
    for (cell, expected) in cases {
        let table = df!("name" => [*cell]).unwrap().lazy();
        let row = get_row(table, parse_number_column(&column, col("name")));
        assert_eq!(row, json!({"name": expected}), "{cell}");
    }
}

#[test]
fn test_parse_default() {
    assert_parse(
        json!({"type": "number"}),
        &[
            ("1", Some(1.0)),
            ("2", Some(2.0)),
            ("1000", Some(1000.0)),
            ("1.5", Some(1.5)),
            ("4.14159", Some(4.14159)),
            ("-42", Some(-42.0)),
            ("-3.14", Some(-3.14)),
            ("", None),
            ("bad", None),
            ("text", None),
            ("1E-3", Some(0.001)),
            ("1.5e+1", Some(15.0)),
            ("+1", None),
            ("007", None),
            (".5", None),
            ("5.", None),
            (" 1", None),
            ("0x10", None),
            ("NaN", None),
            ("inf", None),
            ("-Infinity", None),
            ("1e", None),
        ],
    );
}

#[test]
fn test_parse_group_char_comma() {
    assert_parse(
        json!({"type": "number", "groupChar": ","}),
        &[
            ("1", Some(1.0)),
            ("1,000", Some(1000.0)),
            ("1,000,000", Some(1000000.0)),
            ("1,234.56", Some(1234.56)),
        ],
    );
}

#[test]
fn test_parse_decimal_char_comma() {
    assert_parse(
        json!({"type": "number", "decimalChar": ","}),
        &[("1,5", Some(1.5)), ("3,14", Some(3.14))],
    );
}

#[test]
fn test_parse_group_char_dot_decimal_char_comma() {
    assert_parse(
        json!({"type": "number", "groupChar": ".", "decimalChar": ","}),
        &[("1.234,56", Some(1234.56)), ("1.000,00", Some(1000.0))],
    );
}

#[test]
fn test_parse_with_text() {
    assert_parse(
        json!({"type": "number", "withText": true}),
        &[
            ("$1.5", Some(1.5)),
            ("1.5%", Some(1.5)),
            ("\u{20ac}1000", Some(1000.0)),
            ("1000\u{20ac}", Some(1000.0)),
        ],
    );
}

#[test]
fn test_parse_with_text_and_group_char() {
    assert_parse(
        json!({"type": "number", "withText": true, "groupChar": ","}),
        &[("$1,000.00", Some(1000.0)), ("1,234.56$", Some(1234.56))],
    );
}

#[test]
fn test_parse_with_text_group_char_dot_decimal_char_comma() {
    assert_parse(
        json!({"type": "number", "withText": true, "groupChar": ".", "decimalChar": ","}),
        &[
            ("\u{20ac} 1.000,00", Some(1000.0)),
            ("1.000,00 \u{20ac}", Some(1000.0)),
            ("1.234,56 \u{20ac}", Some(1234.56)),
        ],
    );
}

#[test]
fn test_stringify_default() {
    let column = create_column(json!({"type": "number"}));
    let cases = [
        (1.0, "1.0"),
        (2.0, "2.0"),
        (1000.0, "1000.0"),
        (3.14, "3.14"),
        (42.5, "42.5"),
        (-1.0, "-1.0"),
        (-100.5, "-100.5"),
        (0.0, "0.0"),
        (-123.456789, "-123.456789"),
        (1234567890.123, "1234567890.123"),
        (-9876543210.987, "-9876543210.987"),
        (0.001, "0.001"),
        (-0.0001, "-0.0001"),
    ];
    for (value, expected) in cases {
        let table = df!("name" => [value]).unwrap().lazy();
        let row = get_row(table, stringify_number_column(&column, col("name")));
        assert_eq!(row, json!({"name": expected}), "{value}");
    }
}
