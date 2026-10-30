#[allow(unused_imports)]
use super::*;

/// Intentionally synchronous `std::fs`: the panic hook has no async runtime.
pub fn write_crash_report(report: &CrashReport, data_dir: &Path) -> std::io::Result<()> {
    let reports_dir = data_dir.join("crash_reports");
    std::fs::create_dir_all(&reports_dir)?;

    let file_path = reports_dir.join(format!("{}.json", report.id));
    let json = serde_json::to_string_pretty(report)?;
    std::fs::write(&file_path, json)?;

    tracing::info!(
        target: "corpora::crash_report",
        path = %file_path.display(),
        "crash report written to disk"
    );

    Ok(())
}
