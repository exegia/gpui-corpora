#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn rejects_unexpected_host() {
    let msg = parse_deep_link("corpora://evil-host")
        .unwrap_err()
        .to_string();
    assert!(msg.contains("unexpected host"), "got: {msg}");
}
