//! The app's Default theme. Its palettes are built from themes/default.toml and
//! themes/palette.toml by `styles::palette`; this module only names the theme
//! and installs it into Ely.

use crate::ely::theme::{Mode, Palette, Theme};
use crate::styles::palette::build;
use gpui::App;

#[cfg(test)]
#[path = "tokens.test.rs"]
mod tokens_test;

/// Light and dark palettes for the Default theme.
#[derive(Clone, Debug, PartialEq)]
pub struct DefaultTheme {
    pub light: Palette,
    pub dark: Palette,
}

impl DefaultTheme {
    pub const NAME: &'static str = "Default";

    pub fn new() -> Self {
        Self {
            light: build(Mode::Light),
            dark: build(Mode::Dark),
        }
    }

    pub fn palette(&self, mode: Mode) -> &Palette {
        match mode {
            Mode::Light => &self.light,
            Mode::Dark => &self.dark,
        }
    }

    /// Makes these the palettes Ely fades to in each mode. Ely's `Theme`
    /// global must already be initialized.
    pub fn install(self, cx: &mut App) {
        Theme::set_palette(Mode::Light, Some(self.light), cx);
        Theme::set_palette(Mode::Dark, Some(self.dark), cx);
    }
}

impl Default for DefaultTheme {
    fn default() -> Self {
        Self::new()
    }
}

/// Removes Kit focus outlines after initialization or an appearance change.
pub fn disable_focus_outlines(cx: &mut App) {
    gpui_kit::component::Theme::update(cx, |theme| {
        theme.focus_ring = false;
        theme.ring = gpui::transparent_black();
    });
}
