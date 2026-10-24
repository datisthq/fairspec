use std::path::Path;

use fairspec_metadata::Resource;
use polars::prelude::*;
use serde_json::json;

use crate::models::table::{LoadTableOptions, SaveTableOptions};
use crate::plugin::TablePlugin;
use crate::plugins::parquet::actions::table::_test::load::write_frame;
use crate::plugins::parquet::plugin::ParquetPlugin;

fn write_file(dir: &Path, name: &str) -> String {
    let path = dir.join(name);
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    write_frame(path.to_str().unwrap());
    path.to_str().unwrap().to_string()
}

fn create_resource(data: &str, format: Option<&str>) -> Resource {
    let mut resource = json!({"data": data});
    if let Some(format) = format {
        resource["fileDialect"] = json!({"format": format});
    }
    serde_json::from_value(resource).unwrap()
}

fn load(data: &str, format: Option<&str>) -> Option<LazyFrame> {
    ParquetPlugin
        .load_table(&create_resource(data, format), &LoadTableOptions::default())
        .unwrap()
}

fn save(dir: &Path, name: &str, format: Option<&str>) -> Option<std::path::PathBuf> {
    let path = dir.join(name);
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    let options = SaveTableOptions {
        path: path.to_str().unwrap().to_string(),
        file_dialect: format
            .map(|format| serde_json::from_value(json!({"format": format})).unwrap()),
        ..Default::default()
    };
    let table = df!("id" => [1i64]).unwrap().lazy();
    ParquetPlugin
        .save_table(table, &options)
        .unwrap()
        .map(std::path::PathBuf::from)
}

mod load_table {
    use super::*;

    #[test]
    fn test_should_load_table_from_parquet_file() {
        let dir = tempfile::tempdir().unwrap();
        let dir = dir.path();
        assert!(load(&write_file(dir, "test.parquet"), None).is_some());
    }

    #[test]
    fn test_should_return_none_for_non_parquet_files() {
        assert!(load("test.csv", None).is_none());
    }

    #[test]
    fn test_should_handle_explicit_format() {
        let dir = tempfile::tempdir().unwrap();
        let dir = dir.path();
        assert!(load(&write_file(dir, "test.txt"), Some("parquet")).is_some());
    }

    #[test]
    fn test_should_pass_through_load_options() {
        let dir = tempfile::tempdir().unwrap();
        let dir = dir.path();
        let resource = create_resource(&write_file(dir, "test.parquet"), None);
        let options = LoadTableOptions {
            denormalized: Some(true),
            ..Default::default()
        };
        assert!(
            ParquetPlugin
                .load_table(&resource, &options)
                .unwrap()
                .is_some()
        );
    }

    #[test]
    fn test_should_handle_paths_with_directories() {
        let dir = tempfile::tempdir().unwrap();
        let dir = dir.path();
        assert!(load(&write_file(dir, "path/to/data.parquet"), None).is_some());
    }

    #[test]
    fn test_should_return_none_for_arrow_files() {
        assert!(load("test.arrow", None).is_none());
    }

    #[test]
    fn test_should_return_none_for_json_files() {
        assert!(load("test.json", None).is_none());
    }
}

mod save_table {
    use super::*;

    #[test]
    fn test_should_save_table_to_parquet_file() {
        let dir = tempfile::tempdir().unwrap();
        let dir = dir.path();
        assert_eq!(
            save(dir, "output.parquet", None),
            Some(dir.join("output.parquet"))
        );
    }

    #[test]
    fn test_should_return_none_for_non_parquet_files() {
        let dir = tempfile::tempdir().unwrap();
        let dir = dir.path();
        assert_eq!(save(dir, "output.csv", None), None);
    }

    #[test]
    fn test_should_handle_explicit_format() {
        let dir = tempfile::tempdir().unwrap();
        let dir = dir.path();
        assert_eq!(
            save(dir, "output.txt", Some("parquet")),
            Some(dir.join("output.txt"))
        );
    }

    #[test]
    fn test_should_handle_paths_with_directories() {
        let dir = tempfile::tempdir().unwrap();
        let dir = dir.path();
        assert_eq!(
            save(dir, "path/to/output.parquet", None),
            Some(dir.join("path/to/output.parquet"))
        );
    }

    #[test]
    fn test_should_return_none_for_files_without_extension() {
        let dir = tempfile::tempdir().unwrap();
        let dir = dir.path();
        assert_eq!(save(dir, "output", None), None);
    }

    #[test]
    fn test_should_return_none_for_arrow_files() {
        let dir = tempfile::tempdir().unwrap();
        let dir = dir.path();
        assert_eq!(save(dir, "output.arrow", None), None);
    }

    #[test]
    fn test_should_return_none_for_json_files() {
        let dir = tempfile::tempdir().unwrap();
        let dir = dir.path();
        assert_eq!(save(dir, "output.json", None), None);
    }
}
