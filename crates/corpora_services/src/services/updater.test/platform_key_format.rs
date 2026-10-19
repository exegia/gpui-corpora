#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn platform_key_format() {
    let key = platform_key();
    let parts: Vec<&str> = key.split('-').collect();
    assert!(
        parts.len() >= 2,
        "platform_key should contain at least os and arch separated by dash, got: {key}"
    );
    let arch = std::env::consts::ARCH;
    assert!(
        key.contains(arch),
        "platform_key should contain arch '{arch}', got: {key}"
    );
}
