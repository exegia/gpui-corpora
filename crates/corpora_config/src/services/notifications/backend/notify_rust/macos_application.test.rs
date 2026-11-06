use super::*;
use crate::notifications::NotificationRequest;
use crate::services::notifications::backend::{NotificationBackend, NotifyRustBackend};

#[test]
fn unbundled_notification_returns_error_before_default_application_lookup() {
    assert!(NSBundle::mainBundle().bundleIdentifier().is_none());
    let runtime = tokio::runtime::Runtime::new().unwrap();
    let error = runtime
        .block_on(NotifyRustBackend::new().send(&NotificationRequest::foreground("test", "test")))
        .expect_err("an unbundled process must not invoke native notification delivery");
    assert!(error.to_string().contains("requires an app bundle"));
}

#[test]
fn macos_fallback_advertises_packaged_runtime_requirement() {
    assert!(
        NotifyRustBackend::new()
            .capabilities()
            .requires_packaged_runtime
    );
}
