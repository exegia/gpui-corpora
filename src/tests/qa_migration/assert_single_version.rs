#[allow(unused_imports)]
use super::*;

pub(crate) fn assert_single_version(lock: &str, package: &str, version: &str) {
    let blocks = lock_blocks(lock, package);
    assert_eq!(
        blocks.len(),
        1,
        "expected exactly one {package} in Cargo.lock, found {} \
         (a second copy reintroduces the gpui-kit-assets links conflict)",
        blocks.len()
    );
    assert!(
        blocks[0].contains(&format!("\nversion = \"{version}\"")),
        "{package} must resolve to {version}"
    );
}
