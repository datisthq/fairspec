use serde_json::{Value, json};

use crate::utils::sniffer::metadata::{Metadata, Quote};
use crate::utils::sniffer::sample::{SampleSize, SampleSizeType};
use crate::utils::sniffer::sniffer::Sniffer;

fn sniff(text: &str) -> Metadata {
    Sniffer::new().sniff_bytes(text.as_bytes()).unwrap()
}

fn sniff_rows(rows: Value) -> Metadata {
    let rows: Vec<Vec<Value>> = serde_json::from_value(rows).unwrap();
    Sniffer::new().sniff_rows(&rows).unwrap()
}

#[test]
fn test_detect_comma_delimited_csv() {
    let metadata = sniff("id,name,age\n1,Alice,25\n2,Bob,30");
    assert_eq!(metadata.dialect.delimiter, 44);
    assert!(metadata.dialect.header.has_header_row);
    assert_eq!(metadata.num_fields, 3);
    assert_eq!(metadata.fields, ["id", "name", "age"]);
}

#[test]
fn test_detect_tab_delimited_tsv() {
    let metadata = sniff("id\tname\tage\n1\tAlice\t25\n2\tBob\t30");
    assert_eq!(metadata.dialect.delimiter, 9);
    assert!(metadata.dialect.header.has_header_row);
    assert_eq!(metadata.num_fields, 3);
}

#[test]
fn test_detect_semicolon_delimited() {
    let metadata = sniff("id;name;age\n1;Alice;25\n2;Bob;30");
    assert_eq!(metadata.dialect.delimiter, 59);
    assert_eq!(metadata.num_fields, 3);
}

#[test]
fn test_detect_pipe_delimited() {
    let metadata = sniff("id|name|age\n1|Alice|25\n2|Bob|30");
    assert_eq!(metadata.dialect.delimiter, 124);
    assert_eq!(metadata.num_fields, 3);
}

#[test]
fn test_detect_quoted_fields() {
    let metadata = sniff(
        "id,name,description\n1,\"Alice\",\"She said, \"\"Hello\"\"\"\n2,\"Bob\",\"Normal text\"",
    );
    assert_eq!(metadata.dialect.delimiter, 44);
    assert_eq!(metadata.dialect.quote.char, Some(34));
}

#[test]
fn test_detect_crlf() {
    let metadata = sniff("id,name\r\n1,Alice\r\n2,Bob");
    assert_eq!(metadata.dialect.delimiter, 44);
    assert_eq!(metadata.num_fields, 2);
}

#[test]
fn test_detect_cr() {
    let metadata = sniff("id,name\r1,Alice\r2,Bob");
    assert_eq!(metadata.dialect.delimiter, 44);
    assert_eq!(metadata.num_fields, 2);
}

#[test]
fn test_detect_no_header() {
    let metadata = sniff("1,Alice,25\n2,Bob,30\n3,Charlie,35");
    assert!(!metadata.dialect.header.has_header_row);
    assert_eq!(metadata.fields, ["field_1", "field_2", "field_3"]);
}

#[test]
fn test_skip_comment_preamble() {
    let metadata = sniff("# This is a comment\n# Another comment\nid,name\n1,Alice\n2,Bob");
    assert_eq!(metadata.dialect.header.num_preamble_rows, 2);
    assert_eq!(metadata.fields, ["id", "name"]);
}

#[test]
fn test_detect_structural_preamble() {
    let metadata = sniff("Report Title\nReport Date: 2024-01-01\nid,name\n1,Alice\n2,Bob");
    assert!(metadata.dialect.header.num_preamble_rows >= 1);
}

#[test]
fn test_handle_utf8_bom() {
    let data = [&[0xEF, 0xBB, 0xBF][..], b"id,name\n1,Alice"].concat();
    let metadata = Sniffer::new().sniff_bytes(&data).unwrap();
    assert_eq!(metadata.dialect.delimiter, 44);
    assert_eq!(metadata.fields, ["id", "name"]);
}

#[test]
fn test_flexible_mode() {
    let metadata = sniff("id,name\n1,Alice\n2,Bob,Extra\n3,Charlie");
    assert!(metadata.dialect.flexible);
}

#[test]
fn test_force_delimiter() {
    let sniffer = Sniffer::new().with_delimiter(59);
    let metadata = sniffer
        .sniff_bytes(b"id;name;age\n1;Alice;25\n2;Bob;30")
        .unwrap();
    assert_eq!(metadata.dialect.delimiter, 59);
}

#[test]
fn test_force_quote() {
    let sniffer = Sniffer::new().with_quote(Quote { char: Some(39) });
    let metadata = sniffer
        .sniff_bytes(b"id,'name','age'\n1,'Alice','25'")
        .unwrap();
    assert_eq!(metadata.dialect.quote.char, Some(39));
}

