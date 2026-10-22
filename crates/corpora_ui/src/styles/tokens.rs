//! The macOS Classic theme (huacnlee/zed-theme-macos-classic) as Ely palettes.

use crate::ely::theme::{Mix as _, Mode, Palette, Syntax, Theme};
use gpui::{App, Hsla, rgb, rgba};

#[cfg(test)]
#[path = "tokens.test.rs"]
mod tokens_test;

/// Light and dark palettes for macOS Classic. Fields the source theme leaves
/// out (chart hues, bright ANSI, tooltips, backdrops) keep Ely's values.
#[derive(Clone, Debug, PartialEq)]
pub struct MacosClassic {
    pub light: Palette,
    pub dark: Palette,
}

impl MacosClassic {
    pub const NAME: &'static str = "macOS Classic";

    pub fn new() -> Self {
        Self {
            light: light(),
            dark: dark(),
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

impl Default for MacosClassic {
    fn default() -> Self {
        Self::new()
    }
}

fn hex(value: u32) -> Hsla {
    rgb(value).into()
}

fn hexa(value: u32) -> Hsla {
    rgba(value).into()
}

/// ANSI 1-6 (red, green, yellow, blue, magenta, cyan) from the theme's base
/// colors; black, white, and the bright row stay Ely's.
fn ansi(mut ansi: [Hsla; 16], base: [u32; 6]) -> [Hsla; 16] {
    for (slot, value) in ansi[1..7].iter_mut().zip(base) {
        *slot = hex(value);
    }
    ansi
}

fn light() -> Palette {
    let ely = Palette::light(false);
    let fg = hex(0x000000);
    // Tailwind amber-500, oklch(76.9% 0.188 70.08).
    let accent = hex(0xfe9a00);
    Palette {
        bg: hex(0xf9f9f9),
        surface: hex(0xfefefe),
        sunken: hex(0xeaeaea),
        overlay: hex(0xf5f5f5),
        hover: hex(0xe5e5e5),
        active: hex(0xd9d9d9),
        border: hex(0xd2d2d2),
        fg,
        fg_muted: hex(0x707070),
        fg_subtle: hex(0x6d6d6d),
        fg_disabled: hex(0xa4abb6),
        accent,
        // The source has no hover shade for its primary color.
        accent_hover: accent.mix(&fg, 0.15),
        // Tailwind amber-950; white text is unreadable on amber.
        on_accent: hex(0x461901),
        focus: accent,
        // Tailwind amber-700: amber-500 text is under 3:1 on the light background.
        link: hex(0xbb4d00),
        selection: hexa(0xfe9a0033),
        success: hex(0x319a00),
        warning: hex(0xb59a00),
        danger: hex(0xd21f07),
        info: hex(0xbb4d00),
        // Tailwind amber-100.
        info_subtle: hex(0xfef3c6),
        glass: hexa(0xf9f9f9b8),
        ansi: ansi(
            ely.ansi,
            [0xd21f07, 0x319a00, 0xb59a00, 0x0060de, 0x9a0068, 0x007e8a],
        ),
        syntax: Syntax {
            keyword: hex(0x0433ff),
            string: hex(0x036a07),
            number: hex(0x0433ff),
            comment: hex(0x007fff),
            function: hex(0x0000a2),
            type_name: hex(0x6f42c1),
            constant: hex(0xc5060b),
            property: hex(0x333333),
            tag: hex(0x0433ff),
            attribute: hex(0x957931),
            operator: fg,
            punctuation: fg,
            variable: hex(0x333333),
        },
        ..ely
    }
}

fn dark() -> Palette {
    let ely = Palette::dark(false);
    let fg = hex(0xdedede);
    let editor_fg = hex(0xcaccca);
    // Tailwind amber-300, oklch(87.9% 0.169 91.605).
    let accent = hex(0xffd230);
    Palette {
        bg: hex(0x131313),
        surface: hex(0x1c1c1e),
        sunken: hex(0x202020),
        overlay: hex(0x101010),
        hover: hexa(0x35353599),
        active: hex(0x353535),
        border: hex(0x303030),
        fg,
        fg_muted: hex(0x9d9d9d),
        fg_subtle: hex(0x9e9e9e),
        fg_disabled: hex(0x5d5945),
        accent,
        // The source has no hover shade for its primary color.
        accent_hover: accent.mix(&fg, 0.15),
        // Tailwind amber-950; white text is unreadable on amber.
        on_accent: hex(0x461901),
        focus: accent,
        link: accent,
        // Tailwind amber-800, which keeps the light text over it readable.
        selection: hex(0x973c00),
        success: hex(0x30d158),
        warning: hex(0xffc600),
        danger: hex(0xff5257),
        info: accent,
        success_subtle: hex(0x0c4619),
        warning_subtle: hex(0x3a310e),
        danger_subtle: hex(0x46190c),
        // Tailwind amber-900.
        info_subtle: hex(0x7b3306),
        glass: hexa(0x131313b8),
        ansi: ansi(
            ely.ansi,
            [0xff5257, 0x30d158, 0xffc600, 0x419cff, 0xa550a7, 0x07fdd3],
        ),
        syntax: Syntax {
            keyword: hex(0xc28b12),
            string: hex(0x62ba46),
            number: hex(0xe1d797),
            comment: hex(0x9e9e9e),
            function: hex(0xfdd888),
            type_name: hex(0xc75828),
            constant: hex(0xe1d797),
            property: editor_fg,
            tag: hex(0xb5af9a),
            attribute: hex(0xe7cb8f),
            operator: editor_fg,
            punctuation: editor_fg,
            variable: editor_fg,
        },
        ..ely
    }
}
