#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn validate_path_segment_helper() {
    assert!(validate_path_segment("notifications").is_ok());
    assert!(validate_path_segment("..").is_err());
    assert!(validate_path_segment("foo/../bar").is_err());
    assert!(validate_path_segment("seg\x00ment").is_err());
    assert!(validate_path_segment("tab\there").is_err());
}
