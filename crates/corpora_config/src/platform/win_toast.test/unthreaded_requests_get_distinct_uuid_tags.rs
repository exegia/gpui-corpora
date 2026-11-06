#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn unthreaded_requests_get_distinct_uuid_tags() {
    let request = NotificationRequest::foreground("Title", "Body");
    let first = toast_tag(&request);
    let second = toast_tag(&request);
    assert_ne!(first, second, "unthreaded toasts stack, never replace");
    for tag in [first, second] {
        // uuid simple format truncated to 16 chars: hex runs split at 8 and 13.
        assert_eq!(tag.len(), 16);
        assert!(tag.bytes().all(|b| b.is_ascii_hexdigit() || b == b'-'));
        assert_eq!(tag.as_bytes()[8], b'-');
        assert_eq!(tag.as_bytes()[13], b'-');
    }
}
