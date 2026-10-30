#[allow(unused_imports)]
use super::*;

pub(crate) async fn run_download(
    version: String,
    cached_asset: Option<PlatformAsset>,
    rt: Arc<tokio::runtime::Runtime>,
    client: reqwest::Client,
    cx: &AsyncApp,
) -> Result<DownloadOutcome, String> {
    let asset = match cached_asset {
        Some(a) => a,
        None => match super::super::check::fetch_platform_asset(rt.clone(), client.clone()).await {
            Ok(a) => {
                // Cache the fresh asset so apply_update can find the verified
                // signature for the swap marker.
                let fetched = a.clone();
                cx.update(|cx| {
                    UpdateSnapshot::update_global(cx, |snap, _cx| {
                        snap.cached_asset = Some(fetched);
                    });
                });
                a
            }
            Err(err) => {
                tracing::error!(
                    target: "corpora::updater",
                    error = %err,
                    "failed to resolve platform asset for download"
                );
                return Err(err);
            }
        },
    };

    // Download into the app-owned updates dir; a shared $TMPDIR would let a
    // pre-created symlink redirect this write of unverified bytes.
    let Some(dest_dir) = updates_dir() else {
        return Err("no app updates directory available".to_string());
    };

    // The file name comes from the manifest-controlled URL; pin it to a plain
    // last segment so it can never point outside the updates dir.
    let raw_name = asset.url.rsplit('/').next().unwrap_or("");
    let file_name =
        if raw_name.is_empty() || raw_name == "." || raw_name == ".." || raw_name.contains('\\') {
            "update.bin"
        } else {
            raw_name
        };
    let dest_path = dest_dir.join(file_name);
    let signature = asset.signature.clone();

    tracing::info!(
        target: "corpora::updater",
        url = %asset.url,
        dest = %dest_path.display(),
        size = asset.size,
        "starting download"
    );

    let response = match rt
        .spawn(async move {
            client
                .get(&asset.url)
                .timeout(std::time::Duration::from_secs(300))
                .send()
                .await
        })
        .await
    {
        Ok(Ok(r)) => r,
        Ok(Err(e)) => return Err(format!("download request failed: {e}")),
        Err(e) => return Err(format!("download request panicked: {e}")),
    };

    if !response.status().is_success() {
        return Err(format!("download returned status {}", response.status()));
    }

    let total: u64 = response
        .headers()
        .get("content-length")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.parse().ok())
        .unwrap_or(0);

    let file = match std::fs::File::create(&dest_path) {
        Ok(f) => f,
        Err(err) => return Err(format!("failed to create download file: {err}")),
    };

    let progress = Arc::new(AtomicU32::new(0));
    let total_for_task = total;
    let progress_clone = progress.clone();

    // Self-contained 'static task so it can run entirely on the tokio runtime.
    let download_handle = rt.spawn(async move {
        use futures_util::StreamExt as _;
        use std::io::Write as _;
        let mut stream = response.bytes_stream();
        let mut downloaded: u64 = 0;
        let mut last_reported: u32 = 0;
        let mut file = file;
        let mut last_err: Option<String> = None;

        while let Some(chunk_result) = stream.next().await {
            match chunk_result {
                Ok(chunk) => {
                    downloaded += chunk.len() as u64;
                    if let Err(err) = file.write_all(&chunk) {
                        last_err = Some(format!("failed to write download chunk: {err}"));
                        break;
                    }
                    if total_for_task > 0 {
                        let pct = (downloaded as f32 / total_for_task as f32 * 100.0) as u32;
                        if pct / 10 > last_reported / 10 {
                            last_reported = pct;
                            progress_clone.store(pct, Ordering::Relaxed);
                        }
                    }
                }
                Err(e) => {
                    last_err = Some(format!("failed to read download chunk: {e}"));
                    break;
                }
            }
        }

        let _ = file.flush();
        if let Some(err) = last_err {
            Err(err)
        } else {
            Ok(downloaded)
        }
    });

    // The GPUI background-executor timer avoids a fresh tokio task per tick.
    let mut last_progress: u32 = 0;
    loop {
        let cur = progress.load(Ordering::Relaxed);
        if cur != last_progress {
            last_progress = cur;
            cx.update(|cx| {
                super::super::set_status(UpdateStatus::Downloading { progress: cur }, cx);
            });
        }

        if download_handle.is_finished() {
            break;
        }

        cx.background_executor()
            .timer(std::time::Duration::from_millis(200))
            .await;
    }

    let cur = progress.load(Ordering::Relaxed);
    if cur != last_progress {
        cx.update(|cx| {
            super::super::set_status(UpdateStatus::Downloading { progress: cur }, cx);
        });
    }

    let downloaded = match download_handle.await {
        Ok(Ok(bytes)) => bytes,
        Ok(Err(err)) => {
            tracing::error!(target: "corpora::updater", error = %err, "download failed");
            return Err(err);
        }
        Err(e) => return Err(format!("download task panicked: {e}")),
    };

    let path_str = dest_path.to_string_lossy().to_string();
    tracing::info!(
        target: "corpora::updater",
        version = %version,
        path = %path_str,
        downloaded_bytes = downloaded,
        "download complete"
    );

    // Fail closed: a manifest that omits the signature must never install,
    // or any manifest tamper could ship an arbitrary binary.
    if signature.is_empty() {
        tracing::error!(
            target: "corpora::updater",
            path = %path_str,
            "manifest asset has no signature — refusing to install unverified download"
        );
        let _ = std::fs::remove_file(&dest_path);
        return Ok(DownloadOutcome::PermanentFailure(
            "manifest asset has no signature".to_string(),
        ));
    }

    // Hash + Ed25519 verify can take seconds on a large binary — keep it off
    // the foreground thread.
    let dest_for_verify = dest_path.clone();
    let verify_result = cx
        .background_executor()
        .spawn(async move { verify_ed25519_signature(&dest_for_verify, &signature) })
        .await;
    if let Err(err) = verify_result {
        tracing::error!(
            target: "corpora::updater",
            path = %path_str,
            error = %err,
            "Ed25519 signature verification failed — deleting download"
        );
        let _ = std::fs::remove_file(&dest_path);
        return Ok(DownloadOutcome::PermanentFailure(err));
    }
    tracing::info!(
        target: "corpora::updater",
        path = %path_str,
        "Ed25519 signature verification passed"
    );

    #[cfg(target_os = "macos")]
    {
        let dest_for_codesign = dest_path.clone();
        let codesign_result = cx
            .background_executor()
            .spawn(async move { verify_codesign(&dest_for_codesign) })
            .await;
        if let Err(err) = codesign_result {
            tracing::error!(
                target: "corpora::updater",
                path = %path_str,
                error = %err,
                "codesign verification failed"
            );
            return Ok(DownloadOutcome::PermanentFailure(err));
        }
        tracing::info!(
            target: "corpora::updater",
            path = %path_str,
            "codesign verification passed"
        );
    }

    Ok(DownloadOutcome::Success {
        version,
        path: path_str,
    })
}
