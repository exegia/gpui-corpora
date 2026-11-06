#[allow(unused_imports)]
use super::*;

pub fn initialize(cx: &mut App) {
    let endpoint = option_env!("GPUI_CRASH_REPORT_URL")
        .unwrap_or("")
        .to_string();

    let snapshot = CrashReportSnapshot {
        upload_endpoint: endpoint,
        ..CrashReportSnapshot::default()
    };

    cx.set_global(snapshot.clone());

    tracing::info!(
        target: "corpora::crash_report",
        endpoint = %snapshot.upload_endpoint,
        "crash report service initialized"
    );
}
