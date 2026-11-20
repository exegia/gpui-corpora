//! Color math in Oklch, where equal lightness steps look equally far apart.

use std::{collections::HashMap, ops::Index, sync::OnceLock};

use gpui::{Background, Hsla, Rgba, linear_color_stop, linear_gradient, rgb};
use palette::{FromColor, Oklch, Srgb, convert::FromColorUnclamped};

use crate::ely::theme::Mode;
use crate::styles::theme::{CorporaTheme, Role, Semantics, Variant};

#[cfg(test)]
#[path = "utils.test.rs"]
mod utils_test;

/// A color written as `0xRRGGBB`. Converting from a translucent color drops its alpha.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Hex(pub u32);

impl From<Hex> for Hsla {
    fn from(hex: Hex) -> Self {
        rgb(hex.0).into()
    }
}

impl From<Hsla> for Hex {
    fn from(color: Hsla) -> Self {
        let Rgba { r, g, b, .. } = color.to_rgb();
        let byte = |channel: f32| (channel.clamp(0.0, 1.0) * 255.0).round() as u32;
        Self((byte(r) << 16) | (byte(g) << 8) | byte(b))
    }
}

/// Where `ColorExt::to_gradient` fades to.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Toward {
    /// The color lightened by this amount, from 0 to 1.
    Lighter(f32),
    /// The color darkened by this amount, from 0 to 1.
    Darker(f32),
    Custom(Hsla),
}

/// Oklch adjustments for any color that converts to and from `Hsla`, such as
/// `Hsla`, `Rgba` and `Hex`. Results keep the input's alpha.
pub trait ColorExt: Copy + Into<Hsla> + From<Hsla> {
    /// Moves lightness `amount` (0 to 1) of the way to white; 1 gives white.
    fn lighten(self, amount: f32) -> Self {
        shift(self.into(), amount.clamp(0.0, 1.0)).into()
    }

    /// Moves lightness `amount` (0 to 1) of the way to black; 1 gives black.
    fn darken(self, amount: f32) -> Self {
        shift(self.into(), -amount.clamp(0.0, 1.0)).into()
    }

    /// The WCAG 2 contrast ratio against `other`, from 1 to 21. Alpha is ignored.
    fn contrast(self, other: impl Into<Hsla>) -> f32 {
        let (a, b) = (luminance(self.into()), luminance(other.into()));
        (a.max(b) + 0.05) / (a.min(b) + 0.05)
    }

    /// A two-stop gradient at `angle` degrees, from this color to `toward`.
    fn to_gradient(self, toward: Toward, angle: f32) -> Background {
        let from: Hsla = self.into();
        let to = match toward {
            Toward::Lighter(amount) => from.lighten(amount),
            Toward::Darker(amount) => from.darken(amount),
            Toward::Custom(color) => color,
        };
        linear_gradient(
            angle,
            linear_color_stop(from, 0.0),
            linear_color_stop(to, 1.0),
        )
    }
}

/// Tailwind step names, in order, matching the keys in `palette.toml`.
pub const RAMP_STEPS: [u16; 11] = [50, 100, 200, 300, 400, 500, 600, 700, 800, 900, 950];

/// Eleven shades of one color, indexed by Tailwind step (50, 100..900, 950).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Ramp([Hsla; 11]);

impl Ramp {
    /// The shade at `step`, or `None` if it is not a Tailwind step.
    pub fn get(&self, step: u16) -> Option<Hsla> {
        RAMP_STEPS
            .iter()
            .position(|candidate| *candidate == step)
            .map(|index| self.0[index])
    }
}

impl Index<u16> for Ramp {
    type Output = Hsla;

    /// Panics unless `step` is one of `RAMP_STEPS`.
    fn index(&self, step: u16) -> &Hsla {
        match RAMP_STEPS.iter().position(|candidate| *candidate == step) {
            Some(index) => &self.0[index],
            None => panic!("ramp steps are {RAMP_STEPS:?}, got {step}"),
        }
    }
}

