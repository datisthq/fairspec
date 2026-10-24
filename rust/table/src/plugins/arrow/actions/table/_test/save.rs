use fairspec_dataset::get_temp_file_path;
use fairspec_metadata::Resource;
use polars::prelude::*;
use serde_json::json;

use crate::models::table::{LoadTableOptions, SaveTableOptions};
use crate::plugins::_shared::collect_records;
use crate::plugins::arrow::actions::table::load::load_arrow_table;
use crate::plugins::arrow::actions::table::save::save_arrow_table;

fn create_options(path: &str) -> SaveTableOptions {
    SaveTableOptions {
        path: path.to_string(),
        ..Default::default()
    }
}

fn create_resource(path: &str) -> Resource {
    serde_json::from_value(json!({"data": path, "fileDialect": {"format": "arrow"}})).unwrap()
}

#[test]
fn test_should_save_table_to_arrow_file() {
    let path = get_temp_file_path(false, None, None).unwrap();
    let source = df!("id" => [1.0, 2.0, 3.0], "name" => ["Alice", "Bob", "Charlie"])
        .unwrap()
        .lazy();

    save_arrow_table(source, &create_options(&path)).unwrap();

    // Python's 1.0 == 1 hides it, but the whole floats load back as integers there too
    let table = load_arrow_table(&create_resource(&path), &Default::default()).unwrap();
    assert_eq!(
        collect_records(table),
        json!([
            {"id": 1, "name": "Alice"},
            {"id": 2, "name": "Bob"},
            {"id": 3, "name": "Charlie"},
        ])
    );
}

#[test]
fn test_should_save_and_load_various_data_types() {
    let path = get_temp_file_path(false, None, None).unwrap();
    let date = chrono::NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();
    let source = df!(
        "boolean" => [true],
        "date" => [date],
        "datetime" => [date.and_hms_opt(0, 0, 0).unwrap()],
        "integer" => [1i32],
        "number" => [1.1],
        "string" => ["string"],
    )
    .unwrap()
    .lazy()
    .with_column(col("datetime").cast(DataType::Datetime(
        TimeUnit::Microseconds,
        Some(TimeZone::UTC),
    )));

    save_arrow_table(source, &create_options(&path)).unwrap();

    let options = LoadTableOptions {
        denormalized: Some(true),
        ..Default::default()
    };
    let target = load_arrow_table(&create_resource(&path), &options).unwrap();
    let frame = target.clone().collect().unwrap();
    assert_eq!(
        frame.column("datetime").unwrap().dtype(),
        &DataType::Datetime(TimeUnit::Microseconds, Some(TimeZone::UTC))
    );
    let mut records = collect_records(target.drop(cols(["datetime"])));
    records[0]["datetime"] = json!(
        frame
            .column("datetime")
            .unwrap()
            .get(0)
            .unwrap()
            .to_string()
    );
    assert_eq!(
        records,
        json!([{
            "boolean": true,
            "date": "2025-01-01",
            "integer": 1,
            "number": 1.1,
            "string": "string",
            "datetime": "2025-01-01 00:00:00 UTC",
        }])
    );
}
