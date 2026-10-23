use std::collections::HashSet;

use serde_json::json;

use super::{run, write_file};

#[test]
fn test_should_list_every_command_once_in_llms_manifest() {
    let result = run(&["--llms"]);
    let manifest = result.json();
    let names: Vec<&str> = manifest["tools"]
        .as_array()
        .unwrap()
        .iter()
        .map(|tool| tool["name"].as_str().unwrap())
        .collect();
    assert_eq!(names.iter().collect::<HashSet<_>>().len(), names.len());
    for name in [
        "table validate",
        "table infer-dialect",
        "data infer-dialect",
        "file infer-dialect",
        "mcp",
    ] {
        assert!(names.contains(&name), "{name}");
    }
}

#[test]
fn test_should_use_camel_case_keys_in_llms_manifest() {
    let manifest = run(&["--llms"]).json();
    let tool = manifest["tools"]
        .as_array()
        .unwrap()
        .iter()
        .find(|tool| tool["name"] == "table validate")
        .unwrap()
        .clone();
    let properties = &tool["inputSchema"]["properties"];
    assert!(properties.get("headerRows").is_some());
    assert!(properties.get("schema").is_some());
}

#[test]
fn test_should_take_command_input_as_json() {
    let folder = tempfile::tempdir().unwrap();
    let path = write_file(folder.path(), "table.csv", "id,name\n1,alice\n2,bob");
    let input = json!({"path": path}).to_string();

    let result = run(&["table", "preview", "--json", &input]);

    assert_eq!(result.code, 0, "{}", result.stderr);
    assert_eq!(
        result.json(),
        json!([{"id": 1, "name": "alice"}, {"id": 2, "name": "bob"}])
    );
}

#[test]
fn test_should_parse_comma_separated_option_from_command_line() {
    let folder = tempfile::tempdir().unwrap();
    let path = write_file(folder.path(), "table.csv", "id,name\n1,alice\n2,-");

    let result = run(&[
        "table",
        "infer-schema",
        &path,
        "--missing-values",
        "-,n/a",
        "--json",
    ]);

    assert_eq!(result.json()["missingValues"], json!(["-", "n/a"]));
}

#[test]
fn test_should_collect_repeated_check_option() {
    let folder = tempfile::tempdir().unwrap();
    let path = write_file(folder.path(), "table.csv", "id\n1\n2");

    let result = run(&[
        "table", "validate", &path, "--check", "a=id > 1", "--check", "b=id > 2", "--json",
    ]);

    let names: HashSet<String> = result.json()["errors"]
        .as_array()
        .unwrap()
        .iter()
        .map(|error| error["checkName"].as_str().unwrap().to_string())
        .collect();
    assert_eq!(names, HashSet::from(["a".to_string(), "b".to_string()]));
    assert_eq!(result.code, 1);
}
