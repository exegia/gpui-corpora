#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn detect_pending_sorts_newest_first() {
    let dir = tempdir().expect("tempdir");

    // Write two reports with a small delay so timestamps differ.
    let r1 = CrashReport::new("first".into(), "bt".into(), false, vec![]);
    write_crash_report(&r1, dir.path()).expect("write");
    std::thread::sleep(std::time::Duration::from_millis(10));
    let r2 = CrashReport::new("second".into(), "bt".into(), false, vec![]);
    write_crash_report(&r2, dir.path()).expect("write");

    let detected = detect_pending_reports(dir.path());
    assert_eq!(detected.len(), 2);
    assert_eq!(detected[0].panic_message, "second");
    assert_eq!(detected[1].panic_message, "first");
}
