//! Builds the Default theme's Ely palettes from themes/default.toml and
//! themes/palette.toml. The files are embedded and parsed once on first use;
//! no colors are hardcoded here. A value is a `scale.step` (amber.500), a named
//! palette color (white), or a raw `#rrggbb`.

use std::collections::HashMap;
use std::sync::OnceLock;

use gpui::{Hsla, rgb};
use serde::Deserialize;

use crate::ely::theme::{Mode, Palette, Syntax};
use crate::styles::utils::oklch;

#[cfg(test)]
#[path = "palette.test.rs"]
mod palette_test;

#[derive(Deserialize)]
struct PaletteDoc {
    colors: HashMap<String, Entry>,
}

#[derive(Deserialize)]
#[serde(untagged)]
enum Entry {
    Named(String),
    Scale(HashMap<String, String>),
}

#[derive(Deserialize)]
struct ThemeDoc {
    colors: HashMap<String, Role>,
    ansi: Modes<[String; 6]>,
    syntax: Modes<SyntaxRefs>,
}

#[derive(Deserialize)]
struct Role {
    light: String,
    dark: String,
}

#[derive(Deserialize)]
struct Modes<T> {
    light: T,
    dark: T,
}

#[derive(Deserialize)]
struct SyntaxRefs {
    keyword: String,
    string: String,
    number: String,
    comment: String,
    function: String,
    type_name: String,
    constant: String,
    property: String,
    tag: String,
    attribute: String,
    operator: String,
    punctuation: String,
    variable: String,
}

struct Parsed {
    scales: HashMap<String, HashMap<String, String>>,
    named: HashMap<String, String>,
    theme: ThemeDoc,
}

impl Parsed {
    fn resolve(&self, reference: &str) -> Hsla {
        let literal = if let Some((scale, step)) = reference.split_once('.') {
            self.scales
                .get(scale)
                .and_then(|steps| steps.get(step))
                .unwrap_or_else(|| panic!("unknown palette color `{reference}`"))
        } else if reference.starts_with('#') {
            return parse_color(reference);
        } else {
            self.named
                .get(reference)
                .unwrap_or_else(|| panic!("unknown palette color `{reference}`"))
        };
        parse_color(literal)
    }

    fn role(&self, name: &str, mode: Mode) -> Hsla {
        let role = self
            .theme
            .colors
            .get(name)
            .unwrap_or_else(|| panic!("default.toml has no role `{name}`"));
        self.resolve(pick(mode, &role.light, &role.dark))
    }

    fn ansi(&self, mode: Mode) -> [Hsla; 6] {
        let refs = pick(mode, &self.theme.ansi.light, &self.theme.ansi.dark);
        std::array::from_fn(|i| self.resolve(&refs[i]))
    }

    fn syntax(&self, mode: Mode) -> Syntax {
        let s = pick(mode, &self.theme.syntax.light, &self.theme.syntax.dark);
        Syntax {
            keyword: self.resolve(&s.keyword),
            string: self.resolve(&s.string),
            number: self.resolve(&s.number),
            comment: self.resolve(&s.comment),
            function: self.resolve(&s.function),
            type_name: self.resolve(&s.type_name),
            constant: self.resolve(&s.constant),
            property: self.resolve(&s.property),
            tag: self.resolve(&s.tag),
            attribute: self.resolve(&s.attribute),
            operator: self.resolve(&s.operator),
            punctuation: self.resolve(&s.punctuation),
            variable: self.resolve(&s.variable),
        }
    }
}

fn pick<T>(mode: Mode, light: T, dark: T) -> T {
    match mode {
        Mode::Light => light,
        Mode::Dark => dark,
    }
}

