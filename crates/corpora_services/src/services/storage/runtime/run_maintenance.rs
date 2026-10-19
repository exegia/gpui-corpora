#[allow(unused_imports)]
use super::*;

/// Run storage maintenance (e.g. `PRAGMA optimize`) without blocking the
/// main GPUI render loop.
///
/// The result is written back to the global [`StorageSnapshot`] via an
/// `cx.update` callback once maintenance completes. Maintenance runs on a
/// background thread.
pub fn run_maintenance(cx: &mut App) {
    let Some(runtime) = cx.try_global::<StorageRuntime>().cloned() else {
        return;
    };
    let bg = cx.background_executor().clone();
    cx.spawn(async move |cx| {
        let backend = runtime.backend.clone();
        let result = bg.spawn(async move { backend.maintenance().await }).await;

        let _ = cx.update(|cx| {
            cx.update_global::<StorageSnapshot, _>(|snap, _cx| match result {
                Ok(()) => {
                    snap.last_maintenance_at = Some(chrono::Utc::now().to_rfc3339());
                    snap.last_error = None;
                }
                Err(err) => {
                    snap.last_error = Some(err.to_string());
                }
            });
        });
    })
    .detach();
}
