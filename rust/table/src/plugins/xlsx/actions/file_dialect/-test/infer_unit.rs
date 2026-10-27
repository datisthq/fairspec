use fairspec_dataset::write_temp_file;
use fairspec_metadata::{FileDialect, Resource};
use serde_json::{Value, json};

use crate::plugins::xlsx::actions::file_dialect::infer::infer_xlsx_file_dialect;
use crate::plugins::xlsx::actions::table::_test::test_data::create_rows;
use crate::plugins::xlsx::helpers::write::{SheetData, write_ods_buffer, write_xlsx_buffer};

fn create_sheets(sheets: Value) -> Vec<SheetData> {
    sheets
        .as_array()
        .unwrap()
        .iter()
        .map(|sheet| SheetData {
            name: sheet["name"].as_str().unwrap().to_string(),
            rows: create_rows(sheet["rows"].clone()),
        })
        .collect()
}

fn create_xlsx_file(rows: Value) -> String {
    let buffer =
        write_xlsx_buffer(&create_sheets(json!([{"name": "Sheet1", "rows": rows}]))).unwrap();
    write_temp_file(buffer, false, None, Some("xlsx")).unwrap()
}

fn infer(resource: Value) -> Option<FileDialect> {
    let resource: Resource = serde_json::from_value(resource).unwrap();
    infer_xlsx_file_dialect(&resource, None).unwrap()
}

fn create_dialect(value: Value) -> Option<FileDialect> {
    Some(serde_json::from_value(value).unwrap())
}

#[test]
fn test_should_detect_header_row_with_text_headers() {
    let path = create_xlsx_file(json!([
        ["id", "name", "age"],
        [1, "Alice", 25],
        [2, "Bob", 30]
    ]));
    assert_eq!(
        infer(json!({"data": path})),
        create_dialect(json!({"format": "xlsx", "headerRows": [1]}))
    );
}

#[test]
fn test_should_not_detect_header_when_first_row_is_numeric() {
    let path = create_xlsx_file(json!([[1, 2, 3], [4, 5, 6], [7, 8, 9]]));
    assert_eq!(
        infer(json!({"data": path})),
        create_dialect(json!({"format": "xlsx", "headerRows": false}))
    );
}

#[test]
fn test_should_detect_header_with_mixed_case_and_underscores() {
    let path = create_xlsx_file(json!([
        ["user_id", "User_Name", "EmailAddress"],
        [1, "alice", "alice@example.com"],
        [2, "bob", "bob@example.com"],
    ]));
    assert_eq!(
        infer(json!({"data": path})),
        create_dialect(json!({"format": "xlsx", "headerRows": [1]}))
    );
}

#[test]
fn test_should_not_detect_header_with_boolean_values_in_first_row() {
    let path = create_xlsx_file(json!([
        ["someId", 37257, 695.8, false, "2024-01-01"],
        ["anotherId", 68694, 337.73, true, "2024-01-02"],
        ["thirdId", 52019, 988.74, false, "2024-01-03"],
    ]));
    assert_eq!(
        infer(json!({"data": path})),
        create_dialect(json!({"format": "xlsx", "headerRows": false}))
    );
}

#[test]
fn test_should_handle_single_row_files() {
    let path = create_xlsx_file(json!([["id", "name", "age"]]));
    assert_eq!(
        infer(json!({"data": path})),
        create_dialect(json!({"format": "xlsx", "headerRows": false}))
    );
}

#[test]
fn test_should_handle_empty_files() {
    let path = create_xlsx_file(json!([]));
    assert_eq!(
        infer(json!({"data": path})),
        create_dialect(json!({"format": "xlsx"}))
    );
}

#[test]
fn test_should_return_none_for_incompatible_format() {
    assert_eq!(
        infer(json!({"data": "test.csv", "fileDialect": {"format": "csv"}})),
        None
    );
}

#[test]
fn test_should_return_none_for_resources_without_path() {
    assert_eq!(infer(json!({"data": [{"id": 1, "name": "alice"}]})), None);
}

#[test]
fn test_should_respect_sheet_number_from_existing_dialect() {
    let buffer = write_xlsx_buffer(&create_sheets(json!([
        {"name": "Sheet1", "rows": [[1, 2, 3], [4, 5, 6]]},
        {"name": "Sheet2", "rows": [["id", "name"], [1, "Alice"], [2, "Bob"]]},
    ])))
    .unwrap();
    let path = write_temp_file(buffer, false, None, None).unwrap();
    assert_eq!(
        infer(json!({"data": path, "fileDialect": {"format": "xlsx", "sheetNumber": 2}})),
        create_dialect(json!({"format": "xlsx", "headerRows": [1]}))
    );
}

#[test]
fn test_should_respect_sheet_name_from_existing_dialect() {
    let buffer = write_xlsx_buffer(&create_sheets(json!([
        {"name": "Data", "rows": [[1, 2, 3]]},
        {"name": "Headers", "rows": [["id", "name"], [1, "Alice"]]},
    ])))
    .unwrap();
    let path = write_temp_file(buffer, false, None, None).unwrap();
    assert_eq!(
        infer(json!({"data": path, "fileDialect": {"format": "xlsx", "sheetName": "Headers"}})),
        create_dialect(json!({"format": "xlsx", "headerRows": [1]}))
    );
}

#[test]
fn test_should_support_ods_format() {
    let buffer = write_ods_buffer(&create_sheets(json!([
        {"name": "Sheet1", "rows": [["id", "name", "age"], [1, "Alice", 25], [2, "Bob", 30]]},
    ])))
    .unwrap();
    let path = write_temp_file(buffer, false, None, None).unwrap();
    assert_eq!(
        infer(json!({"data": path, "fileDialect": {"format": "ods"}})),
        create_dialect(json!({"format": "ods", "headerRows": [1]}))
    );
}
