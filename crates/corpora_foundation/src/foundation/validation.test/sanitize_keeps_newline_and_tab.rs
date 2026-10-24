#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn sanitize_keeps_newline_and_tab() {
    assert_eq!(sanitize_string("line1\nline2\ttab"), "line1\nline2\ttab");
}
