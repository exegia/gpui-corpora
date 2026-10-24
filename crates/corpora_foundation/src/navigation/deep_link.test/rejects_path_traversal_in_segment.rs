#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn rejects_path_traversal_in_segment() {
    let msg = parse_deep_link("corpora://settings/..%2Fetc")
        .unwrap_err()
        .to_string();
    assert!(
        msg.contains("path traversal") || msg.contains("forbidden character"),
        "got: {msg}"
    );
}
