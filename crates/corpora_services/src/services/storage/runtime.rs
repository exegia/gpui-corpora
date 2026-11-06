use std::sync::Arc;

use gpui::{App, BorrowAppContext as _, Global};

use super::{StorageBackend, StorageError, StorageSnapshot};

use std::path::{Path, PathBuf};

mod capability_status;
mod db_path;
mod init_db;
mod initialize;
mod record_health_result;
mod run_health_check;
mod run_maintenance;
mod set_capabilities;
mod shutdown;
mod snapshot;
mod storage_runtime;

pub(super) use capability_status::capability_status;
use db_path::db_path;
pub(crate) use init_db::init_db;
pub use initialize::initialize;
pub(super) use record_health_result::record_health_result;
pub use run_health_check::run_health_check;
pub use run_maintenance::run_maintenance;
use set_capabilities::set_capabilities;
pub use shutdown::shutdown;
pub use snapshot::snapshot;
pub use storage_runtime::StorageRuntime;
