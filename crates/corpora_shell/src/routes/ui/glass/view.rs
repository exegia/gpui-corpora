use std::{cell::Cell, rc::Rc};

use corpora_ui::{
    atoms::button::{Button, ButtonSize, ButtonVariant},
    ely::{
        primitives::IconName,
        theme::{ActiveTheme, Radius},
        typography::{Heading, Paragraph},
    },
    ui::material::GlassSurfaceExt as _,
};
use gpui::{
    Context, DragMoveEvent, ElementId, Empty, IntoElement, Pixels, Point, Render, Window, div,
    point, prelude::*, px, rems,
};
use gpui_kit::{
    base::{StyledExt as _, TestSupportExt as _},
    component::{slider::Slider, switch::Switch},
};

use super::{
    GlassShowcase, SettingSection,
    components::{backdrop, choice, collapsible_control_section, color_choice, control_section},
    model::{Backdrop, Preset, ProbeControl, ThemeColor},
};

fn section_trigger(
    section: SettingSection,
    title: &'static str,
    open: bool,
    cx: &mut Context<GlassShowcase>,
) -> Button {
    Button::new(
        (ElementId::from("glass-section"), section.id()),
        format!("Toggle {title} settings"),
    )
    .child(title)
    .icon(if open {
        IconName::ChevronDown
    } else {
        IconName::ChevronRight
    })
    .size(ButtonSize::Sm)
    .variant(ButtonVariant::Ghost)
    .w_full()
    .justify_start()
    .on_click(cx.listener(move |this, _, _, cx| this.toggle_section(section, cx)))
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
            .h(px(400.))
            .w_full()
            .overflow_hidden()
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
            .child(probe)
            .border_1()
            .border_color(colors.border)
            .overflow_hidden()
            .rounded(cx.theme().radius(Radius::Lg));

        let parameter_row = |control: ProbeControl, disabled: bool| {
            div()
                .id((ElementId::from("glass-control"), control.id()))
                .w_full()
                .flex()
                .items_center()
                .gap_4()
                .py_2()
                .child(
                    div()
                        .w(rems(18.))
                        .flex_none()
                        .flex()
                        .flex_col()
                        .gap_1()
                        .child(div().text_sm().font_medium().child(control.label()))
                        .child(
                            div()
                                .text_xs()
                                .text_color(colors.fg_muted)
                                .child(control.description()),
                        ),
                )
                .child(
                    div()
                        .id((ElementId::from("glass-slider"), control.id()))
                        .test_support()
                        .min_w_0()
                        .flex_1()
                        .child(
                            Slider::new(self.sliders.get(control))
                                .disabled(disabled)
                                .w_full(),
                        ),
                )
                .child(
                    div()
                        .w(rems(5.))
                        .flex_none()
                        .flex()
                        .justify_end()
                        .text_sm()
                        .font_semibold()
                        .child(self.formatted_value(control)),
                )
        };

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
        let preset_section = control_section(
            "Preset",
            "Starts with a useful glass recipe, or resets every control to its original value.",
            preset_controls,
            cx,
        );

        let size_open = self.section_is_open(SettingSection::Size);
        let size_section = collapsible_control_section(
            section_trigger(SettingSection::Size, "Size", size_open, cx),
            "Changes the glass box itself without changing how the material behaves.",
            size_open,
            div()
                .w_full()
                .flex()
                .flex_col()
                .children(ProbeControl::SIZE.map(|control| parameter_row(control, false))),
            cx,
        );

        let optics_open = self.section_is_open(SettingSection::Optics);
        let optics_section = collapsible_control_section(
            section_trigger(SettingSection::Optics, "Optics", optics_open, cx),
            "Changes what the picture and colors behind the glass look like.",
            optics_open,
            div()
                .w_full()
                .flex()
                .flex_col()
                .children(ProbeControl::OPTICS.map(|control| parameter_row(control, false))),
            cx,
        );

        let edge_open = self.section_is_open(SettingSection::Edge);
        let edge_section = collapsible_control_section(
            section_trigger(SettingSection::Edge, "Edge", edge_open, cx),
            "Changes the bright rim that makes the glass shape easy to see.",
            edge_open,
            div()
                .w_full()
                .flex()
                .flex_col()
                .children(ProbeControl::EDGE.map(|control| parameter_row(control, false))),
            cx,
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
        let tint_disabled = !self.tint_enabled;
        let tint_picker = div()
            .flex()
            .flex_col()
            .gap_2()
            .child(div().text_sm().font_medium().child("Tint color"))
            .child(
                div()
                    .text_xs()
                    .text_color(colors.fg_muted)
                    .child("Picks the theme color mixed into the glass."),
            )
            .child(
                div()
                    .flex()
                    .flex_wrap()
                    .gap_1()
                    .children(ThemeColor::ALL.map(|color| {
                        color_choice(
                            "glass-tint-color",
                            color,
                            self.tint_color == color,
                            tint_disabled,
                            cx,
                        )
                        .on_click(cx.listener(move |this, _, _, cx| {
                            if this.tint_enabled {
                                this.tint_color = color;
                                cx.notify();
                            }
                        }))
                    })),
            );
        let tint_open = self.section_is_open(SettingSection::Tint);
        let tint_section = collapsible_control_section(
            section_trigger(SettingSection::Tint, "Tint", tint_open, cx),
            "Adds a theme color to the glass; turn it off for clear glass.",
            tint_open,
            div()
                .w_full()
                .flex()
                .flex_col()
                .gap_3()
                .child(
                    div()
                        .flex()
                        .items_center()
                        .justify_between()
                        .child(
                            div()
                                .flex()
                                .flex_col()
                                .gap_1()
                                .child(div().text_sm().font_medium().child("Use tint"))
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(colors.fg_muted)
                                        .child("Turns the colored glass layer on or off."),
                                ),
                        )
                        .child(tint_switch),
                )
                .child(tint_picker)
                .child(parameter_row(ProbeControl::TintFill, tint_disabled)),
            cx,
        );

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
        let background_picker = div()
            .flex()
            .flex_col()
            .gap_2()
            .child(div().text_sm().font_medium().child("Background color"))
            .child(
                div()
                    .text_xs()
                    .text_color(colors.fg_muted)
                    .child("Picks a theme color to place behind the glass."),
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
                            false,
                            cx,
                        )
                        .on_click(cx.listener(move |this, _, _, cx| {
                            this.background_color = color;
                            cx.notify();
                        }))
                    })),
            );
        let backdrop_open = self.section_is_open(SettingSection::Backdrop);
        let backdrop_section = collapsible_control_section(
            section_trigger(SettingSection::Backdrop, "Backdrop", backdrop_open, cx),
            "Changes the picture behind the glass so each material effect is easier to spot.",
            backdrop_open,
            div()
                .w_full()
                .flex()
                .flex_col()
                .gap_3()
                .child(backdrop_controls)
                .child(background_picker),
            cx,
        );

        let code = self.code_snippet();
        let copy_code = code.clone();
        let code_section = control_section(
            "Rust configuration",
            "Copy this snippet to recreate the glass settings currently shown in the probe.",
            div()
                .relative()
                .w_full()
                .overflow_hidden()
                .rounded(cx.theme().radius(Radius::Lg))
                .border_1()
                .border_color(colors.border)
                .bg(colors.sunken)
                .child(
                    div().absolute().top_2().right_2().child(
                        Button::new("glass-copy-code", "Copy glass configuration")
                            .child("Copy")
                            .size(ButtonSize::Sm)
                            .variant(ButtonVariant::Outline)
                            .on_click(move |_, _, cx| {
                                let _ = crate::desktop_actions::copy_text(&copy_code, cx);
                            }),
                    ),
                )
                .child(
                    div()
                        .id("glass-code")
                        .test_support()
                        .p_4()
                        .pr_16()
                        .text_xs()
                        .text_color(colors.fg)
                        .child(code),
                ),
            cx,
        );

        div()
            .id("glass-showcase")
            .w_full()
            .min_w_0()
            .flex()
            .flex_col()
            .gap_8()
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
            .child(super::super::example(
                "Material",
                "Text and broad edges make displacement visible; repeating narrow stripes can conceal refraction.",
                div()
                    .flex()
                    .flex_col()
                    .gap_5()
                    .child(stage)
                    .child(preset_section)
                    .child(size_section)
                    .child(optics_section)
                    .child(edge_section)
                    .child(tint_section)
                    .child(backdrop_section)
                    .child(code_section),
                cx,
            ))
    }
}
