#[allow(unused_imports)]
use super::*;

/// No-op without an endpoint or backend.
pub fn upload_pending_reports(cx: &mut App) {
    let snap = snapshot(cx);
    if snap.upload_endpoint.is_empty() {
        tracing::debug!(
            target: "corpora::crash_report",
            "no upload endpoint configured, skipping pending report upload"
        );
        return;
    }

    let Some(runtime) = cx.try_global::<crate::services::tokio_runtime::TokioRuntimeGlobal>()
    else {
        tracing::warn!(
            target: "corpora::crash_report",
            "tokio runtime not available for crash report upload"
        );
        return;
    };

    let Some(storage_runtime) = cx.try_global::<crate::storage::StorageRuntime>() else {
        tracing::warn!(
            target: "corpora::crash_report",
            "storage runtime not available for crash report upload"
        );
        return;
    };

    let endpoint = snap.upload_endpoint.clone();
    let http_client = runtime.0.http_client.clone();
    let backend = storage_runtime.backend.clone();

    cx.spawn(async move |cx| {
        let reports = {
            let backend = backend.clone();
            cx.background_executor()
                .spawn(async move { backend.load_pending_crash_reports(50).await })
                .await
        };

        let reports = match reports {
            Ok(r) => r,
            Err(err) => {
                tracing::warn!(
                    target: "corpora::crash_report",
                    error = %err,
                    "failed to load pending crash reports"
                );
                return;
            }
        };

        if reports.is_empty() {
            return;
        }

        tracing::info!(
            target: "corpora::crash_report",
            count = reports.len(),
            "uploading pending crash reports"
        );

        for report in &reports {
            let body = match serde_json::to_string(report) {
                Ok(b) => b,
                Err(err) => {
                    tracing::warn!(
                        target: "corpora::crash_report",
                        id = %report.id,
                        error = %err,
                        "failed to serialize crash report"
                    );
                    continue;
                }
            };

            let result = http_client
                .post(&endpoint)
                .header("Content-Type", "application/json")
                .body(body)
                .send()
                .await;

            match result {
                Ok(resp) if resp.status().is_success() => {
                    let uploaded_at = chrono::Utc::now().to_rfc3339();
                    let backend = backend.clone();
                    let id = report.id.clone();
                    let mark_result = cx
                        .background_executor()
                        .spawn(async move {
                            backend.mark_crash_report_uploaded(&id, &uploaded_at).await
                        })
                        .await;
                    if let Err(err) = mark_result {
                        tracing::warn!(
                            target: "corpora::crash_report",
                            id = %report.id,
                            error = %err,
                            "failed to mark crash report as uploaded"
                        );
                    }
                }
                Ok(resp) => {
                    tracing::warn!(
                        target: "corpora::crash_report",
                        id = %report.id,
                        status = %resp.status(),
                        "crash report upload returned non-success status"
                    );
                }
                Err(err) => {
                    tracing::warn!(
                        target: "corpora::crash_report",
                        id = %report.id,
                        error = %err,
                        "crash report upload failed"
                    );
                }
            }
        }

        // One 1-row query yields both the pending count and newest timestamp.
        let backend = backend.clone();
        let latest = cx
            .background_executor()
            .spawn(async move { backend.load_pending_crash_reports(1).await })
            .await;
        let (count, timestamp) = match latest {
            Ok(r) => (r.len(), r.first().map(|r| r.timestamp.clone())),
            Err(_) => (0, None),
        };
        let _ = cx.update(|cx| {
            cx.update_global::<CrashReportSnapshot, _>(|snap, _cx| {
                snap.pending_count = count;
                snap.last_crash_timestamp = timestamp;
            });
        });
    })
    .detach();
}
