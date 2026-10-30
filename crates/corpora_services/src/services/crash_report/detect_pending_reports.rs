#[allow(unused_imports)]
use super::*;

/// Non-JSON files and malformed entries are silently skipped.
pub fn detect_pending_reports(data_dir: &Path) -> Vec<CrashReport> {
    let reports_dir = data_dir.join("crash_reports");
    if !reports_dir.exists() {
        return Vec::new();
    }

    let Ok(entries) = std::fs::read_dir(&reports_dir) else {
        return Vec::new();
    };

    let mut reports = Vec::new();
    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("json") {
            continue;
        }
        let Ok(contents) = std::fs::read_to_string(&path) else {
            continue;
        };
        match serde_json::from_str::<CrashReport>(&contents) {
            Ok(report) => reports.push(report),
            Err(err) => {
                tracing::warn!(
                    target: "corpora::crash_report",
                    path = %path.display(),
                    error = %err,
                    "skipping malformed crash report file"
                );
            }
        }
    }

    reports.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
    reports
}
