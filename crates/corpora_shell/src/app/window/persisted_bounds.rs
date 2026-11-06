#[allow(unused_imports)]
use super::*;

/// The config shape for a window's bounds.
pub fn persisted_bounds(bounds: Bounds<Pixels>) -> crate::app_state::PersistedWindowBounds {
    crate::app_state::PersistedWindowBounds {
        x: bounds.origin.x.into(),
        y: bounds.origin.y.into(),
        width: bounds.size.width.into(),
        height: bounds.size.height.into(),
    }
}
