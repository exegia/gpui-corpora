#[allow(unused_imports)]
use super::*;

pub struct UserNotifyBackend {
    #[cfg(target_os = "macos")]
    pub(crate) manager: Arc<dyn user_notify::NotificationManager>,
    // Windows sends through win_toast; user-notify's backend there has no
    // action support, so only its AUMID shortcut setup is reused.
    #[cfg(target_os = "windows")]
    pub(crate) sink: crate::platform::win_toast::ResponseSink,
}

impl UserNotifyBackend {
    pub fn new() -> anyhow::Result<Self> {
        tracing::info!(target: LOG, app_id = APP_ID, "initializing user-notify backend");

        if let Err(reason) = platform_primary_runtime_status() {
            tracing::warn!(target: LOG, reason, "user-notify backend unavailable");
            anyhow::bail!("{reason}");
        }

        // Windows silently drops toasts whose AUMID has no Start Menu
        // shortcut; failure is logged, never fatal to the backend.
        #[cfg(target_os = "windows")]
        if let Err(reason) = crate::platform::toast_identity::ensure_toast_identity(APP_ID) {
            tracing::warn!(target: LOG, reason, "toast identity setup failed");
        }

        #[cfg(target_os = "windows")]
        let sink = {
            use std::sync::OnceLock;

            let sink: crate::platform::win_toast::ResponseSink = Arc::new(OnceLock::new());
            let _ = sink.set(Box::new(|response| {
                tracing::info!(
                    target: LOG,
                    action = ?response.action,
                    user_text = ?response.user_text,
                    tag = %response.tag,
                    "native notification response"
                );
            }));
            sink
        };

        #[cfg(target_os = "macos")]
        let manager = {
            let manager = user_notify::get_notification_manager(APP_ID.to_string(), None);
            if let Err(err) = manager.register(
                Box::new(|response| {
                    tracing::info!(
                        target: LOG,
                        action = ?response.action,
                        user_text = ?response.user_text,
                        user_info = ?response.user_info,
                        "native notification response"
                    );
                }),
                categories(),
            ) {
                tracing::warn!(target: LOG, error = %err, "failed to register user-notify manager");
                return Err(err.into());
            }
            manager
        };

        tracing::info!(target: LOG, "user-notify backend initialized");
        Ok(Self {
            #[cfg(target_os = "macos")]
            manager,
            #[cfg(target_os = "windows")]
            sink,
        })
    }
}

#[async_trait]
impl NotificationBackend for UserNotifyBackend {
    fn kind(&self) -> NotificationBackendKind {
        NotificationBackendKind::UserNotify
    }

    fn capabilities(&self) -> NotificationCapabilities {
        platform_capabilities()
    }

    async fn refresh_permission_state(&self) -> NotificationPermissionState {
        tracing::debug!(target: LOG, "refreshing user-notify permission state");
        let state = platform_permission_state().await;
        tracing::info!(target: LOG, ?state, "refreshed user-notify permission state");
        state
    }

    async fn request_permission(&self) -> NotificationPermissionState {
        platform_request_permission(self).await
    }

    async fn send(&self, request: &NotificationRequest) -> anyhow::Result<()> {
        tracing::info!(
            target: LOG,
            title = %request.title,
            importance = %request.importance,
            has_thread_id = request.thread_id.is_some(),
            has_category = request.category.is_some(),
            "sending notification through user-notify"
        );

        #[cfg(target_os = "macos")]
        {
            let mut user_info = HashMap::new();
            user_info.insert("importance".to_string(), request.importance.to_string());
            user_info.insert("play_sound".to_string(), request.play_sound.to_string());

            let mut builder = user_notify::NotificationBuilder::new()
                .title(&request.title)
                .body(&request.body)
                .set_user_info(user_info);

            if let Some(thread_id) = &request.thread_id {
                builder = builder.set_thread_id(thread_id);
            }

            if let Some(category) = &request.category {
                builder = builder.set_category_id(category);
            }

            match self.manager.send_notification(builder).await {
                Ok(_) => {
                    tracing::info!(target: LOG, "user-notify send succeeded");
                    Ok(())
                }
                Err(err) => {
                    tracing::warn!(target: LOG, error = %err, "user-notify send failed");
                    Err(err.into())
                }
            }
        }

        #[cfg(target_os = "windows")]
        {
            match crate::platform::win_toast::send(request, &self.sink) {
                Ok(_) => {
                    tracing::info!(target: LOG, "user-notify send succeeded");
                    Ok(())
                }
                Err(err) => {
                    tracing::warn!(target: LOG, error = %err, "user-notify send failed");
                    Err(err)
                }
            }
        }
    }
}