/// A parsed color palette: `black`, `white` and named `Ramp`s such as `gray`.
///
/// ```ignore
/// let palette = parser("src/themes/palette.toml")?;
/// let gray: Hsla = palette["gray"][500];
/// let same = palette.color("gray", 500);
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct Palette {
    pub name: String,
    pub description: String,
    pub black: Hsla,
    pub white: Hsla,
    ramps: HashMap<String, Ramp>,
}

impl Palette {
    pub fn ramp(&self, name: &str) -> Option<&Ramp> {
        self.ramps.get(name)
    }

    /// The shade `name`/`step`, e.g. `color("gray", 500)`.
    pub fn color(&self, name: &str, step: u16) -> Option<Hsla> {
        self.ramps.get(name)?.get(step)
    }

    pub fn ramp_names(&self) -> impl Iterator<Item = &str> {
        self.ramps.keys().map(String::as_str)
    }
}

impl Index<&str> for Palette {
    type Output = Ramp;

    /// Panics if there is no ramp called `name`.
    fn index(&self, name: &str) -> &Ramp {
        match self.ramps.get(name) {
            Some(ramp) => ramp,
            None => panic!("no color ramp named {name:?}"),
        }
    }
}

#[derive(Debug)]
pub enum PaletteError {
    Io(std::io::Error),
    Toml(toml::de::Error),
    /// The file parsed but does not match the expected schema.
    Schema(String),
}

impl std::fmt::Display for PaletteError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(error) => write!(f, "cannot read palette: {error}"),
            Self::Toml(error) => write!(f, "invalid palette TOML: {error}"),
            Self::Schema(message) => write!(f, "invalid palette: {message}"),
        }
    }
}

impl std::error::Error for PaletteError {}

#[derive(serde::Deserialize)]
struct PaletteFile {
    #[serde(default)]
    name: String,
    #[serde(default)]
    description: String,
    colors: HashMap<String, toml::Value>,
}

/// Loads and parses a palette `.toml` file.
///
/// Expected schema: a `[colors]` table holding string colors (`black`,
/// `white`) and one sub-table per ramp, keyed by Tailwind step 50 to 950.
/// Values are `oklch(L% C H)` or `#rgb` / `#rrggbb`.
pub fn parser(path: &str) -> Result<Palette, PaletteError> {
    let source = std::fs::read_to_string(path).map_err(PaletteError::Io)?;
    parse_palette(&source)
}

/// The lit and dim bezel tones for `mode`: a white highlight and a black shade,
/// each carrying its own alpha so it can be mixed during animations.
pub fn bezel_tones(mode: Mode) -> (Hsla, Hsla) {
    let (lit, dim) = match mode {
        Mode::Light => (0.32, 0.22),
        Mode::Dark => (0.24, 0.3),
    };
    (
        palette("white", 0).opacity(lit),
        palette("black", 0).opacity(dim),
    )
}

/// A bezel for a surface: `lit` lights the top inner edge and `dim` shades the
/// bottom one, painted with `gpui-specular`. Use `bezel_tones` for the defaults.
pub fn bezel(lit: Hsla, dim: Hsla) -> gpui_specular::Surface {
    use gpui_specular::{Layer, Surface};
    Surface::new(
        Layer::new(lit, 1.0).offset(0.0, 1.0).blur(1.4),
        Layer::new(dim, 1.0).offset(0.0, -1.0).blur(1.4),
    )
}

/// The color at `name` and `scale` in `themes/palette.toml`, e.g. `palette("gray", 500)`.
/// `"black"` and `"white"` ignore `scale`.
///
/// The file is embedded and parsed once. Panics on an unknown name or step,
/// since both come from code, not user input.
pub fn palette(name: &str, scale: u16) -> Hsla {
    static PALETTE: OnceLock<Palette> = OnceLock::new();
    let palette = PALETTE.get_or_init(|| {
        parse_palette(include_str!("../themes/palette.toml")).expect("themes/palette.toml is valid")
    });
    match name {
        "black" => palette.black,
        "white" => palette.white,
        _ => palette
            .color(name, scale)
            .unwrap_or_else(|| panic!("no palette color {name}.{scale}")),
    }
}

