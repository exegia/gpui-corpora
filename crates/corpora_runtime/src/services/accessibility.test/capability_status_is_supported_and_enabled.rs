#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn capability_status_is_supported_and_enabled() {
    let status = capability_status();
    assert!(status.supported);
    assert!(status.enabled);
    assert!(!status.degraded);
    assert!(status.reason.is_some());
}
