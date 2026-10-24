use fairspec_dataset::get_temp_file_path;
use fairspec_metadata::Resource;
use polars::prelude::*;
use serde_json::json;

use super::test_data::read_test_data;
use crate::models::table::{LoadTableOptions, SaveTableOptions};
use crate::plugins::_shared::collect_records;
use crate::plugins::xlsx::actions::table::load::load_xlsx_table;
use crate::plugins::xlsx::actions::table::save::save_xlsx_table;

fn create_options(path: &str, format: &str) -> SaveTableOptions {
    SaveTableOptions {
        path: path.to_string(),
        file_dialect: Some(serde_json::from_value(json!({"format": format})).unwrap()),
        ..Default::default()
    }
}

fn check_save_table_to_file(format: &str) {
    let path = get_temp_file_path(false, None, None).unwrap();
    let table = df!("id" => [1i64, 2], "name" => ["english", "中文"])
        .unwrap()
        .lazy();

    save_xlsx_table(table, &create_options(&path, format)).unwrap();

    assert_eq!(
        read_test_data(&path),
        json!([{"id": 1, "name": "english"}, {"id": 2, "name": "中文"}])
    );
}

fn check_save_and_load_various_data_types(format: &str) {
    let path = get_temp_file_path(false, None, Some(format)).unwrap();
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

    save_xlsx_table(source, &create_options(&path, format)).unwrap();

    let resource: Resource =
        serde_json::from_value(json!({"data": path, "fileDialect": {"format": format}})).unwrap();
    let options = LoadTableOptions {
        denormalized: Some(true),
        ..Default::default()
    };
    // Sheets hold numbers as floats, so the integer reads back as 1.0
    assert_eq!(
        collect_records(load_xlsx_table(&resource, &options).unwrap()),
        json!([{
            "boolean": true,
            "date": "2025-01-01",
            "datetime": "2025-01-01T00:00:00",
            "integer": 1.0,
            "number": 1.1,
            "string": "string",
        }])
    );
}

mod xlsx {
    use super::*;

    #[test]
    fn test_should_save_table_to_file() {
        check_save_table_to_file("xlsx");
    }

    #[test]
    fn test_should_save_and_load_various_data_types() {
        check_save_and_load_various_data_types("xlsx");
    }
}

mod ods {
    use super::*;

    #[test]
    fn test_should_save_table_to_file() {
        check_save_table_to_file("ods");
    }

    #[test]
    fn test_should_save_and_load_various_data_types() {
        check_save_and_load_various_data_types("ods");
    }
}
