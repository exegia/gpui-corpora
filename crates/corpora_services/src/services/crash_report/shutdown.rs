#[allow(unused_imports)]
use super::*;

pub fn shutdown(cx: &mut App) {
    tracing::debug!(
        target: "corpora::crash_report",
        "crash report service shutdown requested"
    );
    upload_pending_reports(cx);
}
