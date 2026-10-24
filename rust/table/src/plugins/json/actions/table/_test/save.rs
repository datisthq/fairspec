use fairspec_dataset::get_temp_file_path;
use fairspec_metadata::Resource;
use polars::prelude::*;
use serde_json::{Value, json};

use crate::models::table::{LoadTableOptions, SaveTableOptions};
use crate::plugins::_shared::collect_records;
use crate::plugins::json::actions::table::load::load_json_table;
use crate::plugins::json::actions::table::save::save_json_table;

fn create_table() -> LazyFrame {
    df!("id" => [1i64, 2], "name" => ["english", "中文"])
        .unwrap()
        .lazy()
}

fn save(dialect: Value) -> String {
    let path = get_temp_file_path(false, None, None).unwrap();
    let options = SaveTableOptions {
        path: path.clone(),
        file_dialect: Some(serde_json::from_value(dialect).unwrap()),
        ..Default::default()
    };
    save_json_table(create_table(), &options).unwrap();
    std::fs::read_to_string(path).unwrap()
}

mod json {
    use super::*;

    #[test]
    fn test_should_save_table_to_file() {
        assert_eq!(
            save(json!({"format": "json"})),
            "[\n  {\n    \"id\": 1,\n    \"name\": \"english\"\n  },\n  {\n    \"id\": 2,\n    \"name\": \"中文\"\n  }\n]"
        );
    }

    #[test]
    fn test_should_handle_property() {
        assert_eq!(
            save(json!({"format": "json", "jsonPointer": "/key"})),
            "{\n  \"key\": [\n    {\n      \"id\": 1,\n      \"name\": \"english\"\n    },\n    {\n      \"id\": 2,\n      \"name\": \"中文\"\n    }\n  ]\n}"
        );
    }

    #[test]
    fn test_should_handle_item_keys() {
        assert_eq!(
            save(json!({"format": "json", "columnNames": ["name"]})),
            "[\n  {\n    \"name\": \"english\"\n  },\n  {\n    \"name\": \"中文\"\n  }\n]"
        );
    }

    #[test]
    fn test_should_handle_item_type_array() {
        assert_eq!(
            save(json!({"format": "json", "rowType": "array"})),
            "[\n  [\n    \"id\",\n    \"name\"\n  ],\n  [\n    1,\n    \"english\"\n  ],\n  [\n    2,\n    \"中文\"\n  ]\n]"
        );
    }

    #[test]
    fn test_should_save_and_load_various_data_types() {
        let path = get_temp_file_path(false, None, Some("json")).unwrap();
        let source = df!(
            "boolean" => [true],
            "date" => ["2025-01-01"],
            "integer" => [1i32],
            "number" => [1.1],
            "string" => ["string"],
        )
        .unwrap()
        .lazy()
        .with_column(
            lit(Series::new(
                "list".into(),
                [Series::new("".into(), [1.0f32, 2.0, 3.0])],
            ))
            .first()
            .alias("list"),
        )
        .select([cols(["boolean", "date", "integer", "list", "number", "string"]).as_expr()]);

        let options = SaveTableOptions {
            path: path.clone(),
            ..Default::default()
        };
        save_json_table(source, &options).unwrap();

        let resource: Resource = serde_json::from_value(json!({"data": path})).unwrap();
        let options = LoadTableOptions {
            denormalized: Some(true),
            ..Default::default()
        };
        let target = load_json_table(&resource, &options).unwrap().unwrap();
        assert_eq!(
            collect_records(target),
            json!([{
                "boolean": true,
                "date": "2025-01-01",
                "integer": 1,
                "list": [1.0, 2.0, 3.0],
                "number": 1.1,
                "string": "string",
            }])
        );
    }
}

mod jsonl {
    use super::*;

    #[test]
    fn test_should_save_table_to_file() {
        assert_eq!(
            save(json!({"format": "jsonl"})),
            "{\"id\": 1, \"name\": \"english\"}\n{\"id\": 2, \"name\": \"中文\"}"
        );
    }

    #[test]
    fn test_should_handle_item_keys() {
        assert_eq!(
            save(json!({"format": "jsonl", "columnNames": ["name"]})),
            "{\"name\": \"english\"}\n{\"name\": \"中文\"}"
        );
    }

    #[test]
    fn test_should_handle_item_type_array() {
        assert_eq!(
            save(json!({"format": "jsonl", "rowType": "array"})),
            "[\"id\", \"name\"]\n[1, \"english\"]\n[2, \"中文\"]"
        );
    }

    #[test]
    fn test_should_handle_item_type_object() {
        assert_eq!(
            save(json!({"format": "jsonl", "rowType": "object"})),
            "{\"id\": 1, \"name\": \"english\"}\n{\"id\": 2, \"name\": \"中文\"}"
        );
    }
}
