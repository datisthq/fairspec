use chrono::NaiveDate;
use fairspec_metadata::DateTimeColumn;
use polars::prelude::*;
use serde_json::json;

use crate::actions::column::types::date_time::{
    parse_date_time_column, stringify_date_time_column,
};
use crate::actions::column::types::temporal::get_utc_datetime_type;

const RENDER_FORMAT: &str = "%Y-%m-%dT%H:%M:%S%.6f%:z";

fn create_column(temporal_format: Option<&str>) -> DateTimeColumn {
    let mut property = json!({"type": "string", "format": "date-time"});
    if let Some(temporal_format) = temporal_format {
        property["temporalFormat"] = json!(temporal_format);
    }
    serde_json::from_value(json!({"name": "name", "property": property})).unwrap()
}

fn parse(temporal_format: Option<&str>, cell: &str) -> Option<String> {
    let column = create_column(temporal_format);
    let frame = df!("name" => [cell])
        .unwrap()
        .lazy()
        .select([parse_date_time_column(&column, col("name"))])
        .collect()
        .unwrap();
    assert_eq!(
        frame.column("name").unwrap().dtype(),
        &get_utc_datetime_type()
    );
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
        .cast(&get_utc_datetime_type())
        .unwrap();
    let frame = DataFrame::new_infer_height(vec![series.into()])
        .unwrap()
        .lazy()
        .select([stringify_date_time_column(&column, col("name"))])
        .collect()
        .unwrap();
    let value = frame.column("name").unwrap().str().unwrap().get(0);
    value.map(str::to_string)
}

fn utc_micros(year: i32, month: u32, day: u32, hour: u32, minute: u32, micro: u32) -> i64 {
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
        (
            "2014-01-01T06:00:00Z",
            Some("2014-01-01T06:00:00.000000+00:00"),
        ),
        (
            "2014-01-01T06:00:00+00:00",
            Some("2014-01-01T06:00:00.000000+00:00"),
        ),
        (
            "2014-01-01T06:00:00-05:00",
            Some("2014-01-01T11:00:00.000000+00:00"),
        ),
        (
            "2014-01-01T06:00:00.25+02:00",
            Some("2014-01-01T04:00:00.250000+00:00"),
        ),
        ("2014-01-01T06:00:00", None),
        ("2014-01-01T06:00:00+02", None),
        ("2014-01-01T06:00:00+0200", None),
        ("2014-01-01T06:00:00Zx", None),
        ("x2014-01-01T06:00:00Z", None),
        ("2014-01-01", None),
        ("invalid", None),
        ("", None),
    ];
    for (cell, expected) in cases {
        assert_eq!(parse(None, cell).as_deref(), expected, "{cell:?}");
    }
}

#[test]
fn test_parse_temporal_format() {
    let cases = [
        (
            "21/11/2006 16:30+0200",
            Some("2006-11-21T14:30:00.000000+00:00"),
        ),
        (
            "21/11/2006 16:30+02:00",
            Some("2006-11-21T14:30:00.000000+00:00"),
        ),
        ("21/11/2006 16:30", None),
        ("invalid", None),
        ("", None),
    ];
    for (cell, expected) in cases {
        assert_eq!(
            parse(Some("%d/%m/%Y %H:%M%z"), cell).as_deref(),
            expected,
            "{cell:?}"
        );
    }
}

#[test]
fn test_stringify_default() {
    let cases = [
        (utc_micros(2014, 1, 1, 6, 0, 0), "2014-01-01T06:00:00Z"),
        (
            utc_micros(2014, 1, 1, 6, 0, 125000),
            "2014-01-01T06:00:00.125Z",
        ),
    ];
    for (value, expected) in cases {
        assert_eq!(stringify(None, value).as_deref(), Some(expected));
    }
}

#[test]
fn test_stringify_temporal_format() {
    assert_eq!(
        stringify(
            Some("%d/%m/%Y %H:%M%:z"),
            utc_micros(2006, 11, 21, 16, 30, 0)
        )
        .as_deref(),
        Some("21/11/2006 16:30+00:00")
    );
}
