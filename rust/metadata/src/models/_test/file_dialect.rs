use serde_json::json;

use super::{parse, round_trip};
use crate::models::file_dialect::common::HeaderRows;
use crate::models::file_dialect::file_dialect::FileDialect;

#[test]
fn test_dispatch_by_format() {
    for format in [
        "csv", "tsv", "json", "jsonl", "xlsx", "ods", "sqlite", "parquet", "arrow",
    ] {
        let dialect: FileDialect = parse(json!({"format": format}));
        assert_eq!(dialect.format(), Some(format));
        assert!(!matches!(dialect, FileDialect::Custom(_)));
    }
}

#[test]
fn test_custom_fallback() {
    let dialect: FileDialect = parse(json!({"format": "custom", "title": "t"}));
    assert!(matches!(dialect, FileDialect::Custom(_)));
    assert_eq!(dialect.format(), Some("custom"));
    assert_eq!(
        serde_json::to_value(&dialect).unwrap(),
        json!({"format": "custom", "title": "t"})
    );
}

#[test]
fn test_missing_format() {
    let result = serde_json::from_value::<FileDialect>(json!({"$schema": "x"}));
    assert!(result.is_err());
}

#[test]
fn test_round_trip_fields() {
    for input in [
        json!({"$schema": "https://fairspec.org/profiles/latest/file-dialect.json", "format": "csv",
               "delimiter": ";", "headerRows": false, "commentPrefix": "#", "columnNames": ["a"]}),
        json!({"format": "tsv", "headerRows": [1, 2], "headerJoin": " ", "nullSequence": "\\N"}),
        json!({"format": "json", "jsonPointer": "/data", "rowType": "object"}),
        json!({"format": "jsonl", "rowType": "array", "commentRows": [3]}),
        json!({"format": "xlsx", "sheetName": "Sheet1", "sheetNumber": 2}),
        json!({"format": "ods", "title": "t", "description": "d"}),
        json!({"format": "sqlite", "tableName": "t"}),
    ] {
        assert_eq!(round_trip::<FileDialect>(&input), input);
    }
}

#[test]
fn test_header_rows() {
    let FileDialect::Csv(dialect) = parse(json!({"format": "csv", "headerRows": false})) else {
        panic!("expected a csv dialect");
    };
    assert_eq!(dialect.header_rows, Some(HeaderRows::Disabled));
    assert!(
        serde_json::from_value::<FileDialect>(json!({"format": "csv", "headerRows": true}))
            .is_err()
    );
}
