#[allow(unused_imports)]
use super::*;

/// Bounds beyond this cannot be a real window on any display; they indicate a
/// corrupt file. Generous enough that exotic multi-monitor spans still persist.
pub(crate) const MAX_PLAUSIBLE_DIM: f32 = 100_000.0;
