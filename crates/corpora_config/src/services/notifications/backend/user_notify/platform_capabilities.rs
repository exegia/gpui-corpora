#[allow(unused_imports)]
use super::*;

// Compiled only on macOS/Windows (backend/mod.rs gates this module), so the
// capability table needs no cfg arms of its own.
pub(crate) fn platform_capabilities() -> NotificationCapabilities {
    NotificationCapabilities {
        can_request_permission: cfg!(target_os = "macos"),
        can_read_permission_state: cfg!(target_os = "macos"),
        can_send_immediate_native: true,
        can_send_interactive: cfg!(any(target_os = "macos", target_os = "windows")),
        requires_packaged_runtime: cfg!(target_os = "macos"),
    }
}
