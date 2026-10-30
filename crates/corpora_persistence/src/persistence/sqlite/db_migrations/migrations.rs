#[allow(unused_imports)]
use super::*;

pub(crate) const MIGRATIONS: &[Migration] = &[
    Migration {
        version: 1,
        name: "kv_store",
        up_sql: r#"
            CREATE TABLE IF NOT EXISTS kv_store (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL,
                updated_at TEXT NOT NULL
            );
        "#,
    },
    Migration {
        version: 2,
        name: "error_log",
        up_sql: r#"
            CREATE TABLE IF NOT EXISTS error_log (
                id TEXT PRIMARY KEY,
                occurred_at TEXT NOT NULL,
                severity TEXT NOT NULL,
                category TEXT NOT NULL,
                message TEXT NOT NULL,
                actions TEXT NOT NULL
            );
            CREATE INDEX IF NOT EXISTS idx_error_log_occurred_at
                ON error_log (occurred_at DESC);
        "#,
    },
    Migration {
        version: 3,
        name: "crash_reports",
        up_sql: r#"
            CREATE TABLE IF NOT EXISTS crash_reports (
                id TEXT PRIMARY KEY,
                panic_message TEXT NOT NULL,
                backtrace TEXT NOT NULL,
                app_version TEXT NOT NULL,
                os TEXT NOT NULL,
                arch TEXT NOT NULL,
                timestamp TEXT NOT NULL,
                render_path BOOLEAN NOT NULL DEFAULT 0,
                recent_errors TEXT NOT NULL DEFAULT "[]",
                uploaded BOOLEAN NOT NULL DEFAULT 0,
                uploaded_at TEXT,
                upload_error TEXT
            );
            CREATE INDEX IF NOT EXISTS idx_crash_reports_timestamp
                ON crash_reports (timestamp DESC);
            CREATE INDEX IF NOT EXISTS idx_crash_reports_uploaded
                ON crash_reports (uploaded);
        "#,
    },
];
