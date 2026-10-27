use fairspec_dataset::write_temp_file;
use fairspec_metadata::Resource;
use serde_json::{Value, json};

use crate::plugins::_shared::collect_records;
use crate::plugins::json::actions::table::load::load_json_table;

fn load(data: Value, dialect: Value) -> Option<Value> {
    let resource: Resource =
        serde_json::from_value(json!({"data": data, "fileDialect": dialect})).unwrap();
    load_json_table(&resource, &Default::default())
        .unwrap()
        .map(collect_records)
}

fn write(body: &str) -> String {
    write_temp_file(body, false, None, None).unwrap()
}

const BODY: &str = r#"[{"id":1,"name":"english"},{"id":2,"name":"中文"}]"#;
const LINES: &str = "{\"id\":1,\"name\":\"english\"}\n{\"id\":2,\"name\":\"中文\"}";

fn create_records() -> Value {
    json!([{"id": 1, "name": "english"}, {"id": 2, "name": "中文"}])
}

fn create_double_records() -> Value {
    json!([
        {"id": 1, "name": "english"},
        {"id": 2, "name": "中文"},
        {"id": 1, "name": "english"},
        {"id": 2, "name": "中文"},
    ])
}

mod json {
    use super::*;

    #[test]
    fn test_should_return_none_for_top_level_object() {
        assert_eq!(
            load(json!(write(r#"{"id": 1}"#)), json!({"format": "json"})),
            None
        );
    }

    #[test]
    fn test_should_load_local_file() {
        assert_eq!(
            load(json!(write(BODY)), json!({"format": "json"})),
            Some(create_records())
        );
    }

    #[test]
    fn test_should_load_local_file_multipart() {
        let paths = json!([write(BODY), write(BODY)]);
        assert_eq!(
            load(paths, json!({"format": "json"})),
            Some(create_double_records())
        );
    }

    #[test]
    fn test_should_handle_nested_json_pointer() {
        let path = write(r#"{"data": {"a/b": [{"id":1,"name":"english"}]}}"#);
        assert_eq!(
            load(
                json!(path),
                json!({"format": "json", "jsonPointer": "/data/a~1b"})
            ),
            Some(json!([{"id": 1, "name": "english"}]))
        );
    }

    #[test]
    fn test_should_take_column_names_for_an_empty_array() {
        let resource: Resource = serde_json::from_value(json!({
            "data": write(r#"{"items":[]}"#),
            "fileDialect": {
                "format": "json",
                "jsonPointer": "/items",
                "rowType": "object",
                "columnNames": ["id", "name"],
            },
        }))
        .unwrap();
        let frame = load_json_table(&resource, &Default::default())
            .unwrap()
            .unwrap()
            .collect()
            .unwrap();

        let names: Vec<String> = frame
            .get_column_names()
            .iter()
            .map(|name| name.to_string())
            .collect();
        assert_eq!(names, vec!["id", "name"]);
        assert_eq!(frame.height(), 0);
    }

    #[test]
    fn test_should_handle_property() {
        let path = write(r#"{"key": [{"id":1,"name":"english"},{"id":2,"name":"中文"}]}"#);
        assert_eq!(
            load(
                json!(path),
                json!({"format": "json", "jsonPointer": "/key"})
            ),
            Some(create_records())
        );
    }

    #[test]
    fn test_should_handle_item_keys() {
        assert_eq!(
            load(
                json!(write(BODY)),
                json!({"format": "json", "columnNames": ["name"]})
            ),
            Some(json!([{"name": "english"}, {"name": "中文"}]))
        );
    }

    #[test]
    fn test_should_handle_item_type_array() {
        let path = write(r#"[["id","name"],[1,"english"],[2,"中文"]]"#);
        assert_eq!(
            load(json!(path), json!({"format": "json", "rowType": "array"})),
            Some(create_records())
        );
    }

    #[test]
    fn test_should_load_item_type_object() {
        assert_eq!(
            load(
                json!(write(BODY)),
                json!({"format": "json", "rowType": "object"})
            ),
            Some(create_records())
        );
    }
}

mod jsonl {
    use super::*;

    #[test]
    fn test_should_load_local_file() {
        assert_eq!(
            load(json!(write(LINES)), json!({"format": "jsonl"})),
            Some(create_records())
        );
    }

    #[test]
    fn test_should_load_local_file_multipart() {
        let paths = json!([write(LINES), write(LINES)]);
        assert_eq!(
            load(paths, json!({"format": "jsonl"})),
            Some(create_double_records())
        );
    }

    #[test]
    fn test_should_handle_item_keys() {
        assert_eq!(
            load(
                json!(write(LINES)),
                json!({"format": "jsonl", "columnNames": ["name"]})
            ),
            Some(json!([{"name": "english"}, {"name": "中文"}]))
        );
    }

    #[test]
    fn test_should_handle_item_type_array() {
        let path = write("[\"id\",\"name\"]\n[1,\"english\"]\n[2,\"中文\"]");
        assert_eq!(
            load(json!(path), json!({"format": "jsonl", "rowType": "array"})),
            Some(create_records())
        );
    }

    #[test]
    fn test_should_read_array_rows_when_row_type_is_absent() {
        let path = write("[\"id\",\"name\"]\n[1,\"english\"]");
        assert_eq!(
            load(json!(path), json!({"format": "jsonl"})),
            Some(json!([{"id": 1, "name": "english"}]))
        );
    }

    #[test]
    fn test_should_load_item_type_object() {
        assert_eq!(
            load(
                json!(write(LINES)),
                json!({"format": "jsonl", "rowType": "object"})
            ),
            Some(create_records())
        );
    }
}
