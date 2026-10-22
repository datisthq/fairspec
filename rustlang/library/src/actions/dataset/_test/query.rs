use fairspec_metadata::Dataset;
use serde_json::{Value, json};

use crate::actions::dataset::query::query_dataset;

fn create_dataset(resources: Value) -> Dataset {
    serde_json::from_value(json!({"resources": resources})).unwrap()
}

fn users() -> Value {
    json!([{"id": 1, "name": "Alice"}, {"id": 2, "name": "Bob"}])
}

fn posts() -> Value {
    json!([{"id": 1, "user_id": 2}, {"id": 2, "user_id": 1}, {"id": 3, "user_id": 2}])
}

fn collect_records(dataset: Dataset, query: &str) -> Vec<Value> {
    let frame = query_dataset(dataset, query, &Default::default())
        .unwrap()
        .collect()
        .unwrap();
    (0..frame.height())
        .map(|index| {
            let row = frame.get_row(index).unwrap();
            let record: serde_json::Map<String, Value> = frame
                .get_column_names()
                .iter()
                .zip(row.0)
                .map(|(name, value)| {
                    let value = match value.extract::<i64>() {
                        Some(integer) if value.is_integer() => json!(integer),
                        _ => json!(value.get_str().unwrap_or_default()),
                    };
                    (name.to_string(), value)
                })
                .collect();
            Value::Object(record)
        })
        .collect()
}

#[test]
fn test_join_resources_by_name() {
    let dataset = create_dataset(json!([
        {"name": "users", "data": users()},
        {"name": "posts", "data": posts()},
    ]));
    let records = collect_records(
        dataset,
        "SELECT name, COUNT(*) AS n FROM posts JOIN users ON posts.user_id = users.id \
         GROUP BY name ORDER BY name",
    );
    assert_eq!(
        records,
        [
            json!({"name": "Alice", "n": 1}),
            json!({"name": "Bob", "n": 2})
        ]
    );
}

#[test]
fn test_unnamed_resource_uses_inferred_name() {
    let dataset = create_dataset(json!([{"data": users()}]));
    let records = collect_records(dataset, "SELECT COUNT(*) AS n FROM resource1");
    assert_eq!(records, [json!({"n": 2})]);
}

#[test]
fn test_unreferenced_resource_is_not_loaded() {
    let dataset = create_dataset(json!([
        {"name": "users", "data": users()},
        {"name": "broken", "data": "missing.csv"},
    ]));
    let records = collect_records(dataset, "SELECT COUNT(*) AS n FROM users");
    assert_eq!(records, [json!({"n": 2})]);
}

#[test]
fn test_non_table_resource_is_skipped() {
    let dataset = create_dataset(json!([
        {"name": "users", "data": users()},
        {"name": "config", "data": {"key": "value"}},
    ]));
    let records = collect_records(
        dataset,
        "SELECT name FROM users WHERE name <> 'config' ORDER BY name",
    );
    assert_eq!(records, [json!({"name": "Alice"}), json!({"name": "Bob"})]);
}

#[test]
fn test_duplicate_name_raises() {
    let dataset = create_dataset(json!([
        {"name": "users", "data": users()},
        {"name": "users", "data": users()},
    ]));
    let error = query_dataset(dataset, "SELECT * FROM users", &Default::default())
        .err()
        .unwrap();
    assert_eq!(
        error.to_string(),
        "More than one resource is named \"users\""
    );
}
