#[allow(unused_imports)]
use super::*;

/// Below this the window is unusably small; above this it likely exceeds a
/// typical display and will be clamped by the platform.
pub(crate) const MIN_WINDOW_DIM: f32 = 100.0;
