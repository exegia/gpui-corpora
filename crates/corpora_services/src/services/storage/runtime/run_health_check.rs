#[allow(unused_imports)]
use super::*;

/// Run a health check against the storage backend without blocking the main
/// GPUI render loop.
///
/// The result is written back to the global [`StorageSnapshot`] via an
/// `cx.update` callback once the check completes. The check runs on a
/// background thread (synchronous SQLite I/O).
pub fn run_health_check(cx: &mut App) {
    let Some(runtime) = cx.try_global::<StorageRuntime>().cloned() else {
        return;
    };
    let bg = cx.background_executor().clone();
    cx.spawn(async move |cx| {
        let backend = runtime.backend.clone();
        let version_backend = Arc::clone(&backend);
        let (result, version_result) = {
            let health_task = bg.spawn(async move { backend.health_check().await });
            let version_task = bg.spawn(async move { version_backend.schema_version().await });
            (health_task.await, version_task.await)
        };

        let _ = cx.update(|cx| {
            cx.update_global::<StorageSnapshot, _>(|snap, _cx| match result {
                Ok(()) => {
                    snap.healthy = true;
                    snap.last_error = None;
                    if let Ok(version) = version_result {
                        snap.schema_version = version;
                    }
                }
                Err(err) => {
                    snap.healthy = false;
                    snap.last_error = Some(err.to_string());
                }
            });
        });
    })
    .detach();
}
