#[allow(unused_imports)]
use super::*;

/// Select the preferred (primary) notification backend, or `None` to let the
/// synchronous NotifyRustBackend be the active path (Linux native).
pub(crate) fn select_primary_backend() -> (Option<Arc<dyn NotificationBackend>>, Option<String>) {
    // macOS / Windows: user-notify gives the richest native experience.
    #[cfg(any(target_os = "macos", target_os = "windows"))]
    {
        match UserNotifyBackend::new() {
            Ok(backend) => {
                tracing::info!(
                    target: LOG,
                    backend = %NotificationBackendKind::UserNotify,
                    "primary notification backend selected"
                );
                (
                    Some(Arc::new(backend) as Arc<dyn NotificationBackend>),
                    None,
                )
            }
            Err(err) => {
                tracing::warn!(
                    target: LOG,
                    backend = %NotificationBackendKind::UserNotify,
                    error = %err,
                    "primary notification backend unavailable; falling back"
                );
                (None, Some(err.to_string()))
            }
        }
    }

    // Sandboxed (Flatpak/Snap) Linux + the opt-in portal feature: route through
    // the XDG Desktop Portal instead of org.freedesktop.Notifications directly.
    #[cfg(all(feature = "notifications-portal", target_os = "linux"))]
    if crate::platform::environment::is_sandboxed() {
        tracing::info!(
            target: LOG,
            backend = %NotificationBackendKind::Portal,
            "sandbox detected; selecting XDG portal backend"
        );
        return (
            Some(Arc::new(super::super::PortalBackend::new()) as Arc<dyn NotificationBackend>),
            None,
        );
    } else {
        tracing::info!(
            target: LOG,
            "native (non-sandboxed) Linux; notify-rust (FreeDesktop/D-Bus) is the active backend"
        );
    }

    // Linux native (or portal feature off): NotifyRust is the active backend.
    // The cfg keeps the tail from being flagged unreachable on other targets.
    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    {
        (None, None)
    }
}
