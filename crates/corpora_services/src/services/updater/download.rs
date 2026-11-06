use std::sync::Arc;
use std::sync::atomic::{AtomicU32, Ordering};

use super::types::*;
use gpui::App;
use gpui::AsyncApp;
use gpui::UpdateGlobal as _;

mod download_outcome;
mod download_update;
mod handle_download_failure;
mod run_download;
mod verify_codesign;
mod verify_ed25519_signature;

use download_outcome::DownloadOutcome;
pub use download_update::download_update;
use handle_download_failure::handle_download_failure;
use run_download::run_download;
#[cfg(target_os = "macos")]
use verify_codesign::verify_codesign;
pub(super) use verify_ed25519_signature::verify_ed25519_signature;
