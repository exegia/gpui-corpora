#[allow(unused_imports)]
use super::*;

/// Ring buffer of recent errors attached to the next crash report.
pub(crate) static RECENT_ERRORS: OnceLock<Mutex<Vec<String>>> = OnceLock::new();
