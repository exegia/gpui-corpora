//! Color math in Oklch, where equal lightness steps look equally far apart.

use std::ops::Index;

use gpui::{Background, Hsla, Rgba, linear_color_stop, linear_gradient, rgb};
use palette::{FromColor, Oklch, Srgb, convert::FromColorUnclamped};

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
