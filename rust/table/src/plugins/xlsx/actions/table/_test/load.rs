use fairspec_dataset::get_temp_file_path;
use fairspec_metadata::Resource;
use serde_json::{Value, json};

use super::test_data::{create_rows, write_test_data};
use crate::plugins::_shared::collect_records;
use crate::plugins::xlsx::actions::table::load::load_xlsx_table;

fn load(data: Value, format: &str, dialect: Value) -> Value {
    let mut dialect = dialect;
    dialect["format"] = json!(format);
    let resource: Resource =
        serde_json::from_value(json!({"data": data, "fileDialect": dialect})).unwrap();
    collect_records(load_xlsx_table(&resource, &Default::default()).unwrap())
}

fn write(rows: Value, sheet_number: usize, sheet_name: Option<&str>, format: &str) -> String {
    let path = get_temp_file_path(false, None, None).unwrap();
    write_test_data(&path, &create_rows(rows), sheet_number, sheet_name, format);
    path
}

mod xlsx {
    use super::*;

    #[test]
    fn test_should_load_local_file() {
        let path = write(
            json!([["id", "name"], [1, "english"], [2, "中文"]]),
            1,
            None,
            "xlsx",
        );
        let records = load(json!(path), "xlsx", json!({}));
        assert_eq!(
            records,
            json!([{"id": 1, "name": "english"}, {"id": 2, "name": "中文"}])
        );
    }
    #[test]
    fn test_should_load_local_file_multipart() {
        let path1 = write(
            json!([["id", "name"], [1, "english"], [2, "中文"]]),
            1,
            None,
            "xlsx",
        );
        let path2 = write(
            json!([["id", "name"], [1, "english"], [2, "中文"]]),
            1,
            None,
            "xlsx",
        );
        let records = load(json!([path1, path2]), "xlsx", json!({}));
        assert_eq!(
            records,
            json!([{"id": 1, "name": "english"}, {"id": 2, "name": "中文"}, {"id": 1, "name": "english"}, {"id": 2, "name": "中文"}])
        );
    }
    #[test]
    fn test_should_support_sheet_number() {
        let path = write(
            json!([["id", "name"], [1, "english"], [2, "中文"]]),
            2,
            None,
            "xlsx",
        );
        let records = load(json!(path), "xlsx", json!({"sheetNumber": 2}));
        assert_eq!(
            records,
            json!([{"id": 1, "name": "english"}, {"id": 2, "name": "中文"}])
        );
    }
    #[test]
    fn test_should_support_sheet_name() {
        let path = write(
            json!([["id", "name"], [1, "english"], [2, "中文"]]),
            1,
            Some("Sheet2"),
            "xlsx",
        );
        let records = load(json!(path), "xlsx", json!({"sheetName": "Sheet2"}));
        assert_eq!(
            records,
            json!([{"id": 1, "name": "english"}, {"id": 2, "name": "中文"}])
        );
    }
    #[test]
    fn test_should_support_no_header() {
        let path = write(json!([[1, "english"], [2, "中文"]]), 1, None, "xlsx");
        let records = load(json!(path), "xlsx", json!({"headerRows": false}));
        assert_eq!(
            records,
            json!([{"column1": 1, "column2": "english"}, {"column1": 2, "column2": "中文"}])
        );
    }
    #[test]
    fn test_should_support_header_rows_offset() {
        let path = write(
            json!([["id", "name"], [1, "english"], [2, "中文"]]),
            1,
            None,
            "xlsx",
        );
        let records = load(json!(path), "xlsx", json!({"headerRows": [2]}));
        assert_eq!(records, json!([{"1": 2, "english": "中文"}]));
    }
    #[test]
    fn test_should_support_multiline_header_rows() {
        let path = write(
            json!([["id", "name"], [1, "english"], [2, "中文"]]),
            1,
            None,
            "xlsx",
        );
        let records = load(json!(path), "xlsx", json!({"headerRows": [1, 2]}));
        assert_eq!(records, json!([{"id 1": 2, "name english": "中文"}]));
    }
    #[test]
    fn test_should_support_header_join() {
        let path = write(
            json!([["id", "name"], [1, "english"], [2, "中文"]]),
            1,
            None,
            "xlsx",
        );
        let records = load(
            json!(path),
            "xlsx",
            json!({"headerRows": [1, 2], "headerJoin": "-"}),
        );
        assert_eq!(records, json!([{"id-1": 2, "name-english": "中文"}]));
    }
    #[test]
    fn test_should_support_comment_rows() {
        let path = write(
            json!([["id", "name"], [1, "english"], [2, "中文"]]),
            1,
            None,
            "xlsx",
        );
        let records = load(json!(path), "xlsx", json!({"commentRows": [2]}));
        assert_eq!(records, json!([{"id": 2, "name": "中文"}]));
    }
    #[test]
    fn test_should_support_comment_prefix() {
        let path = write(
            json!([["id", "name"], [1, "english"], [2, "中文"], ["#comment"]]),
            1,
            None,
            "xlsx",
        );
        let records = load(json!(path), "xlsx", json!({"commentPrefix": "#"}));
        assert_eq!(
            records,
            json!([{"id": 1, "name": "english"}, {"id": 2, "name": "中文"}])
        );
    }
    #[test]
    fn test_should_handle_longer_rows() {
        let path = write(
            json!([
                ["id", "name"],
                [1, "english"],
                [2, "中文"],
                [3, "german", "bad"]
            ]),
            1,
            None,
            "xlsx",
        );
        let records = load(json!(path), "xlsx", json!({"commentPrefix": "#"}));
        assert_eq!(
            records,
            json!([{"id": 1, "name": "english"}, {"id": 2, "name": "中文"}, {"id": 3, "name": "german"}])
        );
    }
    #[test]
    fn test_should_handle_shorter_rows() {
        let path = write(
            json!([["id", "name"], [1, "english"], [2, "中文"], [3]]),
            1,
            None,
            "xlsx",
        );
        let records = load(json!(path), "xlsx", json!({"commentPrefix": "#"}));
        assert_eq!(
            records,
            json!([{"id": 1, "name": "english"}, {"id": 2, "name": "中文"}, {"id": 3, "name": null}])
        );
    }
}

