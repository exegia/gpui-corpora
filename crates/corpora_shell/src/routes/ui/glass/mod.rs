use corpora_ui::ui::material::GlassSpec;
use gpui::{Context, Pixels, Point, Subscription, Window, point, px};
use gpui_kit::component::slider::SliderEvent;

mod components;
mod model;
mod view;

use model::{Backdrop, Preset, ProbeControl, ProbeSliders, ThemeColor};

#[derive(Clone, Copy)]
#[repr(usize)]
pub(super) enum SettingSection {
    Size,
    Optics,
    Edge,
    Tint,
    Backdrop,
}

impl SettingSection {
    pub(super) const fn id(self) -> &'static str {
        match self {
            Self::Size => "size",
            Self::Optics => "optics",
            Self::Edge => "edge",
            Self::Tint => "tint",
            Self::Backdrop => "backdrop",
        }
    }
}

/// Interactive glass material probe retained by the workspace tab.
pub struct GlassShowcase {
    backdrop: Backdrop,
    preset: Preset,
    background_color: ThemeColor,
    tint_color: ThemeColor,
    position: Point<Pixels>,
    width: f32,
    height: f32,
    radius: f32,
    lift: f32,
    tint_enabled: bool,
    tint_fill: f32,
    sections_open: [bool; 5],
    spec: GlassSpec,
    sliders: ProbeSliders,
    _subscriptions: Vec<Subscription>,
}

