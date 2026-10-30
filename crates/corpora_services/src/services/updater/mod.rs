mod apply;
mod check;
mod download;
mod types;

use gpui::UpdateGlobal as _;

// Re-exported so external import paths stay stable.
pub use types::{CheckForUpdates, PlatformAsset, UpdateManifest, UpdateSnapshot, UpdateStatus};

#[cfg(test)]
#[path = "../updater.test/mod.rs"]
mod updater_test;

mod apply_update;
mod check_for_updates;
mod check_pending_swap;
mod dispatch_background_notification;
mod download_update;
mod initialize;
mod notify_update_available;
mod notify_update_downloaded;
mod notify_update_error;
mod reset_check_retry;
mod reset_download_retry;
mod set_channel;
mod set_status;
mod snapshot;

pub use apply_update::apply_update;
pub use check_for_updates::check_for_updates;
pub use check_pending_swap::check_pending_swap;
use dispatch_background_notification::dispatch_background_notification;
pub use download_update::download_update;
pub use initialize::initialize;
use notify_update_available::notify_update_available;
use notify_update_downloaded::notify_update_downloaded;
use notify_update_error::notify_update_error;
use reset_check_retry::reset_check_retry;
use reset_download_retry::reset_download_retry;
pub use set_channel::set_channel;
use set_status::set_status;
pub use snapshot::snapshot;
