use fairspec_metadata::ListColumn;
use polars::prelude::*;
use serde_json::{Value, json};

use crate::actions::column::types::list::{parse_list_column, stringify_list_column};

fn create_column(extra: Value) -> ListColumn {
    let mut property = json!({"type": "string", "format": "list"});
    property
        .as_object_mut()
        .unwrap()
        .extend(extra.as_object().unwrap().clone());
    serde_json::from_value(json!({"name": "name", "property": property})).unwrap()
}

fn parse(extra: Value, cell: &str) -> Column {
    let column = create_column(extra);
    let frame = df!("name" => [cell])
        .unwrap()
        .lazy()
        .select([parse_list_column(&column, col("name")).alias("name")])
        .collect()
        .unwrap();
    frame.column("name").unwrap().clone()
}

fn get_items(column: &Column) -> Option<Series> {
    column.list().unwrap().get_as_series(0)
}

fn parse_strings(extra: Value, cell: &str) -> Option<Vec<Option<String>>> {
    let column = parse(extra, cell);
    assert_eq!(column.dtype(), &DataType::List(Box::new(DataType::String)));
    get_items(&column).map(|items| {
        let items = items.str().unwrap();
        items.iter().map(|item| item.map(str::to_string)).collect()
    })
}

fn parse_integers(cell: &str) -> Option<Vec<Option<i64>>> {
    let column = parse(json!({"itemType": "integer"}), cell);
    assert_eq!(column.dtype(), &DataType::List(Box::new(DataType::Int64)));
    get_items(&column).map(|items| items.i64().unwrap().iter().collect())
}

fn parse_numbers(cell: &str) -> Option<Vec<Option<f64>>> {
    let column = parse(json!({"itemType": "number"}), cell);
    assert_eq!(column.dtype(), &DataType::List(Box::new(DataType::Float64)));
    get_items(&column).map(|items| items.f64().unwrap().iter().collect())
}

fn stringify(extra: Value, items: Series) -> Option<String> {
    let column = create_column(extra);
    let series = Series::new("name".into(), [items]);
    let frame = DataFrame::new_infer_height(vec![series.into()])
        .unwrap()
        .lazy()
        .select([stringify_list_column(&column, col("name")).alias("name")])
        .collect()
        .unwrap();
    let value = frame.column("name").unwrap().str().unwrap().get(0);
    value.map(str::to_string)
}

fn create_strings(items: &[Option<&str>]) -> Series {
    Series::new("".into(), items)
        .cast(&DataType::String)
        .unwrap()
}

fn to_owned(items: &[Option<&str>]) -> Vec<Option<String>> {
    items.iter().map(|item| item.map(str::to_string)).collect()
}

#[test]
fn test_parse_default() {
    let cases: [(&str, &[Option<&str>]); 7] = [
        ("a,b,c", &[Some("a"), Some("b"), Some("c")]),
        ("1,2,3", &[Some("1"), Some("2"), Some("3")]),
        ("foo,bar,baz", &[Some("foo"), Some("bar"), Some("baz")]),
        ("single", &[Some("single")]),
        ("a,,c", &[Some("a"), Some(""), Some("c")]),
        (",b,", &[Some(""), Some("b"), Some("")]),
        (",,,", &[Some(""), Some(""), Some(""), Some("")]),
    ];
    for (cell, expected) in cases {
        assert_eq!(
            parse_strings(json!({}), cell),
            Some(to_owned(expected)),
            "{cell:?}"
        );
    }
}

#[test]
fn test_parse_items_integer() {
    let cases: [(&str, Option<&[Option<i64>]>); 8] = [
        ("1,2,3", Some(&[Some(1), Some(2), Some(3)])),
        ("0,-1,42", Some(&[Some(0), Some(-1), Some(42)])),
        ("-10,0,10", Some(&[Some(-10), Some(0), Some(10)])),
        ("42", Some(&[Some(42)])),
        ("1,,3", Some(&[Some(1), None, Some(3)])),
        (",2,", Some(&[None, Some(2), None])),
        ("1,a,3", None),
        ("1.5,2,3", None),
    ];
    for (cell, expected) in cases {
        assert_eq!(
            parse_integers(cell),
            expected.map(<[_]>::to_vec),
            "{cell:?}"
        );
    }
}

#[test]
#[allow(clippy::approx_constant)]
fn test_parse_items_number() {
    let cases: [(&str, Option<&[Option<f64>]>); 7] = [
        ("1.5,2.1,3.7", Some(&[Some(1.5), Some(2.1), Some(3.7)])),
        ("0,-1.1,42", Some(&[Some(0.0), Some(-1.1), Some(42.0)])),
        ("-10.5,0,10", Some(&[Some(-10.5), Some(0.0), Some(10.0)])),
        ("3.14", Some(&[Some(3.14)])),
        ("1.1,,3.3", Some(&[Some(1.1), None, Some(3.3)])),
        (",2.2,", Some(&[None, Some(2.2), None])),
        ("1.1,a,3.3", None),
    ];
    for (cell, expected) in cases {
        assert_eq!(parse_numbers(cell), expected.map(<[_]>::to_vec), "{cell:?}");
    }
}

