#[allow(unused_imports)]
use super::*;

pub(crate) async fn fetch_platform_asset(
    rt: Arc<tokio::runtime::Runtime>,
    client: reqwest::Client,
) -> Result<PlatformAsset, String> {
    let manifest = fetch_manifest(rt, client).await?;
    let key = platform_key();
    manifest
        .platforms
        .get(&key)
        .cloned()
        .ok_or_else(|| format!("no asset for platform '{key}' in manifest"))
}
