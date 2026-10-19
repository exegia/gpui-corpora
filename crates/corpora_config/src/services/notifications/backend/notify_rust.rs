use async_trait::async_trait;

use super::NotificationBackend;
use crate::notifications::{
    APP_DISPLAY_NAME, NotificationBackendKind, NotificationCapabilities,
    NotificationPermissionState, NotificationRequest,
};
// Linux/D-Bus-only icon wiring (see `send`); importing these unconditionally
// leaves them unused on macOS/Windows.
#[cfg(target_os = "linux")]
use std::path::{Path, PathBuf};

#[cfg(target_os = "linux")]
use crate::notifications::{APP_ICON_NAME, DESKTOP_ENTRY_ID};

#[cfg(all(test, target_os = "linux"))]
#[path = "notify_rust.test/mod.rs"]
mod notify_rust_test;

mod icon_candidates;
mod log;
mod notify_rust_backend;
mod probe_app_icon;
mod resolve_app_icon;

#[cfg(target_os = "linux")]
use icon_candidates::icon_candidates;
use log::LOG;
pub use notify_rust_backend::NotifyRustBackend;
#[cfg(target_os = "linux")]
use probe_app_icon::probe_app_icon;
#[cfg(target_os = "linux")]
use resolve_app_icon::resolve_app_icon;
