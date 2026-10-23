use std::io::{BufRead, BufReader, Write};
use std::net::TcpListener;
use std::thread;

use crate::actions::host::address::get_is_public_address;
use crate::actions::remote::agent::{create_http_error, create_policy_agent};
use crate::models::host::HostPolicy;

#[test]
fn test_any_reaches_loopback() {
    let base_url = serve(vec!["HTTP/1.1 200 OK\r\ncontent-length: 2\r\n\r\nok"]);
    let agent = create_policy_agent(
        HostPolicy::Any,
        ureq::Agent::config_builder(),
        get_is_public_address,
    );
    let text = agent
        .get(&base_url)
        .call()
        .unwrap()
        .body_mut()
        .read_to_string()
        .unwrap();
    assert_eq!(text, "ok");
}

#[test]
fn test_public_blocks_loopback_literal() {
    let base_url = serve(vec![]);
    assert_eq!(
        fetch_public(&base_url, get_is_public_address),
        "Host 127.0.0.1 resolves to non-public address 127.0.0.1 (set FAIRSPEC_HOST_POLICY=any to allow)"
    );
}

#[test]
fn test_public_blocks_resolved_name() {
    let base_url = serve(vec![]).replace("127.0.0.1", "localhost");
    assert!(
        fetch_public(&base_url, get_is_public_address)
            .starts_with("Host localhost resolves to non-public address")
    );
}

#[test]
fn test_public_rechecks_redirect() {
    let base_url = serve(vec![
        "HTTP/1.1 302 Found\r\nlocation: http://169.254.169.254/latest/meta-data/\r\ncontent-length: 0\r\n\r\n",
    ]);
    assert_eq!(
        fetch_public(&base_url, |address| address == "127.0.0.1"),
        "Host 169.254.169.254 resolves to non-public address 169.254.169.254 (set FAIRSPEC_HOST_POLICY=any to allow)"
    );
}

fn fetch_public(url: &str, is_public: fn(&str) -> bool) -> String {
    let agent = create_policy_agent(HostPolicy::Public, ureq::Agent::config_builder(), is_public);
    let error = agent.get(url).call().unwrap_err();
    create_http_error(url, error).to_string()
}

fn serve(responses: Vec<&'static str>) -> String {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let base_url = format!("http://{}/", listener.local_addr().unwrap());
    thread::spawn(move || {
        for (stream, response) in listener.incoming().zip(responses) {
            let mut stream = stream.unwrap();
            let mut reader = BufReader::new(stream.try_clone().unwrap());
            let mut line = String::new();
            while reader.read_line(&mut line).unwrap() > 2 {
                line.clear();
            }
            stream.write_all(response.as_bytes()).unwrap();
        }
    });
    base_url
}
