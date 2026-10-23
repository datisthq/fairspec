use fairspec_library::fairspec_dataset::write_temp_file;
use rmcp::model::CallToolResult;
use serde_json::{Value, json};

use crate::helpers::schema::TOOL_DEFINITIONS;
use crate::server::call_tool;

fn call(name: &str, arguments: Value) -> CallToolResult {
    call_tool(name, Some(arguments))
}

fn get_text(result: &CallToolResult) -> String {
    serde_json::to_string(&result.content).unwrap()
}

#[test]
fn test_should_list_tools_with_typescript_names() {
    let mut names: Vec<&str> = TOOL_DEFINITIONS
        .iter()
        .map(|tool| tool.name.as_ref())
        .collect();
    names.sort();
    assert_eq!(
        names,
        [
            "inferDataSchemaTool",
            "inferDatasetTool",
            "inferFileDialectTool",
            "inferTableSchemaTool",
            "previewFileTool",
            "queryDatasetTool",
            "queryTableTool",
            "validateDataSchemaTool",
            "validateDataTool",
            "validateDatasetTool",
            "validateTableSchemaTool",
            "validateTableTool",
        ]
    );
    for tool in TOOL_DEFINITIONS.iter() {
        assert!(
            tool.description
                .as_ref()
                .is_some_and(|text| !text.is_empty())
        );
        assert!(tool.output_schema.is_some());
    }
}

#[test]
fn test_should_call_tool_with_structured_output() {
    let path = write_temp_file("id,name\n1,alice\n2,bob\n", false, None, Some("csv")).unwrap();

    let result = call("inferTableSchemaTool", json!({"resource": {"data": path}}));

    assert_eq!(result.is_error, Some(false));
    assert_eq!(
        result.structured_content,
        Some(json!({
            "tableSchema": {"properties": {"id": {"type": "integer"}, "name": {"type": "string"}}}
        }))
    );
}

#[test]
fn test_should_report_tool_failure_as_error() {
    let result = call(
        "queryTableTool",
        json!({"resource": {}, "query": "SELECT * FROM self"}),
    );
    assert_eq!(result.is_error, Some(true));
}

#[test]
fn test_should_report_real_message_of_failed_query() {
    let path = write_temp_file("id,name\n1,alice\n", false, None, Some("csv")).unwrap();

    let result = call(
        "queryTableTool",
        json!({"resource": {"data": path}, "query": "SELEC nonsense"}),
    );

    assert_eq!(result.is_error, Some(true));
    assert!(
        get_text(&result).to_lowercase().contains("sql"),
        "{}",
        get_text(&result)
    );
}

#[test]
fn test_should_report_real_message_of_missing_file() {
    let result = call(
        "validateDatasetTool",
        json!({"source": "missing/dataset.json"}),
    );

    assert_eq!(result.is_error, Some(true));
    assert!(
        get_text(&result).contains("missing"),
        "{}",
        get_text(&result)
    );
}

#[test]
fn test_should_reject_response_over_budget() {
    let text = "x".repeat(60_000);
    let result = call(
        "inferDatasetTool",
        json!({"dataset": {"resources": [{"data": [{"text": text}]}]}}),
    );

    assert_eq!(result.is_error, Some(true));
    assert!(get_text(&result).contains("exceeds 50000 characters"));
}

#[test]
fn test_should_send_compact_json_text() {
    let result = call(
        "validateTableSchemaTool",
        json!({"source": {"properties": {}}}),
    );
    assert_eq!(
        get_text(&result),
        r#"[{"type":"text","text":"{\"report\":{\"valid\":true,\"errors\":[]},\"truncated\":false}"}]"#
    );
}

#[test]
fn test_should_report_unknown_tool() {
    let result = call("nopeTool", json!({}));
    assert_eq!(result.is_error, Some(true));
    assert!(get_text(&result).contains("Unknown tool: nopeTool"));
}
