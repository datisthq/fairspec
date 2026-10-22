use fairspec_metadata::WkbColumn;
use polars::prelude::*;
use serde_json::json;

use crate::actions::column::types::wkb::inspect_wkb_column;

fn get_is_valid(cell: &str) -> bool {
    let column: WkbColumn = serde_json::from_value(
        json!({"name": "name", "property": {"type": "string", "format": "wkb"}}),
    )
    .unwrap();
    let table = df!("name" => [cell]).unwrap().lazy();
    inspect_wkb_column(&column, table, None).unwrap().is_empty()
}

#[test]
fn test_wkb_validation() {
    let cases = [
        ("0101000000000000000000f03f0000000000000040", true),
        ("ghijkl", false),
        ("0x1234", false),
        ("hello world", false),
    ];
    for (cell, valid) in cases {
        assert_eq!(get_is_valid(cell), valid, "{cell}");
    }
}

// Expected verdicts are shapely 2.1 (GEOS 3.13) `from_wkb(bytes.fromhex(...))` raising or not
#[test]
fn test_wkb_validation_matches_shapely() {
    let point = "0101000000000000000000f03f0000000000000040";
    let zeros = |count: usize| "00".repeat(count);
    let cases = [
        (point.to_uppercase(), true),
        (format!("{point}00"), true),
        (point[..point.len() - 2].to_string(), false),
        (String::new(), false),
        ("01".to_string(), false),
        (format!("01 {}", &point[2..]), true),
        (format!("\t{point}\n"), true),
        (point[..3].to_string(), false),
        (
            "0000000001".to_string() + "3ff0000000000000" + "4000000000000000",
            true,
        ),
        (format!("01e9030000{}", zeros(24)), true),
        (format!("0101000080{}", zeros(24)), true),
        (format!("0101000040{}", zeros(24)), true),
        (format!("01d1070000{}", zeros(24)), true),
        (format!("01b90b0000{}", zeros(32)), true),
        (format!("0101000020e6100000{}", zeros(16)), true),
        (format!("010200000001000000{}", zeros(16)), false),
        ("010200000000000000".to_string(), true),
        (
            format!(
                "01030000000100000003000000{}000000000000f03f{}{}",
                zeros(16),
                zeros(8),
                zeros(16)
            ),
            true,
        ),
        (
            format!(
                "01030000000100000003000000{}000000000000f03f{}000000000000f03f{}",
                zeros(16),
                zeros(8),
                zeros(8)
            ),
            false,
        ),
        (format!("010400000001000000{point}"), true),
        ("010400000001000000010200000000000000".to_string(), false),
        (format!("010700000001000000{point}"), true),
        ("010800000000000000".to_string(), false),
        (
            "0101000000000000000000f87f000000000000f87f".to_string(),
            true,
        ),
        ("010f00000000000000".to_string(), false),
        ("0102000000ffffffff".to_string(), false),
        (format!("02{}", &point[2..]), true),
        ("010300000000000000".to_string(), true),
        ("010600000001000000010300000000000000".to_string(), true),
    ];
    for (cell, valid) in cases {
        assert_eq!(get_is_valid(&cell), valid, "{cell:?}");
    }
}
