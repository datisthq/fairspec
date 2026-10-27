use fairspec_metadata::DurationColumn;
use polars::prelude::*;
use serde_json::json;

use crate::actions::column::types::duration::inspect_duration_column;

fn get_is_valid(cell: &str) -> bool {
    let column: DurationColumn = serde_json::from_value(
        json!({"name": "name", "property": {"type": "string", "format": "duration"}}),
    )
    .unwrap();
    let table = df!("name" => [cell]).unwrap().lazy();
    inspect_duration_column(&column, table, None)
        .unwrap()
        .is_empty()
}

#[test]
fn test_inspect() {
    let cases = [
        ("P23DT23H", true),
        ("P1Y2M3DT4H5M6S", true),
        ("PT30M", true),
        ("P1D", true),
        ("PT1H", true),
        ("P1W", true),
        ("PT0S", true),
        ("ghijkl", false),
        ("0x1234", false),
        ("hello world", false),
        ("PT", false),
        ("P1DT", false),
        ("P1DT\n", false),
    ];
    for (cell, valid) in cases {
        assert_eq!(get_is_valid(cell), valid, "{cell}");
    }
}

#[test]
fn test_inspect_follows_rfc3339_appendix_a() {
    let cases = [
        ("P", false),
        ("PT", false),
        ("p1d", true),
        ("p1dt2h", true),
        ("P1W", true),
        ("P1Y2M", true),
        ("PT2M3S", true),
        ("P99999999999999999999D", true),
        ("-P1D", false),
        ("+P1D", false),
        ("P1D\n", false),
        ("P1D ", false),
        ("P1.5D", false),
        ("PT1.5S", false),
        ("PT1,5S", false),
        ("P1M1Y", false),
        ("PT1D", false),
        ("P1W2D", false),
        ("P1Y1W", false),
        ("P1Y1D", false),
        ("PT1H1S", false),
        ("P0001-02-03T04:05:06", false),
    ];
    for (cell, valid) in cases {
        assert_eq!(get_is_valid(cell), valid, "{cell:?}");
    }
}
