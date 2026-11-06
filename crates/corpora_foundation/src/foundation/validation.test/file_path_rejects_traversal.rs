#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn file_path_rejects_traversal() {
    let err = validate_file_path("../../etc/passwd", &[]).unwrap_err();
    assert!(matches!(err, ValidationError::PathTraversal { .. }));
}
