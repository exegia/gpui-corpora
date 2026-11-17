use super::*;
use std::{
    io::{Read, Write},
    net::TcpListener,
};

fn response(status: u16, body: &'static str, cache: &'static str) -> reqwest::Response {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let address = listener.local_addr().unwrap();
    std::thread::spawn(move || {
        let (mut stream, _) = listener.accept().unwrap();
        let mut request = [0; 4096];
        stream.read(&mut request).unwrap();
        write!(stream, "HTTP/1.1 {status} OK\r\nContent-Length: {}\r\nCache-Control: {cache}\r\nConnection: close\r\n\r\n{body}", body.len()).unwrap();
    });
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async {
        Client::new()
            .get(format!("http://{address}"))
            .send()
            .await
            .unwrap()
    })
}

#[test]
fn detail_without_no_store_is_rejected() {
    let response = response(200, "{}", "public");
    let runtime = tokio::runtime::Runtime::new().unwrap();
    let error = runtime
        .block_on(decode(response, "/corpora/example/content", 200))
        .unwrap_err();
    assert!(error.to_string().contains("no-store"));
}

#[test]
fn unexpected_response_does_not_reveal_body() {
    let response = response(503, "private upstream diagnostic", "no-store");
    let runtime = tokio::runtime::Runtime::new().unwrap();
    let error = runtime
        .block_on(decode(response, "/corpora/example/imports", 201))
        .unwrap_err();
    assert!(!error.to_string().contains("private upstream"));
    assert!(error.to_string().contains("503"));
}

#[test]
fn receipt_preserves_ids_without_credentials() {
    let report = Report {
        corpus_id: Some(Uuid::new_v4()),
        import_id: Some(Uuid::new_v4()),
        deletion_id: Some(Uuid::new_v4()),
        ..Report::default()
    };
    let value = serde_json::to_value(&report).unwrap();
    assert_eq!(value["corpus_id"], report.corpus_id.unwrap().to_string());
    assert!(value.get("user_token").is_none());
}
