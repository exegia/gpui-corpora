use std::sync::atomic::{AtomicU64, Ordering};

#[cfg(test)]
#[path = "frame_time.test/mod.rs"]
mod frame_time_test;

mod is_slow_frame;
mod last_frame_time_us;
mod slow_frame_threshold_us;
mod store_frame_time;

pub(crate) use is_slow_frame::is_slow_frame;
use last_frame_time_us::LAST_FRAME_TIME_US;
pub use last_frame_time_us::last_frame_time_us;
pub(crate) use slow_frame_threshold_us::SLOW_FRAME_THRESHOLD_US;
pub(crate) use store_frame_time::store_frame_time;
