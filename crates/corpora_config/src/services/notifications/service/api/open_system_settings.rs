#[allow(unused_imports)]
use super::*;

pub fn open_system_settings(cx: &mut App) {
    #[cfg(target_os = "macos")]
    {
        tracing::info!(target: LOG, "opening macOS notification settings");
        if let Err(err) = std::process::Command::new("open")
            .arg("x-apple.systempreferences:com.apple.Notifications-Settings.extension")
            .spawn()
        {
            tracing::warn!(target: LOG, error = %err, "failed to open macOS notification settings");
            mutate_snapshot(cx, |snapshot| {
                snapshot.last_backend_error =
                    Some(format!("failed to open settings: {err}").into());
            });
        }
    }

    #[cfg(target_os = "linux")]
    {
        // Best-effort: a missing binary just logs.
        tracing::info!(target: LOG, "opening Linux notification settings");
        if std::process::Command::new("gnome-control-center")
            .arg("notifications")
            .spawn()
            .is_ok()
        {
            return;
        }
        tracing::debug!(target: LOG, "gnome-control-center unavailable; trying xdg-open");
        if let Err(err) = std::process::Command::new("xdg-open")
            .arg("settings://notifications")
            .spawn()
        {
            tracing::warn!(target: LOG, error = %err, "failed to open Linux notification settings");
            mutate_snapshot(cx, |snapshot| {
                snapshot.last_backend_error =
                    Some("could not open notification settings; open them manually".into());
            });
        }
    }

    #[cfg(target_os = "windows")]
    {
        tracing::info!(target: LOG, "opening Windows notification settings");
        if let Err(err) = open::that_detached("ms-settings:notifications") {
            tracing::warn!(target: LOG, error = %err, "failed to open Windows notification settings");
            mutate_snapshot(cx, |snapshot| {
                snapshot.last_backend_error =
                    Some("could not open notification settings; open them manually".into());
            });
        }
    }

    #[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
    {
        tracing::warn!(target: LOG, "system notification settings unsupported on this platform");
        mutate_snapshot(cx, |snapshot| {
            snapshot.last_backend_error =
                Some("system notification settings are not supported on this platform".into());
        });
    }
}
