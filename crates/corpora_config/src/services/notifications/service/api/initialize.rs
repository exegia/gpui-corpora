#[allow(unused_imports)]
use super::*;

pub fn initialize(cx: &mut App) {
    let service = Arc::new(NotificationService::new());
    let snapshot = NotificationRuntimeSnapshot::new(&service);
    tracing::info!(
        target: LOG,
        active_backend = %snapshot.active_backend,
        permission = ?snapshot.permission,
        capabilities = ?snapshot.capabilities,
        degraded_reason = ?snapshot.degraded_reason,
        "installing native notification global state"
    );
    let degraded = snapshot.degraded_reason.is_some();
    let reason = snapshot.degraded_reason.clone();
    let last_error = snapshot.last_backend_error.clone();
    cx.set_global(NativeNotificationState { service, snapshot });
    crate::capabilities::set(
        "native_notifications",
        crate::capabilities::CapabilityStatus {
            supported: true,
            enabled: true,
            degraded,
            reason,
            last_error,
        },
        cx,
    );
    refresh_permission_state(cx);
    refresh_daemon_state(cx);
}
