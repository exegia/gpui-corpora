#[allow(unused_imports)]
use super::*;

pub(crate) fn store_frame_time(elapsed_us: u64) {
    LAST_FRAME_TIME_US.store(elapsed_us, Ordering::Relaxed);
}
