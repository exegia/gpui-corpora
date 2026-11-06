#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn persist_and_load_crash_report_roundtrip() {
    let dir = tempdir().expect("tempdir");
    let db_path = dir.path().join("test_crash.db");
    init_db(&db_path).expect("init db");
    let backend = SqliteStorage::new(db_path);

    let report = crate::services::crash_report::CrashReport::new(
        "test panic".to_string(),
        "backtrace here".to_string(),
        true,
        vec!["error1".to_string(), "error2".to_string()],
    );

    block_on(backend.persist_crash_report(&report)).expect("persist crash report");

    let loaded = block_on(backend.load_pending_crash_reports(10)).expect("load pending");
    assert_eq!(loaded.len(), 1);
    assert_eq!(loaded[0].id, report.id);
    assert_eq!(loaded[0].panic_message, "test panic");
    assert_eq!(loaded[0].backtrace, "backtrace here");
    assert!(loaded[0].render_path);
    assert_eq!(loaded[0].recent_errors, vec!["error1", "error2"]);
}
