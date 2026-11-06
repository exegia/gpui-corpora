#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn kv_store_table_is_usable() {
    let dir = tempdir().expect("tempdir");
    let db_path = dir.path().join("test.db");
    let conn = Connection::open(&db_path).expect("open db");

    run_migrations(&conn).expect("run migrations");

    conn.execute(
        "INSERT INTO kv_store (key, value, updated_at) VALUES ('k', 'v', datetime('now'))",
        [],
    )
    .expect("insert");

    let value: String = conn
        .query_row("SELECT value FROM kv_store WHERE key = 'k'", [], |row| {
            row.get(0)
        })
        .expect("select");
    assert_eq!(value, "v");
}
