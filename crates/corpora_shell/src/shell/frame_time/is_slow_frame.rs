#[allow(unused_imports)]
use super::*;

pub(crate) fn is_slow_frame(elapsed_us: u64) -> bool {
    elapsed_us > SLOW_FRAME_THRESHOLD_US
}
