#[allow(unused_imports)]
use super::*;

#[derive(Clone, Debug, Default)]
pub struct CrashReportSnapshot {
    pub pending_count: usize,
    pub last_crash_timestamp: Option<String>,
    pub upload_endpoint: String,
    pub last_upload_error: Option<String>,
}

impl Global for CrashReportSnapshot {}
