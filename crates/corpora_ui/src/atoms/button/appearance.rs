use crate::ely::theme::{Mix, Palette, Theme};
use crate::styles::utils::ColorExt as _;
use gpui::{Hsla, transparent_black};

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
            Self::Default | Self::Secondary | Self::Destructive => Colors {
                background: c.accent,
                foreground: c.accent.darken(1.0),
                ..base
            },
            Self::Outline | Self::DestructiveOutline => Colors {
                background: c.bg.lighten(0.5),
                foreground: c.fg.darken(1.0),
                ..base
            },
            Self::Ghost => Colors {
                background: transparent_black(),
                foreground: c.fg.darken(1.0),
                ..base
            },
            Self::Link => Colors {
                foreground: c.accent.darken(1.0),
                ..base
            },
        }
    }

    fn dark(self, c: &Palette) -> Colors {
        let base = Colors::base(c);
        match self {
            Self::Default | Self::Secondary => Colors {
                background: c.accent,
                foreground: c.accent.darken(0.9),
                border: c.accent.lighten(0.1),
                ..base
            },
            Self::Destructive => Colors {
                background: c.accent,
                foreground: c.accent.darken(0.9),
                ..base
            },
            Self::Outline | Self::DestructiveOutline | Self::Ghost => Colors {
                border: c.accent,
                foreground: c.accent.darken(0.9),
                ..base
            },
            Self::Link => Colors {
                foreground: c.accent.darken(0.9),
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
        Self {
            background: self.background.opacity(fade),
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
        }
    }
}
