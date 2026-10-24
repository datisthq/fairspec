use std::path::Path;

use fairspec_metadata::Resource;
use polars::prelude::*;
use serde_json::json;

use crate::models::table::{LoadTableOptions, SaveTableOptions};
use crate::plugin::TablePlugin;
use crate::plugins::sqlite::plugin::SqlitePlugin;

fn write_database(dir: &Path, name: &str) -> String {
    let path = dir.join(name);
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    let connection = rusqlite::Connection::open(&path).unwrap();
    connection
        .execute_batch("CREATE TABLE t (id integer); INSERT INTO t VALUES (1);")
        .unwrap();
    path.to_str().unwrap().to_string()
}

fn create_resource(data: &str, format: Option<&str>) -> Resource {
    let mut resource = json!({"data": data});
    if let Some(format) = format {
        resource["fileDialect"] = json!({"format": format});
    }
    serde_json::from_value(resource).unwrap()
}

fn load(resource: &Resource, denormalized: bool) -> Option<LazyFrame> {
    let options = LoadTableOptions {
        denormalized: Some(denormalized),
        ..Default::default()
    };
    SqlitePlugin.load_table(resource, &options).unwrap()
}

/// Saves over an existing database, whose first table names the target when the
/// dialect does not
fn save(dir: &Path, name: &str, format: Option<&str>) -> Option<String> {
    let path = write_database(dir, name);
    let options = SaveTableOptions {
        path,
        file_dialect: format
            .map(|format| serde_json::from_value(json!({"format": format})).unwrap()),
        overwrite: Some(true),
        ..Default::default()
    };
    let table = df!("id" => [1i64]).unwrap().lazy();
    SqlitePlugin.save_table(table, &options).unwrap()
}

mod load_table {
    use super::*;

    #[test]
    fn test_should_load_table_from_sqlite_file() {
        let dir = tempfile::tempdir().unwrap();
        let path = write_database(dir.path(), "test.sqlite");
        assert!(load(&create_resource(&path, None), false).is_some());
    }

    #[test]
    fn test_should_return_none_for_non_sqlite_files() {
        assert!(load(&create_resource("test.csv", None), false).is_none());
    }

    #[test]
    fn test_should_handle_explicit_sqlite_format() {
        let dir = tempfile::tempdir().unwrap();
        let path = write_database(dir.path(), "test.db");
        assert!(load(&create_resource(&path, Some("sqlite")), false).is_some());
    }

    #[test]
    fn test_should_pass_through_load_options() {
        let dir = tempfile::tempdir().unwrap();
        let path = write_database(dir.path(), "test.sqlite");
        assert!(load(&create_resource(&path, None), true).is_some());
    }

    #[test]
    fn test_should_handle_paths_with_directories() {
        let dir = tempfile::tempdir().unwrap();
        let path = write_database(dir.path(), "path/to/data.sqlite");
        assert!(load(&create_resource(&path, None), false).is_some());
    }
}

mod save_table {
    use super::*;

    #[test]
    fn test_should_save_table_to_sqlite_file() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("output.sqlite");
        assert_eq!(
            save(dir.path(), "output.sqlite", None),
            Some(path.to_str().unwrap().to_string())
        );
    }

    #[test]
    fn test_should_return_none_for_non_sqlite_files() {
        let dir = tempfile::tempdir().unwrap();
        assert_eq!(save(dir.path(), "output.csv", None), None);
    }

    #[test]
    fn test_should_handle_explicit_sqlite_format() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("output.db");
        assert_eq!(
            save(dir.path(), "output.db", Some("sqlite")),
            Some(path.to_str().unwrap().to_string())
        );
    }

    #[test]
    fn test_should_handle_paths_with_directories() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("path/to/output.sqlite");
        assert_eq!(
            save(dir.path(), "path/to/output.sqlite", None),
            Some(path.to_str().unwrap().to_string())
        );
    }

    #[test]
    fn test_should_return_none_for_files_without_extension() {
        let dir = tempfile::tempdir().unwrap();
        assert_eq!(save(dir.path(), "output", None), None);
    }
}
