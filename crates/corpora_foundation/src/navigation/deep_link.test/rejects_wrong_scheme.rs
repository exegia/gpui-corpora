#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn rejects_wrong_scheme() {
    let msg = parse_deep_link("https://home").unwrap_err().to_string();
    assert!(msg.contains("unsupported scheme"), "got: {msg}");
}
