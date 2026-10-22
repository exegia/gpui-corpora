use gpui::{Hsla, hsla, transparent_black};

use crate::ely::theme::{Mix, Theme};

/// Native tint and edge-lighting finishes. Live backdrop refraction is unavailable.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum GlassFinish {
    Clear,
    Frosted,
    Subtle,
    Liquid,
    #[default]
    LiquidRefract,
}

impl GlassFinish {
    pub const ALL: [Self; 5] = [
        Self::LiquidRefract,
        Self::Liquid,
        Self::Frosted,
        Self::Clear,
        Self::Subtle,
    ];

    pub fn name(self) -> &'static str {
        match self {
            Self::Clear => "clear",
            Self::Frosted => "frosted",
            Self::Subtle => "subtle",
            Self::Liquid => "liquid",
            Self::LiquidRefract => "liquid-refract",
        }
    }

    fn tint(self, dark: bool) -> Hsla {
        let (lightness, alpha) = match (self, dark) {
            (Self::Clear, true) => (0., 0.25),
            (Self::Frosted, true) => (0., 0.35),
            (Self::Subtle, true) => (1., 0.06),
            (Self::Liquid | Self::LiquidRefract, true) => (1., 0.04),
            (Self::Clear, false) => (1., 0.25),
            (Self::Frosted, false) => (1., 0.55),
            (Self::Subtle, false) => (1., 0.30),
            (Self::Liquid, false) => (1., 0.10),
            (Self::LiquidRefract, false) => (1., 0.08),
        };
        hsla(0., 0., lightness, alpha)
    }

    pub(super) fn edge(self) -> f32 {
        match self {
            Self::Subtle => 0.12,
            Self::Frosted => 0.20,
            Self::Clear => 0.30,
            Self::Liquid => 0.40,
            Self::LiquidRefract => 0.55,
        }
    }
}

/// Visual variants; a glass finish is accepted only by the glass variant.
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
    Glass(GlassFinish),
}

impl ButtonVariant {
    pub const ALL: [Self; 8] = [
        Self::Default,
        Self::Secondary,
        Self::Outline,
        Self::Destructive,
        Self::DestructiveOutline,
        Self::Ghost,
        Self::Link,
        Self::Glass(GlassFinish::LiquidRefract),
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
            Self::Glass(_) => "glass",
        }
    }

    pub(super) fn colors(self, theme: &Theme, hover: bool, pressed: bool) -> Colors {
        let c = &theme.colors;
        let clear = transparent_black();
        let engaged = hover || pressed;
        let (background, foreground, border, shadow, bevel) = match self {
            Self::Default | Self::Destructive => {
                let (base, foreground) = if self == Self::Default {
                    (c.accent, c.on_accent)
                } else {
                    (c.danger, hsla(0., 0., 1., 1.))
                };
                (
                    base.opacity(if engaged { 0.9 } else { 1. }),
                    foreground,
                    base,
                    base.opacity(0.24),
                    hsla(
                        0.,
                        0.,
                        if pressed { 0. } else { 1. },
                        if pressed { 0.08 } else { 0.16 },
                    ),
                )
            }
            Self::Outline | Self::DestructiveOutline => {
                let destructive = self == Self::DestructiveOutline;
                let base = if theme.is_dark() {
                    c.border.opacity(0.32)
                } else {
                    c.overlay
                };
                let bg = if engaged {
                    if destructive {
                        c.danger.opacity(0.04)
                    } else if theme.is_dark() {
                        c.border.opacity(0.64)
                    } else {
                        c.hover.opacity(0.5)
                    }
                } else {
                    base
                };
                (
                    bg,
                    if destructive { c.danger } else { c.fg },
                    if destructive && engaged {
                        c.danger.opacity(0.32)
                    } else {
                        c.border
                    },
                    c.shadow.opacity(0.05),
                    hsla(
                        0.,
                        0.,
                        if theme.is_dark() { 1. } else { 0. },
                        if theme.is_dark() { 0.06 } else { 0.04 },
                    ),
                )
            }
            Self::Secondary => (
                c.sunken.opacity(if pressed {
                    0.8
                } else if hover {
                    0.9
                } else {
                    1.
                }),
                c.fg,
                clear,
                clear,
                clear,
            ),
            Self::Ghost => (
                if engaged { c.hover } else { clear },
                c.fg,
                clear,
                clear,
                clear,
            ),
            Self::Link => (clear, c.fg, clear, clear, clear),
            Self::Glass(finish) => (finish.tint(theme.is_dark()), c.fg, clear, clear, clear),
        };
        Colors {
            background,
            foreground,
            border,
            shadow: if pressed { clear } else { shadow },
            bevel,
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
}

impl gpui_kit::base::motion::Interpolate for Colors {
    fn interpolate(&self, target: &Self, progress: f32) -> Self {
        Self {
            background: self.background.mix(&target.background, progress),
            foreground: self.foreground.mix(&target.foreground, progress),
            border: self.border.mix(&target.border, progress),
            shadow: self.shadow.mix(&target.shadow, progress),
            bevel: self.bevel.mix(&target.bevel, progress),
        }
    }
}
