#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn observe_without_active_windows_reports_ready() {
    let snap = observe(0, 1);
    assert!(!snap.bridge_enabled);
    assert!(snap.status.contains("ready"));
}
