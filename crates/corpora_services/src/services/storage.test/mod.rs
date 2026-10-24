use tempfile::tempdir;

use super::runtime::{capability_status, record_health_result};
use super::{SqliteStorage, StorageBackend, StorageError, StorageSnapshot, init_db};

// === split-modules: generated submodules ===
mod backend_health_and_maintenance_work;
mod block_on;
mod initializes_schema_and_migration_table;
mod load_pending_crash_reports_respects_limit;
mod mark_crash_report_uploaded;
mod native_boot_capability_not_degraded_after_health_check_lands;
mod persist_and_load_crash_report_roundtrip;

use block_on::block_on;
