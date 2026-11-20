//! The app's Default theme. Its palettes are built from themes/default.toml and
//! themes/palette.toml by `styles::palette`; this module only names the theme
//! and installs it into Ely and Kit.

use std::borrow::Cow;

use crate::ely::theme::{Mode, Palette, Theme};
use crate::styles::palette::build;
use gpui::App;

#[cfg(test)]
#[path = "tokens.test.rs"]
mod tokens_test;

/// Family name of the bundled interface typeface.
pub const UI_FONT_FAMILY: &str = "Manhattan UI";

/// Registers the bundled Manhattan UI faces. Call before opening windows.
/// Semibold requests resolve to Bold, the nearest heavier face.
pub fn register_ui_fonts(cx: &App) -> gpui::Result<()> {
    cx.text_system().add_fonts(vec![
        Cow::Borrowed(include_bytes!("../fonts/manhattan/ManhattanUI-Regular.ttf").as_slice()),
        Cow::Borrowed(include_bytes!("../fonts/manhattan/ManhattanUI-Medium.ttf").as_slice()),
        Cow::Borrowed(include_bytes!("../fonts/manhattan/ManhattanUI-Bold.ttf").as_slice()),
    ])
}

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

    /// Makes these the palettes Ely fades to in each mode and sets the UI
    /// font family. Ely's `Theme` global must already be initialized.
    pub fn install(self, cx: &mut App) {
        Theme::set_palette(Mode::Light, Some(self.light), cx);
        Theme::set_palette(Mode::Dark, Some(self.dark), cx);
        cx.global_mut::<Theme>().font_family = UI_FONT_FAMILY.into();
    }
}

impl Default for DefaultTheme {
    fn default() -> Self {
        Self::new()
    }
}

/// Synchronizes Kit colors and the UI font family after initialization or an
/// appearance change. Kit window roots, popovers and tooltips read this font.
pub fn sync_kit_theme(cx: &mut App) {
    gpui_kit::component::Theme::update(cx, |theme| {
        let mode = if theme.is_dark() {
            Mode::Dark
        } else {
            Mode::Light
        };
        theme.colors = crate::styles::kit::colors(mode);
        theme.highlight_theme = std::sync::Arc::new(crate::styles::kit::highlight(mode));
        theme.transparent = crate::styles::palette::color("shadow", mode).opacity(0.1);
        theme.focus_ring = false;
        theme.font_family = UI_FONT_FAMILY.into();
    });
}
