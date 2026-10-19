#[allow(unused_imports)]
use super::*;

/// Apply a [`save_config`] outcome; `true` means a stale write (mutation
/// mid-flight or reordered commits) re-marked dirty — re-arm the debounce.
pub(crate) fn commit_flush(
    state: &mut AppState,
    written_bytes: Vec<u8>,
    result: Result<(), AppError>,
) -> bool {
    match result {
        Ok(()) => {
            state.last_save_error = None;
            let current = serde_json::to_vec(&state.config).unwrap_or_default();
            if current == written_bytes {
                state.dirty = false;
                state.last_flushed_bytes = written_bytes;
                tracing::debug!(
                    target: "corpora::app_state",
                    state_file = %state.paths.state_file.display(),
                    "persisted app state"
                );
                false
            } else {
                // Stale write: re-flush the current bytes, and don't let a
                // concurrent clean commit keep `dirty = false`.
                state.dirty = true;
                tracing::debug!(
                    target: "corpora::app_state",
                    "config changed during flush; will re-flush on next debounce"
                );
                true
            }
        }
        Err(err) => {
            let error = err.to_string();
            tracing::error!(
                target: "corpora::app_state",
                error = %error,
                "failed to persist app state"
            );
            state.last_save_error = Some(error);
            false
        }
    }
}
