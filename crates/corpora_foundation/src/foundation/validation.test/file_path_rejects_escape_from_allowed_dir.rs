#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn file_path_rejects_escape_from_allowed_dir() {
    let tmp = tempfile::tempdir().unwrap();
    let file_path = tmp.path().join("inner.txt");
    fs::write(&file_path, "hello").unwrap();

    // Using a sibling temp dir as the allowed dir should fail.
    let other_tmp = tempfile::tempdir().unwrap();
    let err = validate_file_path(
        file_path.to_str().unwrap(),
        &[other_tmp.path().to_path_buf()],
    )
    .unwrap_err();
    assert!(matches!(err, ValidationError::EscapesAllowedDir { .. }));
}
