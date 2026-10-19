use std::sync::Arc;

use super::types::*;
use gpui::{App, UpdateGlobal as _};

mod check_for_updates;
mod check_retry_field;
mod download_retry_field;
mod fetch_manifest;
mod fetch_platform_asset;
mod handle_check_failure;
mod schedule_retry;

pub use check_for_updates::check_for_updates;
pub(super) use check_retry_field::check_retry_field;
pub(super) use download_retry_field::download_retry_field;
pub(crate) use fetch_manifest::fetch_manifest;
pub(crate) use fetch_platform_asset::fetch_platform_asset;
use handle_check_failure::handle_check_failure;
pub(super) use schedule_retry::schedule_retry;
