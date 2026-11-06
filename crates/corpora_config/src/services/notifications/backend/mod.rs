mod notify_rust;
// ashpd has no non-Unix build; the feature exists for Flatpak/Snap Linux runs.
#[cfg(all(feature = "notifications-portal", target_os = "linux"))]
mod portal;
#[cfg(any(target_os = "macos", target_os = "windows"))]
mod user_notify;

pub use notify_rust::NotifyRustBackend;
#[cfg(all(feature = "notifications-portal", target_os = "linux"))]
pub use portal::PortalBackend;
#[cfg(any(target_os = "macos", target_os = "windows"))]
pub use user_notify::UserNotifyBackend;

use async_trait::async_trait;

use crate::notifications::{
    NotificationBackendKind, NotificationCapabilities, NotificationPermissionState,
    NotificationRequest,
};

// nightly clippy flags async_trait's generated #[must_use] wrappers as
// double_must_use; the allow lives here since the attribute is macro-emitted.
#[allow(clippy::double_must_use)]
#[async_trait]
pub trait NotificationBackend: Send + Sync {
    fn kind(&self) -> NotificationBackendKind;
    fn capabilities(&self) -> NotificationCapabilities;
    async fn refresh_permission_state(&self) -> NotificationPermissionState;
    async fn request_permission(&self) -> NotificationPermissionState;
    async fn send(&self, request: &NotificationRequest) -> anyhow::Result<()>;
}
