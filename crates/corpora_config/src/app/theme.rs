use ely_gpui_component::theme::{ActiveTheme as _, Mode, Theme};
use gpui::App;
use serde::{Deserialize, Serialize};

#[cfg(test)]
#[path = "theme.test/mod.rs"]
mod theme_test;

mod current_mode;
mod mode;
mod set_theme_mode;
mod set_theme_mode_with_record;
mod theme_mode;

pub use current_mode::current_mode;
pub use set_theme_mode::set_theme_mode;
pub use set_theme_mode_with_record::set_theme_mode_with_record;
pub use theme_mode::ThemeMode;
