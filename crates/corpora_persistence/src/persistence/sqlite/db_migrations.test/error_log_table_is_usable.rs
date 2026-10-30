#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn error_log_table_is_usable() {
    let dir = tempdir().expect("tempdir");
    let db_path = dir.path().join("test.db");
    let conn = Connection::open(&db_path).expect("open db");

    run_migrations(&conn).expect("run migrations");

    conn.execute(
        "INSERT INTO error_log (id, occurred_at, severity, category, message, actions)
         VALUES ('test-id', '2025-01-01T00:00:00Z', 'Error', 'Network', 'timeout', \"[]\")",
        [],
    )
    .expect("insert");

    let message: String = conn
        .query_row(
            "SELECT message FROM error_log WHERE id = 'test-id'",
            [],
            |row| row.get(0),
        )
        .expect("select");
    assert_eq!(message, "timeout");
}
