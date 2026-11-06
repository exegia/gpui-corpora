use corpora_ui::{ely::theme::ActiveTheme, ui::material::GlassSpec};
use gpui::{App, AppContext as _, Context, Entity, Hsla, px};
use gpui_kit::component::slider::SliderState;

use super::GlassShowcase;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Backdrop {
    Flat,
    Bars,
    HorizontalRuler,
    VerticalRuler,
    Gradient,
    Text,
    Coded,
}

impl Backdrop {
    pub(super) const ALL: [Self; 7] = [
        Self::Flat,
        Self::Bars,
        Self::HorizontalRuler,
        Self::VerticalRuler,
        Self::Gradient,
        Self::Text,
        Self::Coded,
    ];

    pub(super) const fn id(self) -> &'static str {
        match self {
            Self::Flat => "flat",
            Self::Bars => "bars",
            Self::HorizontalRuler => "h-ruler",
            Self::VerticalRuler => "v-ruler",
            Self::Gradient => "gradient",
            Self::Text => "text",
            Self::Coded => "coded",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Preset {
    Blur,
    Glass,
    Deep,
    Custom,
}

impl Preset {
    pub(super) const ALL: [Self; 3] = [Self::Blur, Self::Glass, Self::Deep];

    pub(super) const fn name(self) -> &'static str {
        match self {
            Self::Blur => "blur",
            Self::Glass => "glass",
            Self::Deep => "deep",
            Self::Custom => "custom",
        }
    }

    pub(super) fn spec(self) -> GlassSpec {
        match self {
            Self::Blur => GlassSpec {
                lens: px(0.),
                reach: px(0.),
                magnify: 0.,
                dispersion: 0.,
                edge: 0.,
                edge_width: px(0.),
                ..GlassSpec::default()
            },
            Self::Deep => GlassSpec {
                magnify: 1.38,
                dispersion: 0.02,
                edge: 0.34,
                edge_width: px(2.),
                ..GlassSpec::default()
            },
            Self::Glass | Self::Custom => GlassSpec::default(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum ThemeColor {
    Background,
    Surface,
    Elevated,
    Sunken,
    Accent,
    Success,
    Warning,
    Danger,
    Info,
    OnMedia,
    Paper,
    Ink,
}

impl ThemeColor {
    pub(super) const ALL: [Self; 12] = [
        Self::Background,
        Self::Surface,
        Self::Elevated,
        Self::Sunken,
        Self::Accent,
        Self::Success,
        Self::Warning,
        Self::Danger,
        Self::Info,
        Self::OnMedia,
        Self::Paper,
        Self::Ink,
    ];

    pub(super) const fn id(self) -> &'static str {
        match self {
            Self::Background => "background",
            Self::Surface => "surface",
            Self::Elevated => "elevated",
            Self::Sunken => "sunken",
            Self::Accent => "accent",
            Self::Success => "success",
            Self::Warning => "warning",
            Self::Danger => "danger",
            Self::Info => "info",
            Self::OnMedia => "on-media",
            Self::Paper => "paper",
            Self::Ink => "ink",
        }
    }

    pub(super) const fn code_field(self) -> &'static str {
        match self {
            Self::Background => "bg",
            Self::Surface => "surface",
            Self::Elevated => "overlay",
            Self::Sunken => "sunken",
            Self::Accent => "accent",
            Self::Success => "success",
            Self::Warning => "warning",
            Self::Danger => "danger",
            Self::Info => "info",
            Self::OnMedia => "on_media",
            Self::Paper => "paper",
            Self::Ink => "ink",
        }
    }

    pub(super) fn value(self, cx: &App) -> Hsla {
        let colors = &cx.theme().colors;
        match self {
            Self::Background => colors.bg,
            Self::Surface => colors.surface,
            Self::Elevated => colors.overlay,
            Self::Sunken => colors.sunken,
            Self::Accent => colors.accent,
            Self::Success => colors.success,
            Self::Warning => colors.warning,
            Self::Danger => colors.danger,
            Self::Info => colors.info,
            Self::OnMedia => colors.on_media,
            Self::Paper => colors.paper,
            Self::Ink => colors.ink,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(usize)]
pub(super) enum ProbeControl {
    Width,
    Height,
    Radius,
    Blur,
    Lens,
    Magnify,
    Gain,
    Saturation,
    Lift,
    Edge,
    EdgeWidth,
    Dispersion,
    TintFill,
}

impl ProbeControl {
    pub(super) const ALL: [Self; 13] = [
        Self::Width,
        Self::Height,
        Self::Radius,
        Self::Blur,
        Self::Lens,
        Self::Magnify,
        Self::Gain,
        Self::Saturation,
        Self::Lift,
        Self::Edge,
        Self::EdgeWidth,
        Self::Dispersion,
        Self::TintFill,
    ];
    pub(super) const SIZE: [Self; 3] = [Self::Width, Self::Height, Self::Radius];
    pub(super) const OPTICS: [Self; 7] = [
        Self::Blur,
        Self::Lens,
        Self::Magnify,
        Self::Gain,
        Self::Saturation,
        Self::Dispersion,
        Self::Lift,
    ];
    pub(super) const EDGE: [Self; 2] = [Self::Edge, Self::EdgeWidth];

    pub(super) const fn id(self) -> &'static str {
        match self {
            Self::Width => "width",
            Self::Height => "height",
            Self::Radius => "radius",
            Self::Blur => "blur",
            Self::Lens => "lens",
            Self::Magnify => "magnify",
            Self::Gain => "gain",
            Self::Saturation => "saturation",
            Self::Lift => "lift",
            Self::Edge => "edge",
            Self::EdgeWidth => "edge-width",
            Self::Dispersion => "dispersion",
            Self::TintFill => "tint-fill",
        }
    }

    pub(super) const fn label(self) -> &'static str {
        match self {
            Self::Width => "Width",
            Self::Height => "Height",
            Self::Radius => "Radius",
            Self::Blur => "Blur",
            Self::Lens => "Rim / lens",
            Self::Magnify => "Magnification",
            Self::Gain => "Gain",
            Self::Saturation => "Saturation",
            Self::Lift => "Lift",
            Self::Edge => "Edge",
            Self::EdgeWidth => "Edge width",
            Self::Dispersion => "Dispersion",
            Self::TintFill => "Tint fill",
        }
    }

    pub(super) const fn description(self) -> &'static str {
        match self {
            Self::Width => "Makes the glass box wider or narrower.",
            Self::Height => "Makes the glass box taller or shorter.",
            Self::Radius => "Rounds the glass corners more or less.",
            Self::Blur => "Makes things behind the glass look softer.",
            Self::Lens => "Changes how far the glass bends the picture near its rim.",
            Self::Magnify => "Makes the picture behind the glass look bigger or smaller.",
            Self::Gain => "Makes the light behind the glass brighter or darker.",
            Self::Saturation => "Makes the colors behind the glass stronger or duller.",
            Self::Lift => "Adds a soft white veil over the glass.",
            Self::Edge => "Changes how brightly the glass rim shines.",
            Self::EdgeWidth => "Makes the shiny rim thicker or thinner.",
            Self::Dispersion => "Splits colors near the rim like a tiny rainbow.",
            Self::TintFill => "Changes how much tint color is mixed into the glass.",
        }
    }

    const fn range(self) -> (f32, f32, f32) {
        match self {
            Self::Width => (120., 600., 1.),
            Self::Height => (30., 250., 1.),
            Self::Radius => (0., 125., 1.),
            Self::Blur => (0., 24., 0.1),
            Self::Lens => (0., 120., 0.5),
            Self::Magnify => (-16., 16., 0.01),
            Self::Gain => (0., 2., 0.001),
            Self::Saturation => (0., 2., 0.01),
            Self::Lift | Self::TintFill => (0., 1., 0.01),
            Self::Edge => (0., 1., 0.01),
            Self::EdgeWidth => (0., 12., 0.1),
            Self::Dispersion => (0., 0.2, 0.001),
        }
    }
}

pub(super) struct ProbeSliders {
    values: [Entity<SliderState>; 13],
}

impl ProbeSliders {
    pub(super) fn new(cx: &mut Context<GlassShowcase>) -> Self {
        Self {
            values: ProbeControl::ALL.map(|control| {
                let (min, max, step) = control.range();
                cx.new(|_| {
                    SliderState::new()
                        .max(max)
                        .min(min)
                        .step(step)
                        .default_value(initial_value(control))
                })
            }),
        }
    }

    pub(super) fn get(&self, control: ProbeControl) -> &Entity<SliderState> {
        &self.values[control as usize]
    }
}

fn initial_value(control: ProbeControl) -> f32 {
    let spec = GlassSpec::default();
    match control {
        ProbeControl::Width => 168.,
        ProbeControl::Height => 168.,
        ProbeControl::Radius => 34.,
        ProbeControl::Blur => spec.blur_radius.into(),
        ProbeControl::Lens => spec.lens.into(),
        ProbeControl::Magnify => spec.magnify,
        ProbeControl::Gain => spec.gain_light,
        ProbeControl::Saturation => spec.saturation,
        ProbeControl::Lift => 18.8 / 255.,
        ProbeControl::Edge => spec.edge,
        ProbeControl::EdgeWidth => spec.edge_width.into(),
        ProbeControl::Dispersion => spec.dispersion,
        ProbeControl::TintFill => 0.34,
    }
}
