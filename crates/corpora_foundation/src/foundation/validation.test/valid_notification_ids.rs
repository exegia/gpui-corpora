#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn valid_notification_ids() {
    assert!(validate_notification_id("abc123"));
    assert!(validate_notification_id("a-b-c"));
    assert!(validate_notification_id("ABC-123-xyz"));
    assert!(validate_notification_id(
        "550e8400-e29b-41d4-a716-446655440000"
    ));
}
