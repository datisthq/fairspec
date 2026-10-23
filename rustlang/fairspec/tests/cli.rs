use std::io::{BufRead, BufReader, Write};
use std::process::{Child, Command, Stdio};
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

use serde_json::{Value, json};

const BINARY: &str = env!("CARGO_BIN_EXE_fairspec");
const TIMEOUT: Duration = Duration::from_secs(30);

fn run(args: &[&str]) -> (i32, String, String) {
    let output = Command::new(BINARY)
        .args(args)
        .stdin(Stdio::null())
        .output()
        .unwrap();
    (
        output.status.code().unwrap_or(-1),
        String::from_utf8_lossy(&output.stdout).into_owned(),
        String::from_utf8_lossy(&output.stderr).into_owned(),
    )
}

fn write_file(folder: &std::path::Path, name: &str, content: &str) -> String {
    let path = folder.join(name);
    std::fs::write(&path, content).unwrap();
    path.to_string_lossy().into_owned()
}

#[test]
fn test_should_print_the_package_version() {
    let (code, stdout, _) = run(&["--version"]);
    assert_eq!(code, 0);
    assert_eq!(stdout.trim(), env!("CARGO_PKG_VERSION"));
}

#[test]
fn test_should_describe_a_file() {
    let folder = tempfile::tempdir().unwrap();
    let path = write_file(folder.path(), "table.csv", "id,name\n1,alice\n");

    let (code, stdout, _) = run(&["file", "describe", &path, "--json"]);

    assert_eq!(code, 0);
    assert_eq!(
        serde_json::from_str::<Value>(&stdout).unwrap(),
        json!({
            "bytes": 16,
            "textual": true,
            "integrity": {
                "type": "sha256",
                "hash": "2b19eb0a580a952891021da1cb7fcca130eadd486130eb565605fc655b6a350b",
            },
        })
    );
}

#[test]
fn test_should_load_a_table_with_its_schema_types() {
    let folder = tempfile::tempdir().unwrap();
    let path = write_file(
        folder.path(),
        "table.csv",
        "id,name,active\n1,alice,true\n2,bob,false\n",
    );

    let (code, stdout, _) = run(&["table", "preview", &path, "--json"]);

    assert_eq!(code, 0);
    assert_eq!(
        serde_json::from_str::<Value>(&stdout).unwrap(),
        json!([{"id": 1, "name": "alice", "active": true}, {"id": 2, "name": "bob", "active": false}])
    );
}

#[test]
fn test_should_exit_one_for_an_invalid_table() {
    let folder = tempfile::tempdir().unwrap();
    let path = write_file(folder.path(), "table.csv", "id\n1\nx\n");
    let schema = write_file(
        folder.path(),
        "schema.json",
        r#"{"properties": {"id": {"type": "integer"}}}"#,
    );

    let (code, stdout, _) = run(&["table", "validate", &path, "--schema", &schema, "--json"]);

    assert_eq!(code, 1);
    assert_eq!(
        serde_json::from_str::<Value>(&stdout).unwrap(),
        json!({"valid": false, "errors": [
            {"type": "cell/type", "columnName": "id", "rowNumber": 2, "cell": "x", "columnType": "integer"}
        ]})
    );
}

#[test]
fn test_should_report_an_error_as_json() {
    let (code, stdout, _) = run(&["dataset", "list", "missing/dataset.json", "--json"]);

    assert_eq!(code, 1);
    assert!(serde_json::from_str::<Value>(&stdout).unwrap()["error"].is_string());
}

#[test]
fn test_should_report_a_missing_argument() {
    let (code, _, stderr) = run(&["dataset", "list"]);

    assert_eq!(code, 1);
    assert_eq!(stderr.trim(), "error: missing required argument 'path'");
}

/// Sends JSON-RPC lines to `fairspec mcp` and returns the response to each request; a
/// server that stops answering is killed after a timeout, so the test fails instead of hanging.
fn call_mcp(requests: &[Value]) -> Vec<Value> {
    let mut child = Command::new(BINARY)
        .arg("mcp")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .unwrap();
    let mut stdin = child.stdin.take().unwrap();
    let mut lines = vec![
        json!({"jsonrpc": "2.0", "id": 0, "method": "initialize", "params": {
            "protocolVersion": "2025-06-18", "capabilities": {}, "clientInfo": {"name": "test", "version": "1"},
        }}),
        json!({"jsonrpc": "2.0", "method": "notifications/initialized"}),
    ];
    lines.extend(requests.iter().cloned());
    for line in &lines {
        writeln!(stdin, "{line}").unwrap();
    }

    let stdout = child.stdout.take().unwrap();
    let (sender, receiver) = mpsc::channel();
    thread::spawn(move || {
        for line in BufReader::new(stdout).lines().map_while(Result::ok) {
            if sender.send(line).is_err() {
                break;
            }
        }
    });

    let mut responses = Vec::new();
    while responses.len() < requests.len() + 1 {
        match receiver.recv_timeout(TIMEOUT) {
            Ok(line) => responses.push(serde_json::from_str::<Value>(&line).unwrap()),
            Err(_) => {
                let _ = child.kill();
                let _ = child.wait();
                panic!(
                    "fairspec mcp gave {} of {} responses",
                    responses.len(),
                    requests.len() + 1
                );
            }
        }
    }
    drop(stdin);
    wait_or_kill(&mut child);

    responses.retain(|response| response["id"] != 0);
    responses.sort_by_key(|response| response["id"].as_i64());
    responses
}

fn wait_or_kill(child: &mut Child) {
    let deadline = Instant::now() + TIMEOUT;
    while Instant::now() < deadline {
        if child.try_wait().ok().flatten().is_some() {
            return;
        }
        thread::sleep(Duration::from_millis(50));
    }
    let _ = child.kill();
    let _ = child.wait();
    panic!("fairspec mcp did not exit after stdin closed");
}

#[test]
fn test_should_serve_tools_over_stdio() {
    let responses = call_mcp(&[json!({"jsonrpc": "2.0", "id": 1, "method": "tools/list"})]);

    let names: Vec<&str> = responses[0]["result"]["tools"]
        .as_array()
        .unwrap()
        .iter()
        .map(|tool| tool["name"].as_str().unwrap())
        .collect();
    assert!(names.contains(&"inferTableSchemaTool"));
}

#[test]
fn test_should_call_tool_over_stdio() {
    let folder = tempfile::tempdir().unwrap();
    let path = write_file(folder.path(), "table.csv", "id,name\n1,alice\n2,bob\n");

    let responses = call_mcp(&[
        json!({"jsonrpc": "2.0", "id": 1, "method": "tools/call", "params": {
            "name": "inferTableSchemaTool", "arguments": {"resource": {"data": path}},
        }}),
    ]);

    let result = &responses[0]["result"];
    assert_eq!(result["isError"], false);
    assert_eq!(
        result["structuredContent"],
        json!({"tableSchema": {"properties": {"id": {"type": "integer"}, "name": {"type": "string"}}}})
    );
}

#[test]
fn test_should_report_real_message_of_failed_tool_over_stdio() {
    let responses = call_mcp(&[
        json!({"jsonrpc": "2.0", "id": 1, "method": "tools/call", "params": {
            "name": "validateDatasetTool", "arguments": {"source": "missing/dataset.json"},
        }}),
    ]);

    let result = &responses[0]["result"];
    assert_eq!(result["isError"], true);
    assert!(result["content"].to_string().contains("missing"));
}
