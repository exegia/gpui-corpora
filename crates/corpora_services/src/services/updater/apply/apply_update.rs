#[allow(unused_imports)]
use super::*;

pub fn apply_update(cx: &mut App) {
    let current = super::super::snapshot(cx);
    let (version, path) = match &current.status {
        UpdateStatus::Downloaded { version, path } => (version.clone(), path.clone()),
        _ => {
            tracing::warn!(
                target: "corpora::updater",
                status = ?current.status,
                "apply requested but no downloaded update"
            );
            return;
        }
    };

    // The marker carries the manifest signature so the swap can re-verify the
    // file at install time (it lives in a shared temp dir until then). Without
    // a cached signature there is nothing to re-verify against — refuse.
    let signature = current
        .cached_asset
        .map(|asset| asset.signature)
        .unwrap_or_default();
    if signature.is_empty() {
        tracing::error!(
            target: "corpora::updater",
            "no cached signature for downloaded update — refusing to schedule swap"
        );
        super::super::set_status(
            UpdateStatus::Error("cannot schedule swap: no signature available".to_string()),
            cx,
        );
        return;
    }

    tracing::info!(
        target: "corpora::updater",
        version = %version,
        path = %path,
        "scheduling update swap on next launch"
    );

    super::super::set_status(UpdateStatus::ReadyToInstall, cx);

    let Some(marker_path) = pending_swap_path() else {
        tracing::error!(
            target: "corpora::updater",
            "no app updates directory; cannot schedule swap"
        );
        super::super::set_status(
            UpdateStatus::Error("cannot schedule swap: no app data dir".to_string()),
            cx,
        );
        return;
    };
    let pending = serde_json::json!({
        "version": version,
        "source_path": path,
        "signature": signature,
        "scheduled_at": chrono::Utc::now().to_rfc3339(),
    });
    if let Err(err) = std::fs::write(&marker_path, pending.to_string()) {
        tracing::error!(
            target: "corpora::updater",
            path = %marker_path.display(),
            error = %err,
            "failed to write pending swap marker"
        );
        super::super::set_status(
            UpdateStatus::Error(format!("failed to schedule swap: {err}")),
            cx,
        );
    }
}
