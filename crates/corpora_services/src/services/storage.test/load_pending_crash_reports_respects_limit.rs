#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn load_pending_crash_reports_respects_limit() {
    let dir = tempdir().expect("tempdir");
    let db_path = dir.path().join("test_limit.db");
    init_db(&db_path).expect("init db");
    let backend = SqliteStorage::new(db_path);

    for _ in 0..5 {
        let report = crate::services::crash_report::CrashReport::new(
            "panic".to_string(),
            "bt".to_string(),
            false,
            vec![],
        );
        block_on(backend.persist_crash_report(&report)).expect("persist");
    }

    let loaded = block_on(backend.load_pending_crash_reports(3)).expect("load");
    assert_eq!(loaded.len(), 3);
}
