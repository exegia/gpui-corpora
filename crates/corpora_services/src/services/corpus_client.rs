//! Supabase sign-in and session-bound corpus reads for the desktop workspace.

use std::{
    sync::Arc,
    time::{Duration, Instant},
};

use anyhow::{Context, Result, bail, ensure};
use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use reqwest::{Client, Response};
use serde::Deserialize;
use sha2::{Digest, Sha256};
use tokio::{
    io::{AsyncBufReadExt, AsyncRead, AsyncReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
    sync::Mutex,
};
use url::Url;
use uuid::Uuid;

const CALLBACK_PORT: u16 = 5173;
const MAX_RESPONSE: usize = 4_000_000;

#[derive(Clone)]
pub struct Config {
    auth_url: String,
    api_url: String,
    publishable_key: String,
}

impl Config {
    pub fn from_environment() -> Result<Self> {
        let auth_url = std::env::var("CORPORA_SUPABASE_URL")
            .unwrap_or_else(|_| "https://ivaecofevxactmmupvyp.supabase.co".into());
        let api_url =
            std::env::var("CORPORA_API_URL").unwrap_or_else(|_| "https://api.exegia.co/v1".into());
        let publishable_key = std::env::var("CORPORA_SUPABASE_PUBLISHABLE_KEY")
            .ok()
            .or_else(|| option_env!("CORPORA_SUPABASE_PUBLISHABLE_KEY").map(str::to_owned))
            .filter(|s| !s.trim().is_empty())
            .context("Supabase publishable key is not configured")?;
        for value in [&auth_url, &api_url] {
            let url = Url::parse(value).context("Invalid service URL")?;
            ensure!(
                url.scheme() == "https"
                    || (url.scheme() == "http"
                        && matches!(url.host_str(), Some("localhost" | "127.0.0.1" | "::1"))),
                "Service URLs must use HTTPS"
            );
            ensure!(
                url.username().is_empty()
                    && url.password().is_none()
                    && url.query().is_none()
                    && url.fragment().is_none(),
                "Invalid service URL"
            );
        }
        Ok(Self {
            auth_url: auth_url.trim_end_matches('/').into(),
            api_url: api_url.trim_end_matches('/').into(),
            publishable_key,
        })
    }
}

#[derive(Deserialize)]
struct TokenResponse {
    access_token: String,
    refresh_token: String,
    expires_in: u64,
    user: User,
}
#[derive(Deserialize)]
struct User {
    id: Uuid,
    email: Option<String>,
}

pub struct Session {
    access_token: String,
    refresh_token: String,
    expires_at: Instant,
    pub user_id: Uuid,
    pub email: Option<String>,
}
pub type SharedSession = Arc<Mutex<Session>>;

pub fn share(session: Session) -> SharedSession {
    Arc::new(Mutex::new(session))
}

impl Session {
    fn from_response(value: TokenResponse) -> Result<Self> {
        ensure!(
            !value.access_token.is_empty()
                && !value.refresh_token.is_empty()
                && value.expires_in > 0,
            "Invalid sign-in response"
        );
        Ok(Self {
            access_token: value.access_token,
            refresh_token: value.refresh_token,
            expires_at: Instant::now() + Duration::from_secs(value.expires_in.min(86400)),
            user_id: value.user.id,
            email: value.user.email,
        })
    }
}

#[derive(Clone, Deserialize)]
pub struct Corpus {
    pub id: Uuid,
    pub title: String,
    pub state: String,
    pub language: Option<String>,
}
#[derive(Deserialize)]
struct CorpusPage {
    items: Vec<Corpus>,
    next_cursor: Option<Uuid>,
}
#[derive(Clone, Deserialize)]
pub struct Passage {
    pub node: i64,
    #[serde(rename = "ref")]
    pub reference: String,
    pub text: String,
}
#[derive(Deserialize)]
pub struct Content {
    pub passages: Vec<Passage>,
    pub next_offset: Option<u64>,
}

#[derive(Clone, Deserialize)]
pub struct Section {
    #[serde(rename = "ref")]
    pub reference: String,
    #[serde(default)]
    pub child_count: u64,
}

#[derive(Deserialize)]
pub struct Sections {
    pub items: Vec<Section>,
    pub next_offset: Option<u64>,
}

fn client() -> Result<Client> {
    Client::builder()
        .connect_timeout(Duration::from_secs(10))
        .timeout(Duration::from_secs(30))
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .context("Could not start the network client")
}

async fn json<T: for<'de> Deserialize<'de>>(mut response: Response) -> Result<T> {
    let status = response.status();
    if !status.is_success() {
        bail!("Request failed (HTTP {status})");
    }
    let mut body = Vec::new();
    while let Some(chunk) = response
        .chunk()
        .await
        .map_err(|_| anyhow::anyhow!("Response interrupted"))?
    {
        ensure!(
            body.len() + chunk.len() <= MAX_RESPONSE,
            "Response is too large"
        );
        body.extend_from_slice(&chunk);
    }
    serde_json::from_slice(&body).context("Unexpected server response")
}

pub async fn password_sign_in(config: &Config, email: String, password: String) -> Result<Session> {
    let response = client()?
        .post(format!(
            "{}/auth/v1/token?grant_type=password",
            config.auth_url
        ))
        .header("apikey", &config.publishable_key)
        .json(&serde_json::json!({"email":email,"password":password}))
        .send()
        .await
        .map_err(|_| anyhow::anyhow!("Could not connect to sign in"))?;
    Session::from_response(json(response).await?)
}

fn pkce() -> (String, String, String) {
    let verifier = format!("{}{}", Uuid::new_v4().simple(), Uuid::new_v4().simple());
    let challenge = URL_SAFE_NO_PAD.encode(Sha256::digest(verifier.as_bytes()));
    (verifier, challenge, Uuid::new_v4().simple().to_string())
}

#[derive(Debug, PartialEq, Eq)]
enum Callback {
    Code(String),
    Cancelled,
}

fn callback_code(target: &str, path: &str) -> Option<Callback> {
    if !target.starts_with('/') || target.starts_with("//") {
        return None;
    }
    let url = Url::parse(&format!("http://localhost:{CALLBACK_PORT}{target}")).ok()?;
    if url.path() != path {
        return None;
    }
    if url.query_pairs().any(|(key, _)| key == "error") {
        return Some(Callback::Cancelled);
    }
    let codes: Vec<_> = url
        .query_pairs()
        .filter(|(key, _)| key == "code")
        .map(|(_, value)| value.into_owned())
        .collect();
    if codes.len() == 1 && !codes[0].is_empty() && codes[0].len() <= 4096 {
        Some(Callback::Code(codes[0].clone()))
    } else {
        None
    }
}

async fn callback_line(stream: &mut (impl AsyncRead + Unpin)) -> std::io::Result<String> {
    let mut bytes = Vec::new();
    BufReader::new(stream.take(8192))
        .read_until(b'\n', &mut bytes)
        .await?;
    if !bytes.ends_with(b"\r\n") {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Invalid callback",
        ));
    }
    String::from_utf8(bytes)
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid callback"))
}

