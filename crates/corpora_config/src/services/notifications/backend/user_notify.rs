#[cfg(target_os = "macos")]
use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;

use super::NotificationBackend;
#[cfg(target_os = "macos")]
use crate::notifications::{
    ACTION_OPEN, ACTION_REPLY, ACTION_SNOOZE, CATEGORY_ACTIONS, CATEGORY_REPLY,
};
use crate::notifications::{
    NotificationBackendKind, NotificationCapabilities, NotificationPermissionState,
    NotificationRequest,
};

#[cfg(test)]
mod tests {
    use super::platform_capabilities;

    #[test]
    fn interactive_capability_is_advertised() {
        // Pin: the Windows toast path carries real actions now, so a false
        // here means the capability regressed to macOS-only again.
        assert!(platform_capabilities().can_send_interactive);
    }
}

mod app_id;
mod categories;
mod log;
mod platform_capabilities;
mod platform_permission_state;
mod platform_primary_runtime_status;
mod platform_request_permission;
mod user_notify_backend;

use app_id::APP_ID;
#[cfg(target_os = "macos")]
use categories::categories;
use log::LOG;
pub(crate) use platform_capabilities::platform_capabilities;
#[cfg(target_os = "macos")]
use platform_permission_state::platform_permission_state;
#[cfg(not(target_os = "macos"))]
use platform_permission_state::platform_permission_state;
#[cfg(target_os = "macos")]
use platform_primary_runtime_status::platform_primary_runtime_status;
#[cfg(not(target_os = "macos"))]
use platform_primary_runtime_status::platform_primary_runtime_status;
#[cfg(target_os = "macos")]
use platform_request_permission::platform_request_permission;
#[cfg(not(target_os = "macos"))]
use platform_request_permission::platform_request_permission;
pub use user_notify_backend::UserNotifyBackend;
