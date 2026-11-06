use std::path::Path;

use gpui::App;
use gpui::BorrowAppContext as _;
use gpui::Global;
use serde::{Deserialize, Serialize};

#[cfg(test)]
#[path = "crash_report.test/mod.rs"]
mod crash_report_test;

mod crash_report_snapshot;
mod detect_pending_reports;
mod initialize;
mod max_backtrace_chars;
mod max_panic_message_chars;
mod max_recent_error_chars;
mod max_recent_errors;
mod report;
mod scrub;
mod shutdown;
mod snapshot;
mod upload_pending_reports;
mod write_crash_report;

pub use crash_report_snapshot::CrashReportSnapshot;
pub use detect_pending_reports::detect_pending_reports;
pub use initialize::initialize;
use max_backtrace_chars::MAX_BACKTRACE_CHARS;
use max_panic_message_chars::MAX_PANIC_MESSAGE_CHARS;
use max_recent_error_chars::MAX_RECENT_ERROR_CHARS;
use max_recent_errors::MAX_RECENT_ERRORS;
pub use report::CrashReport;
use scrub::scrub;
pub use shutdown::shutdown;
pub use snapshot::snapshot;
pub use upload_pending_reports::upload_pending_reports;
pub use write_crash_report::write_crash_report;
