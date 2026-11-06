#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn oversized_config_is_quarantined_without_parsing() {
    let dir = tempdir().unwrap();
    let state_file = dir.path().join("state.json");
    std::fs::write(&state_file, vec![b' '; (MAX_STATE_FILE_BYTES + 1) as usize]).unwrap();

    let (loaded, err) = load_config(&state_file);

    assert_eq!(loaded, AppConfig::default());
    assert!(
        err.is_some(),
        "over-cap file must be reported as a load error"
    );
    assert!(!state_file.exists());
    assert!(state_file.with_extension("json.bad").exists());
}
