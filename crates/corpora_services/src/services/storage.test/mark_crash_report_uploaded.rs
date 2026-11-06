#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn mark_crash_report_uploaded() {
    let dir = tempdir().expect("tempdir");
    let db_path = dir.path().join("test_upload.db");
    init_db(&db_path).expect("init db");
    let backend = SqliteStorage::new(db_path);

    let report = crate::services::crash_report::CrashReport::new(
        "panic".to_string(),
        "bt".to_string(),
        false,
        vec![],
    );

    block_on(backend.persist_crash_report(&report)).expect("persist");
    let pending = block_on(backend.load_pending_crash_reports(10)).expect("load");
    assert_eq!(pending.len(), 1);

    block_on(backend.mark_crash_report_uploaded(&report.id, "2025-01-01T00:00:00Z"))
        .expect("mark uploaded");

    let pending_after = block_on(backend.load_pending_crash_reports(10)).expect("load after");
    assert_eq!(pending_after.len(), 0);
}
