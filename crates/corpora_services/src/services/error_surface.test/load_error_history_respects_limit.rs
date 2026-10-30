#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn load_error_history_respects_limit() {
    use crate::storage::{SqliteStorage, StorageBackend as _};

    let dir = tempfile::tempdir().expect("tempdir");
    let db_path = dir.path().join("test_limit.db");

    {
        let conn = rusqlite::Connection::open(&db_path).expect("open");
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS error_log (
                id TEXT PRIMARY KEY,
                occurred_at TEXT NOT NULL,
                severity TEXT NOT NULL,
                category TEXT NOT NULL,
                message TEXT NOT NULL,
                actions TEXT NOT NULL
            );",
        )
        .expect("create table");
    }

    let backend = SqliteStorage::new_for_test(db_path);
    let runtime = tokio::runtime::Runtime::new().expect("tokio runtime for test");

    for i in 0..5 {
        let record = ErrorRecord {
            id: EventId::new(),
            occurred_at: AppTimestamp::now(),
            severity: crate::errors::AppErrorSeverity::Info,
            category: ErrorCategory::System,
            message: format!("error {i}"),
            actions: vec![],
        };
        runtime
            .block_on(persist_error(&backend, &record))
            .expect("persist");
    }

    let loaded = runtime
        .block_on(backend.load_error_history(3))
        .expect("load");
    assert_eq!(loaded.len(), 3);
}
