#[allow(unused_imports)]
use super::*;

/// Open (or create) the database and apply migrations from `persistence` —
/// the single source of truth for the schema.
pub(crate) fn init_db(path: &Path) -> anyhow::Result<i64> {
    let conn = rusqlite::Connection::open(path)?;
    conn.execute_batch("PRAGMA journal_mode = WAL;")?;
    Ok(crate::db_migrations::run_migrations(&conn)? as i64)
}
