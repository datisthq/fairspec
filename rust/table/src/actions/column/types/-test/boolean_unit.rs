use fairspec_metadata::BooleanColumn;
use polars::prelude::*;
use serde_json::{Value, json};

use crate::actions::column::types::boolean::{parse_boolean_column, stringify_boolean_column};
use crate::helpers::value::get_json_value;

fn create_column(property: Value) -> BooleanColumn {
    serde_json::from_value(json!({"name": "name", "property": property})).unwrap()
}

fn get_row(table: LazyFrame, expr: Expr) -> Value {
    let frame = table.select([expr]).collect().unwrap();
    let column = &frame.columns()[0];
    json!({column.name().as_str(): get_json_value(&column.get(0).unwrap())})
}

fn assert_parse(property: Value, cases: &[(&str, Option<bool>)]) {
    let column = create_column(property);
    for (cell, expected) in cases {
        let table = df!("name" => [*cell]).unwrap().lazy();
        let row = get_row(table, parse_boolean_column(&column, col("name")));
        assert_eq!(row, json!({"name": expected}), "{cell}");
    }
}

fn assert_stringify(property: Value, cases: &[(bool, &str)]) {
    let column = create_column(property);
    for (value, expected) in cases {
        let table = df!("name" => [*value]).unwrap().lazy();
        let row = get_row(table, stringify_boolean_column(&column, col("name")));
        assert_eq!(row, json!({"name": expected}), "{value}");
    }
}

#[test]
fn test_parse_default() {
    assert_parse(
        json!({"type": "boolean"}),
        &[
            ("true", Some(true)),
            ("True", Some(true)),
            ("TRUE", Some(true)),
            ("1", Some(true)),
            ("false", Some(false)),
            ("False", Some(false)),
            ("FALSE", Some(false)),
            ("0", Some(false)),
            ("", None),
            ("invalid", None),
            ("truthy", None),
            ("falsy", None),
            ("2", None),
            ("-100", None),
            ("t", None),
            ("f", None),
            ("3.14", None),
            ("01", None),
            ("+1", None),
            ("-0", None),
            ("true ", None),
            (" true", None),
        ],
    );
}

#[test]
fn test_parse_true_values() {
    assert_parse(
        json!({"type": "boolean", "trueValues": ["Y", "y", "yes"]}),
        &[
            ("Y", Some(true)),
            ("y", Some(true)),
            ("yes", Some(true)),
            ("true", None),
            ("1", None),
            ("0", Some(false)),
        ],
    );
}

#[test]
fn test_parse_false_values() {
    assert_parse(
        json!({"type": "boolean", "falseValues": ["N", "n", "no"]}),
        &[
            ("N", Some(false)),
            ("n", Some(false)),
            ("no", Some(false)),
            ("false", None),
            ("0", None),
            ("1", Some(true)),
        ],
    );
}

#[test]
fn test_parse_true_values_and_false_values() {
    assert_parse(
        json!({"type": "boolean", "trueValues": ["oui", "si"], "falseValues": ["non", "no"]}),
        &[
            ("oui", Some(true)),
            ("si", Some(true)),
            ("non", Some(false)),
            ("no", Some(false)),
            ("1", None),
            ("0", None),
        ],
    );
}

#[test]
fn test_stringify_default() {
    assert_stringify(
        json!({"type": "boolean"}),
        &[(true, "true"), (false, "false")],
    );
}

#[test]
fn test_stringify_true_values() {
    assert_stringify(
        json!({"type": "boolean", "trueValues": ["Y", "y", "yes"]}),
        &[(true, "Y"), (false, "false")],
    );
}

#[test]
fn test_stringify_false_values() {
    assert_stringify(
        json!({"type": "boolean", "falseValues": ["N", "n", "no"]}),
        &[(true, "true"), (false, "N")],
    );
}

#[test]
fn test_stringify_true_values_and_false_values() {
    assert_stringify(
        json!({"type": "boolean", "trueValues": ["oui", "si"], "falseValues": ["non", "no"]}),
        &[(true, "oui"), (false, "non")],
    );
}
