use fairspec_dataset::get_temp_file_path;
use fairspec_table::SaveTableOptions;
use polars::prelude::*;

use crate::actions::table::save::save_table;

fn save(table: LazyFrame, format: &str) -> Option<String> {
    let options = SaveTableOptions {
        path: get_temp_file_path(false, None, Some(format)).unwrap(),
        ..Default::default()
    };
    save_table(table, &options).unwrap()
}

#[test]
fn test_should_save_table_to_csv() {
    let table = df!("id" => [1i64, 2], "name" => ["english", "中文"])
        .unwrap()
        .lazy();
    assert!(save(table, "csv").is_some());
}

#[test]
fn test_should_save_table_to_json() {
    let table = df!("id" => [1i64, 2], "name" => ["english", "中文"])
        .unwrap()
        .lazy();
    assert!(save(table, "json").is_some());
}

#[test]
fn test_should_return_none_for_unknown_format() {
    let table = df!("id" => [1i64, 2]).unwrap().lazy();
    assert!(save(table, "unknown").is_none());
}
