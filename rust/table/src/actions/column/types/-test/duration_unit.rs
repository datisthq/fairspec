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

// Expected verdicts are isodate 0.7's `parse_duration` raising or not, except a trailing
// `T`, which the duration column rejects before isodate reads it
#[test]
fn test_inspect_matches_isodate() {
    let cases = [
        ("P", false),
        ("PT", false),
        ("-P1D", true),
        ("+P1D", true),
        ("-P", false),
        ("P1D\n", true),
        ("P1D ", false),
        ("p1d", false),
        ("P1.5D", true),
        ("P1,5D", true),
        ("P1.D", false),
        ("PT1.5S", true),
        ("P1M1Y", false),
        ("P1DT", false),
        ("PT1D", false),
        ("P1Y1W", true),
        ("P999999999D", true),
        ("P1000000000D", false),
        ("-P999999999DT1S", false),
        ("P999999999DT1S", true),
        ("P2000000000W", false),
        ("PT99999999999999999999H", false),
        ("P0001-02-03T04:05:06", true),
        ("P00010203T040506", true),
        ("P0000-02-03T04:05:06", false),
        ("P0001-13-03T04:05:06", false),
        ("P0001-00-03T04:05:06", true),
        ("P0001-02-30T04:05:06", false),
        ("P0001-02-03T24:00:00", false),
        ("P0001-02-03T23:59:60", false),
        ("P0001-02-03T23:59:59.999", true),
        ("P0001-02-03T04:05", true),
        ("P0001-02-03T04", true),
        ("P0001-02-03T04.5", true),
        ("P0001-02-03T0405", true),
        ("P0001-02-03T04:05:06Z", true),
        ("P0001-02-03T04:05:06+01:00", true),
        ("P0001-02-03T04:05:06+0100", true),
        ("P0001-02-03T04:05:06+01", true),
        ("P0001-02-03", false),
        ("P0001-02-03TT04", false),
        ("P2020-W01-1T00", true),
        ("P2020W011T00", true),
        ("P2020-W53T00", true),
        ("P2020-123T00", true),
        ("P2020123T00", true),
        ("P2020-000T00", true),
        ("P0001-000T00", false),
        ("P9999-999T00", false),
        ("P2020-02T00", true),
        ("P202002T00", true),
        ("P2020T00", true),
        ("P20T00", true),
        ("P2020-1T00", false),
        ("PT12:00", false),
    ];
    for (cell, valid) in cases {
        assert_eq!(get_is_valid(cell), valid, "{cell:?}");
    }
}
