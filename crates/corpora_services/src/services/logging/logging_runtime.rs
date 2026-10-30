#[allow(unused_imports)]
use super::*;

#[derive(Clone, Debug)]
pub struct LoggingRuntime {
    pub enabled: bool,
    pub log_dir: String,
    pub file_prefix: String,
    pub has_guard: bool,
    pub last_error: Option<String>,
}
