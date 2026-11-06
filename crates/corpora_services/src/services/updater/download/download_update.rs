#[allow(unused_imports)]
use super::*;

pub fn download_update(cx: &mut App) {
    let current = super::super::snapshot(cx);
    let version = match &current.status {
        UpdateStatus::Available { version, .. } => version.clone(),
        _ => {
            tracing::warn!(
                target: "corpora::updater",
                status = ?current.status,
                "download requested but no update available"
            );
            return;
        }
    };

    super::super::set_status(UpdateStatus::Downloading { progress: 0 }, cx);

    let (rt, client) = crate::services::tokio_runtime::runtime_and_client(cx)
        .expect("tokio runtime global must be installed before downloading updates");
    // Reuse the asset cached by the most recent check to skip a second
    // manifest fetch.
    let cached_asset = current.cached_asset.clone();

    cx.spawn(async move |cx| {
        match run_download(version, cached_asset, rt, client, cx).await {
            Ok(DownloadOutcome::Success { version, path }) => {
                cx.update(|cx| {
                    super::super::reset_download_retry(cx);
                    super::super::set_status(
                        UpdateStatus::Downloaded {
                            version: version.clone(),
                            path,
                        },
                        cx,
                    );
                    super::super::notify_update_downloaded(&version, cx);
                });
            }
            Ok(DownloadOutcome::PermanentFailure(err)) => {
                // Signature/codesign mismatches must not be retried.
                cx.update(|cx| {
                    super::super::set_status(UpdateStatus::Error(err), cx);
                });
            }
            Err(err) => {
                cx.update(|cx| handle_download_failure(err, cx));
            }
        }
    })
    .detach();
}
