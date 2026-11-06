#[allow(unused_imports)]
use super::*;

/// Install the toast identity for `app_id`: explicit process AUMID plus a
/// Start Menu shortcut pointing at the running exe. `Err(reason)` never
/// blocks notification sending — callers log and continue.
pub fn ensure_toast_identity(app_id: &str) -> Result<(), String> {
    // Best-effort: the process AUMID affects taskbar grouping; the shortcut
    // below is what makes toasts display.
    unsafe {
        if let Err(err) = SetCurrentProcessExplicitAppUserModelID(&HSTRING::from(app_id)) {
            tracing::warn!(target: LOG, error = %err, "explicit process AUMID rejected");
        }
    }

    let programs = programs_dir()?;
    let exe = std::env::current_exe().map_err(|err| format!("current_exe: {err}"))?;
    ensure_shortcut_at(&programs, app_id, &exe)
}