pub async fn oauth_sign_in(config: &Config, provider: &'static str) -> Result<Session> {
    ensure!(
        matches!(provider, "apple" | "google"),
        "Unsupported provider"
    );
    let listener = TcpListener::bind((std::net::Ipv4Addr::LOCALHOST, CALLBACK_PORT))
        .await
        .map_err(|_| {
            anyhow::anyhow!("Port 5173 is in use. Stop the local web server and try again.")
        })?;
    let (verifier, challenge, nonce) = pkce();
    let path = format!("/native-auth/{nonce}");
    let mut authorize = Url::parse(&format!("{}/auth/v1/authorize", config.auth_url))?;
    authorize
        .query_pairs_mut()
        .append_pair("provider", provider)
        .append_pair(
            "redirect_to",
            &format!("http://localhost:{CALLBACK_PORT}{path}"),
        )
        .append_pair("code_challenge", &challenge)
        .append_pair("code_challenge_method", "s256");
    open::that(authorize.as_str())
        .map_err(|_| anyhow::anyhow!("Could not open the sign-in browser"))?;
    let code = tokio::time::timeout(Duration::from_secs(180), async {
        for _ in 0..20 {
            let (mut stream, peer) = listener.accept().await?;
            if !peer.ip().is_loopback() { continue; }
            let line = match tokio::time::timeout(Duration::from_secs(3), callback_line(&mut stream)).await {
                Ok(Ok(line)) => line,
                _ => continue,
            };
            let mut parts = line.split_whitespace();
            let code = match (parts.next(), parts.next(), parts.next(), parts.next()) {
                (Some("GET"), Some(target), Some("HTTP/1.1" | "HTTP/1.0"), None) => callback_code(target, &path),
                _ => None,
            };
            let body = match &code { Some(Callback::Code(_)) => "Sign-in received. You can return to Corpora.", Some(Callback::Cancelled) => "Sign-in was cancelled. You can return to Corpora.", None => "Invalid sign-in callback." };
            let reply = format!("HTTP/1.1 {}\r\nContent-Type: text/plain; charset=utf-8\r\nCache-Control: no-store\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}", if code.is_some() { "200 OK" } else { "400 Bad Request" }, body.len(), body);
            let _ = stream.write_all(reply.as_bytes()).await;
            match code { Some(Callback::Code(code)) => return Ok::<_, anyhow::Error>(code), Some(Callback::Cancelled) => bail!("Sign-in was not completed"), None => {} }
        }
        bail!("Too many invalid sign-in callbacks")
    }).await.map_err(|_| anyhow::anyhow!("Sign-in timed out"))??;
    let response = client()?
        .post(format!("{}/auth/v1/token?grant_type=pkce", config.auth_url))
        .header("apikey", &config.publishable_key)
        .json(&serde_json::json!({"auth_code":code,"code_verifier":verifier}))
        .send()
        .await
        .map_err(|_| anyhow::anyhow!("Could not complete sign-in"))?;
    Session::from_response(json(response).await?)
}

