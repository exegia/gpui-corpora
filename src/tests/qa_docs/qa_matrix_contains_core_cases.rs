#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn qa_matrix_contains_core_cases() {
    let content = std::fs::read_to_string("docs/qa-matrix.md").expect("read qa matrix");
    let normalized = content.to_lowercase();
    assert!(normalized.contains("second-instance forwarding"));
    assert!(normalized.contains("open logs folder"));
    assert!(normalized.contains("secure storage unavailable path"));
}
