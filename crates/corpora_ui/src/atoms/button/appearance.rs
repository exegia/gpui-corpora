use crate::ely::theme::{Mix, Palette, Radius, Theme};
use crate::styles::tokens::ButtonPalette;
use crate::ui::material::GlassSpec;
use gpui::{Hsla, px, transparent_black};

/// A Button's visual treatment. Colors come from the active theme's semantic roles.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ButtonVariant {
    #[default]
    Default,
    Destructive,
    DestructiveOutline,
    Outline,
    Secondary,
    Ghost,
    Link,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ButtonShape {
    #[default]
    Default,
    Rounded,
}

impl ButtonShape {
    pub fn name(self) -> &'static Radius {
        match self {
            Self::Default => &Radius::Md,
            Self::Rounded => &Radius::Xl,
        }
    }

    pub fn is_rounded(self) -> bool {
        matches!(self, Self::Rounded)
    }

    pub fn is_default(self) -> bool {
        matches!(self, Self::Default)
    }
}

impl ButtonVariant {
    pub const ALL: [Self; 7] = [
        Self::Default,
        Self::Secondary,
        Self::Outline,
        Self::Destructive,
        Self::DestructiveOutline,
        Self::Ghost,
        Self::Link,
    ];

    pub fn name(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Destructive => "destructive",
            Self::DestructiveOutline => "destructive-outline",
            Self::Outline => "outline",
            Self::Secondary => "secondary",
            Self::Ghost => "ghost",
            Self::Link => "link",
        }
    }

    /// Composites the animated tint over native glass, with a solid fallback elsewhere.
    pub(super) fn glass_spec(self, _theme: &Theme, tint: Hsla, engage: f32) -> GlassSpec {
        let filled = matches!(self, Self::Default | Self::Secondary | Self::Destructive);
        GlassSpec {
            tint: Some(tint),
            gain_light: 1. - tint.a,
            gain_dark: 1. - tint.a,
            saturation: 1.,
            blur_radius: px(1.2),
            lens: px(2.),
            reach: px(4.),
            magnify: 1.15,
            dispersion: 0.,
            edge: if filled { 0.08 * engage } else { 0. },
            edge_width: px(1.),
            ..GlassSpec::default()
        }
    }

    /// The variant's colors in the active theme, adjusted for pointer state.
    pub(super) fn colors(self, theme: &Theme, hovered: bool, pressed: bool) -> Colors {
        let mut colors = if theme.is_dark() {
            self.dark(&theme.colors, hovered || pressed)
        } else {
            self.light(&theme.colors, hovered || pressed)
        };
        let palette = ButtonPalette::new(&theme.colors, theme.is_dark());
        colors.bevel = palette.bezel_lit;
        colors.shadow = palette.bezel_dim;
        colors.engage = if pressed {
            0.85
        } else if hovered {
            1.3
        } else {
            1.
        };
        colors
    }

    fn light(self, c: &Palette, active: bool) -> Colors {
        let base = Colors::base(c);
        let tokens = ButtonPalette::new(c, false);
        let (tint, foreground, border) = match self {
            Self::Default => (
                if active {
                    c.accent_hover.opacity(0.61 + 0.67 * 0.39)
                } else {
                    c.accent.opacity(0.95)
                },
                c.on_accent,
                transparent_black(),
            ),
            Self::Secondary => (
                tokens.neutral.opacity(if active { 0.96 } else { 0.87 }),
                tokens.on_neutral,
                transparent_black(),
            ),
            Self::Destructive => (
                if active {
                    tokens.danger_hover
                } else {
                    tokens.danger_fill.opacity(0.9)
                },
                tokens.on_danger,
                transparent_black(),
            ),
            Self::Outline => (
                tokens.neutral.opacity(if active { 0.05 } else { 0.02 }),
                c.fg,
                c.fg.opacity(if active { 0.41 } else { 0.16 }),
            ),
            Self::DestructiveOutline => (
                c.danger.opacity(if active { 0.05 } else { 0.02 }),
                if active {
                    c.danger
                } else {
                    c.danger.mix(&c.shadow, 0.2)
                },
                c.danger.opacity(if active { 0.5 } else { 0.35 }),
            ),
            Self::Ghost => (
                tokens.neutral.opacity(if active { 0.08 } else { 0. }),
                c.fg,
                transparent_black(),
            ),
            Self::Link => (
                transparent_black(),
                if active {
                    c.link.mix(&c.shadow, 0.15)
                } else {
                    c.link
                },
                transparent_black(),
            ),
        };
        Colors {
            background: tint,
            glass_tint: tint,
            foreground,
            border,
            ..base
        }
    }

    fn dark(self, c: &Palette, active: bool) -> Colors {
        let base = Colors::base(c);
        let tokens = ButtonPalette::new(c, true);
        let (tint, foreground, border) = match self {
            Self::Default => (
                if active {
                    c.accent_hover.opacity(0.95)
                } else {
                    c.accent.opacity(0.85)
                },
                c.fg,
                transparent_black(),
            ),
            Self::Secondary => (
                tokens.neutral.opacity(if active { 0.14 } else { 0.08 }),
                c.fg,
                c.border,
            ),
            Self::Destructive => (
                if active {
                    tokens.danger_hover.opacity(0.98)
                } else {
                    tokens.danger_fill.opacity(0.9)
                },
                tokens.on_danger,
                transparent_black(),
            ),
            Self::Outline => (
                tokens.neutral.opacity(if active { 0.05 } else { 0.03 }),
                c.fg,
                if active { c.fg.opacity(0.4) } else { c.border },
            ),
            Self::DestructiveOutline => (
                c.danger.opacity(if active { 0.05 } else { 0.03 }),
                c.danger,
                c.danger.opacity(if active { 0.75 } else { 0.5 }),
            ),
            Self::Ghost => (
                tokens.neutral.opacity(if active { 0.08 } else { 0. }),
                c.fg,
                transparent_black(),
            ),
            Self::Link => (
                transparent_black(),
                if active {
                    c.link.mix(&c.on_media, 0.2)
                } else {
                    c.link
                },
                transparent_black(),
            ),
        };
        Colors {
            background: tint,
            glass_tint: tint,
            foreground,
            border,
            ..base
        }
    }
}

