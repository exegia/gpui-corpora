#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn thread_id_drives_tag_replacement() {
    let first = toast_tag(&NotificationRequest::reply("Title", "Body"));
    let second = toast_tag(&NotificationRequest::reply("Other", "Body"));
    assert_eq!(first, second, "same thread_id must reuse the toast slot");
    assert_eq!(first, "settings-reply");

    let actions = toast_tag(&NotificationRequest::action_buttons("Title", "Body"));
    assert_ne!(first, actions, "different threads must not collide");

    let long_thread = {
        let mut request = NotificationRequest::foreground("Title", "Body");
        request.thread_id = Some("settings-background-worthy-long".to_string());
        request
    };
    assert_eq!(toast_tag(&long_thread).len(), 16, "tag caps at 16 chars");
}
