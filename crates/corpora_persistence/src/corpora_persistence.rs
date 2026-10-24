//! SQLite schema migrations.

pub mod persistence;

pub use persistence::sqlite::db_migrations;
