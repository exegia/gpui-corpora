#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn default_snapshot_without_windows() {
    let snap = AccessibilitySnapshot::default();

    // No assistive tech can be active with zero windows, so never enabled here.
    assert!(!snap.bridge_enabled);
    assert_eq!(snap.active_windows, 0);
    assert_eq!(snap.total_windows, 0);
    assert!(!snap.status.is_empty());
}
