//! Disposable API checks used by the native QA screen.

use std::{io::Write, path::PathBuf, time::Duration};

use anyhow::{Context, Result, bail, ensure};
use reqwest::{Client, Method, multipart};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use uuid::Uuid;

const BASE: &str = "https://api.exegia.co/v1";
const ARCHIVE: &[u8] = include_bytes!("api_qa/tiny.corpus");
const VIEWS: [&str; 5] = ["manifest", "index", "sections", "content", "nodes/3"];

/// User credentials loaded only when a protected test is explicitly started.
#[derive(Deserialize)]
pub struct Credentials {
    user_token: String,
    foreign_token: String,
    supabase_url: String,
    publishable_key: String,
}

/// A safe-to-share outcome; tokens and response headers are never included.
#[derive(Clone, Default, Serialize)]
pub struct Report {
    pub rows: Vec<String>,
    pub corpus_id: Option<Uuid>,
    pub import_id: Option<Uuid>,
    pub deletion_id: Option<Uuid>,
    pub deleted: bool,
    pub passed: bool,
}

impl Report {
    fn note(&mut self, text: impl Into<String>) {
        self.rows.push(text.into());
    }
}

fn client() -> Result<Client> {
    Client::builder()
        .timeout(Duration::from_secs(120))
        .connect_timeout(Duration::from_secs(15))
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .context("Could not start the QA HTTP client")
}

async fn request(
    client: &Client,
    method: Method,
    path: &str,
    token: Option<&str>,
    body: Option<Value>,
    expected: u16,
) -> Result<Value> {
    let mut request = client.request(method, format!("{BASE}{path}"));
    if let Some(token) = token {
        request = request.bearer_auth(token);
    }
    if let Some(body) = body {
        request = request.json(&body);
    }
    let response = request
        .send()
        .await
        .map_err(|_| anyhow::anyhow!("Network request failed for {path}"))?;
    decode(response, path, expected).await
}

async fn decode(mut response: reqwest::Response, path: &str, expected: u16) -> Result<Value> {
    ensure!(
        response.status().as_u16() == expected,
        "{path}: expected HTTP {expected}, got HTTP {}. Inspect the operation receipt before retrying a write.",
        response.status().as_u16()
    );
    if expected == 200 && VIEWS.iter().any(|view| path.ends_with(view)) {
        ensure!(
            response
                .headers()
                .get("cache-control")
                .and_then(|v| v.to_str().ok())
                == Some("no-store"),
            "{path}: missing no-store response header"
        );
    }
    let mut bytes = Vec::new();
    while let Some(chunk) = response
        .chunk()
        .await
        .map_err(|_| anyhow::anyhow!("Response interrupted for {path}"))?
    {
        ensure!(
            bytes.len() + chunk.len() <= 4_000_000,
            "Response exceeds QA size limit"
        );
        bytes.extend_from_slice(&chunk);
    }
    serde_json::from_slice(&bytes).map_err(|_| anyhow::anyhow!("{path}: expected JSON response"))
}

/// Check public health, capability schema, and unauthenticated access denial.
pub async fn public_checks() -> Report {
    let mut report = Report::default();
    let result = async {
        let client = client()?;
        let health = request(&client, Method::GET, "/health", None, None, 200).await?;
        ensure!(health["status"] == "ok", "Health response was not ok");
        report.note("PASS · Health: HTTP 200, status ok");
        let caps = request(&client, Method::GET, "/capabilities", None, None, 200).await?;
        ensure!(
            caps["auth_required"].is_boolean() && caps["hub_writable"].is_boolean(),
            "Capability fields are missing"
        );
        report.note(format!(
            "PASS · Capabilities: auth_required={}, legacy hub_writable={}",
            caps["auth_required"], caps["hub_writable"]
        ));
        request(&client, Method::GET, "/corpora", None, None, 401).await?;
        report.note("PASS · Private corpus list rejects anonymous access (401)");
        Ok::<_, anyhow::Error>(())
    }
    .await;
    finish(&mut report, result);
    report
}

fn finish(report: &mut Report, result: Result<()>) {
    match result {
        Ok(()) => report.passed = true,
        Err(error) => report.note(format!("FAIL · {error}")),
    }
}

