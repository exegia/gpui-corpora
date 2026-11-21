//! Shared UI pieces: toasts and the Default theme palettes.

/// Ely types for app controls. One copy for all profiles: linking a second Ely
/// duplicates its action names and panics at startup.
pub use ely_gpui_component as ely;

pub mod styles {
    mod kit;
    pub mod palette;
    pub mod theme;
    pub mod tokens;
    pub mod utils;
}
pub mod atoms;
pub mod composed;
pub mod ui;
