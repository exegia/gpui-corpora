use std::panic::AssertUnwindSafe;
use std::sync::Arc;

use futures_util::FutureExt;
use gpui::{Global, SharedString};

#[cfg(any(target_os = "macos", target_os = "windows"))]
use super::UserNotifyBackend;
use super::types::{
    NotificationBackendKind, NotificationCapabilities, NotificationImportance,
    NotificationPermissionState, NotificationRequest, NotificationSendResult,
};
use super::{NotificationBackend, NotifyRustBackend};

#[cfg(test)]
mod tests {
    use super::NotificationService;
    use crate::services::notifications::backend::NotificationBackend;
    #[cfg(target_os = "linux")]
    use crate::services::notifications::backend::NotifyRustBackend;
    use crate::services::notifications::service::types::{
        NotificationBackendKind, NotificationCapabilities, NotificationPermissionState,
        NotificationRequest,
    };

    use std::sync::Arc;

    use async_trait::async_trait;

    enum SendOutcome {
        Ok,
        Err(&'static str),
        Panic(&'static str),
    }

    struct FakeBackend {
        kind: NotificationBackendKind,
        outcome: SendOutcome,
    }

    #[async_trait]
    impl NotificationBackend for FakeBackend {
        fn kind(&self) -> NotificationBackendKind {
            self.kind
        }
        fn capabilities(&self) -> NotificationCapabilities {
            NotificationCapabilities::default()
        }
        async fn refresh_permission_state(&self) -> NotificationPermissionState {
            NotificationPermissionState::Unsupported
        }
        async fn request_permission(&self) -> NotificationPermissionState {
            NotificationPermissionState::Unsupported
        }
        async fn send(&self, _request: &NotificationRequest) -> anyhow::Result<()> {
            match self.outcome {
                SendOutcome::Ok => Ok(()),
                SendOutcome::Err(msg) => Err(anyhow::anyhow!("{msg}")),
                SendOutcome::Panic(msg) => panic!("{msg}"),
            }
        }
    }

    fn fake(kind: NotificationBackendKind, outcome: SendOutcome) -> Arc<dyn NotificationBackend> {
        Arc::new(FakeBackend { kind, outcome })
    }

    fn request() -> NotificationRequest {
        NotificationRequest::foreground("test title", "test body")
    }

    fn runtime() -> tokio::runtime::Runtime {
        tokio::runtime::Runtime::new().expect("failed to build tokio runtime for test")
    }

    #[test]
    fn fallback_used_when_primary_returns_err() {
        let svc = NotificationService::with_backends(
            Some(fake(
                NotificationBackendKind::UserNotify,
                SendOutcome::Err("primary boom"),
            )),
            fake(NotificationBackendKind::NotifyRust, SendOutcome::Ok),
            None,
        );
        let result = runtime().block_on(async { svc.send(request(), true).await });
        assert!(result.delivered_natively, "secondary should deliver");
        assert_eq!(result.backend_used, NotificationBackendKind::NotifyRust);
        assert!(
            result.degraded,
            "a primary existed, so the result is degraded"
        );
        let summary = result
            .error_summary
            .expect("should carry the primary error");
        assert!(
            summary.contains("primary boom"),
            "error summary should include the primary failure: {summary}"
        );
    }

    #[test]
    fn degrades_to_ui_only_when_both_backends_fail() {
        let svc = NotificationService::with_backends(
            Some(fake(
                NotificationBackendKind::UserNotify,
                SendOutcome::Err("primary-fail"),
            )),
            fake(
                NotificationBackendKind::NotifyRust,
                SendOutcome::Err("secondary-fail"),
            ),
            None,
        );
        let result = runtime().block_on(async { svc.send(request(), true).await });
        assert!(!result.delivered_natively);
        assert_eq!(result.backend_used, NotificationBackendKind::UiOnly);
        assert!(result.degraded);
        let summary = result
            .error_summary
            .expect("both backend errors are recorded");
        assert!(
            summary.contains("primary-fail") && summary.contains("secondary-fail"),
            "error summary should include both failures: {summary}"
        );
    }

    #[test]
    fn primary_panic_does_not_bypass_fallback() {
        let svc = NotificationService::with_backends(
            Some(fake(
                NotificationBackendKind::UserNotify,
                SendOutcome::Panic("primary panicked"),
            )),
            fake(NotificationBackendKind::NotifyRust, SendOutcome::Ok),
            None,
        );
        let result = runtime().block_on(async { svc.send(request(), true).await });
        assert!(
            result.delivered_natively,
            "secondary must deliver despite the primary panic"
        );
        assert_eq!(result.backend_used, NotificationBackendKind::NotifyRust);
        let summary = result
            .error_summary
            .expect("panic must be recorded as an error");
        assert!(
            summary.to_lowercase().contains("panic"),
            "error summary should mention the panic: {summary}"
        );
    }

    #[test]
    fn native_send_skipped_when_disabled_by_user() {
        let svc = NotificationService::with_backends(
            Some(fake(NotificationBackendKind::UserNotify, SendOutcome::Ok)),
            fake(NotificationBackendKind::NotifyRust, SendOutcome::Ok),
            None,
        );
        let result = runtime().block_on(async { svc.send(request(), false).await });
        assert!(!result.delivered_natively);
        assert_eq!(result.backend_used, NotificationBackendKind::UiOnly);
    }

    /// Linux skips the user-notify backend (nothing over notify-rust), so
    /// `new()` must leave `primary = None` and report notify-rust as active.
    #[cfg(target_os = "linux")]
    #[test]
    fn linux_selects_notify_rust_as_active_backend() {
        let svc = NotificationService::new();
        assert!(svc.primary.is_none(), "primary must be None on Linux");
        assert_eq!(svc.active_backend(), NotificationBackendKind::NotifyRust);
    }

    /// Linux/FreeDesktop has no per-app notification permission model, so the
    /// notify-rust backend must report `Unsupported` (never `Denied`/`Authorized`).
    #[cfg(target_os = "linux")]
    #[test]
    fn linux_notify_rust_reports_unsupported_permission() {
        let backend = NotifyRustBackend::new();
        let state = runtime().block_on(async { backend.refresh_permission_state().await });
        assert_eq!(state, NotificationPermissionState::Unsupported);
    }

    #[cfg(all(feature = "notifications-portal", target_os = "linux"))]
    #[test]
    fn portal_backend_reports_kind_and_capabilities() {
        use crate::services::notifications::backend::PortalBackend;
        let backend = PortalBackend::new();
        assert_eq!(backend.kind(), NotificationBackendKind::Portal);
        let caps = backend.capabilities();
        assert!(caps.can_send_immediate_native);
        assert!(caps.requires_packaged_runtime);
    }
}

mod log;
mod native_notification_state;
mod notification_runtime_snapshot;
mod notification_service;
mod panic_payload_to_string;
mod select_primary_backend;

pub use log::LOG;
pub use native_notification_state::NativeNotificationState;
pub use notification_runtime_snapshot::NotificationRuntimeSnapshot;
pub use notification_service::NotificationService;
use panic_payload_to_string::panic_payload_to_string;
use select_primary_backend::select_primary_backend;