/// The semantic color for `variant` and `role` under `mode`, e.g.
/// `theme(Variant::Danger, Role::Background, Mode::Dark)`.
pub fn theme(variant: Variant, role: Role, mode: Mode) -> Hsla {
    static THEME: OnceLock<Semantics> = OnceLock::new();
    let semantics = THEME.get_or_init(|| {
        let palette = parse_palette(include_str!("../themes/palette.toml"))
            .expect("themes/palette.toml is valid");
        Semantics::from_palette(&palette).expect("palette has every ramp the theme needs")
    });
    semantics.level(variant).role(role).get(mode)
}

/// Like `parser`, but reads from a string, so it also works with `include_str!`.
pub fn parse_palette(source: &str) -> Result<Palette, PaletteError> {
    let file: PaletteFile = toml::from_str(source).map_err(PaletteError::Toml)?;
    let schema = |message: String| PaletteError::Schema(message);

    let mut black = None;
    let mut white = None;
    let mut ramps = HashMap::new();

    for (name, value) in file.colors {
        match value {
            toml::Value::String(text) => {
                let color = parse_color(&text).map_err(|e| schema(format!("{name}: {e}")))?;
                match name.as_str() {
                    "black" => black = Some(color),
                    "white" => white = Some(color),
                    _ => return Err(schema(format!("unexpected single color {name:?}"))),
                }
            }
            toml::Value::Table(table) => {
                let mut shades = [Hsla::default(); 11];
                for (slot, step) in shades.iter_mut().zip(RAMP_STEPS) {
                    let text = table
                        .get(&step.to_string())
                        .and_then(toml::Value::as_str)
                        .ok_or_else(|| schema(format!("{name} is missing step {step}")))?;
                    *slot = parse_color(text).map_err(|e| schema(format!("{name}.{step}: {e}")))?;
                }
                ramps.insert(name, Ramp(shades));
            }
            _ => return Err(schema(format!("{name} must be a string or a table"))),
        }
    }

    Ok(Palette {
        name: file.name,
        description: file.description,
        black: black.ok_or_else(|| schema("missing colors.black".into()))?,
        white: white.ok_or_else(|| schema("missing colors.white".into()))?,
        ramps,
    })
}

/// Parses `oklch(L% C H)` (optional `/ alpha`) or a hex color.
fn parse_color(text: &str) -> Result<Hsla, String> {
    let text = text.trim();
    if let Some(hex) = text.strip_prefix('#') {
        return parse_hex(hex);
    }
    let inner = text
        .strip_prefix("oklch(")
        .and_then(|rest| rest.strip_suffix(')'))
        .ok_or_else(|| format!("unsupported color {text:?}"))?;
    let (components, alpha) = match inner.split_once('/') {
        Some((components, alpha)) => (components, Some(alpha)),
        None => (inner, None),
    };
    let number = |s: &str| {
        s.trim_end_matches('%')
            .parse::<f32>()
            .map_err(|_| format!("bad number {s:?} in {text:?}"))
    };
    let parts: Vec<&str> = components.split_whitespace().collect();
    let [lightness, chroma, hue] = parts[..] else {
        return Err(format!("expected 3 components in {text:?}"));
    };
    let lightness = if lightness.ends_with('%') {
        number(lightness)? / 100.0
    } else {
        number(lightness)?
    };
    let mut color = oklch(lightness, number(chroma)?, number(hue)?);
    if let Some(alpha) = alpha {
        let alpha = alpha.trim();
        color.a = if alpha.ends_with('%') {
            number(alpha)? / 100.0
        } else {
            number(alpha)?
        };
    }
    Ok(color)
}

fn parse_hex(hex: &str) -> Result<Hsla, String> {
    let expanded: String = match hex.len() {
        3 => hex.chars().flat_map(|c| [c, c]).collect(),
        6 => hex.to_owned(),
        _ => return Err(format!("bad hex color #{hex}")),
    };
    u32::from_str_radix(&expanded, 16)
        .map(|value| Hex(value).into())
        .map_err(|_| format!("bad hex color #{hex}"))
}

impl<C: Copy + Into<Hsla> + From<Hsla>> ColorExt for C {}

/// How far a `Scale` reaches from its base, as the share of the remaining
/// lightness covered at step 100 (toward white) and step 900 (toward black).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Spread {
    pub lighter: f32,
    pub darker: f32,
}

