use std::path::Path;

use fairspec_metadata::Resource;
use polars::prelude::*;
use serde_json::json;

use crate::models::table::{LoadTableOptions, SaveTableOptions};
use crate::plugin::TablePlugin;
use crate::plugins::xlsx::actions::table::_test::test_data::{create_rows, write_test_data};
use crate::plugins::xlsx::plugin::XlsxPlugin;

fn write_file(dir: &Path, name: &str, format: &str) -> String {
    let path = dir.join(name);
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    let path = path.to_str().unwrap().to_string();
    write_test_data(&path, &create_rows(json!([["id"], [1]])), 1, None, format);
    path
}

fn load(data: &str, format: Option<&str>, denormalized: bool) -> Option<LazyFrame> {
    let mut resource = json!({"data": data});
    if let Some(format) = format {
        resource["fileDialect"] = json!({"format": format});
    }
    let resource: Resource = serde_json::from_value(resource).unwrap();
    let options = LoadTableOptions {
        denormalized: Some(denormalized),
        ..Default::default()
    };
    XlsxPlugin.load_table(&resource, &options).unwrap()
}

fn save(dir: &Path, name: &str, format: Option<&str>) -> Option<String> {
    let path = dir.join(name);
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    let options = SaveTableOptions {
        path: path.to_str().unwrap().to_string(),
        file_dialect: format
            .map(|format| serde_json::from_value(json!({"format": format})).unwrap()),
        ..Default::default()
    };
    let table = df!("id" => [1i64]).unwrap().lazy();
    XlsxPlugin.save_table(table, &options).unwrap()
}

fn check_saved(dir: &Path, name: &str, format: Option<&str>) {
    let path = dir.join(name).to_str().unwrap().to_string();
    assert_eq!(save(dir, name, format), Some(path));
}

mod load_table {
    use super::*;

    #[test]
    fn test_should_load_table_from_xlsx_file() {
        let dir = tempfile::tempdir().unwrap();
        let dir = dir.path();
        assert!(load(&write_file(dir, "test.xlsx", "xlsx"), None, false).is_some());
    }

    #[test]
    fn test_should_handle_explicit_xlsx_format() {
        let dir = tempfile::tempdir().unwrap();
        let dir = dir.path();
        assert!(load(&write_file(dir, "test.txt", "xlsx"), Some("xlsx"), false).is_some());
    }

    #[test]
    fn test_should_pass_through_load_options() {
        let dir = tempfile::tempdir().unwrap();
        let dir = dir.path();
        assert!(load(&write_file(dir, "test.xlsx", "xlsx"), None, true).is_some());
    }

    #[test]
    fn test_should_handle_paths_with_directories() {
        let dir = tempfile::tempdir().unwrap();
        let dir = dir.path();
        assert!(load(&write_file(dir, "path/to/data.xlsx", "xlsx"), None, false).is_some());
    }

    #[test]
    fn test_should_load_table_from_ods_file() {
        let dir = tempfile::tempdir().unwrap();
        let dir = dir.path();
        assert!(load(&write_file(dir, "test.ods", "ods"), None, false).is_some());
    }

    #[test]
    fn test_should_handle_explicit_ods_format() {
        let dir = tempfile::tempdir().unwrap();
        let dir = dir.path();
        assert!(load(&write_file(dir, "test.txt", "ods"), Some("ods"), false).is_some());
    }

    #[test]
    fn test_should_pass_through_ods_load_options() {
        let dir = tempfile::tempdir().unwrap();
        let dir = dir.path();
        assert!(load(&write_file(dir, "test.ods", "ods"), None, true).is_some());
    }

    #[test]
    fn test_should_handle_ods_paths_with_directories() {
        let dir = tempfile::tempdir().unwrap();
        let dir = dir.path();
        assert!(load(&write_file(dir, "path/to/data.ods", "ods"), None, false).is_some());
    }

    #[test]
    fn test_should_return_none_for_non_xlsx_ods_files() {
        assert!(load("test.csv", None, false).is_none());
    }

    #[test]
    fn test_should_return_none_for_json_files() {
        assert!(load("test.json", None, false).is_none());
    }
}

mod save_table {
    use super::*;

    #[test]
    fn test_should_save_table_to_xlsx_file() {
        let dir = tempfile::tempdir().unwrap();
        let dir = dir.path();
        check_saved(dir, "output.xlsx", None);
    }

    #[test]
    fn test_should_handle_explicit_xlsx_format() {
        let dir = tempfile::tempdir().unwrap();
        let dir = dir.path();
        check_saved(dir, "output.txt", Some("xlsx"));
    }

    #[test]
    fn test_should_handle_paths_with_directories() {
        let dir = tempfile::tempdir().unwrap();
        let dir = dir.path();
        check_saved(dir, "path/to/output.xlsx", None);
    }

    #[test]
    fn test_should_save_table_to_ods_file() {
        let dir = tempfile::tempdir().unwrap();
        let dir = dir.path();
        check_saved(dir, "output.ods", None);
    }

    #[test]
    fn test_should_handle_explicit_ods_format() {
        let dir = tempfile::tempdir().unwrap();
        let dir = dir.path();
        check_saved(dir, "output.txt", Some("ods"));
    }

    #[test]
    fn test_should_handle_ods_paths_with_directories() {
        let dir = tempfile::tempdir().unwrap();
        let dir = dir.path();
        check_saved(dir, "path/to/output.ods", None);
    }

    #[test]
    fn test_should_return_none_for_non_xlsx_ods_files() {
        let dir = tempfile::tempdir().unwrap();
        let dir = dir.path();
        assert_eq!(save(dir, "output.csv", None), None);
    }

    #[test]
    fn test_should_return_none_for_files_without_extension() {
        let dir = tempfile::tempdir().unwrap();
        let dir = dir.path();
        assert_eq!(save(dir, "output", None), None);
    }

    #[test]
    fn test_should_return_none_for_json_files() {
        let dir = tempfile::tempdir().unwrap();
        let dir = dir.path();
        assert_eq!(save(dir, "output.json", None), None);
    }
}
