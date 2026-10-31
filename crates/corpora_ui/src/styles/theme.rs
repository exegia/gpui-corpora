//! Main module to define the app theme.
//!
//! A `Semantics` holds one `Level` per `Variant`, and each `Level` holds a
//! light/dark `Scheme` of concrete colors per role. It is derived from the
//! parsed `palette.toml`, so no color values live in Rust.

use gpui::{Hsla, transparent_black};

use crate::ely::theme::Mode;
use crate::styles::utils::{Palette, PaletteError, Ramp, parser};

/// One color in both appearances.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Scheme {
    pub light: Hsla,
    pub dark: Hsla,
}

impl Scheme {
    pub fn get(&self, mode: Mode) -> Hsla {
        match mode {
            Mode::Light => self.light,
            Mode::Dark => self.dark,
        }
    }
}

/// The colors a component pulls from one semantic variant.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Level {
    pub foreground: Scheme,
    pub background: Scheme,
    pub surface: Scheme,
    pub elevation: Scheme,
    pub outline: Scheme,
    pub shadow: Scheme,
}

impl Level {
    pub fn role(&self, role: Role) -> &Scheme {
        match role {
            Role::Foreground => &self.foreground,
            Role::Background => &self.background,
            Role::Surface => &self.surface,
            Role::Elevation => &self.elevation,
            Role::Outline => &self.outline,
            Role::Shadow => &self.shadow,
        }
    }
}

/// A color slot within a `Level`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Role {
    Foreground,
    Background,
    Surface,
    Elevation,
    Outline,
    Shadow,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Variant {
    Accent,
    Filled,
    Outline,
    Subtle,
    Danger,
    Info,
    Warning,
    Success,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Semantics {
    pub accent: Level,
    pub filled: Level,
    pub outline: Level,
    pub subtle: Level,
    pub danger: Level,
    pub info: Level,
    pub warning: Level,
    pub success: Level,
}

/// A theme built from a parsed palette.
pub trait CorporaTheme: Sized {
    /// Derives the theme from `palette`. Fails if a ramp the theme needs is missing.
    fn from_palette(palette: &Palette) -> Result<Self, PaletteError>;

    /// Loads the palette `.toml` at `path` and derives the theme from it.
    fn load(path: &str) -> Result<Self, PaletteError> {
        Self::from_palette(&parser(path)?)
    }

    fn level(&self, variant: Variant) -> &Level;

    /// The `Level` for `variant` resolved to one appearance.
    fn resolve(&self, variant: Variant, mode: Mode) -> Resolved {
        let level = self.level(variant);
        Resolved {
            foreground: level.foreground.get(mode),
            background: level.background.get(mode),
            surface: level.surface.get(mode),
            elevation: level.elevation.get(mode),
            outline: level.outline.get(mode),
            shadow: level.shadow.get(mode),
        }
    }
}

/// A `Level` for a single appearance.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Resolved {
    pub foreground: Hsla,
    pub background: Hsla,
    pub surface: Hsla,
    pub elevation: Hsla,
    pub outline: Hsla,
    pub shadow: Hsla,
}

impl Resolved {
    pub fn role_color(&self, role: Role) -> Hsla {
        match role {
            Role::Foreground => self.foreground,
            Role::Background => self.background,
            Role::Surface => self.surface,
            Role::Elevation => self.elevation,
            Role::Outline => self.outline,
            Role::Shadow => self.shadow,
        }
    }
}

/// How a variant uses its ramp.
#[derive(Clone, Copy)]
enum Style {
    /// Filled with the ramp color; text is white (light) or the darkest step (dark).
    Solid,
    /// Transparent fill with ramp-colored text and border.
    Outlined,
    /// Faint tint of the ramp with dark text.
    Tinted,
}

fn ramp<'a>(palette: &'a Palette, name: &str) -> Result<&'a Ramp, PaletteError> {
    palette
        .ramp(name)
        .ok_or_else(|| PaletteError::Schema(format!("theme needs color ramp {name:?}")))
}

fn scheme(light: Hsla, dark: Hsla) -> Scheme {
    Scheme { light, dark }
}

fn with_alpha(color: Hsla, alpha: f32) -> Hsla {
    Hsla { a: alpha, ..color }
}

fn level(palette: &Palette, ramp_name: &str, style: Style) -> Result<Level, PaletteError> {
    let r = ramp(palette, ramp_name)?;
    let shadow = scheme(
        with_alpha(palette.black, 0.25),
        with_alpha(palette.black, 0.5),
    );
    Ok(match style {
        Style::Solid => Level {
            foreground: scheme(palette.white, r[950]),
            background: scheme(r[600], r[500]),
            surface: scheme(r[500], r[400]),
            elevation: scheme(r[700], r[300]),
            outline: scheme(r[700], r[300]),
            shadow,
        },
        Style::Outlined => Level {
            foreground: scheme(r[700], r[300]),
            background: scheme(transparent_black(), transparent_black()),
            surface: scheme(r[50], r[950]),
            elevation: scheme(r[100], r[900]),
            outline: scheme(r[600], r[400]),
            shadow,
        },
        Style::Tinted => Level {
            foreground: scheme(r[800], r[200]),
            background: scheme(r[100], r[950]),
            surface: scheme(r[200], r[900]),
            elevation: scheme(r[300], r[800]),
            outline: scheme(r[200], r[800]),
            shadow,
        },
    })
}

impl CorporaTheme for Semantics {
    fn from_palette(palette: &Palette) -> Result<Self, PaletteError> {
        Ok(Self {
            accent: level(palette, "indigo", Style::Solid)?,
            filled: level(palette, "neutral", Style::Solid)?,
            outline: level(palette, "neutral", Style::Outlined)?,
            subtle: level(palette, "slate", Style::Tinted)?,
            danger: level(palette, "red", Style::Solid)?,
            info: level(palette, "sky", Style::Solid)?,
            warning: level(palette, "amber", Style::Solid)?,
            success: level(palette, "green", Style::Solid)?,
        })
    }

    fn level(&self, variant: Variant) -> &Level {
        match variant {
            Variant::Accent => &self.accent,
            Variant::Filled => &self.filled,
            Variant::Outline => &self.outline,
            Variant::Subtle => &self.subtle,
            Variant::Danger => &self.danger,
            Variant::Info => &self.info,
            Variant::Warning => &self.warning,
            Variant::Success => &self.success,
        }
    }
}