mod ods {
    use super::*;

    #[test]
    fn test_should_load_local_file() {
        let path = write(
            json!([["id", "name"], [1, "english"], [2, "中文"]]),
            1,
            None,
            "ods",
        );
        let records = load(json!(path), "ods", json!({}));
        assert_eq!(
            records,
            json!([{"id": 1, "name": "english"}, {"id": 2, "name": "中文"}])
        );
    }
    #[test]
    fn test_should_load_local_file_multipart() {
        let path1 = write(
            json!([["id", "name"], [1, "english"], [2, "中文"]]),
            1,
            None,
            "ods",
        );
        let path2 = write(
            json!([["id", "name"], [1, "english"], [2, "中文"]]),
            1,
            None,
            "ods",
        );
        let records = load(json!([path1, path2]), "ods", json!({}));
        assert_eq!(
            records,
            json!([{"id": 1, "name": "english"}, {"id": 2, "name": "中文"}, {"id": 1, "name": "english"}, {"id": 2, "name": "中文"}])
        );
    }
    #[test]
    fn test_should_support_sheet_number() {
        let path = write(
            json!([["id", "name"], [1, "english"], [2, "中文"]]),
            2,
            None,
            "ods",
        );
        let records = load(json!(path), "ods", json!({"sheetNumber": 2}));
        assert_eq!(
            records,
            json!([{"id": 1, "name": "english"}, {"id": 2, "name": "中文"}])
        );
    }
    #[test]
    fn test_should_support_sheet_name() {
        let path = write(
            json!([["id", "name"], [1, "english"], [2, "中文"]]),
            1,
            Some("Sheet2"),
            "ods",
        );
        let records = load(json!(path), "ods", json!({"sheetName": "Sheet2"}));
        assert_eq!(
            records,
            json!([{"id": 1, "name": "english"}, {"id": 2, "name": "中文"}])
        );
    }
    #[test]
    fn test_should_support_no_header() {
        let path = write(json!([[1, "english"], [2, "中文"]]), 1, None, "ods");
        let records = load(json!(path), "ods", json!({"headerRows": false}));
        assert_eq!(
            records,
            json!([{"column1": 1, "column2": "english"}, {"column1": 2, "column2": "中文"}])
        );
    }
    #[test]
    fn test_should_support_header_rows_offset() {
        let path = write(
            json!([["id", "name"], [1, "english"], [2, "中文"]]),
            1,
            None,
            "ods",
        );
        let records = load(json!(path), "ods", json!({"headerRows": [2]}));
        assert_eq!(records, json!([{"1": 2, "english": "中文"}]));
    }
    #[test]
    fn test_should_support_multiline_header_rows() {
        let path = write(
            json!([["id", "name"], [1, "english"], [2, "中文"]]),
            1,
            None,
            "ods",
        );
        let records = load(json!(path), "ods", json!({"headerRows": [1, 2]}));
        assert_eq!(records, json!([{"id 1": 2, "name english": "中文"}]));
    }
    #[test]
    fn test_should_support_header_join() {
        let path = write(
            json!([["id", "name"], [1, "english"], [2, "中文"]]),
            1,
            None,
            "ods",
        );
        let records = load(
            json!(path),
            "ods",
            json!({"headerRows": [1, 2], "headerJoin": "-"}),
        );
        assert_eq!(records, json!([{"id-1": 2, "name-english": "中文"}]));
    }
    #[test]
    fn test_should_support_comment_rows() {
        let path = write(
            json!([["id", "name"], [1, "english"], [2, "中文"]]),
            1,
            None,
            "ods",
        );
        let records = load(json!(path), "ods", json!({"commentRows": [2]}));
        assert_eq!(records, json!([{"id": 2, "name": "中文"}]));
    }
    #[test]
    fn test_should_support_comment_prefix() {
        let path = write(
            json!([["id", "name"], [1, "english"], [2, "中文"], ["#comment"]]),
            1,
            None,
            "ods",
        );
        let records = load(json!(path), "ods", json!({"commentPrefix": "#"}));
        assert_eq!(
            records,
            json!([{"id": 1, "name": "english"}, {"id": 2, "name": "中文"}])
        );
    }
    #[test]
    fn test_should_handle_longer_rows() {
        let path = write(
            json!([
                ["id", "name"],
                [1, "english"],
                [2, "中文"],
                [3, "german", "bad"]
            ]),
            1,
            None,
            "ods",
        );
        let records = load(json!(path), "ods", json!({"commentPrefix": "#"}));
        assert_eq!(
            records,
            json!([{"id": 1, "name": "english"}, {"id": 2, "name": "中文"}, {"id": 3, "name": "german"}])
        );
    }
    #[test]
    fn test_should_handle_shorter_rows() {
        let path = write(
            json!([["id", "name"], [1, "english"], [2, "中文"], [3]]),
            1,
            None,
            "ods",
        );
        let records = load(json!(path), "ods", json!({"commentPrefix": "#"}));
        assert_eq!(
            records,
            json!([{"id": 1, "name": "english"}, {"id": 2, "name": "中文"}, {"id": 3, "name": null}])
        );
    }
}
