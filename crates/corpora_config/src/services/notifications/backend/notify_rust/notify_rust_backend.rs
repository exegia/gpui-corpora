#[allow(unused_imports)]
use super::*;

pub struct NotifyRustBackend;

impl NotifyRustBackend {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl NotificationBackend for NotifyRustBackend {
    fn kind(&self) -> NotificationBackendKind {
        NotificationBackendKind::NotifyRust
    }

    fn capabilities(&self) -> NotificationCapabilities {
        NotificationCapabilities {
            can_request_permission: false,
            can_read_permission_state: false,
            can_send_immediate_native: true,
            can_send_interactive: false,
            requires_packaged_runtime: cfg!(any(target_os = "macos", target_os = "windows")),
        }
    }

    async fn refresh_permission_state(&self) -> NotificationPermissionState {
        NotificationPermissionState::Unsupported
    }

    async fn request_permission(&self) -> NotificationPermissionState {
        NotificationPermissionState::Unsupported
    }

    async fn send(&self, request: &NotificationRequest) -> anyhow::Result<()> {
        tracing::info!(
            target: LOG,
            title = %request.title,
            importance = %request.importance,
            "sending notification through notify-rust"
        );

        #[cfg(target_os = "macos")]
        macos_application()?;

        let mut notification = notify_rust::Notification::new();
        notification
            .appname(APP_DISPLAY_NAME)
            .summary(&request.title)
            .body(&request.body);

        // Tells the daemon which .desktop entry we are so it can resolve icon
        // and app name (`.appname` alone is freeform, never looked up).
        #[cfg(target_os = "linux")]
        notification.hint(notify_rust::Hint::DesktopEntry(
            DESKTOP_ENTRY_ID.to_string(),
        ));

        // The hint brands installs only; portable runs (tarball, un-integrated
        // AppImage) need an explicit path or the theme name.
        #[cfg(target_os = "linux")]
        notification.icon(&resolve_app_icon());

        // GNOME suppresses Normal urgency under DND/mute/busy/fullscreen; only
        // Critical pierces those gates (GNOME Shell js/ui/messageTray.js).
        #[cfg(target_os = "linux")]
        notification.urgency(match request.importance {
            crate::notifications::NotificationImportance::BackgroundWorthy => {
                notify_rust::Urgency::Critical
            }
            crate::notifications::NotificationImportance::ForegroundOnly => {
                notify_rust::Urgency::Normal
            }
        });

        if request.play_sound {
            notification.sound_name("default");
        }

        #[cfg(target_os = "linux")]
        {
            // show_async() is driven by zbus's own executor; the sync show()
            // would block_on (async-io) and park the executor polling this trait.
            match notification.show_async().await {
                Ok(_) => {
                    tracing::info!(target: LOG, "notify-rust send succeeded");
                    Ok(())
                }
                Err(err) => {
                    tracing::warn!(target: LOG, error = %err, "notify-rust send failed");
                    Err(err.into())
                }
            }
        }
        #[cfg(not(target_os = "linux"))]
        {
            // macOS/Windows: only the sync show() exists (native backend — no
            // zbus block_on, so it doesn't park the executor like on Linux).
            match notification.show() {
                Ok(_) => {
                    tracing::info!(target: LOG, "notify-rust send succeeded");
                    Ok(())
                }
                Err(err) => {
                    tracing::warn!(target: LOG, error = %err, "notify-rust send failed");
                    Err(err.into())
                }
            }
        }
    }
}
