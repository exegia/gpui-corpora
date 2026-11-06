#[allow(unused_imports)]
use super::*;

/// A clean store: nothing pending, the default config already flushed, so
/// only a real mutation produces bytes.
pub(crate) fn clean_state(state_file: &std::path::Path) -> AppState {
    let mut state = dirty_state(state_file);
    state.dirty = false;
    state.last_flushed_bytes = serde_json::to_vec(&state.config).unwrap();
    state
}