#[derive(Clone, PartialEq)]
pub(super) struct Colors {
    pub background: Hsla,
    pub foreground: Hsla,
    pub border: Hsla,
    pub shadow: Hsla,
    pub bevel: Hsla,
    pub glass_tint: Hsla,
    /// Glass strength multiplier: 1 at rest, higher on hover, lower while pressed.
    pub engage: f32,
    /// Opacity of a foreground-colored wash for variants with no glass.
    pub highlight: f32,
}

impl Colors {
    /// The colors every variant starts from before its own adjustments.
    fn base(c: &Palette) -> Self {
        Self {
            background: c.bg.alpha(0.1),
            foreground: c.fg,
            border: transparent_black(),
            shadow: transparent_black(),
            bevel: transparent_black(),
            glass_tint: transparent_black(),
            engage: 1.,
            highlight: 0.,
        }
    }
}

impl gpui_kit::base::motion::Interpolate for Colors {
    fn interpolate(&self, target: &Self, progress: f32) -> Self {
        Self {
            background: self.background.mix(&target.background, progress),
            foreground: self.foreground.mix(&target.foreground, progress),
            border: self.border.mix(&target.border, progress),
            shadow: self.shadow.mix(&target.shadow, progress),
            bevel: self.bevel.mix(&target.bevel, progress),
            glass_tint: self.glass_tint.mix(&target.glass_tint, progress),
            engage: self.engage + (target.engage - self.engage) * progress,
            highlight: self.highlight + (target.highlight - self.highlight) * progress,
        }
    }
}
