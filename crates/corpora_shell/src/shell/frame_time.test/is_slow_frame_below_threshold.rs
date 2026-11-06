#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn is_slow_frame_below_threshold() {
    assert!(!is_slow_frame(3_999));
}
