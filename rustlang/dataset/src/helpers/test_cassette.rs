use std::collections::BTreeMap;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpListener;
use std::thread;

use serde::Deserialize;

/// Replays a VCR cassette on 127.0.0.1 and returns the server's base URL. Each request is
/// answered by the first unused interaction with the same method, path and query; absolute
/// URLs in a `Location` header are rewritten to the base URL so redirects stay local.
pub fn serve_cassette(path: &str) -> String {
    let text = std::fs::read_to_string(path).unwrap();
    let cassette: Cassette = serde_norway::from_str(&text).unwrap();
    let mut interactions: Vec<Option<Interaction>> =
        cassette.interactions.into_iter().map(Some).collect();

    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let base_url = format!("http://{}", listener.local_addr().unwrap());
    let local_base_url = base_url.clone();
    thread::spawn(move || {
        for stream in listener.incoming() {
            let mut stream = stream.unwrap();
            let (method, target) = read_request(&mut stream);
            let interaction = interactions
                .iter_mut()
                .find(|item| {
                    item.as_ref().is_some_and(|interaction| {
                        interaction.request.method == method
                            && get_target(&interaction.request.uri) == target
                    })
                })
                .and_then(Option::take);
            let response = match interaction {
                Some(interaction) => create_response(interaction.response, &local_base_url),
                None => b"HTTP/1.1 599 No Cassette Match\r\ncontent-length: 0\r\nconnection: close\r\n\r\n".to_vec(),
            };
            stream.write_all(&response).unwrap();
        }
    });
    base_url
}

#[derive(Deserialize)]
struct Cassette {
    interactions: Vec<Interaction>,
}

#[derive(Deserialize)]
struct Interaction {
    request: CassetteRequest,
    response: CassetteResponse,
}

#[derive(Deserialize)]
struct CassetteRequest {
    method: String,
    uri: String,
}

#[derive(Deserialize)]
struct CassetteResponse {
    body: CassetteBody,
    #[serde(default)]
    headers: BTreeMap<String, Vec<String>>,
    status: CassetteStatus,
}

#[derive(Deserialize)]
struct CassetteBody {
    string: String,
}

#[derive(Deserialize)]
struct CassetteStatus {
    code: u16,
}

fn read_request(stream: &mut std::net::TcpStream) -> (String, String) {
    let mut reader = BufReader::new(stream.try_clone().unwrap());
    let mut request_line = String::new();
    reader.read_line(&mut request_line).unwrap();
    let mut content_length = 0;
    loop {
        let mut line = String::new();
        reader.read_line(&mut line).unwrap();
        let line = line.trim_end().to_lowercase();
        if line.is_empty() {
            break;
        }
        if let Some(length) = line.strip_prefix("content-length:") {
            content_length = length.trim().parse().unwrap_or(0);
        }
    }
    let mut body = vec![0; content_length];
    reader.read_exact(&mut body).unwrap();

    let mut parts = request_line.split_whitespace();
    let method = parts.next().unwrap_or_default().to_string();
    let target = parts.next().unwrap_or_default().to_string();
    (method, target)
}

fn get_target(uri: &str) -> String {
    let url = url::Url::parse(uri).unwrap();
    url[url::Position::BeforePath..url::Position::AfterQuery].to_string()
}

fn create_response(response: CassetteResponse, base_url: &str) -> Vec<u8> {
    let mut head = format!("HTTP/1.1 {} Replayed\r\n", response.status.code);
    for (name, values) in &response.headers {
        let name = name.to_lowercase();
        if name == "content-type" {
            head.push_str(&format!("content-type: {}\r\n", values.join(", ")));
        }
        if name == "location" {
            let location = url::Url::parse(&values[0]).unwrap();
            let target = &location[url::Position::BeforePath..];
            head.push_str(&format!("location: {base_url}{target}\r\n"));
        }
    }
    let body = response.body.string.into_bytes();
    head.push_str(&format!(
        "content-length: {}\r\nconnection: close\r\n\r\n",
        body.len()
    ));
    let mut bytes = head.into_bytes();
    bytes.extend(body);
    bytes
}
