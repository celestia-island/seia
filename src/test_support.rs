//! In-crate test helpers: a tiny HTTP/1.1 mock server and a proxy-free HTTP
//! client.
//!
//! The mock server exists so engine tests can assert on the *request* they
//! build (method, path, headers, JSON body) as well as the response parsing,
//! without adding a dev-dependency or touching the public internet.

use std::{
    sync::{Arc, Mutex},
    time::Duration,
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

/// A fully recorded inbound request.
#[derive(Debug, Clone)]
pub struct RecordedRequest {
    pub method: String,
    pub path: String,
    pub headers: Vec<(String, String)>,
    pub body: String,
}

impl RecordedRequest {
    /// Case-insensitive header lookup.
    #[must_use]
    pub fn header(&self, name: &str) -> Option<&str> {
        self.headers
            .iter()
            .find(|(key, _)| key.eq_ignore_ascii_case(name))
            .map(|(_, value)| value.as_str())
    }

    /// Parse the request body as JSON (panics with the raw body on failure).
    #[must_use]
    pub fn json(&self) -> serde_json::Value {
        serde_json::from_str(&self.body)
            .unwrap_or_else(|e| panic!("request body is not JSON ({e}): {}", self.body))
    }
}

/// A loopback HTTP server that answers with canned responses and records the
/// requests it receives.
///
/// The i-th request gets `responses[i]`; the last response repeats for any
/// further requests. Requests are recorded before the response is written, so
/// assertions right after a client call are race-free.
pub struct MockServer {
    /// Base URL (e.g. `http://127.0.0.1:41234`) to point the engine at.
    pub base_url: String,
    requests: Arc<Mutex<Vec<RecordedRequest>>>,
}

impl MockServer {
    pub async fn spawn(responses: Vec<(u16, &str)>) -> Self {
        assert!(
            !responses.is_empty(),
            "mock server needs at least one canned response"
        );
        let responses = Arc::new(
            responses
                .into_iter()
                .map(|(status, body)| (status, body.to_string()))
                .collect::<Vec<_>>(),
        );
        let listener = TcpListener::bind(("127.0.0.1", 0))
            .await
            .expect("bind mock server");
        let addr = listener.local_addr().expect("mock server local addr");
        let requests = Arc::new(Mutex::new(Vec::new()));
        let recorded = Arc::clone(&requests);

        tokio::spawn(async move {
            let mut served = 0usize;
            loop {
                let Ok((mut sock, _)) = listener.accept().await else {
                    break;
                };
                let (status, body) = responses[served.min(responses.len() - 1)].clone();
                served += 1;
                let recorded = Arc::clone(&recorded);
                tokio::spawn(async move {
                    let Some(request) = read_request(&mut sock).await else {
                        return;
                    };
                    recorded.lock().expect("mock request log").push(request);
                    let _ = write_response(&mut sock, status, &body).await;
                });
            }
        });

        Self {
            base_url: format!("http://{addr}"),
            requests,
        }
    }

    /// One-shot 200 with a JSON body.
    pub async fn spawn_ok(body: &str) -> Self {
        Self::spawn(vec![(200, body)]).await
    }

    #[must_use]
    pub fn requests(&self) -> Vec<RecordedRequest> {
        self.requests.lock().expect("mock request log").clone()
    }

    #[must_use]
    pub fn last_request(&self) -> RecordedRequest {
        self.requests
            .lock()
            .expect("mock request log")
            .last()
            .expect("mock server received no request")
            .clone()
    }
}

/// A proxy-free HTTP client for tests (loopback mocks must never be routed
/// through an ambient `HTTP_PROXY`).
#[must_use]
pub fn http_client() -> reqwest::Client {
    let _ = rustls::crypto::ring::default_provider().install_default();
    reqwest::Client::builder()
        .no_proxy()
        .timeout(Duration::from_secs(10))
        .build()
        .expect("test HTTP client")
}

async fn read_request(sock: &mut TcpStream) -> Option<RecordedRequest> {
    let mut buf = Vec::new();
    let mut chunk = [0u8; 4096];

    let head_end = loop {
        if let Some(pos) = find_subsequence(&buf, b"\r\n\r\n") {
            break pos;
        }
        let n = sock.read(&mut chunk).await.ok()?;
        if n == 0 {
            return None;
        }
        buf.extend_from_slice(&chunk[..n]);
        if buf.len() > (1 << 20) {
            return None;
        }
    };

    let head = String::from_utf8_lossy(&buf[..head_end]).to_string();
    let mut lines = head.lines();
    let mut request_line = lines.next()?.split_whitespace();
    let method = request_line.next()?.to_string();
    let path = request_line.next()?.to_string();
    let headers: Vec<(String, String)> = lines
        .filter_map(|line| {
            let (key, value) = line.split_once(':')?;
            Some((key.trim().to_string(), value.trim().to_string()))
        })
        .collect();

    let content_length = headers
        .iter()
        .find(|(key, _)| key.eq_ignore_ascii_case("content-length"))
        .and_then(|(_, value)| value.parse::<usize>().ok())
        .unwrap_or(0);

    let body_start = head_end + 4;
    let mut body = buf[body_start..].to_vec();
    while body.len() < content_length {
        let n = sock.read(&mut chunk).await.ok()?;
        if n == 0 {
            break;
        }
        body.extend_from_slice(&chunk[..n]);
    }
    body.truncate(content_length);

    Some(RecordedRequest {
        method,
        path,
        headers,
        body: String::from_utf8_lossy(&body).to_string(),
    })
}

async fn write_response(sock: &mut TcpStream, status: u16, body: &str) -> std::io::Result<()> {
    let reason = match status {
        200 => "OK",
        201 => "Created",
        204 => "No Content",
        400 => "Bad Request",
        401 => "Unauthorized",
        403 => "Forbidden",
        404 => "Not Found",
        429 => "Too Many Requests",
        500 => "Internal Server Error",
        502 => "Bad Gateway",
        503 => "Service Unavailable",
        _ => "Status",
    };
    let response = format!(
        "HTTP/1.1 {status} {reason}\r\ncontent-type: application/json\r\ncontent-length: {}\r\nconnection: close\r\n\r\n{body}",
        body.len()
    );
    sock.write_all(response.as_bytes()).await?;
    sock.shutdown().await
}

fn find_subsequence(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack
        .windows(needle.len())
        .position(|window| window == needle)
}
