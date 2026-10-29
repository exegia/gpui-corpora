use crate::ely::theme::{Mix, Palette, Radius, Theme};
use crate::ui::material::GlassSpec;
use gpui::{Hsla, px, transparent_black};
use gpui_agentation::ely::theme::Mode;

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

    /// Composites the animated tint over the backdrop at its specified opacity.
    pub(super) fn glass_spec(self, theme: &Theme, tint: Hsla, engage: f32) -> Option<GlassSpec> {
        if matches!(self, Self::Ghost | Self::Link) {
            return None;
        }
        let mode = if theme.is_dark() {
            Mode::Dark
        } else {
            Mode::Light
        };
        let spec = match self {
            Self::Default | Self::Outline => GlassSpec::accent(mode, None),
            Self::Secondary => GlassSpec::secondary(mode, None),
            Self::Destructive | Self::DestructiveOutline => GlassSpec::destructive(mode, None),
            Self::Ghost | Self::Link => unreachable!(),
        };
        if theme.is_dark() {
            return Some(GlassSpec {
                tint: Some(if tint.a > 0. {
                    tint
                } else {
                    spec.tint.unwrap_or(theme.colors.glass)
                }),
                edge: spec.edge * engage,
                ..spec
            });
        }
        Some(GlassSpec {
            tint: Some(tint),
            gain_light: 1. - tint.a,
            gain_dark: 1. - tint.a,
            saturation: 1.,
            edge: if theme.is_dark() {
                spec.edge * engage
            } else {
                0.08
            },
            edge_width: px(1.),
            ..spec
        })
    }

    /// The variant's colors in the active theme, adjusted for pointer state.
    pub(super) fn colors(self, theme: &Theme, hovered: bool, pressed: bool) -> Colors {
        if theme.is_dark() {
            self.dark(&theme.colors).engaged(hovered, pressed)
        } else {
            self.light(&theme.colors, hovered || pressed)
        }
    }

    fn light(self, c: &Palette, active: bool) -> Colors {
        let base = Colors::base(c);
        let color = |hex, alpha| gpui::Hsla::from(gpui::rgb(hex)).alpha(alpha);
        let (tint, foreground, border) = match self {
            Self::Default => (
                if active {
                    color(0xffcc00, 1.)
                        .mix(&color(0xffb700, 1.), 0.61 / (0.61 + 0.67 * 0.39))
                        .opacity(0.61 + 0.67 * 0.39)
                } else {
                    color(0xffcc00, 0.95)
                },
                color(0x401600, 1.),
                transparent_black(),
            ),
            Self::Secondary => (
                color(0x1a1a1a, if active { 0.96 } else { 0.87 }),
                color(0xf1f1f1, 1.),
                transparent_black(),
            ),
            Self::Destructive => (
                if active {
                    color(0xc9000a, 1.)
                } else {
                    color(0xe7000b, 0.83)
                },
                color(0xf1f1f1, 1.),
                transparent_black(),
            ),
            Self::Outline => (
                color(0x262626, if active { 0.06 } else { 0. }),
                color(0x0a0a0a, 1.),
                color(0x0a0a0a, if active { 0.41 } else { 0.16 }),
            ),
            Self::DestructiveOutline => (
                color(0xae0000, if active { 0.1 } else { 0.01 }),
                color(if active { 0xb50009 } else { 0x7a0006 }, 1.),
                color(0x930007, if active { 0.35 } else { 0.25 }),
            ),
            Self::Ghost => (
                color(0xc9c9c9, if active { 0.42 } else { 0. }),
                color(0x0c0c0c, 0.93),
                transparent_black(),
            ),
            Self::Link => (transparent_black(), c.link, transparent_black()),
        };
        Colors {
            background: tint,
            glass_tint: tint,
            foreground,
            border,
            ..base
        }
    }

    fn dark(self, c: &Palette) -> Colors {
        let base = Colors::base(c);
        match self {
            Self::Default => Colors {
                foreground: c.bg,
                ..base
            },
            Self::Secondary => Colors {
                background: c.surface.alpha(0.5),
                foreground: c.fg,
                glass_tint: c.surface.alpha(0.5),
                ..base
            },
            Self::Destructive => Colors {
                foreground: c.on_media,
                ..base
            },
            Self::Outline => Colors {
                foreground: c.fg,
                ..base
            },
            Self::DestructiveOutline => Colors {
                foreground: c.fg,
                ..base
            },
            Self::Ghost => Colors {
                foreground: c.fg,
                ..base
            },
            Self::Link => Colors {
                foreground: c.link,
                ..base
            },
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

    /// Hover fades the background slightly and a press fades it further.
    fn engaged(self, hovered: bool, pressed: bool) -> Self {
        let fade = if pressed {
            0.8
        } else if hovered {
            0.9
        } else {
            1.0
        };
        let (engage, highlight) = if pressed {
            (0.85, 0.14)
        } else if hovered {
            (1.3, 0.08)
        } else {
            (1., 0.)
        };
        Self {
            background: self.foreground.opacity(highlight),
            glass_tint: self.glass_tint.opacity(self.glass_tint.a * fade),
            engage,
            highlight,
            ..self
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