fn parsed() -> &'static Parsed {
    static CACHE: OnceLock<Parsed> = OnceLock::new();
    CACHE.get_or_init(|| {
        let palette: PaletteDoc = toml::from_str(include_str!("../themes/palette.toml"))
            .expect("palette.toml is malformed");
        let theme: ThemeDoc = toml::from_str(include_str!("../themes/default.toml"))
            .expect("default.toml is malformed");
        let (mut scales, mut named) = (HashMap::new(), HashMap::new());
        for (name, entry) in palette.colors {
            match entry {
                Entry::Named(color) => drop(named.insert(name, color)),
                Entry::Scale(steps) => drop(scales.insert(name, steps)),
            }
        }
        Parsed {
            scales,
            named,
            theme,
        }
    })
}

fn parse_color(literal: &str) -> Hsla {
    let literal = literal.trim();
    if let Some(hex) = literal.strip_prefix('#') {
        parse_hex(hex)
    } else if let Some(inner) = literal
        .strip_prefix("oklch(")
        .and_then(|r| r.strip_suffix(')'))
    {
        parse_oklch(inner)
    } else {
        panic!("unrecognized color `{literal}`")
    }
}

fn parse_hex(hex: &str) -> Hsla {
    let expanded;
    let digits = if hex.len() == 3 {
        expanded = hex.chars().flat_map(|c| [c, c]).collect::<String>();
        expanded.as_str()
    } else {
        hex
    };
    rgb(u32::from_str_radix(digits, 16).expect("hex color digits")).into()
}

fn parse_oklch(inner: &str) -> Hsla {
    let mut parts = inner.split_whitespace();
    let lightness = parts.next().expect("oklch lightness");
    let chroma = parts.next().expect("oklch chroma");
    let hue = parts.next().expect("oklch hue");
    let lightness = lightness.strip_suffix('%').map_or_else(
        || lightness.parse::<f32>().expect("oklch lightness"),
        |percent| percent.parse::<f32>().expect("oklch lightness") / 100.0,
    );
    oklch(
        lightness,
        chroma.parse::<f32>().expect("oklch chroma"),
        hue.parse::<f32>().expect("oklch hue"),
    )
}

/// The Default theme's `Palette` for one mode, mapping each default.toml role
/// onto its Ely field. Unlisted fields keep Ely's values via `..base`.
pub fn build(mode: Mode) -> Palette {
    let parsed = parsed();
    let base = match mode {
        Mode::Light => Palette::light(false),
        Mode::Dark => Palette::dark(false),
    };
    let role = |name| parsed.role(name, mode);
    let bg = role("background");
    Palette {
        bg,
        surface: role("surface"),
        sunken: role("sunken"),
        overlay: role("elevated"),
        hover: role("hover"),
        active: role("active"),
        border: role("border"),
        fg: role("foreground"),
        fg_muted: role("muted"),
        fg_subtle: role("subtle"),
        fg_disabled: role("disabled"),
        accent: role("accent"),
        accent_hover: role("accent-hover"),
        on_accent: role("on-accent"),
        focus: role("focus"),
        link: role("link"),
        selection: role("selection"),
        success: role("success"),
        warning: role("warning"),
        danger: role("danger"),
        info: role("info"),
        success_subtle: role("success-subtle"),
        warning_subtle: role("warning-subtle"),
        danger_subtle: role("danger-subtle"),
        info_subtle: role("info-subtle"),
        glass: Hsla { a: 0.72, ..bg },
        ansi: merge_ansi(base.ansi, parsed.ansi(mode)),
        syntax: parsed.syntax(mode),
        ..base
    }
}

/// The Default theme's color for a semantic role (`background`, `border`, ...)
/// in the given mode. Panics if default.toml has no such role.
pub fn color(role: &str, mode: Mode) -> Hsla {
    parsed().role(role, mode)
}

/// Overwrites ANSI 1-6 with the theme's colors; the rest keep Ely's row.
fn merge_ansi(mut row: [Hsla; 16], overrides: [Hsla; 6]) -> [Hsla; 16] {
    for (slot, color) in row[1..7].iter_mut().zip(overrides) {
        *slot = color;
    }
    row
}

/// Resolves a palette reference (`amber.500`, `white`, `#rrggbb`) to a color.
#[cfg(test)]
pub(crate) fn resolve(reference: &str) -> Hsla {
    parsed().resolve(reference)
}
