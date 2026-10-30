#[allow(unused_imports)]
use super::*;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PersistedWindowBounds {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl PersistedWindowBounds {
    /// Finite, positive dimensions within the corruption ceiling; negative
    /// x/y stay legal (multi-monitor).
    pub(crate) fn is_plausible(&self) -> bool {
        self.x.is_finite()
            && self.y.is_finite()
            && self.width.is_finite()
            && self.height.is_finite()
            && self.width > 0.0
            && self.height > 0.0
            && self.width <= MAX_PLAUSIBLE_DIM
            && self.height <= MAX_PLAUSIBLE_DIM
    }
}
