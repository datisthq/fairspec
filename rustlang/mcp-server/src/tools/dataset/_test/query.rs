use serde_json::{Value, json};

use crate::tools::dataset::query::{QueryDatasetInput, QueryDatasetResult, query_dataset_tool};

fn query(input: Value) -> QueryDatasetResult {
    let input: QueryDatasetInput = serde_json::from_value(input).unwrap();
    query_dataset_tool(input).unwrap()
}

#[test]
fn test_should_join_two_resources_by_name() {
    let result = query(json!({
        "source": {"resources": [
            {"name": "users", "data": [{"id": 1, "name": "alice"}, {"id": 2, "name": "bob"}]},
            {"name": "posts", "data": [{"id": 1, "user_id": 2}, {"id": 2, "user_id": 2}]},
        ]},
        "query": "SELECT users.name, COUNT(*) AS n FROM posts \
                  JOIN users ON posts.user_id = users.id GROUP BY users.name",
    }));
    assert_eq!(json!(result.records), json!([{"name": "bob", "n": 2}]));
}

#[test]
fn test_should_return_at_most_limit_records() {
    let data: Vec<Value> = (0..10).map(|i| json!({"id": i})).collect();
    let result = query(json!({
        "source": {"resources": [{"name": "items", "data": data}]},
        "query": "SELECT * FROM items",
        "limit": 5,
    }));
    assert_eq!(result.records.len(), 5);
    assert!(result.truncated);
}
