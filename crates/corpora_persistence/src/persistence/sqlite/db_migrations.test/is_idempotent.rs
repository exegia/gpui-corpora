#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn is_idempotent() {
    let dir = tempdir().expect("tempdir");
    let db_path = dir.path().join("test.db");

    let conn = Connection::open(&db_path).expect("open db");
    let first = run_migrations(&conn).expect("first run");
    let second = run_migrations(&conn).expect("second run");
    assert_eq!(first, second);

    let count: i64 = conn
        .query_row("SELECT COUNT(*) FROM schema_migrations", [], |row| {
            row.get(0)
        })
        .expect("count rows");
    assert_eq!(count, 3);
}
