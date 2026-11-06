#[allow(unused_imports)]
use super::*;

pub fn check_pending_swap(cx: &mut App) {
    let Some(marker_path) = pending_swap_path() else {
        return;
    };
    if !marker_path.exists() {
        return;
    }

    tracing::info!(
        target: "corpora::updater",
        path = %marker_path.display(),
        "pending swap marker found, attempting binary swap"
    );

    let data = match std::fs::read_to_string(&marker_path) {
        Ok(d) => d,
        Err(err) => {
            tracing::error!(
                target: "corpora::updater",
                error = %err,
                "failed to read pending swap marker"
            );
            return;
        }
    };

    let pending: serde_json::Value = match serde_json::from_str(&data) {
        Ok(v) => v,
        Err(err) => {
            tracing::error!(
                target: "corpora::updater",
                error = %err,
                "failed to parse pending swap marker"
            );
            // Corrupt marker: drop it so we don't retry indefinitely.
            let _ = std::fs::remove_file(&marker_path);
            return;
        }
    };

    let source_path = match pending.get("source_path").and_then(|v| v.as_str()) {
        Some(p) => PathBuf::from(p),
        None => {
            tracing::error!(
                target: "corpora::updater",
                "pending swap marker missing source_path"
            );
            let _ = std::fs::remove_file(&marker_path);
            return;
        }
    };

    let version = pending
        .get("version")
        .and_then(|v| v.as_str())
        .unwrap_or("unknown");

    // Fail closed: the file is re-verified against the manifest signature at
    // swap time, so a tampered temp file can never replace the executable.
    // Markers without a signature (pre-dating this check) are refused.
    let signature = pending
        .get("signature")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    if signature.is_empty() {
        tracing::error!(
            target: "corpora::updater",
            "pending swap marker has no signature — refusing unverified swap"
        );
        let _ = std::fs::remove_file(&marker_path);
        super::super::set_status(
            UpdateStatus::Error("pending swap refused: no signature in marker".to_string()),
            cx,
        );
        return;
    }
    if let Err(err) = super::super::download::verify_ed25519_signature(&source_path, signature) {
        tracing::error!(
            target: "corpora::updater",
            path = %source_path.display(),
            error = %err,
            "pending swap source failed signature re-verification — removing marker"
        );
        let _ = std::fs::remove_file(&marker_path);
        super::super::set_status(
            UpdateStatus::Error(format!("pending swap refused: {err}")),
            cx,
        );
        return;
    }

    let current_exe = match std::env::current_exe() {
        Ok(e) => e,
        Err(err) => {
            tracing::error!(
                target: "corpora::updater",
                error = %err,
                "failed to determine current executable path"
            );
            return;
        }
    };

    let swap_result: Result<(), String> = (|| {
        #[cfg(target_os = "macos")]
        {
            // Inside a .app bundle: swap the whole bundle when the source is
            // one, else replace the binary in place.
            if let Some(bundle_path) = current_exe
                .ancestors()
                .find(|a| a.extension().is_some_and(|ext| ext == "app"))
            {
                let source_bundle = if source_path.extension().is_some_and(|ext| ext == "app") {
                    source_path.clone()
                } else if source_path.is_dir() {
                    source_path.clone()
                } else {
                    let dest_binary = current_exe.clone();
                    std::process::Command::new("mv")
                        .arg("-f")
                        .arg(&source_path)
                        .arg(&dest_binary)
                        .status()
                        .map_err(|e| format!("failed to mv binary: {e}"))?;
                    return Ok(());
                };

                tracing::info!(
                    target: "corpora::updater",
                    source = %source_bundle.display(),
                    dest = %bundle_path.display(),
                    "swapping .app bundle"
                );
                if bundle_path.exists() {
                    std::fs::remove_dir_all(bundle_path)
                        .map_err(|e| format!("failed to remove old bundle: {e}"))?;
                }
                std::process::Command::new("mv")
                    .arg(&source_bundle)
                    .arg(bundle_path)
                    .status()
                    .map_err(|e| format!("failed to mv bundle: {e}"))?;
                return Ok(());
            }
        }

        let dest = current_exe.clone();
        std::fs::rename(&source_path, &dest)
            .map_err(|e| format!("failed to rename binary: {e}"))?;

        Ok(())
    })();

    match swap_result {
        Ok(()) => {
            tracing::info!(
                target: "corpora::updater",
                version = %version,
                "pending swap applied successfully"
            );
            let _ = std::fs::remove_file(&marker_path);
            super::super::set_status(UpdateStatus::Idle, cx);
        }
        Err(err) => {
            tracing::error!(
                target: "corpora::updater",
                error = %err,
                "pending swap failed"
            );
            // Marker stays so the user/admin can investigate.
            super::super::set_status(UpdateStatus::Error(format!("swap failed: {err}")), cx);
        }
    }
}
