#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn oversized_ipc_frame_is_dropped() {
    if !GenericNamespaced::is_supported() {
        return;
    }
    let line = bounded_line_harness(MAX_LINE_BYTES + 1);
    assert_eq!(line, None, "frame past the 16 KiB cap must be dropped");
}
