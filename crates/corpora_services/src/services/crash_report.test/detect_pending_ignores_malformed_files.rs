#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn detect_pending_ignores_malformed_files() {
    let dir = tempdir().expect("tempdir");
    let reports_dir = dir.path().join("crash_reports");
    std::fs::create_dir_all(&reports_dir).expect("mkdir");

    let report = CrashReport::new("good".into(), "bt".into(), false, vec![]);
    write_crash_report(&report, dir.path()).expect("write");

    let bad_path = reports_dir.join("bad.json");
    std::fs::write(&bad_path, "not json").expect("write bad");

    let detected = detect_pending_reports(dir.path());
    assert_eq!(detected.len(), 1);
    assert_eq!(detected[0].panic_message, "good");
}