#[test]
fn test_parse_items_date() {
    let cases = [
        ("2024-01-15,2024-02-29", true),
        ("2024-01-15,2024-1-5", false),
        ("2024-01-15, 2024-01-16", false),
    ];
    for (cell, valid) in cases {
        let column = parse(json!({"itemType": "date"}), cell);
        assert_eq!(column.dtype(), &DataType::List(Box::new(DataType::Date)));
        assert_eq!(get_items(&column).is_some(), valid, "{cell:?}");
    }
}

#[test]
fn test_parse_delimiter_semicolon() {
    let cases: [(&str, &[Option<&str>]); 4] = [
        ("a;b;c", &[Some("a"), Some("b"), Some("c")]),
        ("1;2;3", &[Some("1"), Some("2"), Some("3")]),
        ("single", &[Some("single")]),
        ("a;;c", &[Some("a"), Some(""), Some("c")]),
    ];
    for (cell, expected) in cases {
        assert_eq!(
            parse_strings(json!({"delimiter": ";"}), cell),
            Some(to_owned(expected)),
            "{cell:?}"
        );
    }
}

#[test]
fn test_stringify_default() {
    let cases: [(&[Option<&str>], &str); 10] = [
        (&[Some("a"), Some("b"), Some("c")], "a,b,c"),
        (&[Some("foo"), Some("bar"), Some("baz")], "foo,bar,baz"),
        (&[Some("1"), Some("2"), Some("3")], "1,2,3"),
        (&[Some("single")], "single"),
        (&[Some("a"), Some(""), Some("c")], "a,,c"),
        (&[Some(""), Some("b"), Some("")], ",b,"),
        (&[Some(""), Some(""), Some(""), Some("")], ",,,"),
        (&[None, Some("b"), None], "b"),
        (&[Some("a"), None, Some("c")], "a,c"),
        (&[], ""),
    ];
    for (value, expected) in cases {
        assert_eq!(
            stringify(json!({}), create_strings(value)).as_deref(),
            Some(expected),
            "{value:?}"
        );
    }
}

#[test]
fn test_stringify_items_integer() {
    let cases: [(&[Option<i16>], &str); 7] = [
        (&[Some(1), Some(2), Some(3)], "1,2,3"),
        (&[Some(0), Some(-1), Some(42)], "0,-1,42"),
        (&[Some(-10), Some(0), Some(10)], "-10,0,10"),
        (&[Some(42)], "42"),
        (&[Some(1), None, Some(3)], "1,3"),
        (&[None, Some(2), None], "2"),
        (&[], ""),
    ];
    for (value, expected) in cases {
        let items = Series::new("".into(), value);
        assert_eq!(
            stringify(json!({"itemType": "integer"}), items).as_deref(),
            Some(expected),
            "{value:?}"
        );
    }
}

#[test]
#[allow(clippy::approx_constant)]
fn test_stringify_items_number() {
    let cases: [(&[Option<f64>], &str); 7] = [
        (&[Some(1.5), Some(2.1), Some(3.7)], "1.5,2.1,3.7"),
        (&[Some(0.0), Some(-1.1), Some(42.0)], "0.0,-1.1,42.0"),
        (&[Some(-10.5), Some(0.0), Some(10.0)], "-10.5,0.0,10.0"),
        (&[Some(3.14)], "3.14"),
        (&[Some(1.1), None, Some(3.3)], "1.1,3.3"),
        (&[None, Some(2.2), None], "2.2"),
        (&[], ""),
    ];
    for (value, expected) in cases {
        let items = Series::new("".into(), value);
        assert_eq!(
            stringify(json!({"itemType": "number"}), items).as_deref(),
            Some(expected),
            "{value:?}"
        );
    }
}

#[test]
fn test_stringify_delimiter_semicolon() {
    let cases: [(&[Option<&str>], &str); 6] = [
        (&[Some("a"), Some("b"), Some("c")], "a;b;c"),
        (&[Some("1"), Some("2"), Some("3")], "1;2;3"),
        (&[Some("single")], "single"),
        (&[Some("a"), Some(""), Some("c")], "a;;c"),
        (&[Some(""), Some("b"), Some("")], ";b;"),
        (&[], ""),
    ];
    for (value, expected) in cases {
        assert_eq!(
            stringify(json!({"delimiter": ";"}), create_strings(value)).as_deref(),
            Some(expected),
            "{value:?}"
        );
    }
}
