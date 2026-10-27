use serde_json::json;

use crate::actions::file_dialect::infer::infer_file_dialect_format;
use crate::models::resource::Resource;

#[test]
fn test_infer_file_dialect_format() {
    let cases = [
        ("table.csv", Some("csv")),
        ("table.tsv", Some("tsv")),
        ("table.json", Some("json")),
        ("table.jsonl", Some("jsonl")),
        ("table.ndjson", Some("jsonl")),
        ("table.xlsx", Some("xlsx")),
        ("table.ods", Some("ods")),
        ("table.parquet", Some("parquet")),
        ("table.arrow", Some("arrow")),
        ("table.feather", Some("arrow")),
        ("table.sqlite", Some("sqlite")),
        ("table.xyz", None),
    ];
    for (path, expected) in cases {
        let resource = Resource {
            data: Some(json!(path)),
            ..Resource::default()
        };
        assert_eq!(infer_file_dialect_format(&resource), expected, "{path}");
    }
}

#[test]
fn test_no_data() {
    assert_eq!(infer_file_dialect_format(&Resource::default()), None);
}
