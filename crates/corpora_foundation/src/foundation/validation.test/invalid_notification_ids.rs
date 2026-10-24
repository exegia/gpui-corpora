#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn invalid_notification_ids() {
    assert!(!validate_notification_id(""));
    assert!(!validate_notification_id("has space"));
    assert!(!validate_notification_id("under_score"));
    assert!(!validate_notification_id("special!char"));
    assert!(!validate_notification_id("dot.name"));
}
