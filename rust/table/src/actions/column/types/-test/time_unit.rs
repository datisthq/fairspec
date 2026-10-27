use fairspec_metadata::TimeColumn;
use polars::prelude::*;
use serde_json::json;

use crate::actions::column::types::time::{parse_time_column, stringify_time_column};

fn create_column(temporal_format: Option<&str>) -> TimeColumn {
    let mut property = json!({"type": "string", "format": "time"});
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
        .select([parse_time_column(&column, col("name"))])
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
        .select([stringify_time_column(&column, col("name"))])
        .collect()
        .unwrap();
    let value = frame.column("name").unwrap().str().unwrap().get(0);
    value.map(str::to_string)
}

fn time_nanos(hour: i64, minute: i64, micro: i64) -> i64 {
    ((hour * 60 + minute) * 60_000_000 + micro) * 1000
}

#[test]
fn test_parse_default() {
    let cases = [
        ("06:00:00Z", Some("06:00:00.000000")),
        ("06:00:00+02:00", Some("04:00:00.000000")),
        ("01:00:00+02:00", Some("23:00:00.000000")),
        ("06:00:00.5-05:00", Some("11:00:00.500000")),
        ("06:00:00z", Some("06:00:00.000000")),
        ("23:59:60Z", Some("23:59:59.000000")),
        ("15:59:60-08:00", Some("23:59:59.000000")),
        ("12:30:60Z", None),
        ("23:59:60+01:00", None),
        ("06:00:00", None),
        ("06:00:00+0200", None),
        ("09:00Z", None),
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
        ("06:00+02:00", Some("04:00:00.000000")),
        ("06:00+0200", Some("04:00:00.000000")),
        ("06:00", None),
        ("invalid", None),
    ];
    for (cell, expected) in cases {
        assert_eq!(
            parse(Some("%H:%M%:z"), cell).as_deref(),
            expected,
            "{cell:?}"
        );
    }
}

#[test]
fn test_stringify_default() {
    let cases = [
        (time_nanos(6, 0, 0), "06:00:00Z"),
        (time_nanos(16, 30, 500000), "16:30:00.500Z"),
    ];
    for (value, expected) in cases {
        assert_eq!(stringify(None, value).as_deref(), Some(expected));
    }
}

#[test]
fn test_stringify_temporal_format() {
    assert_eq!(
        stringify(Some("%H:%M%z"), time_nanos(16, 30, 0)).as_deref(),
        Some("16:30+0000")
    );
}
