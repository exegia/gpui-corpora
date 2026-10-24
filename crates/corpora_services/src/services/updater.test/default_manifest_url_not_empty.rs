#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn default_manifest_url_not_empty() {
    assert!(!DEFAULT_MANIFEST_URL.is_empty());
}
