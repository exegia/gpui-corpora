#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn sanitize_trims_and_limits_length() {
    let long: String = "a".repeat(5000);
    let result = sanitize_string(&long);
    assert_eq!(result.len(), MAX_SANITIZED_LENGTH);
    // Result is trimmed so no surrounding whitespace.
    assert!(result.starts_with('a'));
}
