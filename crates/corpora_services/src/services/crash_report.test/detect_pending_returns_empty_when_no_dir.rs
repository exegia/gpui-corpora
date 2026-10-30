#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn detect_pending_returns_empty_when_no_dir() {
    let dir = tempdir().expect("tempdir");
    let detected = detect_pending_reports(dir.path());
    assert!(detected.is_empty());
}
