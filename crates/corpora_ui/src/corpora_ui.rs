//! Shared UI pieces: toasts and the Default theme palettes.

/// Standalone Ely types for release builds without annotation APIs.
#[cfg(not(debug_assertions))]
pub use ely_gpui_component as ely;
/// Shared Ely types for app controls and the debug feedback overlay.
#[cfg(debug_assertions)]
pub use gpui_agentation::ely;

pub mod styles {
    pub mod insets;
    pub mod palette;
    pub mod tokens;
    pub mod utils;
}
pub mod atoms;
pub mod composed;
pub mod ui;