impl Spread {
    /// Tailwind-like reach: 100 is a pale tint and 900 a deep shade.
    pub const WIDE: Self = Self {
        lighter: 0.8,
        darker: 0.5,
    };
    /// Small steps for neighboring surfaces such as sunken, hover and border.
    pub const TIGHT: Self = Self {
        lighter: 0.2,
        darker: 0.2,
    };
}

/// Nine shades of one color, indexed 100 to 900, with the base at 500. Lower
/// steps are lighter in both light and dark themes.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Scale([Hsla; 9]);

impl Scale {
    pub fn new(base: impl Into<Hsla>, spread: Spread) -> Self {
        let base = base.into();
        Self(std::array::from_fn(|index| {
            // Index 4 is step 500; each step moves a quarter of the way to an end.
            let t = (4.0 - index as f32) / 4.0;
            if index == 4 {
                base
            } else if t > 0.0 {
                base.lighten(t * spread.lighter)
            } else {
                base.darken(-t * spread.darker)
            }
        }))
    }
}

impl Index<u16> for Scale {
    type Output = Hsla;

    /// Panics unless `step` is a hundred from 100 to 900.
    fn index(&self, step: u16) -> &Hsla {
        assert!(
            step.is_multiple_of(100) && (100..=900).contains(&step),
            "scale steps run 100 to 900 in hundreds, got {step}"
        );
        &self.0[usize::from(step / 100 - 1)]
    }
}

/// The wide scale for `base`: `convert(fg)[300]` is a lighter fg, `[700]` a darker one.
pub fn convert(base: impl Into<Hsla>) -> Scale {
    Scale::new(base, Spread::WIDE)
}

/// Builds an opaque `Hsla` from Oklch components: lightness 0 to 1, chroma, and
/// hue in degrees. Lowers chroma until the color fits sRGB so the hue holds.
pub fn oklch(lightness: f32, chroma: f32, hue: f32) -> Hsla {
    from_oklch(Oklch::new(lightness, chroma, hue), 1.0)
}

fn to_oklch(color: Hsla) -> Oklch {
    let Rgba { r, g, b, .. } = color.to_rgb();
    Oklch::from_color(Srgb::new(r, g, b))
}

fn in_gamut(color: Srgb) -> bool {
    const EPSILON: f32 = 1e-4;
    [color.red, color.green, color.blue]
        .iter()
        .all(|channel| (-EPSILON..=1.0 + EPSILON).contains(channel))
}

/// Converts back to sRGB, lowering chroma until the color fits so the hue holds.
fn from_oklch(color: Oklch, alpha: f32) -> Hsla {
    let mut srgb = Srgb::from_color_unclamped(color);
    if !in_gamut(srgb) {
        let (mut low, mut high) = (0.0, color.chroma);
        for _ in 0..20 {
            let chroma = (low + high) / 2.0;
            if in_gamut(Srgb::from_color_unclamped(Oklch { chroma, ..color })) {
                low = chroma;
            } else {
                high = chroma;
            }
        }
        srgb = Srgb::from_color_unclamped(Oklch {
            chroma: low,
            ..color
        });
    }
    let clamp = |channel: f32| channel.clamp(0.0, 1.0);
    Rgba {
        r: clamp(srgb.red),
        g: clamp(srgb.green),
        b: clamp(srgb.blue),
        a: alpha,
    }
    .into()
}

/// Positive `amount` moves lightness toward white, negative toward black.
fn shift(color: Hsla, amount: f32) -> Hsla {
    let mut oklch = to_oklch(color);
    oklch.l = if amount >= 0.0 {
        oklch.l + (1.0 - oklch.l) * amount
    } else {
        oklch.l + oklch.l * amount
    };
    from_oklch(oklch, color.a)
}

fn luminance(color: Hsla) -> f32 {
    let Rgba { r, g, b, .. } = color.to_rgb();
    let linear = |channel: f32| {
        if channel <= 0.04045 {
            channel / 12.92
        } else {
            ((channel + 0.055) / 1.055).powf(2.4)
        }
    };
    0.2126 * linear(r) + 0.7152 * linear(g) + 0.0722 * linear(b)
}
