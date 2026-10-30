#[allow(unused_imports)]
use super::*;

/// Flush pending config changes to disk immediately; a no-op when clean.
/// Called on the shutdown path, where the debounced write cannot fire.
pub fn force_save(cx: &mut App) {
    if cx.try_global::<AppState>().is_none() {
        return;
    }

    SAVE_SCHEDULED.store(false, Ordering::Relaxed);

    // Shutdown: write synchronously (the process must not exit before the
    // write lands); the debounced hot path keeps fsync off the UI thread.
    cx.update_global::<AppState, _>(|state, _cx| {
        // Drop the debounce task first so it cannot race or reorder beneath
        // the synchronous write below.
        state.in_flight_save = None;

        if !state.dirty {
            return;
        }
        if let Some((path, bytes)) = prepare_flush(state) {
            let result = save_config(&path, &bytes);
            // Nothing to re-arm on the shutdown path.
            let _ = commit_flush(state, bytes, result);
        }
    });
}
