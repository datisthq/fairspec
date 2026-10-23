use fairspec_library::fairspec_dataset::write_temp_file;
use serde_json::{Value, json};

use crate::helpers::limit::count_json_chars;
use crate::tools::table::query::{QueryTableInput, QueryTableResult, query_table_tool};

fn query(resource: Value, query: &str, limit: Option<usize>) -> QueryTableResult {
    let mut input = json!({"resource": resource, "query": query});
    if let Some(limit) = limit {
        input["limit"] = json!(limit);
    }
    let input: QueryTableInput = serde_json::from_value(input).unwrap();
    query_table_tool(input).unwrap()
}

fn write_csv(text: &str) -> Value {
    json!({"data": write_temp_file(text, false, None, Some("csv")).unwrap()})
}

#[test]
fn test_should_filter_records() {
    let resource = write_csv("id,name,age\n1,alice,25\n2,bob,30\n3,carol,28\n");
    let result = query(resource, "SELECT * FROM self WHERE age > 25", None);
    assert_eq!(
        json!(result.records),
        json!([{"id": 2, "name": "bob", "age": 30}, {"id": 3, "name": "carol", "age": 28}])
    );
}

#[test]
fn test_should_aggregate_records() {
    let resource = write_csv("id,score\n1,10\n2,20\n");
    let result = query(resource, "SELECT SUM(score) AS total FROM self", None);
    assert_eq!(json!(result.records), json!([{"total": 30}]));
}

#[test]
fn test_should_return_json_values() {
    let resource = write_csv("id,born\n1,2000-01-02\n2,\n");
    let result = query(resource, "SELECT * FROM self", None);
    assert_eq!(
        json!(result.records),
        json!([{"id": 1, "born": "2000-01-02"}, {"id": 2, "born": null}])
    );
}

#[test]
fn test_should_query_inline_data() {
    let resource = json!({"data": [{"id": 1, "name": "alice"}, {"id": 2, "name": "bob"}]});
    let result = query(resource, "SELECT name FROM self WHERE id = 2", None);
    assert_eq!(json!(result.records), json!([{"name": "bob"}]));
}

#[test]
fn test_should_raise_for_unloadable_table() {
    let input: QueryTableInput =
        serde_json::from_value(json!({"resource": {}, "query": "SELECT * FROM self"})).unwrap();
    let Err(error) = query_table_tool(input) else {
        panic!("expected an error");
    };
    assert!(error.to_string().contains("Could not load table"));
}

#[test]
fn test_should_return_at_most_100_records_by_default() {
    let rows: Vec<String> = (0..150).map(|i| format!("{i},name{i}")).collect();
    let resource = write_csv(&format!("id,name\n{}\n", rows.join("\n")));
    let result = query(resource, "SELECT * FROM self", None);
    assert_eq!(result.records.len(), 100);
    assert!(result.truncated);
}

#[test]
fn test_should_respect_limit() {
    let rows: Vec<String> = (0..10).map(|i| format!("{i},name{i}")).collect();
    let resource = write_csv(&format!("id,name\n{}\n", rows.join("\n")));
    let result = query(resource, "SELECT * FROM self", Some(5));
    assert_eq!(result.records.len(), 5);
    assert!(result.truncated);
}

#[test]
fn test_should_not_mark_complete_records_as_truncated() {
    let result = query(write_csv("id\n1\n2\n"), "SELECT * FROM self", None);
    assert!(!result.truncated);
}

#[test]
fn test_should_trim_records_to_response_budget() {
    let rows: Vec<String> = (0..100)
        .map(|i| format!("{i},{}", "x".repeat(1000)))
        .collect();
    let resource = write_csv(&format!("id,text\n{}\n", rows.join("\n")));
    let result = query(resource, "SELECT * FROM self", None);
    assert!(result.records.len() < 50);
    assert!(result.truncated);
    assert!(count_json_chars(&result) <= 50_000);
}
