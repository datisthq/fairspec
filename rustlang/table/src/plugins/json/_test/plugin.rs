use std::path::Path;

use fairspec_dataset::{DatasetPlugin, InferFileDialectOptions};
use fairspec_metadata::{FileDialect, Resource};
use polars::prelude::*;
use serde_json::json;

use crate::models::table::{LoadTableOptions, SaveTableOptions};
use crate::plugin::TablePlugin;
use crate::plugins::json::plugin::JsonPlugin;

const JSON: &str = r#"[{"id": 1}]"#;
const LINES: &str = "{\"id\": 1}\n{\"id\": 2}";

fn write_file(dir: &Path, name: &str, body: &str) -> String {
    let path = dir.join(name);
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    std::fs::write(&path, body).unwrap();
    path.to_str().unwrap().to_string()
}

fn create_resource(data: &str, format: Option<&str>) -> Resource {
    let mut resource = json!({"data": data});
    if let Some(format) = format {
        resource["fileDialect"] = json!({"format": format});
    }
    serde_json::from_value(resource).unwrap()
}

fn create_dialect(format: &str) -> Option<FileDialect> {
    Some(serde_json::from_value(json!({"format": format})).unwrap())
}

fn load(data: &str, format: Option<&str>, denormalized: bool) -> Option<LazyFrame> {
    let options = LoadTableOptions {
        denormalized: Some(denormalized),
        ..Default::default()
    };
    JsonPlugin
        .load_table(&create_resource(data, format), &options)
        .unwrap()
}

/// A missing file infers the bare dialect, as Python's twin returns when it cannot read one.
fn infer(data: &str) -> Option<FileDialect> {
    JsonPlugin
        .infer_file_dialect(
            &create_resource(data, None),
            &InferFileDialectOptions::default(),
        )
        .unwrap()
}

fn save(dir: &Path, name: &str, format: Option<&str>) -> Option<String> {
    let path = dir.join(name);
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    let options = SaveTableOptions {
        path: path.to_str().unwrap().to_string(),
        file_dialect: format.and_then(create_dialect),
        ..Default::default()
    };
    let table = df!("id" => [1i64]).unwrap().lazy();
    JsonPlugin.save_table(table, &options).unwrap()
}

fn check_saved(dir: &Path, name: &str, format: Option<&str>) {
    let path = dir.join(name).to_str().unwrap().to_string();
    assert_eq!(save(dir, name, format), Some(path));
}

mod load_table {
    use super::*;

    #[test]
    fn test_should_load_table_from_json_file() {
        let dir = tempfile::tempdir().unwrap();
        let dir = dir.path();
        assert!(load(&write_file(dir, "test.json", JSON), None, false).is_some());
    }

    #[test]
    fn test_should_load_table_from_jsonl_file() {
        let dir = tempfile::tempdir().unwrap();
        let dir = dir.path();
        assert!(load(&write_file(dir, "test.jsonl", LINES), None, false).is_some());
    }

    #[test]
    fn test_should_load_table_from_ndjson_file() {
        let dir = tempfile::tempdir().unwrap();
        let dir = dir.path();
        assert!(load(&write_file(dir, "test.ndjson", LINES), None, false).is_some());
    }

    #[test]
    fn test_should_return_none_for_non_json_files() {
        assert!(load("test.csv", None, false).is_none());
    }

    #[test]
    fn test_should_handle_explicit_json_format() {
        let dir = tempfile::tempdir().unwrap();
        let dir = dir.path();
        assert!(load(&write_file(dir, "test.txt", JSON), Some("json"), false).is_some());
    }

    #[test]
    fn test_should_pass_through_load_options() {
        let dir = tempfile::tempdir().unwrap();
        let dir = dir.path();
        assert!(load(&write_file(dir, "test.json", JSON), None, true).is_some());
    }

    #[test]
    fn test_should_handle_paths_with_directories() {
        let dir = tempfile::tempdir().unwrap();
        let dir = dir.path();
        assert!(load(&write_file(dir, "path/to/data.json", JSON), None, false).is_some());
    }

    #[test]
    fn test_should_return_none_for_parquet_files() {
        assert!(load("test.parquet", None, false).is_none());
    }
}

mod infer_file_dialect {
    use super::*;

    #[test]
    fn test_should_infer_dialect_for_json_file() {
        assert_eq!(infer("test.json"), create_dialect("json"));
    }

    #[test]
    fn test_should_infer_dialect_for_jsonl_file() {
        assert_eq!(infer("test.jsonl"), create_dialect("jsonl"));
    }

    #[test]
    fn test_should_return_none_for_non_json_files() {
        assert_eq!(infer("test.csv"), None);
    }
}

mod save_table {
    use super::*;

    #[test]
    fn test_should_save_table_to_json_file() {
        let dir = tempfile::tempdir().unwrap();
        let dir = dir.path();
        check_saved(dir, "output.json", None);
    }

    #[test]
    fn test_should_save_table_to_jsonl_file() {
        let dir = tempfile::tempdir().unwrap();
        let dir = dir.path();
        check_saved(dir, "output.jsonl", None);
    }

    #[test]
    fn test_should_save_table_to_ndjson_file() {
        let dir = tempfile::tempdir().unwrap();
        let dir = dir.path();
        check_saved(dir, "output.ndjson", None);
    }

    #[test]
    fn test_should_return_none_for_non_json_files() {
        let dir = tempfile::tempdir().unwrap();
        let dir = dir.path();
        assert_eq!(save(dir, "output.csv", None), None);
    }

    #[test]
    fn test_should_handle_explicit_json_format() {
        let dir = tempfile::tempdir().unwrap();
        let dir = dir.path();
        check_saved(dir, "output.txt", Some("json"));
    }

    #[test]
    fn test_should_handle_paths_with_directories() {
        let dir = tempfile::tempdir().unwrap();
        let dir = dir.path();
        check_saved(dir, "path/to/output.json", None);
    }

    #[test]
    fn test_should_return_none_for_files_without_extension() {
        let dir = tempfile::tempdir().unwrap();
        let dir = dir.path();
        assert_eq!(save(dir, "output", None), None);
    }

    #[test]
    fn test_should_return_none_for_parquet_files() {
        let dir = tempfile::tempdir().unwrap();
        let dir = dir.path();
        assert_eq!(save(dir, "output.parquet", None), None);
    }
}
