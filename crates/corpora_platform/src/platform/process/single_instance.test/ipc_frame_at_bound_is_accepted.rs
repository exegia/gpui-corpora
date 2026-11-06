#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn ipc_frame_at_bound_is_accepted() {
    if !GenericNamespaced::is_supported() {
        return;
    }
    // Payload of exactly MAX_LINE_BYTES plus newline: the reader's
    // take(MAX + 1) window still sees the terminator.
    let line = bounded_line_harness(MAX_LINE_BYTES).expect("frame at the cap must be accepted");
    assert_eq!(line.len(), MAX_LINE_BYTES);
}
