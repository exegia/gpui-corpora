#[allow(unused_imports)]
use super::*;

/// Portal notification backend (no runtime handle needed — ashpd drives
/// zbus's own async-io executor thread).
pub struct PortalBackend;

impl PortalBackend {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl NotificationBackend for PortalBackend {
    fn kind(&self) -> NotificationBackendKind {
        NotificationBackendKind::Portal
    }

    fn capabilities(&self) -> NotificationCapabilities {
        NotificationCapabilities {
            can_request_permission: false,
            can_read_permission_state: false,
            can_send_immediate_native: true,
            can_send_interactive: false,
            // The portal is the correct path inside a packaged (Flatpak/Snap) runtime.
            requires_packaged_runtime: true,
        }
    }

    async fn refresh_permission_state(&self) -> NotificationPermissionState {
        NotificationPermissionState::Unsupported
    }

    async fn request_permission(&self) -> NotificationPermissionState {
        NotificationPermissionState::Unsupported
    }

    async fn send(&self, request: &NotificationRequest) -> anyhow::Result<()> {
        tracing::info!(target: LOG, title = %request.title, "sending notification through the XDG portal");

        // A successful AddNotification only means the portal accepted it —
        // there is no display confirmation.
        let proxy = ashpd::desktop::notification::NotificationProxy::new().await?;
        let notification = portal_notification(request);
        match proxy.add_notification(DESKTOP_ENTRY_ID, notification).await {
            Ok(()) => {
                tracing::info!(target: LOG, "portal notification accepted");
                Ok(())
            }
            Err(err) => {
                tracing::warn!(target: LOG, error = %err, "portal notification send failed");
                Err(err.into())
            }
        }
    }
}
