//! Shared UI pieces: toasts and the Default theme palettes.

/// Ely types for app controls and the debug feedback overlay. One copy for all
/// profiles: linking a second Ely duplicates its action names and panics at startup.
pub use gpui_agentation::ely;

pub mod styles {
    mod kit;
    pub mod palette;
    pub mod tokens;
    pub mod utils;
}
pub mod atoms;
pub mod composed;
pub mod ui;
