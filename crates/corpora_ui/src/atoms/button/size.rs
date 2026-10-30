/// Text and square icon sizes from the Button reference.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ButtonSize {
    Xs,
    Sm,
    #[default]
    Default,
    Lg,
    Xl,
    IconXs,
    IconSm,
    Icon,
    IconLg,
    IconXl,
}

impl ButtonSize {
    pub const ALL: [Self; 10] = [
        Self::Xs,
        Self::Sm,
        Self::Default,
        Self::Lg,
        Self::Xl,
        Self::IconXs,
        Self::IconSm,
        Self::Icon,
        Self::IconLg,
        Self::IconXl,
    ];

    pub fn name(self) -> &'static str {
        match self {
            Self::Xs => "xs",
            Self::Sm => "sm",
            Self::Default => "default",
            Self::Lg => "lg",
            Self::Xl => "xl",
            Self::IconXs => "icon-xs",
            Self::IconSm => "icon-sm",
            Self::Icon => "icon",
            Self::IconLg => "icon-lg",
            Self::IconXl => "icon-xl",
        }
    }

    pub fn is_icon(self) -> bool {
        matches!(
            self,
            Self::IconXs | Self::IconSm | Self::Icon | Self::IconLg | Self::IconXl
        )
    }

    pub(super) fn metrics(self, compact: bool) -> Metrics {
        let (height, padding, gap) = match self {
            Self::Xs => (28., 7., 4.),
            Self::Sm => (32., 9., 6.),
            Self::Default => (36., 11., 8.),
            Self::Lg => (40., 13., 8.),
            Self::Xl => (44., 15., 8.),
            Self::IconXs | Self::IconSm => (28., 0., 0.),
            Self::Icon => (36., 0., 0.),
            Self::IconLg => (32., 0., 0.),
            Self::IconXl => (40., 0., 0.),
        };
        let font = match self {
            Self::Xs => {
                if compact {
                    12.
                } else {
                    14.
                }
            }
            Self::Xl => {
                if compact {
                    16.
                } else {
                    18.
                }
            }
            _ => {
                if compact {
                    14.
                } else {
                    16.
                }
            }
        };
        let icon = match self {
            Self::Xs | Self::IconXs => {
                if compact {
                    14.
                } else {
                    16.
                }
            }
            Self::Xl | Self::IconXl => {
                if compact {
                    18.
                } else {
                    20.
                }
            }
            _ => {
                if compact {
                    16.
                } else {
                    18.
                }
            }
        };
        Metrics {
            height: height
                - if compact && self != Self::IconLg {
                    4.
                } else {
                    0.
                },
            padding,
            gap,
            font,
            icon,
        }
    }
}

pub(super) struct Metrics {
    pub height: f32,
    pub padding: f32,
    pub gap: f32,
    pub font: f32,
    pub icon: f32,
}