fn credentials() -> Result<Credentials> {
    let path = std::env::var_os("CORPORA_QA_CREDENTIALS")
        .context("Set CORPORA_QA_CREDENTIALS to a private JSON file, then restart the app")?;
    let bytes = std::fs::read(path).context("Cannot read the QA credentials file")?;
    let config: Credentials = serde_json::from_slice(&bytes)
        .map_err(|_| anyhow::anyhow!("Invalid QA credentials JSON"))?;
    ensure!(
        !config.user_token.is_empty() && !config.foreign_token.is_empty(),
        "Two dedicated user sessions are required"
    );
    ensure!(
        config.supabase_url == "https://ivaecofevxactmmupvyp.supabase.co",
        "QA sessions must belong to the production API's authentication project"
    );
    Ok(config)
}

fn receipt_path() -> Result<PathBuf> {
    std::env::var_os("CORPORA_QA_RECEIPT")
        .map(PathBuf::from)
        .context("Set CORPORA_QA_RECEIPT to a new local receipt path, then restart the app")
}

fn save(path: &std::path::Path, report: &Report) -> Result<()> {
    let content = serde_json::to_vec_pretty(report)?;
    let mut file = atomic_write_file::AtomicWriteFile::open(path)
        .context("Could not prepare the QA receipt")?;
    file.write_all(&content)
        .context("Could not write the QA receipt")?;
    file.commit().context("Could not save the QA receipt")
}

async fn import(
    client: &Client,
    path: &str,
    operation: Uuid,
    token: &str,
    expected: u16,
) -> Result<Value> {
    let form = multipart::Form::new()
        .text("operation_id", operation.to_string())
        .text("expected_revision", "1")
        .part(
            "file",
            multipart::Part::bytes(ARCHIVE).file_name("tiny.corpus"),
        );
    let response = client
        .post(format!("{BASE}{path}/imports"))
        .bearer_auth(token)
        .multipart(form)
        .send()
        .await
        .map_err(|_| {
            anyhow::anyhow!("Import request interrupted; inspect its operation receipt")
        })?;
    decode(response, &format!("{path}/imports"), expected).await
}

async fn views(client: &Client, path: &str, token: Option<&str>, status: u16) -> Result<()> {
    for view in VIEWS {
        let value = request(
            client,
            Method::GET,
            &format!("{path}/{view}"),
            token,
            None,
            status,
        )
        .await?;
        if status == 200 {
            match view {
                "manifest" => ensure!(value["name"] == "Tiny", "Unexpected manifest"),
                "index" => ensure!(
                    value["sections"]["levels"] == json!(["book", "passage"]),
                    "Unexpected index"
                ),
                "sections" => ensure!(value["items"][0]["ref"] == "Mini", "Unexpected section"),
                "content" => {
                    let passages = value["passages"].as_array().context("Missing passages")?;
                    let text: String = passages.iter().filter_map(|p| p["text"].as_str()).collect();
                    ensure!(text == "In the gap and out.", "Source text was incomplete");
                }
                "nodes/3" => ensure!(
                    value["node"] == 3
                        && value["text"] == "gap "
                        && value["features"]["annotation"] == "uncovered",
                    "Source node or annotation changed"
                ),
                _ => unreachable!(),
            }
        }
    }
    Ok(())
}

/// Create, import, read, publish, make private, and delete one synthetic corpus.
/// An unfinished receipt prevents a new run until its recorded IDs are reconciled.
pub async fn lifecycle() -> Report {
    let mut report = Report::default();
    let result = async {
        let config = credentials()?;
        let receipt = receipt_path()?;
        if receipt.exists() {
            bail!("Receipt already exists. Inspect that run before choosing a fresh receipt path.");
        }
        let corpus = Uuid::new_v4();
        let operation = Uuid::new_v4();
        let deletion = Uuid::new_v4();
        report.corpus_id = Some(corpus);
        report.import_id = Some(operation);
        report.deletion_id = Some(deletion);
        // Persist identities before the first remote effect, including an uncertain creation.
        let file = std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&receipt)
            .context("Could not reserve a fresh QA receipt")?;
        serde_json::to_writer_pretty(&file, &report)?;
        file.sync_all()?;
        let result = run_lifecycle(&config, corpus, operation, deletion, &mut report).await;
        finish(&mut report, result);
        save(&receipt, &report)?;
        Ok::<_, anyhow::Error>(())
    }
    .await;
    if let Err(error) = result {
        report.passed = false;
        report.note(format!("FAIL · {error}"));
    }
    report
}

