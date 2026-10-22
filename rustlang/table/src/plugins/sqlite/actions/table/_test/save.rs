use fairspec_dataset::get_temp_file_path;
use fairspec_metadata::{FileDialect, Resource};
use polars::prelude::*;
use serde_json::json;

use crate::models::table::{LoadTableOptions, SaveTableOptions};
use crate::plugins::_shared::collect_records;
use crate::plugins::sqlite::actions::table::load::load_sqlite_table;
use crate::plugins::sqlite::actions::table::save::save_sqlite_table;

fn create_dialect() -> FileDialect {
    serde_json::from_value(json!({"format": "sqlite", "tableName": "fairspec"})).unwrap()
}

fn save(table: LazyFrame, path: &str) {
    let options = SaveTableOptions {
        path: path.to_string(),
        file_dialect: Some(create_dialect()),
        overwrite: Some(true),
        ..Default::default()
    };
    save_sqlite_table(table, &options).unwrap();
}

fn load(path: &str, denormalized: bool) -> LazyFrame {
    let resource: Resource =
        serde_json::from_value(json!({"data": path, "fileDialect": create_dialect()})).unwrap();
    let options = LoadTableOptions {
        denormalized: Some(denormalized),
        ..Default::default()
    };
    load_sqlite_table(&resource, &options).unwrap()
}

fn create_table() -> LazyFrame {
    df!("id" => [1i64, 2], "name" => ["english", "中文"])
        .unwrap()
        .lazy()
}

#[test]
fn test_should_save_and_load_table() {
    let path = get_temp_file_path(false, None, None).unwrap();

    save(create_table(), &path);

    assert_eq!(
        collect_records(load(&path, false)),
        json!([{"id": 1, "name": "english"}, {"id": 2, "name": "中文"}])
    );
}

#[test]
fn test_should_save_and_load_table_with_protocol() {
    let path = format!(
        "sqlite://{}",
        get_temp_file_path(false, None, None).unwrap()
    );

    save(create_table(), &path);

    assert_eq!(
        collect_records(load(&path, false)),
        json!([{"id": 1, "name": "english"}, {"id": 2, "name": "中文"}])
    );
}

#[test]
fn test_should_save_and_load_various_data_types() {
    let path = format!(
        "sqlite://{}",
        get_temp_file_path(false, None, None).unwrap()
    );
    let source = df!(
        "boolean" => [true],
        "date" => ["2025-01-01"],
        "datetime" => ["2025-01-01T00:00:00"],
        "integer" => [1i32],
        "number" => [1.1],
        "string" => ["string"],
    )
    .unwrap()
    .lazy();

    save(source, &path);

    assert_eq!(
        collect_records(load(&path, true)),
        json!([{
            "boolean": "true",
            "date": "2025-01-01",
            "datetime": "2025-01-01T00:00:00",
            "integer": 1,
            "number": 1.1,
            "string": "string",
        }])
    );
}

#[test]
fn test_should_quote_table_and_column_names() {
    let path = get_temp_file_path(false, None, None).unwrap();
    let table_name = "x\"; DROP TABLE y; --";
    let dialect: FileDialect =
        serde_json::from_value(json!({"format": "sqlite", "tableName": table_name})).unwrap();
    rusqlite::Connection::open(&path)
        .unwrap()
        .execute_batch("CREATE TABLE \"y\" (\"id\" integer)")
        .unwrap();

    let source = df!("a\"b" => [1i64, 2], "c\"; DROP TABLE y; --" => ["x", "y"])
        .unwrap()
        .lazy();
    let options = SaveTableOptions {
        path: path.clone(),
        file_dialect: Some(dialect.clone()),
        overwrite: Some(true),
        ..Default::default()
    };
    save_sqlite_table(source, &options).unwrap();

    let resource: Resource =
        serde_json::from_value(json!({"data": path, "fileDialect": dialect})).unwrap();
    let target = load_sqlite_table(&resource, &Default::default()).unwrap();
    assert_eq!(
        collect_records(target),
        json!([
            {"a\"b": 1, "c\"; DROP TABLE y; --": "x"},
            {"a\"b": 2, "c\"; DROP TABLE y; --": "y"},
        ])
    );

    let connection = rusqlite::Connection::open(&path).unwrap();
    let mut statement = connection
        .prepare("SELECT name FROM sqlite_master ORDER BY name")
        .unwrap();
    let tables: Vec<String> = statement
        .query_map([], |row| row.get(0))
        .unwrap()
        .collect::<rusqlite::Result<_>>()
        .unwrap();
    assert_eq!(tables, [table_name, "y"]);
}
