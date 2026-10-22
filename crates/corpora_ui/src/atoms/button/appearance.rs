use gpui::{Hsla, hsla, transparent_black};
use crate::ely::theme::{Mix, Theme};

/// Native tint and edge-lighting finishes. Live backdrop refraction is unavailable.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]

pub enum ButtonVariant {
    #[default]
    Default,
    Destructive,
    DestructiveOutline,
    Outline,
    Secondary,
    Ghost,
    Link
}

impl ButtonVariant {
    pub const ALL: [Self; 7] = [
        Self::Default,
        Self::Secondary,
        Self::Outline,
        Self::Destructive,
        Self::DestructiveOutline,
        Self::Ghost,
        Self::Link
    ];

    pub fn name(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Destructive => "destructive",
            Self::DestructiveOutline => "destructive-outline",
            Self::Outline => "outline",
            Self::Secondary => "secondary",
            Self::Ghost => "ghost",
            Self::Link => "link"
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
                    base.opacity(if engaged { 0.9 } else { 1. }), // background
                    foreground,
                    base, // border
                    base.opacity(0.24), // shadow
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
                        c.danger.opacity(0.5)
                    } else if theme.is_dark() {
                        c.border.opacity(0.64)
                    } else {
                        c.hover.opacity(0.5)
                    }
                } else {
                    base
                };
                (
                    bg, // background
                    if destructive { c.danger } else { c.fg }, // foreground
                    if destructive && engaged {
                        c.danger_subtle.lighten(0.5)
                    } else {
                        c.border.lighten(0.6)
                    }, // border
                    c.shadow.opacity(0.5), // shadow
                    hsla(
                        0.,
                        0.,
                        if theme.is_dark() { 1. } else { 0. },
                        if theme.is_dark() { 0.06 } else { 0.04 },
                    ), // bevel
                )
            }
            Self::Secondary => (
                c.sunken.opacity(if pressed {
                    0.8
                } else if hover {
                    0.9
                } else {
                    1.
                }), // background
                c.fg, // foreground
                c.sunken, // border
                clear, // shadow
                clear, // bevel
            ),
            Self::Ghost => (
                if engaged { c.hover } else { clear }, // background
                c.fg, // foreground
                clear, // border
                clear, // shadow
                clear, // bevel
            ),
            Self::Link => (clear, c.fg, clear, clear, clear)
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
            background: self.background.mix(target.background, progress),
            foreground: self.foreground.mix(target.foreground, progress),
            border: self.border.mix(target.border, progress),
            shadow: self.shadow.mix(target.shadow, progress),
            bevel: self.bevel.mix(target.bevel, progress),
        }
    }
}
