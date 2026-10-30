#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn prepare_flush_serializes_a_dirty_state_to_its_file() {
    let dir = tempdir().unwrap();
    let state_file = dir.path().join("state.json");
    let mut state = dirty_state(&state_file);

    let (path, bytes) = prepare_flush(&mut state).expect("dirty state must produce a flush");

    assert_eq!(path, state_file);
    assert_eq!(bytes, serde_json::to_vec(&state.config).unwrap());
}