async fn refresh_if_needed(config: &Config, session: &mut Session) -> Result<()> {
    if Instant::now() + Duration::from_secs(30) < session.expires_at {
        return Ok(());
    }
    let response = client()?
        .post(format!(
            "{}/auth/v1/token?grant_type=refresh_token",
            config.auth_url
        ))
        .header("apikey", &config.publishable_key)
        .json(&serde_json::json!({"refresh_token":session.refresh_token}))
        .send()
        .await
        .map_err(|_| anyhow::anyhow!("Could not refresh the session"))?;
    let refreshed = Session::from_response(json(response).await?)?;
    ensure!(
        refreshed.user_id == session.user_id,
        "Session identity changed"
    );
    *session = refreshed;
    Ok(())
}

pub async fn sign_out(config: &Config, shared: SharedSession) -> Result<()> {
    let mut session = shared.lock().await;
    refresh_if_needed(config, &mut session).await?;
    let response = client()?
        .post(format!("{}/auth/v1/logout?scope=local", config.auth_url))
        .header("apikey", &config.publishable_key)
        .bearer_auth(&session.access_token)
        .send()
        .await
        .map_err(|_| anyhow::anyhow!("Could not revoke the session remotely"))?;
    ensure!(
        response.status().is_success(),
        "Could not revoke the session remotely (HTTP {})",
        response.status()
    );
    Ok(())
}

pub async fn list(
    config: &Config,
    shared: SharedSession,
    shared_corpora: bool,
    cursor: Option<Uuid>,
) -> Result<(Vec<Corpus>, Option<Uuid>)> {
    let mut session = shared.lock().await;
    refresh_if_needed(config, &mut session).await?;
    let path = if shared_corpora {
        "/corpora/shared"
    } else {
        "/corpora"
    };
    let mut request = client()?
        .get(format!("{}{path}", config.api_url))
        .bearer_auth(&session.access_token)
        .query(&[("limit", "50")]);
    if let Some(cursor) = cursor {
        request = request.query(&[("after", cursor.to_string())]);
    }
    let response = request
        .send()
        .await
        .map_err(|_| anyhow::anyhow!("Could not load corpora"))?;
    if shared_corpora && matches!(response.status().as_u16(), 404 | 422) {
        bail!("Shared corpora are not available on this API yet");
    }
    let page: CorpusPage = json(response).await?;
    ensure!(page.items.len() <= 50, "Invalid corpus page");
    if let (Some(previous), Some(next)) = (cursor, page.next_cursor) {
        ensure!(next > previous, "Invalid corpus page cursor");
    }
    Ok((page.items, page.next_cursor))
}

