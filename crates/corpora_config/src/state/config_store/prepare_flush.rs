#[allow(unused_imports)]
use super::*;

/// Serialize on the UI thread (no I/O); `None` when clean or on failure.
pub(crate) fn prepare_flush(state: &mut AppState) -> Option<(PathBuf, Vec<u8>)> {
    let new_bytes = match serde_json::to_vec(&state.config) {
        Ok(bytes) => bytes,
        Err(err) => {
            let error = err.to_string();
            tracing::error!(
                target: "corpora::app_state",
                error = %error,
                "failed to serialize app state"
            );
            state.last_save_error = Some(error);
            return None;
        }
    };

    if new_bytes == state.last_flushed_bytes {
        state.dirty = false;
        return None;
    }

    Some((state.paths.state_file.clone(), new_bytes))
}
