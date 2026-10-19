#[allow(unused_imports)]
use super::*;

#[derive(Clone, Debug, Default)]
pub struct StorageSnapshot {
    pub available: bool,
    pub db_path: String,
    pub schema_version: i64,
    pub healthy: bool,
    pub last_maintenance_at: Option<String>,
    pub last_migration_result: Option<String>,
    pub last_error: Option<String>,
}

impl Global for StorageSnapshot {}
