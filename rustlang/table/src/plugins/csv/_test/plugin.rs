use std::path::Path;

use fairspec_metadata::Resource;
use polars::prelude::*;
use serde_json::json;

use crate::models::table::{LoadTableOptions, SaveTableOptions};
use crate::plugin::TablePlugin;
use crate::plugins::csv::plugin::CsvPlugin;

const CSV: &str = "id,name\n1,english\n";

fn write_file(dir: &Path, name: &str, body: &str) -> String {
    let path = dir.join(name);
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    std::fs::write(&path, body).unwrap();
    path.to_str().unwrap().to_string()
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
    CsvPlugin.load_table(&resource, &options).unwrap()
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
    let table = df!("id" => [1i64], "name" => ["a"]).unwrap().lazy();
    CsvPlugin.save_table(table, &options).unwrap()
}

fn check_saved(name: &str, format: Option<&str>, delimiter: char) {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join(name).to_str().unwrap().to_string();
    assert_eq!(save(dir.path(), name, format), Some(path.clone()));
    let content = std::fs::read_to_string(&path).unwrap();
    assert_eq!(content, format!("id{delimiter}name\n1{delimiter}a\n"));
}

mod load_table {
    use super::*;

    #[test]
    fn test_should_load_table_from_csv_file() {
        let dir = tempfile::tempdir().unwrap();
        assert!(load(&write_file(dir.path(), "test.csv", CSV), None, false).is_some());
    }

    #[test]
    fn test_should_load_table_from_tsv_file() {
        let dir = tempfile::tempdir().unwrap();
        let path = write_file(dir.path(), "test.tsv", "id\tname\n1\tenglish\n");
        assert!(load(&path, None, false).is_some());
    }

    #[test]
    fn test_should_return_none_for_non_csv_files() {
        assert!(load("test.json", None, false).is_none());
    }

    #[test]
    fn test_should_handle_explicit_csv_format() {
        let dir = tempfile::tempdir().unwrap();
        let path = write_file(dir.path(), "test.txt", CSV);
        assert!(load(&path, Some("csv"), false).is_some());
    }

    #[test]
    fn test_should_pass_through_load_options() {
        let dir = tempfile::tempdir().unwrap();
        let table = load(&write_file(dir.path(), "test.csv", CSV), None, true).unwrap();
        let frame = table.collect().unwrap();
        assert_eq!(frame.column("id").unwrap().dtype(), &DataType::String);
    }

    #[test]
    fn test_should_handle_paths_with_directories() {
        let dir = tempfile::tempdir().unwrap();
        let path = write_file(dir.path(), "path/to/data.csv", CSV);
        assert!(load(&path, None, false).is_some());
    }

    #[test]
    fn test_should_handle_explicit_tsv_format() {
        let dir = tempfile::tempdir().unwrap();
        let path = write_file(dir.path(), "test.txt", "id\tname\n1\tenglish\n");
        assert!(load(&path, Some("tsv"), false).is_some());
    }
}

mod save_table {
    use super::*;

    #[test]
    fn test_should_save_table_to_csv_file() {
        check_saved("output.csv", None, ',');
    }

    #[test]
    fn test_should_save_table_to_tsv_file() {
        check_saved("output.tsv", None, '\t');
    }

    #[test]
    fn test_should_return_none_for_non_csv_files() {
        let dir = tempfile::tempdir().unwrap();
        assert_eq!(save(dir.path(), "output.json", None), None);
    }

    #[test]
    fn test_should_handle_explicit_csv_format() {
        check_saved("output.txt", Some("csv"), ',');
    }

    #[test]
    fn test_should_handle_paths_with_directories() {
        check_saved("path/to/output.csv", None, ',');
    }

    #[test]
    fn test_should_return_none_for_files_without_extension() {
        let dir = tempfile::tempdir().unwrap();
        assert_eq!(save(dir.path(), "output", None), None);
    }

    #[test]
    fn test_should_handle_explicit_tsv_format() {
        check_saved("output.txt", Some("tsv"), '\t');
    }
}
