//! [`backend::SqliteStorage`] holds one shared `rusqlite` connection behind a
//! mutex. The trait is async ([`async_trait`]); the SQLite bodies stay
//! synchronous internally.
mod backend;
mod runtime;

#[cfg(test)]
pub(crate) use backend::SqliteStorage;
pub use runtime::*;

use async_trait::async_trait;
use gpui::Global;

#[cfg(test)]
#[path = "../storage.test/mod.rs"]
mod storage_test;

mod storage_backend;
mod storage_error;
mod storage_snapshot;

pub use storage_backend::StorageBackend;
pub use storage_error::StorageError;
pub use storage_snapshot::StorageSnapshot;
