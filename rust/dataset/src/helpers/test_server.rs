use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpListener;
use std::thread::{self, JoinHandle};

/// One request the loopback server received: its request line, lowercased headers and body.
#[derive(Debug, Clone, Default)]
pub struct ReceivedRequest {
    pub request_line: String,
    pub headers: Vec<String>,
    pub body: Vec<u8>,
}

/// Serves `responses` (status, body) in order on 127.0.0.1, one per connection; joining
/// the handle returns what each request sent.
pub fn serve(responses: Vec<(u16, Vec<u8>)>) -> (String, JoinHandle<Vec<ReceivedRequest>>) {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let url = format!("http://{}", listener.local_addr().unwrap());
    let handle = thread::spawn(move || {
        let mut received = Vec::new();
        for (status, body) in responses {
            let (mut stream, _) = listener.accept().unwrap();
            let mut reader = BufReader::new(stream.try_clone().unwrap());
            let mut request = ReceivedRequest::default();
            reader.read_line(&mut request.request_line).unwrap();
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
                request.headers.push(line);
            }
            request.body = vec![0; content_length];
            reader.read_exact(&mut request.body).unwrap();
            let head = format!(
                "HTTP/1.1 {status} OK\r\ncontent-length: {}\r\nconnection: close\r\n\r\n",
                body.len()
            );
            stream.write_all(head.as_bytes()).unwrap();
            stream.write_all(&body).unwrap();
            received.push(request);
        }
        received
    });
    (url, handle)
}
