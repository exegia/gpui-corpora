#[allow(unused_imports)]
use super::*;

pub struct NotificationService {
    pub(crate) primary: Option<Arc<dyn NotificationBackend>>,
    pub(crate) secondary: Arc<dyn NotificationBackend>,
    pub(crate) initial_error: Option<String>,
}

impl NotificationService {
    pub fn new() -> Self {
        tracing::info!(target: LOG, "initializing native notification service");
        let secondary = Arc::new(NotifyRustBackend::new()) as Arc<dyn NotificationBackend>;
        let (primary, primary_error) = select_primary_backend();
        Self::with_backends(primary, secondary, primary_error)
    }

    /// Construct with explicit backends (primarily a test hook): `primary` is
    /// tried first, `secondary` is the fallback on any primary `Err` or panic.
    pub(crate) fn with_backends(
        primary: Option<Arc<dyn NotificationBackend>>,
        secondary: Arc<dyn NotificationBackend>,
        initial_error: Option<String>,
    ) -> Self {
        let service = Self {
            primary,
            secondary,
            initial_error,
        };

        tracing::info!(
            target: LOG,
            active_backend = %service.active_backend(),
            capabilities = ?service.active_capabilities(),
            degraded_reason = ?service.initial_error,
            "native notification service initialized"
        );

        service
    }

    pub(crate) fn active_backend(&self) -> NotificationBackendKind {
        self.primary
            .as_ref()
            .map(|backend| backend.kind())
            .unwrap_or(NotificationBackendKind::NotifyRust)
    }

    pub(crate) fn active_capabilities(&self) -> NotificationCapabilities {
        self.primary
            .as_ref()
            .map(|backend| backend.capabilities())
            .unwrap_or_else(|| self.secondary.capabilities())
    }

    pub(crate) async fn refresh_permission_state(&self) -> NotificationPermissionState {
        tracing::debug!(
            target: LOG,
            active_backend = %self.active_backend(),
            "refreshing notification permission state"
        );

        if let Some(primary) = &self.primary {
            primary.refresh_permission_state().await
        } else if cfg!(target_os = "macos") {
            let state = NotificationPermissionState::Unavailable(
                self.initial_error
                    .clone()
                    .unwrap_or_else(|| "primary backend unavailable".to_string()),
            );
            tracing::info!(target: LOG, ?state, "permission state unavailable without primary backend");
            state
        } else {
            tracing::info!(target: LOG, "permission state unsupported on this platform");
            NotificationPermissionState::Unsupported
        }
    }

    pub(crate) async fn request_permission(&self) -> NotificationPermissionState {
        tracing::info!(
            target: LOG,
            active_backend = %self.active_backend(),
            "requesting notification permission"
        );

        if let Some(primary) = &self.primary {
            primary.request_permission().await
        } else if cfg!(target_os = "macos") {
            let state = NotificationPermissionState::Unavailable(
                self.initial_error
                    .clone()
                    .unwrap_or_else(|| "primary backend unavailable".to_string()),
            );
            tracing::warn!(target: LOG, ?state, "cannot request permission without primary backend");
            state
        } else {
            tracing::info!(target: LOG, "permission request unsupported on this platform");
            NotificationPermissionState::Unsupported
        }
    }

    pub async fn send(
        &self,
        request: NotificationRequest,
        enabled_by_user: bool,
    ) -> NotificationSendResult {
        tracing::info!(
            target: LOG,
            title = %request.title,
            importance = %request.importance,
            prefer_native = request.prefer_native,
            enabled_by_user,
            active_backend = %self.active_backend(),
            "native notification send requested"
        );

        if !enabled_by_user || !request.prefer_native {
            tracing::warn!(
                target: LOG,
                enabled_by_user,
                prefer_native = request.prefer_native,
                importance = %request.importance,
                "native send skipped; using in-app policy"
            );
            return NotificationSendResult {
                backend_used: NotificationBackendKind::UiOnly,
                degraded: request.importance == NotificationImportance::BackgroundWorthy,
                delivered_natively: false,
                error_summary: Some("native notifications are disabled".into()),
                importance: request.importance,
            };
        }

        let mut errors = Vec::new();

        if let Some(primary) = &self.primary {
            tracing::debug!(target: LOG, backend = %primary.kind(), "attempting primary notification send");
            // catch_unwind: a panicking backend must not abort the send task —
            // the panic becomes a regular error so the fallback is attempted.
            match AssertUnwindSafe(primary.send(&request))
                .catch_unwind()
                .await
            {
                Ok(Ok(())) => {
                    tracing::info!(target: LOG, backend = %primary.kind(), "primary notification send succeeded");
                    return NotificationSendResult {
                        backend_used: primary.kind(),
                        degraded: false,
                        delivered_natively: true,
                        error_summary: None,
                        importance: request.importance,
                    };
                }
                Ok(Err(err)) => {
                    tracing::warn!(
                        target: LOG,
                        backend = %primary.kind(),
                        error = %err,
                        "primary notification send failed"
                    );
                    errors.push(format!("{}: {err:#}", primary.kind()));
                }
                Err(panic_payload) => {
                    let msg = panic_payload_to_string(panic_payload);
                    tracing::error!(
                        target: LOG,
                        backend = %primary.kind(),
                        panic = %msg,
                        "primary notification send panicked; falling back to secondary"
                    );
                    errors.push(format!("{}: panic: {msg}", primary.kind()));
                }
            }
        }

        tracing::debug!(
            target: LOG,
            backend = %self.secondary.kind(),
            "attempting fallback notification send"
        );
        match self.secondary.send(&request).await {
            Ok(()) => {
                tracing::info!(
                    target: LOG,
                    backend = %self.secondary.kind(),
                    degraded = self.primary.is_some(),
                    "fallback notification send succeeded"
                );
                NotificationSendResult {
                    backend_used: self.secondary.kind(),
                    degraded: self.primary.is_some(),
                    delivered_natively: true,
                    error_summary: if errors.is_empty() {
                        None
                    } else {
                        Some(errors.join("; ").into())
                    },
                    importance: request.importance,
                }
            }
            Err(err) => {
                tracing::warn!(
                    target: LOG,
                    backend = %self.secondary.kind(),
                    error = %err,
                    "fallback notification send failed; using in-app policy"
                );
                errors.push(format!("{}: {err:#}", self.secondary.kind()));
                NotificationSendResult {
                    backend_used: NotificationBackendKind::UiOnly,
                    degraded: true,
                    delivered_natively: false,
                    error_summary: Some(errors.join("; ").into()),
                    importance: request.importance,
                }
            }
        }
    }
}
