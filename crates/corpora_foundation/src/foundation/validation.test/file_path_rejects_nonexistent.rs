#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn file_path_rejects_nonexistent() {
    let err = validate_file_path("/no/such/file/ever", &[PathBuf::from("/")]).unwrap_err();
    assert!(matches!(err, ValidationError::NotFound { .. }));
}
