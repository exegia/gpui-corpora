#[allow(unused_imports)]
use super::*;

/// Callers that also need a platform asset should use [`fetch_platform_asset`].
pub(crate) async fn fetch_manifest(
    rt: Arc<tokio::runtime::Runtime>,
    client: reqwest::Client,
) -> Result<UpdateManifest, String> {
    let response = rt
        .spawn(async move {
            client
                .get(DEFAULT_MANIFEST_URL)
                .timeout(std::time::Duration::from_secs(30))
                .send()
                .await
        })
        .await
        .map_err(|e| format!("manifest request panicked: {e}"))?
        .map_err(|e| format!("manifest request failed: {e}"))?;

    if !response.status().is_success() {
        return Err(format!("manifest returned status {}", response.status()));
    }

    let body = response
        .bytes()
        .await
        .map_err(|e| format!("failed to read manifest body: {e}"))?;

    serde_json::from_slice::<UpdateManifest>(&body)
        .map_err(|e| format!("failed to parse manifest: {e}"))
}
