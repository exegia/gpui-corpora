#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn semver_newer_available() {
    let manifest_ver = semver::Version::parse("2.0.0").unwrap();
    let cur_ver = semver::Version::parse("1.0.0").unwrap();
    assert!(manifest_ver > cur_ver);
}
