#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn semver_older_manifest() {
    let manifest_ver = semver::Version::parse("0.9.0").unwrap();
    let cur_ver = semver::Version::parse("1.0.0").unwrap();
    assert!(manifest_ver < cur_ver);
}
