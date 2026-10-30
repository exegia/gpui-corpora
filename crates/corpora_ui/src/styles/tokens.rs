//! The app's Default theme. Its palettes are built from themes/default.toml and
//! themes/palette.toml by `styles::palette`; this module only names the theme
//! and installs it into Ely and Kit.

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

/// Synchronizes Kit colors after initialization or an appearance change.
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
    });
}

/// Button-specific roles resolved from the shared palette and semantic colors.
pub(crate) struct ButtonPalette {
    pub neutral: gpui::Hsla,
    pub on_neutral: gpui::Hsla,
    pub danger_fill: gpui::Hsla,
    pub danger_hover: gpui::Hsla,
    pub on_danger: gpui::Hsla,
    pub bezel_lit: gpui::Hsla,
    pub bezel_dim: gpui::Hsla,
}

impl ButtonPalette {
    pub fn new(palette: &Palette, dark: bool) -> Self {
        use crate::ely::theme::Mix as _;
        let danger_fill = crate::styles::palette::color("danger", Mode::Light);
        Self {
            neutral: if dark { palette.on_media } else { palette.fg },
            on_neutral: if dark { palette.fg } else { palette.on_media },
            danger_fill,
            danger_hover: danger_fill.mix(&palette.shadow, 0.12),
            on_danger: palette.on_media,
            bezel_lit: palette.on_media.opacity(if dark { 0.24 } else { 0.32 }),
            bezel_dim: palette.shadow.opacity(if dark { 0.3 } else { 0.22 }),
        }
    }
}
