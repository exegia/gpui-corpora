use std::{cell::Cell, rc::Rc};

use corpora_ui::{
    atoms::button::{Button, ButtonSize, ButtonVariant},
    ely::{
        theme::{ActiveTheme, Radius},
        typography::{Heading, Paragraph},
    },
    ui::material::{GlassSpec, GlassSurfaceExt as _},
};
use gpui::{
    App, Context, DragMoveEvent, ElementId, Empty, Entity, Hsla, IntoElement, Pixels, Point,
    Render, Subscription, Window, div, point, prelude::*, px, rems,
};
use gpui_kit::{
    base::{StyledExt as _, TestSupportExt as _},
    component::{
        slider::{Slider, SliderEvent, SliderState},
        switch::Switch,
    },
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Backdrop {
    Flat,
    Bars,
    HorizontalRuler,
    VerticalRuler,
    Gradient,
    Text,
    Coded,
}

impl Backdrop {
    const ALL: [Self; 7] = [
        Self::Flat,
        Self::Bars,
        Self::HorizontalRuler,
        Self::VerticalRuler,
        Self::Gradient,
        Self::Text,
        Self::Coded,
    ];

    const fn id(self) -> &'static str {
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
enum Preset {
    Blur,
    Glass,
    Deep,
    Custom,
}

impl Preset {
    const ALL: [Self; 3] = [Self::Blur, Self::Glass, Self::Deep];

    const fn name(self) -> &'static str {
        match self {
            Self::Blur => "blur",
            Self::Glass => "glass",
            Self::Deep => "deep",
            Self::Custom => "custom",
        }
    }

    fn spec(self) -> GlassSpec {
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
enum ThemeColor {
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
    const ALL: [Self; 12] = [
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

    const fn id(self) -> &'static str {
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

    fn value(self, cx: &App) -> Hsla {
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
enum ProbeControl {
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
    const ALL: [Self; 13] = [
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

    const fn id(self) -> &'static str {
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

    const fn label(self) -> &'static str {
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

struct ProbeSliders {
    values: [Entity<SliderState>; 13],
}

impl ProbeSliders {
    fn new(cx: &mut Context<GlassShowcase>) -> Self {
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

    fn get(&self, control: ProbeControl) -> &Entity<SliderState> {
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

struct DragProbe {
    start: Cell<Point<Pixels>>,
    origin: Point<Pixels>,
}

struct DragPreview;

impl Render for DragPreview {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        Empty
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
        self.spec = GlassSpec::default();
        self.sync_sliders(window, cx);
        self.preset = Preset::Glass;
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

fn choice(id: (&'static str, &'static str), label: &'static str, selected: bool) -> Button {
    Button::new((ElementId::from(id.0), id.1), label)
        .child(label)
        .size(ButtonSize::Sm)
        .variant(if selected {
            ButtonVariant::Secondary
        } else {
            ButtonVariant::Ghost
        })
}

fn color_choice(group: &'static str, color: ThemeColor, selected: bool, cx: &App) -> Button {
    Button::new(
        (ElementId::from(group), color.id()),
        format!("{} color {}", group.replace('-', " "), color.id()),
    )
    .size(ButtonSize::Sm)
    .variant(if selected {
        ButtonVariant::Secondary
    } else {
        ButtonVariant::Ghost
    })
    .child(
        div()
            .size_3()
            .flex_none()
            .rounded_full()
            .border_1()
            .border_color(cx.theme().colors.border_strong)
            .bg(color.value(cx)),
    )
    .child(color.id())
}

fn backdrop(kind: Backdrop, background: Hsla, cx: &App) -> impl IntoElement {
    let colors = cx.theme().colors.clone();
    let base = div().absolute().inset_0().overflow_hidden().bg(background);
    match kind {
        Backdrop::Flat => base,
        Backdrop::Bars => base.child(
            div()
                .absolute()
                .inset_0()
                .flex()
                .children((0..8).map(move |ix| div().flex_1().h_full().bg(colors.chart[ix]))),
        ),
        Backdrop::HorizontalRuler => base.children((0..15).map(move |ix| {
            div()
                .absolute()
                .top(px(ix as f32 * 24.))
                .left_0()
                .right_0()
                .h(px(if ix % 4 == 0 { 3. } else { 1. }))
                .bg(colors.fg.opacity(if ix % 4 == 0 { 0.8 } else { 0.35 }))
        })),
        Backdrop::VerticalRuler => base.children((0..35).map(move |ix| {
            div()
                .absolute()
                .left(px(ix as f32 * 24.))
                .top_0()
                .bottom_0()
                .w(px(if ix % 4 == 0 { 3. } else { 1. }))
                .bg(colors.fg.opacity(if ix % 4 == 0 { 0.8 } else { 0.35 }))
        })),
        Backdrop::Gradient => base.child(div().absolute().inset_0().flex().children((0..32).map(
            move |ix| {
                div()
                    .flex_1()
                    .h_full()
                    .bg(colors.accent.opacity(ix as f32 / 31.))
            },
        ))),
        Backdrop::Text => base.children((0..12).map(move |ix| {
            div()
                .absolute()
                .top(px(8. + ix as f32 * 28.))
                .left(px(12.))
                .right(px(12.))
                .text_2xl()
                .font_semibold()
                .text_color(colors.fg)
                .whitespace_nowrap()
                .overflow_hidden()
                .child("the quick brown fox jumps over the lazy dog and back again")
        })),
        Backdrop::Coded => base.children((0..70).map(move |ix| {
            div()
                .absolute()
                .left(px(ix as f32 * 12.))
                .top_0()
                .bottom_0()
                .w(px(12.))
                .bg(colors.chart[ix % colors.chart.len()]
                    .opacity(0.35 + 0.65 * (ix % 10) as f32 / 9.))
        })),
    }
}

impl Render for GlassShowcase {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let colors = cx.theme().colors.clone();
        let mut spec = self.spec;
        spec.tint = Some(if self.tint_enabled {
            self.tint_color.value(cx).opacity(self.tint_fill)
        } else {
            colors.on_media.opacity(self.lift)
        });

        let drag = Rc::new(DragProbe {
            start: Cell::new(point(px(0.), px(0.))),
            origin: self.position,
        });
        let probe = div()
            .id("glass-probe")
            .test_support()
            .absolute()
            .left(self.position.x)
            .top(self.position.y)
            .w(px(self.width))
            .h(px(self.height))
            .flex()
            .items_center()
            .justify_center()
            .rounded(px(self.radius.min(self.width.min(self.height) / 2.)))
            .border_1()
            .border_color(colors.border.opacity(0.7))
            .text_sm()
            .font_semibold()
            .text_color(colors.fg)
            .cursor_move()
            .child(if self.tint_enabled {
                "Tinted glass"
            } else {
                "Glass"
            })
            .on_drag(drag, |drag, _, window, cx| {
                drag.start.set(window.mouse_position());
                cx.stop_propagation();
                cx.new(|_| DragPreview)
            })
            .glass_with(spec);

        let stage = div()
            .id("glass-probe-stage")
            .test_support()
            .relative()
            .h(px(340.))
            .w_full()
            .overflow_hidden()
            .rounded(cx.theme().radius(Radius::Lg))
            .border_1()
            .border_color(colors.border)
            .on_drag_move::<Rc<DragProbe>>(cx.listener(
                |this, event: &DragMoveEvent<Rc<DragProbe>>, _, cx| {
                    let drag = event.drag(cx);
                    let delta = event.event.position - drag.start.get();
                    let max_x = (event.bounds.size.width - px(this.width)).max(px(0.));
                    let max_y = (event.bounds.size.height - px(this.height)).max(px(0.));
                    this.position = point(
                        (drag.origin.x + delta.x).clamp(px(0.), max_x),
                        (drag.origin.y + delta.y).clamp(px(0.), max_y),
                    );
                    cx.notify();
                },
            ))
            .child(backdrop(self.backdrop, self.background_color.value(cx), cx))
            .child(probe);

        let backdrop_controls =
            div()
                .flex()
                .flex_wrap()
                .items_center()
                .gap_1()
                .children(Backdrop::ALL.map(|backdrop| {
                    choice(
                        ("glass-backdrop", backdrop.id()),
                        backdrop.id(),
                        self.backdrop == backdrop,
                    )
                    .on_click(cx.listener(move |this, _, _, cx| {
                        this.backdrop = backdrop;
                        cx.notify();
                    }))
                }));

        let preset_controls = div()
            .flex()
            .flex_wrap()
            .items_center()
            .gap_1()
            .children(Preset::ALL.map(|preset| {
                choice(
                    ("glass-preset", preset.name()),
                    preset.name(),
                    self.preset == preset,
                )
                .on_click(cx.listener(move |this, _, window, cx| {
                    this.select_preset(preset, window, cx);
                    cx.notify();
                }))
            }))
            .child(
                Button::new("glass-reset", "Reset glass probe")
                    .child("reset")
                    .size(ButtonSize::Sm)
                    .variant(ButtonVariant::Ghost)
                    .on_click(cx.listener(|this, _, window, cx| {
                        this.reset(window, cx);
                        cx.notify();
                    })),
            );

        let tint_owner = cx.entity().downgrade();
        let tint_switch = Switch::new("glass-tint-switch")
            .checked(self.tint_enabled)
            .label("Tint")
            .on_change(move |checked, _, cx| {
                tint_owner
                    .update(cx, |this, cx| {
                        this.tint_enabled = *checked;
                        cx.notify();
                    })
                    .ok();
            });

        let tint_picker = div()
            .flex()
            .flex_col()
            .gap_2()
            .child(
                div()
                    .text_xs()
                    .text_color(colors.fg_muted)
                    .child("Tint color"),
            )
            .child(
                div()
                    .flex()
                    .flex_wrap()
                    .gap_1()
                    .children(ThemeColor::ALL.map(|color| {
                        color_choice("glass-tint-color", color, self.tint_color == color, cx)
                            .on_click(cx.listener(move |this, _, _, cx| {
                                this.tint_color = color;
                                cx.notify();
                            }))
                    })),
            );

        let background_picker = div()
            .flex()
            .flex_col()
            .gap_2()
            .child(
                div()
                    .text_xs()
                    .text_color(colors.fg_muted)
                    .child("Background color"),
            )
            .child(
                div()
                    .flex()
                    .flex_wrap()
                    .gap_1()
                    .children(ThemeColor::ALL.map(|color| {
                        color_choice(
                            "glass-background-color",
                            color,
                            self.background_color == color,
                            cx,
                        )
                        .on_click(cx.listener(move |this, _, _, cx| {
                            this.background_color = color;
                            cx.notify();
                        }))
                    })),
            );

        let parameter_controls = div()
            .flex()
            .flex_wrap()
            .gap_3()
            .children(ProbeControl::ALL.map(|control| {
                div()
                    .id((ElementId::from("glass-control"), control.id()))
                    .w(rems(17.))
                    .flex()
                    .flex_col()
                    .gap_2()
                    .p_2()
                    .rounded(cx.theme().radius(Radius::Md))
                    .border_1()
                    .border_color(colors.border)
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .justify_between()
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(colors.fg_muted)
                                    .child(control.label()),
                            )
                            .child(
                                div()
                                    .text_sm()
                                    .font_semibold()
                                    .child(self.formatted_value(control)),
                            ),
                    )
                    .child(
                        div()
                            .id((ElementId::from("glass-slider"), control.id()))
                            .test_support()
                            .w_full()
                            .child(Slider::new(self.sliders.get(control)).w_full()),
                    )
            }));

        div()
            .id("glass-showcase")
            .w_full()
            .min_w_0()
            .flex()
            .flex_col()
            .gap_8()
            .child(super::component_navigation(super::Showcase::Glass))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .child(Heading::h1("Glass material probe"))
                    .child(Paragraph::new(
                        "Drag the glass, switch the backdrop, and tune the material to expose blur, refraction, edge light, and tint behavior.",
                    )),
            )
            .child(super::example(
                "Material",
                "Text and broad edges make displacement visible; repeating narrow stripes can conceal refraction.",
                div()
                    .flex()
                    .flex_col()
                    .gap_5()
                    .child(stage)
                    .child(backdrop_controls)
                    .child(preset_controls)
                    .child(tint_switch)
                    .child(tint_picker)
                    .child(background_picker)
                    .child(parameter_controls),
                cx,
            ))
    }
}

#[cfg(test)]
#[path = "glass.test.rs"]
mod tests;
