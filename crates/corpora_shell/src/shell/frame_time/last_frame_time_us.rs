#[allow(unused_imports)]
use super::*;

/// Last frame render time in microseconds, stored atomically so the status bar
/// can read it without requiring a reference to `AppRoot`.
pub(crate) static LAST_FRAME_TIME_US: AtomicU64 = AtomicU64::new(0);

pub fn last_frame_time_us() -> u64 {
    LAST_FRAME_TIME_US.load(Ordering::Relaxed)
}