#[test]
fn test_avg_record_len() {
    assert!(sniff("id,name\n1,Alice\n2,Bob").avg_record_len > 0.0);
}

#[test]
fn test_empty_file() {
    assert_eq!(sniff("").num_fields, 0);
}

#[test]
fn test_single_line() {
    assert_eq!(sniff("id,name,age").num_fields, 3);
}

#[test]
fn test_rows_detect_comma_with_header() {
    let metadata = sniff_rows(json!([
        ["id", "name", "age"],
        [1, "Alice", 25],
        [2, "Bob", 30]
    ]));
    assert_eq!(metadata.dialect.delimiter, 44);
    assert!(metadata.dialect.header.has_header_row);
    assert_eq!(metadata.fields, ["id", "name", "age"]);
    assert_eq!(metadata.num_fields, 3);
}

#[test]
fn test_rows_detect_no_header() {
    let metadata = sniff_rows(json!([
        [1, "Alice", 25],
        [2, "Bob", 30],
        [3, "Charlie", 35]
    ]));
    assert_eq!(metadata.dialect.delimiter, 44);
    assert!(!metadata.dialect.header.has_header_row);
    assert_eq!(metadata.fields, ["field_1", "field_2", "field_3"]);
}

#[test]
fn test_rows_string_values_with_commas() {
    let metadata = sniff_rows(json!([
        ["name", "city"],
        ["Smith, John", "New York"],
        ["Doe, Jane", "Los Angeles"],
    ]));
    assert_eq!(metadata.dialect.delimiter, 44);
    assert_eq!(metadata.fields, ["name", "city"]);
}

#[test]
fn test_rows_string_values_with_quotes() {
    let metadata = sniff_rows(json!([
        ["text", "author"],
        ["He said \"Hello\"", "Alice"],
        ["She said \"Hi\"", "Bob"],
    ]));
    assert_eq!(metadata.dialect.quote.char, Some(34));
}

#[test]
fn test_rows_null_to_empty_string() {
    let metadata = sniff_rows(json!([
        ["id", "name", "optional"],
        [1, "Alice", null],
        [2, "Bob", null]
    ]));
    assert_eq!(metadata.fields, ["id", "name", "optional"]);
}

#[test]
fn test_rows_numbers_and_booleans() {
    let metadata = sniff_rows(json!([
        ["count", "price", "active"],
        [42, 19.99, true],
        [100, 5.5, false],
    ]));
    assert_eq!(metadata.num_fields, 3);
}

#[test]
fn test_rows_date_objects() {
    let metadata = sniff_rows(json!([
        ["event", "timestamp"],
        ["Login", "2024-01-01T10:00:00+00:00"],
        ["Logout", "2024-01-01T11:00:00+00:00"],
    ]));
    assert_eq!(metadata.fields, ["event", "timestamp"]);
}

#[test]
fn test_rows_objects_and_arrays() {
    let metadata = sniff_rows(json!([
        ["id", "metadata", "tags"],
        [1, {"key": "value"}, ["a", "b"]],
        [2, {"key": "other"}, ["c"]],
    ]));
    assert_eq!(metadata.num_fields, 3);
}

#[test]
fn test_rows_variable_lengths_flexible() {
    let metadata = sniff_rows(json!([
        ["id", "name"],
        [1, "Alice"],
        [2, "Bob", "extra"],
        [3]
    ]));
    assert!(metadata.dialect.flexible);
}

#[test]
fn test_rows_empty_array() {
    let metadata = sniff_rows(json!([]));
    assert_eq!(metadata.num_fields, 0);
    assert!(metadata.fields.is_empty());
}

#[test]
fn test_rows_single_row() {
    assert_eq!(sniff_rows(json!([[1, "Alice"]])).num_fields, 2);
}

#[test]
fn test_rows_configured_sample_size() {
    let rows: Vec<Vec<Value>> = (0..1000)
        .map(|i| vec![json!(i), json!(format!("data{i}"))])
        .collect();
    let sniffer = Sniffer::new().with_sample_size(SampleSize {
        r#type: SampleSizeType::Records,
        count: 10,
    });
    assert_eq!(sniffer.sniff_rows(&rows).unwrap().num_fields, 2);
}

#[test]
fn test_rows_preamble_rows_with_hash() {
    let metadata = sniff_rows(json!([
        ["# Exported from Excel on 2024-01-01"],
        ["# Data source: Sales Report"],
        ["id", "product", "quantity"],
        [1, "Widget", 100],
        [2, "Gadget", 150],
    ]));
    assert_eq!(metadata.dialect.header.num_preamble_rows, 2);
    assert!(metadata.dialect.header.has_header_row);
    assert_eq!(metadata.fields, ["id", "product", "quantity"]);
    assert_eq!(metadata.num_fields, 3);
}
