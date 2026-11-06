#[allow(unused_imports)]
use super::*;

pub fn snapshot(cx: &App) -> CrashReportSnapshot {
    cx.try_global::<CrashReportSnapshot>()
        .cloned()
        .unwrap_or_default()
}
