#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn sanitize_strips_control_chars() {
    let dirty = "  hello\x00\x01\x02 world\t\n  ";
    assert_eq!(sanitize_string(dirty), "hello world");
}
