#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn observe_reports_active_windows() {
    let snap = observe(2, 3);
    assert!(snap.bridge_enabled);
    assert_eq!(snap.active_windows, 2);
    assert_eq!(snap.total_windows, 3);
    assert!(snap.status.contains("2 of 3"));
}
