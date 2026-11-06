#[allow(unused_imports)]
use super::*;

pub fn report(
    message: impl Into<String>,
    severity: crate::errors::AppErrorSeverity,
    category: ErrorCategory,
    actions: Vec<ErrorAction>,
    cx: &mut App,
) -> EventId {
    // Error text reaches the screen, the sqlite log, and crash reports — scrub
    // home paths so none of them leak the local username.
    let message = scrub_home(&message.into());
    let record = ErrorRecord {
        id: EventId::new(),
        occurred_at: AppTimestamp::now(),
        severity,
        category,
        message,
        actions,
    };
    let id = record.id;
    let record_clone = record.clone();

    // The panic handler attaches recent messages to crash reports.
    crate::lifecycle::track_recent_error(record.message.clone());

    cx.update_global::<ErrorSurfaceState, _>(|state, _cx| {
        state.records.insert(0, record);
        state.records.truncate(MAX_RECORDS);

        // The in-memory mutation stays synchronous; the INSERT is dispatched so
        // the UI thread never blocks on database I/O.
        if let Some(runtime) = _cx.try_global::<crate::storage::StorageRuntime>() {
            let backend = runtime.backend.clone();
            _cx.background_executor()
                .spawn(async move {
                    if let Err(err) = persist_error(&*backend, &record_clone).await {
                        tracing::warn!(
                            target: "corpora::error_surface",
                            error = %err,
                            "failed to persist error to sqlite"
                        );
                    }
                })
                .detach();
        }
    });

    id
}
