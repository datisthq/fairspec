use fairspec_metadata::IntegerColumn;
use polars::prelude::*;
use serde_json::{Value, json};

use crate::actions::column::types::integer::{parse_integer_column, stringify_integer_column};
use crate::helpers::value::get_json_value;

fn create_column(property: Value) -> IntegerColumn {
    serde_json::from_value(json!({"name": "name", "property": property})).unwrap()
}

fn get_row(table: LazyFrame, expr: Expr) -> Value {
    let frame = table.select([expr]).collect().unwrap();
    let column = &frame.columns()[0];
    json!({column.name().as_str(): get_json_value(&column.get(0).unwrap())})
}

fn assert_parse(property: Value, cases: &[(&str, Option<i64>)]) {
    let column = create_column(property);
    for (cell, expected) in cases {
        let table = df!("name" => [*cell]).unwrap().lazy();
        let row = get_row(table, parse_integer_column(&column, col("name")));
        assert_eq!(row, json!({"name": expected}), "{cell}");
    }
}

#[test]
fn test_parse_default() {
    assert_parse(
        json!({"type": "integer"}),
        &[
            ("1", Some(1)),
            ("2", Some(2)),
            ("1000", Some(1000)),
            ("0", Some(0)),
            ("00", None),
            ("01", None),
            ("000835", None),
            ("-0", Some(0)),
            ("1.0", Some(1)),
            ("1e3", Some(1000)),
            ("1.50e1", Some(15)),
            ("100e-2", Some(1)),
            ("1e-1", None),
            ("+1", None),
            (" 1", None),
            ("1.", None),
            ("9223372036854775807", Some(i64::MAX)),
            ("-9223372036854775808", Some(i64::MIN)),
            ("9223372036854775807.0", Some(i64::MAX)),
            ("9223372036854775808", None),
            ("-9223372036854775809", None),
            ("1e19", None),
            ("1e400", None),
            ("0e99999999999999999999", Some(0)),
            ("1e9223372036854775807", None),
            ("11e9223372036854775806", None),
            ("1.5e-9223372036854775808", None),
            ("", None),
            ("2.1", None),
            ("bad", None),
            ("0.0003", None),
            ("3.14", None),
            ("1/2", None),
        ],
    );
}

#[test]
fn test_parse_group_char_comma() {
    assert_parse(
        json!({"type": "integer", "groupChar": ","}),
        &[
            ("1", Some(1)),
            ("1,000", Some(1000)),
            ("1,000,000", Some(1000000)),
            ("000,001", None),
        ],
    );
}

#[test]
fn test_parse_group_char_other() {
    let cases = [
        ("1 000", " ", 1000),
        ("1'000'000", "'", 1000000),
        ("1.000.000", ".", 1000000),
    ];
    for (cell, group_char, expected) in cases {
        assert_parse(
            json!({"type": "integer", "groupChar": group_char}),
            &[(cell, Some(expected))],
        );
    }
}

#[test]
fn test_parse_with_text() {
    assert_parse(
        json!({"type": "integer", "withText": true}),
        &[
            ("1", Some(1)),
            ("1000", Some(1000)),
            ("$1000", Some(1000)),
            ("1000$", Some(1000)),
            ("\u{20ac}1000", Some(1000)),
            ("1000\u{20ac}", Some(1000)),
            ("-12\u{20ac}", Some(-12)),
            ("\u{20ac}-12", Some(-12)),
            ("1,000", None),
        ],
    );
}

#[test]
fn test_parse_with_text_and_group_char() {
    assert_parse(
        json!({"type": "integer", "groupChar": ",", "withText": true}),
        &[("$1,000,000", Some(1000000)), ("1,000,000$", Some(1000000))],
    );
}

#[test]
fn test_stringify_default() {
    let column = create_column(json!({"type": "integer"}));
    let cases = [
        (1, "1"),
        (2, "2"),
        (1000, "1000"),
        (42, "42"),
        (-1, "-1"),
        (-100, "-100"),
        (0, "0"),
        (1234567890, "1234567890"),
        (-1234567890, "-1234567890"),
    ];
    for (value, expected) in cases {
        let table = df!("name" => [value as i64]).unwrap().lazy();
        let row = get_row(table, stringify_integer_column(&column, col("name")));
        assert_eq!(row, json!({"name": expected}), "{value}");
    }
}
