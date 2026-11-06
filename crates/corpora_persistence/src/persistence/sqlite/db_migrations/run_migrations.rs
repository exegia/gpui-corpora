#[allow(unused_imports)]
use super::*;

pub fn run_migrations(conn: &Connection) -> Result<u32> {
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS schema_migrations (
            version INTEGER PRIMARY KEY,
            applied_at TEXT NOT NULL
        );",
    )?;

    let current_version: u32 = conn
        .query_row(
            "SELECT COALESCE(MAX(version), 0) FROM schema_migrations",
            [],
            |row| row.get(0),
        )
        .unwrap_or(0);

    tracing::info!(
        target: "corpora::db_migrations",
        current_version,
        pending = MIGRATIONS.len().saturating_sub(current_version as usize),
        "checking migrations"
    );

    let mut applied = current_version;

    for migration in MIGRATIONS {
        if migration.version <= current_version {
            continue;
        }

        tracing::info!(
            target: "corpora::db_migrations",
            version = migration.version,
            name = migration.name,
            "applying migration"
        );

        let tx = conn.unchecked_transaction()?;

        tx.execute_batch(migration.up_sql)?;

        tx.execute(
            "INSERT INTO schema_migrations (version, applied_at) VALUES (?1, datetime('now'))",
            [migration.version],
        )?;

        tx.commit()?;

        tracing::info!(
            target: "corpora::db_migrations",
            version = migration.version,
            name = migration.name,
            "migration applied"
        );

        applied = migration.version;
    }

    tracing::info!(
        target: "corpora::db_migrations",
        schema_version = applied,
        "migrations complete"
    );

    Ok(applied)
}
