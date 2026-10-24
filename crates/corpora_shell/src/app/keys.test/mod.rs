use super::*;
use gpui::Keystroke;

// === split-modules: generated submodules ===
mod page_bindings_use_secondary;
mod secondary_maps_to_control;
mod secondary_maps_to_platform_modifier;

#[cfg(not(target_os = "macos"))]
use secondary_maps_to_control::secondary_maps_to_control;
