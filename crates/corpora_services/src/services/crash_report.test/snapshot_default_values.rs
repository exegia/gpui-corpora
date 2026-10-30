#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn snapshot_default_values() {
    let snap = CrashReportSnapshot::default();
    assert_eq!(snap.pending_count, 0);
    assert!(snap.last_crash_timestamp.is_none());
    assert!(snap.upload_endpoint.is_empty());
    assert!(snap.last_upload_error.is_none());
}
