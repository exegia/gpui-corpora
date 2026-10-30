#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn rejects_null_byte_in_segment() {
    let msg = parse_deep_link("corpora://settings/\0notifications")
        .unwrap_err()
        .to_string();
    assert!(
        msg.contains("forbidden character") || msg.contains("invalid deep link"),
        "got: {msg}"
    );
}