pub async fn content(
    config: &Config,
    shared: SharedSession,
    corpus: Uuid,
    offset: u64,
    reference: Option<String>,
) -> Result<Content> {
    let mut session = shared.lock().await;
    refresh_if_needed(config, &mut session).await?;
    let mut request = client()?
        .get(format!("{}/corpora/{corpus}/content", config.api_url))
        .bearer_auth(&session.access_token)
        .query(&[("offset", offset), ("limit", 50)]);
    if let Some(reference) = reference {
        request = request.query(&[("ref", reference)]);
    }
    let response = request
        .send()
        .await
        .map_err(|_| anyhow::anyhow!("Could not load corpus text"))?;
    let result: Content = json(response).await?;
    ensure!(result.passages.len() <= 50, "Invalid corpus content page");
    ensure!(
        result.next_offset.is_none_or(|next| next > offset),
        "Invalid content page cursor"
    );
    Ok(result)
}

pub async fn sections(
    config: &Config,
    shared: SharedSession,
    corpus: Uuid,
    offset: u64,
    parent: Option<String>,
) -> Result<Sections> {
    let mut session = shared.lock().await;
    refresh_if_needed(config, &mut session).await?;
    let mut request = client()?
        .get(format!("{}/corpora/{corpus}/sections", config.api_url))
        .bearer_auth(&session.access_token)
        .query(&[("offset", offset), ("limit", 50)]);
    if let Some(parent) = parent {
        request = request.query(&[("parent", parent)]);
    }
    let response = request
        .send()
        .await
        .map_err(|_| anyhow::anyhow!("Could not load corpus sections"))?;
    let page: Sections = json(response).await?;
    ensure!(
        page.items.len() <= 50 && page.next_offset.is_none_or(|next| next > offset),
        "Invalid section page"
    );
    Ok(page)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn callback_accepts_a_fragmented_request_line() {
        tokio::runtime::Runtime::new().unwrap().block_on(async {
            let (mut writer, mut reader) = tokio::io::duplex(64);
            let task = tokio::spawn(async move {
                writer.write_all(b"GET /native-").await.unwrap();
                tokio::task::yield_now().await;
                writer
                    .write_all(b"auth/test?code=ok HTTP/1.1\r\n")
                    .await
                    .unwrap();
            });
            let line = callback_line(&mut reader).await.unwrap();
            assert_eq!(line, "GET /native-auth/test?code=ok HTTP/1.1\r\n");
            task.await.unwrap();
        });
    }
    #[test]
    fn callback_requires_exact_unpredictable_path_and_one_code() {
        assert_eq!(
            callback_code("/native-auth/abc?code=hello", "/native-auth/abc"),
            Some(Callback::Code("hello".into()))
        );
        assert_eq!(
            callback_code("/native-auth/abc?error=access_denied", "/native-auth/abc"),
            Some(Callback::Cancelled)
        );
        assert!(callback_code("/native-auth/wrong?code=hello", "/native-auth/abc").is_none());
        assert!(callback_code("/native-auth/abc?code=a&code=b", "/native-auth/abc").is_none());
        assert!(callback_code("//evil.test/native-auth/abc?code=a", "/native-auth/abc").is_none());
    }
    #[test]
    fn pkce_generates_distinct_verifiers_and_valid_challenges() {
        let (verifier, challenge, nonce) = pkce();
        assert_eq!(verifier.len(), 64);
        assert_eq!(
            challenge,
            URL_SAFE_NO_PAD.encode(Sha256::digest(verifier.as_bytes()))
        );
        assert_ne!(nonce, pkce().2);
    }

    #[test]
    fn reader_sends_session_and_loads_requested_section() {
        tokio::runtime::Runtime::new().expect("runtime").block_on(async {
            let listener = TcpListener::bind((std::net::Ipv4Addr::LOCALHOST, 0)).await.unwrap();
            let address = listener.local_addr().unwrap();
            let server = tokio::spawn(async move {
                let (mut stream, _) = listener.accept().await.unwrap();
                let mut buffer = [0u8; 8192];
                let n = stream.read(&mut buffer).await.unwrap();
                let request = std::str::from_utf8(&buffer[..n]).unwrap().to_ascii_lowercase();
                assert!(request.contains("authorization: bearer test-session"));
                assert!(request.contains("ref=genesis+1"));
                let body = r#"{"passages":[{"node":1,"ref":"Genesis 1","text":"Example text"}],"next_offset":null}"#;
                let response = format!("HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}", body.len(), body);
                stream.write_all(response.as_bytes()).await.unwrap();
            });
            let config = Config { auth_url: format!("http://{address}"), api_url: format!("http://{address}/v1"), publishable_key: "test".into() };
            let session = share(Session { access_token: "test-session".into(), refresh_token: "test-refresh".into(), expires_at: Instant::now() + Duration::from_secs(600), user_id: Uuid::new_v4(), email: None });
            let page = content(&config, session, Uuid::new_v4(), 0, Some("Genesis 1".into())).await.unwrap();
            assert_eq!(page.passages[0].text, "Example text");
            server.await.unwrap();
        });
    }

    #[test]
    fn sections_request_preserves_parent_reference() {
        tokio::runtime::Runtime::new().unwrap().block_on(async {
            let listener = TcpListener::bind((std::net::Ipv4Addr::LOCALHOST, 0)).await.unwrap();
            let address = listener.local_addr().unwrap();
            let server = tokio::spawn(async move {
                let (mut stream, _) = listener.accept().await.unwrap();
                let mut buffer = [0u8; 8192];
                let n = stream.read(&mut buffer).await.unwrap();
                let request = std::str::from_utf8(&buffer[..n]).unwrap();
                assert!(request.contains("parent=Genesis+1"));
                let body = r#"{"items":[{"ref":"Genesis 1:1","child_count":0}],"next_offset":null}"#;
                let response = format!("HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}", body.len(), body);
                stream.write_all(response.as_bytes()).await.unwrap();
            });
            let config = Config { auth_url: format!("http://{address}"), api_url: format!("http://{address}/v1"), publishable_key: "test".into() };
            let session = share(Session { access_token: "test-session".into(), refresh_token: "test-refresh".into(), expires_at: Instant::now() + Duration::from_secs(600), user_id: Uuid::new_v4(), email: None });
            let page = sections(&config, session, Uuid::new_v4(), 0, Some("Genesis 1".into())).await.unwrap();
            assert_eq!(page.items[0].reference, "Genesis 1:1");
            server.await.unwrap();
        });
    }

    #[test]
    #[ignore = "requires a disposable hosted Auth user"]
    fn hosted_sign_in_list_and_sign_out() {
        let email = std::env::var("CORPORA_TEST_AUTH_EMAIL").expect("test email");
        let password = std::env::var("CORPORA_TEST_AUTH_PASSWORD").expect("test password");
        let config = Config::from_environment().expect("test configuration");
        tokio::runtime::Runtime::new()
            .expect("runtime")
            .block_on(async {
                let session = share(
                    password_sign_in(&config, email, password)
                        .await
                        .expect("sign-in"),
                );
                session.lock().await.expires_at = Instant::now();
                let (items, cursor) = list(&config, session.clone(), false, None)
                    .await
                    .expect("list");
                assert!(items.is_empty() && cursor.is_none());
                match list(&config, session.clone(), true, None).await {
                    Ok((shared, cursor)) => assert!(shared.is_empty() && cursor.is_none()),
                    Err(error) => assert_eq!(
                        error.to_string(),
                        "Shared corpora are not available on this API yet"
                    ),
                }
                sign_out(&config, session.clone()).await.expect("sign-out");
                assert!(list(&config, session, false, None).await.is_err());
            });
    }
}
