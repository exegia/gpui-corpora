#[allow(unused_imports)]
use super::*;

#[derive(Clone, Debug, Default, Serialize)]
pub struct UpdateSnapshot {
    pub status: UpdateStatus,
    pub current_version: String,
    pub last_check: Option<String>,
    pub update_channel: String,
    pub check_retry_count: u32,
    pub download_retry_count: u32,
    // In-memory caches so `download_update` can reuse the manifest/asset that
    // `check_for_updates` already fetched, instead of hitting the network twice.
    // Skipped from serialization — they are ephemeral and do not outlive the run.
    #[serde(skip)]
    pub cached_manifest: Option<UpdateManifest>,
    #[serde(skip)]
    pub cached_asset: Option<PlatformAsset>,
}

impl gpui::Global for UpdateSnapshot {}
