#[allow(unused_imports)]
use super::*;

pub fn check_for_updates(cx: &mut App) {
    let current = super::super::snapshot(cx);
    match &current.status {
        UpdateStatus::Checking | UpdateStatus::Downloading { .. } => {
            tracing::debug!(
                target: "corpora::updater",
                status = ?current.status,
                "skipping check — update operation already in progress"
            );
            return;
        }
        _ => {}
    }

    let connectivity = crate::connectivity::snapshot(cx);
    if connectivity.state != crate::connectivity::ConnectivityState::Online {
        tracing::warn!(
            target: "corpora::updater",
            connectivity = ?connectivity.state,
            "skipping update check — not online"
        );
        handle_check_failure("no network connectivity".to_string(), cx);
        return;
    }

    super::super::set_status(UpdateStatus::Checking, cx);

    let (rt, client) = crate::services::tokio_runtime::runtime_and_client(cx)
        .expect("tokio runtime global must be installed before checking for updates");
    let current_version = current.current_version.clone();

    // A cached manifest is fresh for one periodic check interval (matching the
    // re-check cadence in mod.rs); within that window skip the network fetch.
    let cached_manifest = current.cached_manifest.clone();
    let cache_fresh = cached_manifest.is_some()
        && current
            .last_check
            .as_ref()
            .and_then(|ts| chrono::DateTime::parse_from_rfc3339(ts).ok())
            .map(|checked_at| {
                let age = chrono::Utc::now()
                    .signed_duration_since(checked_at.with_timezone(&chrono::Utc));
                age.num_seconds() < PERIODIC_CHECK_INTERVAL_SECS as i64
            })
            .unwrap_or(false);

    cx.spawn(async move |cx| {
        let manifest_result: Result<UpdateManifest, String> = match cached_manifest {
            Some(manifest) if cache_fresh => Ok(manifest),
            _ => fetch_manifest(rt.clone(), client.clone()).await,
        };

        cx.update(move |cx| match manifest_result {
            Ok(manifest) => {
                tracing::info!(
                    target: "corpora::updater",
                    manifest_version = %manifest.version,
                    current_version = %current_version,
                    cache_hit = cache_fresh,
                    "fetched update manifest"
                );

                // Cache the manifest and platform asset so download_update can
                // skip re-fetching; re-storing on a cache hit is idempotent.
                let asset_for_cache = {
                    let key = platform_key();
                    manifest.platforms.get(&key).cloned()
                };
                UpdateSnapshot::update_global(cx, |snap, _cx| {
                    snap.cached_manifest = Some(manifest.clone());
                    snap.cached_asset = asset_for_cache;
                });

                // Stamp last_check only when the network was actually hit.
                if !cache_fresh {
                    let now = chrono::Utc::now().to_rfc3339();
                    UpdateSnapshot::update_global(cx, |snap, _cx| {
                        snap.last_check = Some(now.clone());
                    });
                    crate::app_state::update_config(cx, |config| {
                        config.last_update_check = Some(now);
                    });
                }

                match (
                    semver::Version::parse(&manifest.version),
                    semver::Version::parse(&current_version),
                ) {
                    (Ok(manifest_ver), Ok(cur_ver)) => {
                        super::super::reset_check_retry(cx);
                        if manifest_ver > cur_ver {
                            let status = UpdateStatus::Available {
                                version: manifest.version.clone(),
                                notes: manifest.release_notes.clone(),
                            };
                            super::super::set_status(status, cx);
                            super::super::notify_update_available(&manifest.version, cx);
                        } else {
                            // Equal or older manifest — up to date (or ahead).
                            super::super::set_status(UpdateStatus::UpToDate, cx);
                        }
                    }
                    (Err(e), _) | (_, Err(e)) => {
                        tracing::error!(
                            target: "corpora::updater",
                            error = %e,
                            "failed to parse version for comparison"
                        );
                        super::super::set_status(
                            UpdateStatus::Error(format!("version parse error: {e}")),
                            cx,
                        );
                    }
                }
            }
            Err(err) => {
                tracing::error!(
                    target: "corpora::updater",
                    error = %err,
                    "update check failed"
                );
                handle_check_failure(err, cx);
            }
        });
    })
    .detach();
}
