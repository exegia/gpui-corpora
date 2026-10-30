#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn rejects_wrong_scheme() {
    let err = validate_deep_link_url("https://example.com").unwrap_err();
    assert!(
        matches!(err, AppError::InvalidDeepLink { ref reason, .. } if reason.contains("unsupported scheme"))
    );
}
