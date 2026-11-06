#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn is_slow_frame_above_threshold() {
    assert!(is_slow_frame(4_001));
}
