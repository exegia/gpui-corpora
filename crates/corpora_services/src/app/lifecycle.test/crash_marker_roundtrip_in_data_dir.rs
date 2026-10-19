#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn crash_marker_roundtrip_in_data_dir() {
    let marker = test_data_dir().join("crash-marker");
    let _ = std::fs::remove_file(&marker);

    write_crash_marker();
    #[cfg(target_family = "unix")]
    {
        use std::os::unix::fs::PermissionsExt as _;
        let mode = std::fs::metadata(&marker).unwrap().permissions().mode();
        assert_eq!(mode & 0o777, 0o600, "marker must stay user-only");
    }
    let contents = check_previous_crash();
    assert!(contents.is_some(), "marker should exist after write");
    assert!(
        contents.unwrap().starts_with("pid="),
        "marker should start with pid="
    );

    remove_crash_marker();
    assert!(check_previous_crash().is_none(), "marker should be removed");
    assert!(!marker.exists());
}
