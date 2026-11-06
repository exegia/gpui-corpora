#[allow(unused_imports)]
use super::*;

/// `initialize` registers exactly `capability_status()`, so this pins what
/// that registration may claim.
#[test]
pub(crate) fn capability_registration_is_supported_and_enabled() {
    let status = capability_status();

    assert!(status.supported);
    assert!(status.enabled);
    assert!(!status.degraded, "init-time registration is never degraded");
    assert!(status.last_error.is_none());
    assert!(status.reason.is_some_and(|reason| !reason.is_empty()));
}
