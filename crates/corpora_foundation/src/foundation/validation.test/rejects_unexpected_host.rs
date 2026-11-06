#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn rejects_unexpected_host() {
    let err = validate_deep_link_url("corpora://evil-host").unwrap_err();
    assert!(
        matches!(err, AppError::InvalidDeepLink { ref reason, .. } if reason.contains("unexpected host"))
    );
}
