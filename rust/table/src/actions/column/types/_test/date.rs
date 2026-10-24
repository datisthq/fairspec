use chrono::NaiveDate;
use fairspec_metadata::DateColumn;
use polars::prelude::*;
use serde_json::json;

use crate::actions::column::types::date::{parse_date_column, stringify_date_column};

fn create_column(temporal_format: Option<&str>) -> DateColumn {
    let mut property = json!({"type": "string", "format": "date"});
    if let Some(temporal_format) = temporal_format {
        property["temporalFormat"] = json!(temporal_format);
    }
    serde_json::from_value(json!({"name": "name", "type": "date", "property": property})).unwrap()
}

fn parse(temporal_format: Option<&str>, cell: &str) -> Option<String> {
    let column = create_column(temporal_format);
    let frame = df!("name" => [cell])
        .unwrap()
        .lazy()
        .select([parse_date_column(&column, col("name"))])
        .collect()
        .unwrap();
    assert_eq!(frame.column("name").unwrap().dtype(), &DataType::Date);
    let frame = frame
        .lazy()
        .select([col("name").dt().strftime("%Y-%m-%d")])
        .collect()
        .unwrap();
    let value = frame.column("name").unwrap().str().unwrap().get(0);
    value.map(str::to_string)
}

fn stringify(temporal_format: Option<&str>, value: NaiveDate) -> Option<String> {
    let column = create_column(temporal_format);
    let days = value
        .signed_duration_since(NaiveDate::from_ymd_opt(1970, 1, 1).unwrap())
        .num_days() as i32;
    let series = Series::new("name".into(), [days])
        .cast(&DataType::Date)
        .unwrap();
    let frame = DataFrame::new_infer_height(vec![series.into()])
        .unwrap()
        .lazy()
        .select([stringify_date_column(&column, col("name"))])
        .collect()
        .unwrap();
    let value = frame.column("name").unwrap().str().unwrap().get(0);
    value.map(str::to_string)
}

fn date(year: i32, month: u32, day: u32) -> NaiveDate {
    NaiveDate::from_ymd_opt(year, month, day).unwrap()
}

#[test]
fn test_parse_default() {
    let cases = [
        ("2019-01-01", Some("2019-01-01")),
        ("10th Jan 1969", None),
        ("invalid", None),
        ("", None),
    ];
    for (cell, expected) in cases {
        assert_eq!(parse(None, cell).as_deref(), expected, "{cell:?}");
    }
}

#[test]
fn test_parse_temporal_format_dmy() {
    let cases = [
        ("21/11/2006", Some("2006-11-21")),
        ("invalid", None),
        ("", None),
    ];
    for (cell, expected) in cases {
        assert_eq!(
            parse(Some("%d/%m/%Y"), cell).as_deref(),
            expected,
            "{cell:?}"
        );
    }
}

#[test]
fn test_parse_temporal_format_ymd() {
    assert_eq!(
        parse(Some("%Y/%m/%d"), "2006/11/21").as_deref(),
        Some("2006-11-21")
    );
}

#[test]
fn test_parse_invalid_temporal_format() {
    assert_eq!(parse(Some("invalid"), "21/11/06"), None);
}

#[test]
fn test_stringify_default() {
    let cases = [
        (date(2019, 1, 1), "2019-01-01"),
        (date(2006, 11, 21), "2006-11-21"),
    ];
    for (value, expected) in cases {
        assert_eq!(stringify(None, value).as_deref(), Some(expected));
    }
}

#[test]
fn test_stringify_temporal_format_dmy() {
    assert_eq!(
        stringify(Some("%d/%m/%Y"), date(2006, 11, 21)).as_deref(),
        Some("21/11/2006")
    );
}

#[test]
fn test_stringify_temporal_format_ymd() {
    assert_eq!(
        stringify(Some("%Y/%m/%d"), date(2006, 11, 21)).as_deref(),
        Some("2006/11/21")
    );
}
