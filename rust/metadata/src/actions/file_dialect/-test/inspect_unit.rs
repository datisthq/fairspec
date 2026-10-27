use serde_json::{Value, json};

use crate::actions::file_dialect::inspect::inspect_file_dialect;
use crate::models::error::error::FairspecError;
use crate::models::file_dialect::file_dialect::FileDialect;

fn inspect(value: Value, root: Option<&str>) -> Vec<FairspecError> {
    let file_dialect: FileDialect = serde_json::from_value(value).unwrap();
    inspect_file_dialect(&file_dialect, root)
}

#[test]
fn test_should_report_a_row_listed_in_header_rows_and_comment_rows() {
    let errors = inspect(
        json!({"format": "csv", "headerRows": [1, 2], "commentRows": [2]}),
        Some("/fileDialect"),
    );
    let pointers: Vec<String> = errors
        .iter()
        .map(|error| match error {
            FairspecError::Metadata(error) => error.json_pointer.clone(),
            _ => String::new(),
        })
        .collect();
    assert_eq!(pointers, vec!["/fileDialect/commentRows"]);
}

#[test]
fn test_should_take_the_default_header_row() {
    assert_eq!(
        inspect(json!({"format": "csv", "commentRows": [1]}), None).len(),
        1
    );
}

#[test]
fn test_should_accept_comment_rows_without_a_header() {
    let dialect = json!({"format": "csv", "headerRows": false, "commentRows": [1]});
    assert!(inspect(dialect, None).is_empty());
}

#[test]
fn test_should_skip_object_rows_which_comment_rows_does_not_apply_to() {
    let dialect = json!({"format": "jsonl", "rowType": "object", "commentRows": [1]});
    assert!(inspect(dialect, None).is_empty());
}
