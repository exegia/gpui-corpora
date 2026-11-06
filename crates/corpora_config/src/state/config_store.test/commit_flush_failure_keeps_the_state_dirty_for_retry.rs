#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn commit_flush_failure_keeps_the_state_dirty_for_retry() {
    let dir = tempdir().unwrap();
    let state_file = dir.path().join("state.json");
    let mut state = dirty_state(&state_file);
    let bytes = serde_json::to_vec(&state.config).unwrap();
    let error = AppError::StateWrite {
        path: state_file.to_path_buf(),
        details: "disk full".to_string(),
    };

    let needs_rearm = commit_flush(&mut state, bytes, Err(error));

    assert!(!needs_rearm, "a failed write must not loop immediately");
    assert!(
        state.dirty,
        "a failed write stays pending for the next flush"
    );
    assert!(state.last_save_error.is_some());
}
