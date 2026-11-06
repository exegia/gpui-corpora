// No `use super::*`: gpui's `test` proc-macro glob would make bare `#[test]`
// recurse infinitely. Import the items under test explicitly.
use super::persisted_bounds;
use crate::app_state::PersistedWindowBounds;
use gpui::{Bounds, point, px, size};

// === split-modules: generated submodules ===
mod persisted_bounds_copies_pixel_geometry;
mod persisted_bounds_keeps_fractional_dpi_values;
