#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn error_category_labels() {
    assert_eq!(ErrorCategory::Network.label(), "network");
    assert_eq!(ErrorCategory::Storage.label(), "storage");
    assert_eq!(ErrorCategory::Rendering.label(), "rendering");
    assert_eq!(ErrorCategory::Config.label(), "config");
    assert_eq!(ErrorCategory::System.label(), "system");
}
