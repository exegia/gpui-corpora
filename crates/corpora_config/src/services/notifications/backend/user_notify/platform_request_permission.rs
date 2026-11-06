#[allow(unused_imports)]
use super::*;

#[cfg(target_os = "macos")]
pub(crate) async fn platform_request_permission(
    backend: &UserNotifyBackend,
) -> NotificationPermissionState {
    tracing::info!(target: LOG, "requesting notification permission");
    match backend
        .manager
        .first_time_ask_for_notification_permission()
        .await
    {
        Ok(accepted) => {
            tracing::info!(target: LOG, accepted, "permission request completed");
            platform_permission_state().await
        }
        Err(err) => {
            tracing::warn!(target: LOG, error = %err, "permission request failed");
            NotificationPermissionState::Unavailable(format!("{err:#}"))
        }
    }
}

#[cfg(not(target_os = "macos"))]
pub(crate) async fn platform_request_permission(
    _backend: &UserNotifyBackend,
) -> NotificationPermissionState {
    tracing::info!(target: LOG, "permission request unsupported on this platform");
    NotificationPermissionState::Unsupported
}
