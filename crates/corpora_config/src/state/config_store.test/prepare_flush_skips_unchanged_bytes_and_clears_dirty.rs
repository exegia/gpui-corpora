#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn prepare_flush_skips_unchanged_bytes_and_clears_dirty() {
    let dir = tempdir().unwrap();
    let state_file = dir.path().join("state.json");
    let mut state = dirty_state(&state_file);
    state.last_flushed_bytes = serde_json::to_vec(&state.config).unwrap();

    assert!(
        prepare_flush(&mut state).is_none(),
        "a state identical to the last flush must not write again"
    );
    assert!(!state.dirty, "skipping the write re-cleaned the state");
}
