#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn crash_reports_table_is_usable() {
    let dir = tempdir().expect("tempdir");
    let db_path = dir.path().join("test.db");
    let conn = Connection::open(&db_path).expect("open db");

    run_migrations(&conn).expect("run migrations");

    conn.execute(
        "INSERT INTO crash_reports (id, panic_message, backtrace, app_version, os, arch, timestamp, render_path, recent_errors)
         VALUES ('crash-1', 'index out of bounds', 'frame1\nframe2', '0.2.0', 'macos', 'aarch64', '2025-06-01T12:00:00Z', 1, '[\"err1\"]')",
        [],
    )
    .expect("insert crash report");

    let msg: String = conn
        .query_row(
            "SELECT panic_message FROM crash_reports WHERE id = 'crash-1'",
            [],
            |row| row.get(0),
        )
        .expect("select");
    assert_eq!(msg, "index out of bounds");

    let render_path: bool = conn
        .query_row(
            "SELECT render_path FROM crash_reports WHERE id = 'crash-1'",
            [],
            |row| row.get(0),
        )
        .expect("select render_path");
    assert!(render_path);

    let uploaded: bool = conn
        .query_row(
            "SELECT uploaded FROM crash_reports WHERE id = 'crash-1'",
            [],
            |row| row.get(0),
        )
        .expect("select uploaded");
    assert!(!uploaded);
}
