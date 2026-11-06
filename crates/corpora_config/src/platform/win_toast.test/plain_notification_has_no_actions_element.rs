#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn plain_notification_has_no_actions_element() {
    let xml = toast_xml(&NotificationRequest::foreground("Title", "Body"));
    assert!(
        !xml.contains("<actions"),
        "uncategorized toasts carry no actions"
    );
}
