#[allow(unused_imports)]
use super::*;

#[derive(Clone, Debug)]
pub struct NotificationRuntimeSnapshot {
    pub enabled_by_user: bool,
    pub permission: NotificationPermissionState,
    pub active_backend: NotificationBackendKind,
    pub capabilities: NotificationCapabilities,
    pub last_backend_error: Option<SharedString>,
    pub degraded_reason: Option<SharedString>,
    /// Advisory: freedesktop capabilities the daemon advertises, probed at
    /// startup; servers may ignore hints they nominally advertise.
    pub daemon_capabilities: Option<SharedString>,
}

impl NotificationRuntimeSnapshot {
    pub fn new(service: &NotificationService) -> Self {
        Self {
            enabled_by_user: true,
            permission: NotificationPermissionState::Unknown,
            active_backend: service.active_backend(),
            capabilities: service.active_capabilities(),
            last_backend_error: service.initial_error.clone().map(Into::into),
            degraded_reason: service.initial_error.clone().map(Into::into),
            daemon_capabilities: None,
        }
    }
}
