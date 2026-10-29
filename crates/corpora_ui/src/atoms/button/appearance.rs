use crate::ely::theme::{Mix, Palette, Radius, Theme};
use crate::styles::utils::ColorExt as _;
use crate::ui::material::{GlassOverrides, GlassSpec};
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

    /// The variant's glass material. `tint` is the (animated) semantic color for
    /// the current pointer state. It is applied at the preset's tint strength;
    /// when the variant defines no tint (transparent), the preset's own tint stays.
    /// `engage` scales the tint and rim: above 1 on hover, below 1 while pressed.
    pub(super) fn glass_spec(self, theme: &Theme, tint: Hsla, engage: f32) -> Option<GlassSpec> {
        let mode = if theme.is_dark() {
            Mode::Dark
        } else {
            Mode::Light
        };
        let spec = match self {
            Self::Default => GlassSpec::accent(mode, None),
            Self::Secondary => GlassSpec::secondary(mode, Some(GlassOverrides {
                tint: Some(tint.lighten(0.2)),
                blur_radius: Some(px(1.)),
                tint_opacity: Some(0.2),
                edge: Some(0.1),
                ..Default::default()
            })),
            Self::Outline => GlassSpec::accent(mode, Some(GlassOverrides {
                 tint_opacity: Some(0.05),
                 edge: Some(0.5),
                ..Default::default()
            })),
            Self::Destructive => GlassSpec::destructive(mode, None),
            Self::DestructiveOutline => GlassSpec::destructive(mode, Some(GlassOverrides {
                tint_opacity: Some(0.1),
                edge: Some(0.5),
                gain_dark: Some(1.),
                edge_width: Some(px(2.)),
                ..Default::default()
            })),
            Self::Ghost | Self::Link => return None,
        };
        let base = match spec.tint {
            Some(preset) if tint.a > 0. => tint.opacity(preset.a),
            Some(preset) => preset,
            None => crate::ui::glass::neutral_tint(theme),
        };
        Some(GlassSpec {
            // tint: Some(Hsla {
            //     a: (base.a * engage * 1.2).clamp(0., 1.),
            //     ..base
            // }),
            edge: spec.edge * engage,
            ..spec
        })
    }

    /// The variant's colors in the active theme, adjusted for pointer state.
    pub(super) fn colors(self, theme: &Theme, hovered: bool, pressed: bool) -> Colors {
        let palette = &theme.colors;
        let resting = if theme.is_dark() {
            self.dark(palette)
        } else {
            self.light(palette)
        };
        resting.engaged(hovered, pressed)
    }

    fn light(self, c: &Palette) -> Colors {
        let base = Colors::base(c);
        match self {
            Self::Default => Colors {
                // background: c.accent,
                foreground: c.on_accent,
                 // glass_tint: c.accent,
                ..base
            },
            Self::Secondary => Colors {
                // background: c.surface,
                foreground: c.fg,
                // glass_tint: c.surface,
                ..base
            },
            Self::Destructive => Colors {
                // background: c.danger,
                foreground: c.on_accent,
                // glass_tint: c.danger,
                ..base
            },
            Self::Outline => Colors {
                // background: c.bg.lighten(0.5),
                foreground: c.fg,
                // glass_tint: c.glass,
                ..base
            },
            Self::DestructiveOutline => Colors {
                // background: c.bg.lighten(0.5),
                foreground: c.fg,
                // glass_tint: c.danger,
                ..base
            },
            Self::Ghost => Colors {
                // background: transparent_black(),
                foreground: c.fg,
                ..base
            },
            Self::Link => Colors {
                foreground: c.link,
                ..base
            },
        }
    }

    fn dark(self, c: &Palette) -> Colors {
        let base = Colors::base(c);
        match self {
            Self::Default => Colors {
                 // background: c.accent,
                foreground: c.bg,
                // border: c.accent.alpha(0.),
                // glass_tint: c.accent.alpha(0.),
                ..base
            },
            Self::Secondary => Colors {
                 background: c.surface.alpha(0.5),
                foreground: c.fg,
                // border: c.border,
                 glass_tint: c.surface.alpha(0.5),
                ..base
            },
            Self::Destructive => Colors {
                // background: c.danger,
                foreground: c.on_media,
                // border: c.danger.alpha(0.1),
                // glass_tint: c.danger.alpha(0.),
                ..base
            },
            Self::Outline => Colors {
                // border: c.accent,
                foreground: c.fg,
                // glass_tint: c.glass,
                ..base
            },
            Self::DestructiveOutline => Colors {
                // border: c.danger,
                foreground: c.fg,
                // glass_tint: c.danger,
                ..base
            },
            Self::Ghost => Colors {
                // border: c.accent,
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
            border: c.bg.darken(0.2),
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
            background: self.background.opacity(fade),
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
