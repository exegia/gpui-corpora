#[allow(unused_imports)]
use super::*;

/// Application data directory, set once during startup so the panic hook can
/// write crash report files without needing GPUI context.
pub(crate) static APP_DATA_DIR: OnceLock<PathBuf> = OnceLock::new();
