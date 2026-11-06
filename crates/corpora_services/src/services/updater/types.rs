use std::path::PathBuf;

use gpui::actions;
use serde::{Deserialize, Serialize};

actions!(updater, [CheckForUpdates]);

mod current_app_version;
mod default_manifest_url;
mod max_update_retries;
mod pending_swap_path;
mod periodic_check_interval_secs;
mod platform_asset;
mod platform_key;
mod retry_base_delay_secs;
mod startup_check_delay_secs;
mod update_manifest;
mod update_snapshot;
mod update_status;
mod updater_public_key;
mod updates_dir;

pub(crate) use current_app_version::current_app_version;
pub(crate) use default_manifest_url::DEFAULT_MANIFEST_URL;
pub(crate) use max_update_retries::MAX_UPDATE_RETRIES;
pub(crate) use pending_swap_path::pending_swap_path;
pub(crate) use periodic_check_interval_secs::PERIODIC_CHECK_INTERVAL_SECS;
pub use platform_asset::PlatformAsset;
pub(crate) use platform_key::platform_key;
pub(crate) use retry_base_delay_secs::RETRY_BASE_DELAY_SECS;
pub(crate) use startup_check_delay_secs::STARTUP_CHECK_DELAY_SECS;
pub use update_manifest::UpdateManifest;
pub use update_snapshot::UpdateSnapshot;
pub use update_status::UpdateStatus;
pub(crate) use updater_public_key::UPDATER_PUBLIC_KEY;
pub(crate) use updates_dir::updates_dir;
