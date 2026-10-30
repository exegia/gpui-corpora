#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn sound_flag_maps_onto_audio_element() {
    let mut quiet = NotificationRequest::foreground("Title", "Body");
    quiet.play_sound = false;
    assert!(toast_xml(&quiet).contains(r#"<audio silent="true" />"#));

    let loud = NotificationRequest::foreground("Title", "Body");
    let xml = toast_xml(&loud);
    assert!(xml.contains("ms-winsoundevent:Notification.Default"));
    assert!(!xml.contains("silent"));
}
