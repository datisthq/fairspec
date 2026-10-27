use std::io::{BufRead, BufReader, Write};
use std::net::TcpListener;
use std::thread;

use serde_json::json;

use crate::actions::descriptor::load::load_descriptor;
use crate::models::exception::Error;
use crate::settings::USER_AGENT;

const FIXTURES_DIR: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/actions/descriptor/-test/fixtures"
);

#[test]
fn test_load_local_descriptor() {
    let descriptor = load_descriptor(&format!("{FIXTURES_DIR}/schema.json"), false).unwrap();
    assert_eq!(descriptor["fields"][0]["name"], "id");
    assert_eq!(descriptor["fields"][1]["name"], "name");
}

#[test]
fn test_load_remote_descriptor() {
    let (url, server) = serve_once(r#"{"name": "test"}"#);
    let descriptor = load_descriptor(&url, false).unwrap();
    server.join().unwrap();
    assert_eq!(
        serde_json::Value::Object(descriptor),
        json!({"name": "test"})
    );
}

#[test]
fn test_load_remote_descriptor_bad_protocol() {
    let error = load_descriptor("ftp://example.com/file.json", false).unwrap_err();
    assert!(
        error
            .to_string()
            .contains("Unsupported remote protocol: ftp")
    );
}

#[test]
fn test_only_remote_rejects_local() {
    let error = load_descriptor("local.json", true).unwrap_err();
    assert!(error.to_string().contains("security"));
}

#[test]
fn test_load_remote_descriptor_sends_user_agent() {
    let (url, server) = serve_once("{}");
    load_descriptor(&url, false).unwrap();
    let headers = server.join().unwrap();
    assert!(headers.contains(&format!("user-agent: {USER_AGENT}")));
}

#[test]
fn test_load_remote_descriptor_never_reaches_network() {
    let error = load_descriptor("https://example.com/test.json", false).unwrap_err();
    assert!(matches!(error, Error::Http(_)));
}

/// Serves one HTTP response on the loopback host and returns the request's lowercased headers.
fn serve_once(body: &'static str) -> (String, thread::JoinHandle<String>) {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let url = format!("http://{}/test.json", listener.local_addr().unwrap());
    let server = thread::spawn(move || {
        let (mut stream, _) = listener.accept().unwrap();
        let mut headers = String::new();
        let mut reader = BufReader::new(stream.try_clone().unwrap());
        loop {
            let mut line = String::new();
            reader.read_line(&mut line).unwrap();
            if line.trim().is_empty() {
                break;
            }
            headers.push_str(&line.to_lowercase());
        }
        let response = format!(
            "HTTP/1.1 200 OK\r\ncontent-type: application/json\r\ncontent-length: {}\r\nconnection: close\r\n\r\n{body}",
            body.len()
        );
        stream.write_all(response.as_bytes()).unwrap();
        headers
    });
    (url, server)
}
