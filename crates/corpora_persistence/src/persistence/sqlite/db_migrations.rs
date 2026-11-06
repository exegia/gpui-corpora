use anyhow::Result;
use rusqlite::Connection;

#[cfg(test)]
#[path = "db_migrations.test/mod.rs"]
mod db_migrations_test;

mod migration;
mod migrations;
mod run_migrations;

use migration::Migration;
use migrations::MIGRATIONS;
pub use run_migrations::run_migrations;
