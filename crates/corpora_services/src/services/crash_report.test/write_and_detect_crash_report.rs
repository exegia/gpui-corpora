#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn write_and_detect_crash_report() {
    let dir = tempdir().expect("tempdir");
    let report = CrashReport::new("test panic".into(), "fake backtrace".into(), false, vec![]);

    write_crash_report(&report, dir.path()).expect("write");

    let detected = detect_pending_reports(dir.path());
    assert_eq!(detected.len(), 1);
    assert_eq!(detected[0].id, report.id);
    assert_eq!(detected[0].panic_message, "test panic");
}
