#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn persist_and_load_roundtrip() {
    use crate::storage::{SqliteStorage, StorageBackend as _};

    let dir = tempfile::tempdir().expect("tempdir");
    let db_path = dir.path().join("test_errors.db");

    // Create the error_log table inline (mirrors migration v2).
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

    let record = ErrorRecord {
        id: EventId::new(),
        occurred_at: AppTimestamp::now(),
        severity: crate::errors::AppErrorSeverity::Error,
        category: ErrorCategory::Storage,
        message: "disk full".into(),
        actions: vec![ErrorAction::Dismiss],
    };

    // persist_error/load_error_history are async; the SQLite body resolves
    // on first poll.
    let runtime = || tokio::runtime::Runtime::new().expect("tokio runtime for test");
    runtime()
        .block_on(persist_error(&backend, &record))
        .expect("persist");
    let loaded = runtime()
        .block_on(backend.load_error_history(10))
        .expect("load");
    assert_eq!(loaded.len(), 1);
    assert_eq!(loaded[0].message, "disk full");
    assert_eq!(loaded[0].category, ErrorCategory::Storage);
    assert_eq!(loaded[0].severity, crate::errors::AppErrorSeverity::Error);
}
