#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn creates_schema_table_from_empty_db() {
    let dir = tempdir().expect("tempdir");
    let db_path = dir.path().join("test.db");
    let conn = Connection::open(&db_path).expect("open db");

    let version = run_migrations(&conn).expect("run migrations");
    assert_eq!(version, 3);

    let recorded: u32 = conn
        .query_row(
            "SELECT COALESCE(MAX(version), 0) FROM schema_migrations",
            [],
            |row| row.get(0),
        )
        .expect("read version");
    assert_eq!(recorded, 3);
}
