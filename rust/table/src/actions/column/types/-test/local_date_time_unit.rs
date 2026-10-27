use chrono::NaiveDate;
use fairspec_metadata::LocalDateTimeColumn;
use polars::prelude::*;
use serde_json::json;

use crate::actions::column::types::local_date_time::{
    parse_local_date_time_column, stringify_local_date_time_column,
};

const RENDER_FORMAT: &str = "%Y-%m-%dT%H:%M:%S%.6f";

fn create_column(temporal_format: Option<&str>) -> LocalDateTimeColumn {
    let mut property = json!({"type": "string", "format": "local-date-time"});
    if let Some(temporal_format) = temporal_format {
        property["temporalFormat"] = json!(temporal_format);
    }
    serde_json::from_value(json!({"name": "name", "property": property})).unwrap()
}

fn get_data_type() -> DataType {
    DataType::Datetime(TimeUnit::Microseconds, None)
}

fn parse(temporal_format: Option<&str>, cell: &str) -> Option<String> {
    let column = create_column(temporal_format);
    let frame = df!("name" => [cell])
        .unwrap()
        .lazy()
        .select([parse_local_date_time_column(&column, col("name"))])
        .collect()
        .unwrap();
    assert_eq!(frame.column("name").unwrap().dtype(), &get_data_type());
    let frame = frame
        .lazy()
        .select([col("name").dt().strftime(RENDER_FORMAT)])
        .collect()
        .unwrap();
    let value = frame.column("name").unwrap().str().unwrap().get(0);
    value.map(str::to_string)
}

fn stringify(temporal_format: Option<&str>, micros: i64) -> Option<String> {
    let column = create_column(temporal_format);
    let series = Series::new("name".into(), [micros])
        .cast(&get_data_type())
        .unwrap();
    let frame = DataFrame::new_infer_height(vec![series.into()])
        .unwrap()
        .lazy()
        .select([stringify_local_date_time_column(&column, col("name"))])
        .collect()
        .unwrap();
    let value = frame.column("name").unwrap().str().unwrap().get(0);
    value.map(str::to_string)
}

fn local_micros(year: i32, month: u32, day: u32, hour: u32, minute: u32, micro: u32) -> i64 {
    NaiveDate::from_ymd_opt(year, month, day)
        .unwrap()
        .and_hms_micro_opt(hour, minute, 0, micro)
        .unwrap()
        .and_utc()
        .timestamp_micros()
}

#[test]
fn test_parse_default() {
    let cases = [
        ("2014-01-01T06:00:00", Some("2014-01-01T06:00:00.000000")),
        ("2014-01-01T06:00:00Z", None),
        ("2014-01-01T06:00:00.25", Some("2014-01-01T06:00:00.250000")),
        ("2014-01-01T06:00:00-05:00", None),
        ("2014-01-01T06:00:00x", None),
        ("2014-01-01T06:00", None),
        ("2014-01-01", None),
        ("Mon 1st Jan 2014 9 am", None),
        ("invalid", None),
        ("", None),
        ("2014-1-01T06:00:00", None),
        ("2014-01-01T6:00:00", None),
        (" 2014-01-01T06:00:00", None),
        ("+2014-01-01T06:00:00", None),
    ];
    for (cell, expected) in cases {
        assert_eq!(parse(None, cell).as_deref(), expected, "{cell:?}");
    }
}

#[test]
fn test_parse_temporal_format() {
    let cases = [
        ("21/11/2006 16:30", Some("2006-11-21T16:30:00.000000")),
        ("21/11/2006 16:30:00", None),
        ("16:30 21/11/06", None),
        ("invalid", None),
        ("", None),
    ];
    for (cell, expected) in cases {
        assert_eq!(
            parse(Some("%d/%m/%Y %H:%M"), cell).as_deref(),
            expected,
            "{cell:?}"
        );
    }
}

#[test]
fn test_parse_invalid_temporal_format() {
    assert_eq!(parse(Some("invalid"), "21/11/06 16:30"), None);
}

#[test]
fn test_stringify_default() {
    let cases = [
        (local_micros(2014, 1, 1, 6, 0, 0), "2014-01-01T06:00:00"),
        (local_micros(2006, 11, 21, 16, 30, 0), "2006-11-21T16:30:00"),
        (
            local_micros(2006, 11, 21, 16, 30, 125000),
            "2006-11-21T16:30:00.125",
        ),
    ];
    for (value, expected) in cases {
        assert_eq!(stringify(None, value).as_deref(), Some(expected));
    }
}

#[test]
fn test_stringify_temporal_format_dmy_hm() {
    assert_eq!(
        stringify(
            Some("%d/%m/%Y %H:%M"),
            local_micros(2006, 11, 21, 16, 30, 0)
        )
        .as_deref(),
        Some("21/11/2006 16:30")
    );
}

#[test]
fn test_stringify_temporal_format_ymd_hms() {
    assert_eq!(
        stringify(Some("%Y/%m/%dT%H:%M:%S"), local_micros(2014, 1, 1, 6, 0, 0)).as_deref(),
        Some("2014/01/01T06:00:00")
    );
}
