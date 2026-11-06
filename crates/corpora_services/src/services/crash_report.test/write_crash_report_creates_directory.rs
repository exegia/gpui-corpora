#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn write_crash_report_creates_directory() {
    let dir = tempdir().expect("tempdir");
    let nested = dir.path().join("does_not_exist");
    let report = CrashReport::new("msg".into(), "bt".into(), false, vec![]);
    write_crash_report(&report, &nested).expect("write should create dir");
    assert!(nested.join("crash_reports").exists());
}
