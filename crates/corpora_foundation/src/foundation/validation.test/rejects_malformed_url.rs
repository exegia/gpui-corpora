#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn rejects_malformed_url() {
    assert!(validate_deep_link_url("://missing-scheme").is_err());
}
