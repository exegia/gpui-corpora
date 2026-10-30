#[allow(unused_imports)]
use super::*;

/// Frames whose render preparation takes longer than this are logged at WARN
/// level.
pub(crate) const SLOW_FRAME_THRESHOLD_US: u64 = 4_000;