impl GlassShowcase {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let sliders = ProbeSliders::new(cx);
        let subscriptions = ProbeControl::ALL
            .into_iter()
            .map(|control| {
                cx.subscribe(sliders.get(control), move |this, _, event, cx| {
                    if let SliderEvent::Change(value) = event {
                        this.set_control(control, value.start());
                        cx.notify();
                    }
                })
            })
            .collect();
        Self {
            backdrop: Backdrop::Text,
            preset: Preset::Glass,
            background_color: ThemeColor::Background,
            tint_color: ThemeColor::Accent,
            position: point(px(120.), px(80.)),
            width: 168.,
            height: 168.,
            radius: 34.,
            lift: 18.8 / 255.,
            tint_enabled: false,
            tint_fill: 0.34,
            sections_open: [false; 5],
            spec: GlassSpec::default(),
            sliders,
            _subscriptions: subscriptions,
        }
    }

    fn reset(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.backdrop = Backdrop::Text;
        self.preset = Preset::Glass;
        self.background_color = ThemeColor::Background;
        self.tint_color = ThemeColor::Accent;
        self.position = point(px(120.), px(80.));
        self.width = 168.;
        self.height = 168.;
        self.radius = 34.;
        self.lift = 18.8 / 255.;
        self.tint_enabled = false;
        self.tint_fill = 0.34;
        self.sections_open = [false; 5];
        self.spec = GlassSpec::default();
        self.sync_sliders(window, cx);
        self.preset = Preset::Glass;
    }

    fn toggle_section(&mut self, section: SettingSection, cx: &mut Context<Self>) {
        let open = &mut self.sections_open[section as usize];
        *open = !*open;
        cx.notify();
    }

    fn section_is_open(&self, section: SettingSection) -> bool {
        self.sections_open[section as usize]
    }

    fn select_preset(&mut self, preset: Preset, window: &mut Window, cx: &mut Context<Self>) {
        self.spec = preset.spec();
        self.sync_sliders(window, cx);
        self.preset = preset;
    }

    fn set_control(&mut self, control: ProbeControl, value: f32) {
        self.preset = Preset::Custom;
        match control {
            ProbeControl::Width => self.width = value,
            ProbeControl::Height => self.height = value,
            ProbeControl::Radius => self.radius = value.min(self.width.min(self.height) / 2.),
            ProbeControl::Blur => self.spec.blur_radius = px(value),
            ProbeControl::Lens => self.spec.lens = px(value),
            ProbeControl::Magnify => self.spec.magnify = value,
            ProbeControl::Gain => {
                self.spec.gain_light = value;
                self.spec.gain_dark = value;
            }
            ProbeControl::Saturation => self.spec.saturation = value,
            ProbeControl::Lift => self.lift = value,
            ProbeControl::Edge => self.spec.edge = value,
            ProbeControl::EdgeWidth => self.spec.edge_width = px(value),
            ProbeControl::Dispersion => self.spec.dispersion = value,
            ProbeControl::TintFill => self.tint_fill = value,
        }
        self.radius = self.radius.min(self.width.min(self.height) / 2.);
    }

    fn control_value(&self, control: ProbeControl) -> f32 {
        match control {
            ProbeControl::Width => self.width,
            ProbeControl::Height => self.height,
            ProbeControl::Radius => self.radius,
            ProbeControl::Blur => self.spec.blur_radius.into(),
            ProbeControl::Lens => self.spec.lens.into(),
            ProbeControl::Magnify => self.spec.magnify,
            ProbeControl::Gain => self.spec.gain_light,
            ProbeControl::Saturation => self.spec.saturation,
            ProbeControl::Lift => self.lift,
            ProbeControl::Edge => self.spec.edge,
            ProbeControl::EdgeWidth => self.spec.edge_width.into(),
            ProbeControl::Dispersion => self.spec.dispersion,
            ProbeControl::TintFill => self.tint_fill,
        }
    }

    fn sync_sliders(&self, window: &mut Window, cx: &mut Context<Self>) {
        for control in ProbeControl::ALL {
            let value = self.control_value(control);
            self.sliders
                .get(control)
                .update(cx, |slider, cx| slider.set_value(value, window, cx));
        }
    }

    fn code_snippet(&self) -> String {
        let (tint_field, tint_opacity) = if self.tint_enabled {
            (self.tint_color.code_field(), self.tint_fill)
        } else {
            (ThemeColor::OnMedia.code_field(), self.lift)
        };
        format!(
            "let tint = cx.theme().colors.{tint_field}.opacity({tint_opacity:.3});\n\
             let spec: GlassSpec = GlassSpec {{\n\
             \x20   blur_radius: px({:.1}),\n\
             \x20   lens: px({:.1}),\n\
             \x20   magnify: {:.2},\n\
             \x20   dispersion: {:.3},\n\
             \x20   gain_light: {:.3},\n\
             \x20   gain_dark: {:.3},\n\
             \x20   saturation: {:.2},\n\
             \x20   tint: Some(tint),\n\
             \x20   edge: {:.2},\n\
             \x20   edge_width: px({:.1}),\n\
             \x20   edge_aa: px({:.1}),\n\
             \x20   ..GlassSpec::default()\n\
             }};",
            f32::from(self.spec.blur_radius),
            f32::from(self.spec.lens),
            self.spec.magnify,
            self.spec.dispersion,
            self.spec.gain_light,
            self.spec.gain_dark,
            self.spec.saturation,
            self.spec.edge,
            f32::from(self.spec.edge_width),
            f32::from(self.spec.edge_aa),
        )
    }

    fn formatted_value(&self, control: ProbeControl) -> String {
        let value = self.control_value(control);
        match control {
            ProbeControl::Width | ProbeControl::Height | ProbeControl::Radius => {
                format!("{value:.0} pt")
            }
            ProbeControl::Blur | ProbeControl::Lens | ProbeControl::EdgeWidth => {
                format!("{value:.1} pt")
            }
            ProbeControl::Magnify => format!("{value:+.2}"),
            ProbeControl::Gain | ProbeControl::Dispersion => format!("{value:.3}"),
            ProbeControl::Lift => format!("{:.0}/255", value * 255.),
            ProbeControl::Saturation | ProbeControl::Edge | ProbeControl::TintFill => {
                format!("{value:.2}")
            }
        }
    }
}

#[cfg(test)]
#[path = "glass.test.rs"]
mod tests;