async fn run_lifecycle(
    config: &Credentials,
    corpus: Uuid,
    operation: Uuid,
    deletion: Uuid,
    report: &mut Report,
) -> Result<()> {
    let client = client()?;
    let owner = Some(config.user_token.as_str());
    let foreign = Some(config.foreign_token.as_str());
    let path = format!("/corpora/{corpus}");
    let identity = request(&client, Method::GET, "/auth/me", owner, None, 200).await?;
    let other = request(&client, Method::GET, "/auth/me", foreign, None, 200).await?;
    ensure!(
        identity["user_id"].is_string()
            && other["user_id"].is_string()
            && identity["user_id"] != other["user_id"],
        "Two different dedicated test users are required"
    );
    let create = json!({"id":corpus,"title":"Disposable desktop QA"});
    let created = request(
        &client,
        Method::POST,
        "/corpora",
        owner,
        Some(create.clone()),
        201,
    )
    .await?;
    ensure!(
        created["state"] == "repository_created",
        "Creation is not complete; inspect status"
    );
    report.note(format!("PASS · Created private corpus {corpus}"));
    let imported = import(&client, &path, operation, &config.user_token, 201).await?;
    ensure!(
        imported["state"] == "completed" && imported["sequence"] == 1,
        "Import is not complete"
    );
    ensure!(
        import(&client, &path, operation, &config.user_token, 200).await? == imported,
        "Import retry receipt changed"
    );
    report.note("PASS · Real archive imported; exact retry returned the same receipt");
    views(&client, &path, owner, 200).await?;
    report.note("PASS · Five owner views preserve text and annotation: In the gap and out.");
    views(&client, &path, None, 404).await?;
    views(&client, &path, foreign, 404).await?;
    report.note("PASS · Five private views deny anonymous and foreign users");
    let metadata = request(
        &client,
        Method::GET,
        &format!("{path}/metadata"),
        owner,
        None,
        200,
    )
    .await?;
    let revision = metadata["revision"]
        .as_u64()
        .context("Missing metadata revision")?;
    let public = request(
        &client,
        Method::PATCH,
        &format!("{path}/visibility"),
        owner,
        Some(json!({"expected_revision":revision,"visibility":"public"})),
        200,
    )
    .await?;
    views(&client, &path, None, 200).await?;
    views(&client, &path, foreign, 200).await?;
    views(&client, &path, Some("example-invalid-token"), 401).await?;
    report.note("PASS · Public views allow anonymous/foreign reads; invalid token stays denied");
    let private = request(
        &client,
        Method::PATCH,
        &format!("{path}/visibility"),
        owner,
        Some(json!({"expected_revision":public["revision"],"visibility":"private"})),
        200,
    )
    .await?;
    views(&client, &path, None, 404).await?;
    report.note("PASS · Switching back to private removes anonymous access");
    let remove = json!({"operation_id":deletion,"expected_revision":private["revision"]});
    request(
        &client,
        Method::DELETE,
        &path,
        foreign,
        Some(remove.clone()),
        404,
    )
    .await?;
    let deleted = request(
        &client,
        Method::DELETE,
        &path,
        owner,
        Some(remove.clone()),
        200,
    )
    .await?;
    ensure!(
        deleted["state"] == "completed",
        "Deletion is not complete; inspect its receipt"
    );
    report.deleted = true;
    ensure!(
        request(&client, Method::DELETE, &path, owner, Some(remove), 200).await? == deleted,
        "Deletion retry receipt changed"
    );
    views(&client, &path, owner, 404).await?;
    views(&client, &path, None, 404).await?;
    request(&client, Method::POST, "/corpora", owner, Some(create), 409).await?;
    report.note("PASS · Deleted corpus, verified exact retry, denied reads and UUID reuse");
    let response = client
        .post(format!("{}/auth/v1/logout", config.supabase_url))
        .header("apikey", &config.publishable_key)
        .bearer_auth(&config.user_token)
        .send()
        .await
        .map_err(|_| anyhow::anyhow!("Test session logout request failed"))?;
    ensure!(response.status().is_success(), "Test session logout failed");
    request(
        &client,
        Method::GET,
        &format!("{path}/deletions/{deletion}"),
        owner,
        None,
        401,
    )
    .await?;
    report.note("PASS · Signed out owner; its old token is denied (401)");
    report.note(
        "Account retirement and Hub absence must be verified separately by the QA administrator.",
    );
    Ok(())
}

#[cfg(test)]
#[path = "api_qa.test.rs"]
mod tests;
