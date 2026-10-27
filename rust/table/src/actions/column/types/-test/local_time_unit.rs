use fairspec_metadata::LocalTimeColumn;
use polars::prelude::*;
use serde_json::json;

use crate::actions::column::types::local_time::{
    parse_local_time_column, stringify_local_time_column,
};

fn create_column(temporal_format: Option<&str>) -> LocalTimeColumn {
    let mut property = json!({"type": "string", "format": "local-time"});
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
        .select([parse_local_time_column(&column, col("name"))])
        .collect()
        .unwrap();
    assert_eq!(frame.column("name").unwrap().dtype(), &DataType::Time);
    let frame = frame
        .lazy()
        .select([col("name").dt().strftime("%H:%M:%S%.6f")])
        .collect()
        .unwrap();
    let value = frame.column("name").unwrap().str().unwrap().get(0);
    value.map(str::to_string)
}

fn stringify(temporal_format: Option<&str>, nanos: i64) -> Option<String> {
    let column = create_column(temporal_format);
    let series = Series::new("name".into(), [nanos])
        .cast(&DataType::Time)
        .unwrap();
    let frame = DataFrame::new_infer_height(vec![series.into()])
        .unwrap()
        .lazy()
        .select([stringify_local_time_column(&column, col("name"))])
        .collect()
        .unwrap();
    let value = frame.column("name").unwrap().str().unwrap().get(0);
    value.map(str::to_string)
}

fn time_nanos(hour: i64, minute: i64) -> i64 {
    (hour * 60 + minute) * 60_000_000_000
}

#[test]
fn test_parse_default() {
    let cases = [
        ("06:00:00", Some("06:00:00.000000")),
        ("06:00:00.5", Some("06:00:00.500000")),
        ("23:59:60", Some("23:59:59.000000")),
        ("12:30:60", None),
        ("06:00:00Z", None),
        ("06:00:00+02:00", None),
        ("09:00", None),
        ("3 am", None),
        ("3.00", None),
        ("invalid", None),
        ("", None),
    ];
    for (cell, expected) in cases {
        assert_eq!(parse(None, cell).as_deref(), expected, "{cell:?}");
    }
}

#[test]
fn test_parse_temporal_format_hm() {
    let cases = [
        ("06:00", Some("06:00:00.000000")),
        ("06:50", Some("06:50:00.000000")),
        ("invalid", None),
        ("", None),
    ];
    for (cell, expected) in cases {
        assert_eq!(parse(Some("%H:%M"), cell).as_deref(), expected, "{cell:?}");
    }
}

#[test]
fn test_stringify_default() {
    let cases = [
        (time_nanos(6, 0), "06:00:00"),
        (time_nanos(16, 30), "16:30:00"),
    ];
    for (value, expected) in cases {
        assert_eq!(stringify(None, value).as_deref(), Some(expected));
    }
}

#[test]
fn test_stringify_temporal_format_hm() {
    let cases = [(time_nanos(6, 0), "06:00"), (time_nanos(16, 30), "16:30")];
    for (value, expected) in cases {
        assert_eq!(stringify(Some("%H:%M"), value).as_deref(), Some(expected));
    }
}
