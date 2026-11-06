#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn semver_invalid_version() {
    assert!(semver::Version::parse("not-a-version").is_err());
}
