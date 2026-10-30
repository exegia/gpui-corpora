#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn commit_flush_success_commits_the_written_bytes() {
    let dir = tempdir().unwrap();
    let state_file = dir.path().join("state.json");
    let mut state = dirty_state(&state_file);
    state.last_save_error = Some("previous failure".to_string());
    let bytes = serde_json::to_vec(&state.config).unwrap();

    let needs_rearm = commit_flush(&mut state, bytes.clone(), Ok(()));

    assert!(!needs_rearm, "a clean commit must not re-arm the debounce");
    assert!(!state.dirty);
    assert_eq!(state.last_flushed_bytes, bytes);
    assert_eq!(state.last_save_error, None, "a good write clears the error");
}
