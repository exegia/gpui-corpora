//! XDG Desktop Portal notification backend (Flatpak/Snap, `notifications-portal`
//! feature), selected when sandboxed. ashpd/zbus drive their own executor thread.

#![cfg(all(feature = "notifications-portal", target_os = "linux"))]

use async_trait::async_trait;

use super::NotificationBackend;
use crate::notifications::{
    APP_ICON_NAME, DESKTOP_ENTRY_ID, NotificationBackendKind, NotificationCapabilities,
    NotificationPermissionState, NotificationRequest,
};

#[cfg(test)]
#[path = "portal.test.rs"]
mod portal_test;

mod log;
mod portal_backend;
mod portal_notification;

use log::LOG;
pub use portal_backend::PortalBackend;
use portal_notification::portal_notification;
