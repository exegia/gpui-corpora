//! The macOS Classic theme (huacnlee/zed-theme-macos-classic) as Ely palettes.
//! Each mode names a few base colors; the other semantic colors are steps on
//! their `Scale`, so changing a base re-derives its shades.

use crate::ely::theme::{Mode, Palette, Syntax, Theme};
use crate::styles::utils::{ColorExt, Scale, Spread, convert};
use gpui::{App, Hsla, rgb};
use gpui_kit::component::{Colorize, black};


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

/// Removes Kit focus outlines after initialization or an appearance change.
pub fn disable_focus_outlines(cx: &mut App) {
    gpui_kit::component::Theme::update(cx, |theme| {
        theme.focus_ring = false;
        theme.ring = gpui::transparent_black();
    });
}

fn hex(value: u32) -> Hsla {
    rgb(value).into()
}

/// ANSI 1-6 (red, green, yellow, blue, magenta, cyan) from the theme's base
/// colors; black, white, and the bright row stay Ely's.
fn ansi(mut ansi: [Hsla; 16], base: [u32; 6]) -> [Hsla; 16] {
    for (slot, value) in ansi[1..7].iter_mut().zip(base) {
        *slot = hex(value);
    }
    ansi
}

pub fn light() -> Palette {
    let ely = Palette::light(false);
    let bg = hex(0xf8fafc);
    let fg = hex(0x020617);
    // Tailwind amber-500, oklch(0.82 0.17 79).
    let accent = hex(0xfeb500);
    let (success, warning, danger) = (hex(0x319a00), hex(0xb59a00), hex(0xd21f07));
    // Surfaces sit a few percent apart; text and accents need contrast-sized steps.
    let surfaces = Scale::new(bg, Spread::TIGHT);
    let text = convert(fg);
    let amber = convert(accent);
    Palette {
        bg,
        surface: surfaces[100],
        sunken: surfaces[600],
        overlay: surfaces[400],
        hover: surfaces[600],
        active: surfaces[700],
        border: surfaces[800],
        fg,
        fg_muted: text[300],
        fg_subtle: text[200],
        fg_disabled: text[100],
        accent,
        accent_hover: amber[600],
        on_accent: amber[900],
        focus: gpui::transparent_black(),
        // Amber-500 text is under 3:1 on the light background.
        link: amber[800],
        selection: accent.alpha(0.2),
        success,
        warning,
        danger,
        danger_subtle: hex(0xba3e38).opacity(0.2),
        info: amber[800],
        success_subtle: convert(success)[100],
        warning_subtle: convert(warning)[100],
        info_subtle: amber[100],
        glass: bg.alpha(0.72),
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

pub fn dark() -> Palette {
    let ely = Palette::dark(false);
    let bg = hex(0x020617);
    let fg = hex(0xf8fafc);
    let editor_fg = hex(0xf1f5f9);
    // Tailwind amber-300, oklch(0.849 0.135 79).
    let accent = hex(0xfdc25d);
    let (success, warning, danger) = (hex(0x30d158), hex(0xffc600), hex(0xff5257));
    // Lower steps are lighter, so dark surfaces climb toward 100 and text dims toward 900.
    let surfaces = Scale::new(bg, Spread::TIGHT);
    let text = convert(fg);
    let amber = convert(accent);
    Palette {
        bg: bg.mix_oklab(black(), 0.95),
        surface: surfaces[400],
        sunken: surfaces[300],
        overlay: surfaces[600],
        hover: surfaces[300].darken(0.15),
        active: surfaces[100],
        border: surfaces[300],
        fg,
        fg_muted: text[700],
        fg_subtle: text[800],
        fg_disabled: text[900],
        accent,
        accent_hover: amber[400],
        on_accent: amber[900],
        focus: gpui::transparent_black(),
        link: accent,
        selection: amber[900],
        success,
        warning,
        danger,
        info: accent,
        success_subtle: convert(success)[900],
        warning_subtle: convert(warning)[900],
        danger_subtle: hex(0xff5257),
        info_subtle: amber[900],
        glass: bg.alpha(0.72),
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
