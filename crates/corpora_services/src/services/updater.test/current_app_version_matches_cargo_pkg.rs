#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn current_app_version_matches_cargo_pkg() {
    let version = current_app_version();
    assert_eq!(version, env!("CARGO_PKG_VERSION"));
}
