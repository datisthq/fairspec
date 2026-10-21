use std::io::Read;

use crate::helpers::http::{HttpRequest, load_remote_stream, send_http_request};
use crate::helpers::test_server::serve;

#[test]
fn test_send_http_request_returns_error_statuses() {
    let (url, server) = serve(vec![(404, b"missing".to_vec())]);
    let response = send_http_request(&HttpRequest {
        method: "POST".to_string(),
        url: format!("{url}/api"),
        headers: vec![("Authorization".to_string(), "token".to_string())],
        body: Some(b"payload".to_vec()),
    })
    .unwrap();
    let requests = server.join().unwrap();
    assert_eq!(response.status, 404);
    assert_eq!(response.body, b"missing");
    assert!(requests[0].request_line.starts_with("POST /api"));
    assert!(
        requests[0]
            .headers
            .contains(&"user-agent: fairspec-rust".to_string())
    );
    assert!(
        requests[0]
            .headers
            .contains(&"authorization: token".to_string())
    );
    assert_eq!(requests[0].body, b"payload");
}

#[test]
fn test_load_remote_stream_limits_bytes() {
    let (url, server) = serve(vec![(200, b"0123456789".to_vec())]);
    let mut data = Vec::new();
    load_remote_stream(&url, Some(4))
        .unwrap()
        .read_to_end(&mut data)
        .unwrap();
    server.join().unwrap();
    assert_eq!(data, b"0123");
}

#[test]
fn test_never_reaches_the_network() {
    let error = load_remote_stream("https://example.com/table.csv", None)
        .err()
        .unwrap();
    assert!(
        error
            .to_string()
            .contains("Network access is disabled in tests")
    );
}
