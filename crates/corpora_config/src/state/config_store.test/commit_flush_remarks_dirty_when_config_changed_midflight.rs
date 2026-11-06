#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn commit_flush_remarks_dirty_when_config_changed_midflight() {
    let dir = tempdir().unwrap();
    let state_file = dir.path().join("state.json");
    let mut state = dirty_state(&state_file);
    let stale_bytes = serde_json::to_vec(&state.config).unwrap();
    // The write raced a mutation: the flushed bytes are no longer current.
    state.config.sidebar_collapsed = !state.config.sidebar_collapsed;

    let needs_rearm = commit_flush(&mut state, stale_bytes, Ok(()));

    assert!(needs_rearm, "a stale write must signal the re-arm");
    assert!(
        state.dirty,
        "the concurrent mutation must not be swallowed by the commit"
    );
    assert!(
        state.last_flushed_bytes.is_empty(),
        "stale bytes must not be recorded as flushed"
    );
}
